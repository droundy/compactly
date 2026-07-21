use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
use crate::{Decimal, Small};

#[cfg(test)]
use expect_test::expect;

/// `10^n` as `f64`, indexed by the non-negative magnitude of a decimal power.
///
/// Sized to cover every `i8` magnitude: `power.unsigned_abs()` reaches 128 for
/// `i8::MIN` (a value only a corrupt stream produces, but decode must not
/// panic on it), and the search loops only index up to `i8::MAX` = 127.
///
/// A decimal value is reconstructed as `mantissa * 10^n` or `mantissa / 10^n`,
/// so `to_decimal`'s search and `decimal_value` (encode's round-trip checker
/// and decode) all read this one table and therefore always agree — any pair
/// the search accepts decodes back to the same bits. Built at compile time by
/// repeated multiplication (exact for `n <= 22`, carrying the usual sub-ULP
/// `f64` rounding above that), so a lookup is a single array index rather than
/// a `powi` call. Divisor duty for negative powers is what keeps quantized
/// decimals exact: `mantissa / 10^n` recovers a value produced as
/// `k / 10^n` (e.g. a parsed price), where multiplying by a rounded `10^-n`
/// would not.
const POW10: [f64; 129] = {
    let mut table = [1.0f64; 129];
    let mut i = 1;
    while i < table.len() {
        table[i] = table[i - 1] * 10.0;
        i += 1;
    }
    table
};

macro_rules! impl_float {
    ($t:ident, $intty:ident, $bits:literal, $mod:ident) => {
        mod $mod {
            use super::*;

            // The default float encoding classifies each value into one of
            // three tiers behind two *saturating* selector bits:
            //   is_raw = true               -> raw (bits stored incompressibly)
            //   is_raw = false, is_int=false-> short decimal `mantissa·10^power`
            //   is_raw = false, is_int=true -> large integer, `Small<i64>`
            // The decimal tier is tried first (via `to_decimal`), so round and
            // small integers *fold* their trailing zeros into the power and
            // share a small, highly-compressible mantissa (`5000 -> (5,3)`) —
            // this is what lets the default subsume the `Decimal` strategy. The
            // integer tier catches only whole numbers too large for a decimal
            // `i32` mantissa. Each selector uses `BitContext::SATURATED_TRUE`:
            // once it has only ever been `true` up to the adaptation cap, both
            // sides skip coding it and commit to that branch (a value that
            // needs the other branch then falls back to raw, still exact). So an
            // all-raw column pays no per-value selector and decodes as a bare
            // memcpy. Only genuinely high-entropy mantissas and out-of-range
            // magnitudes reach the raw tier.
            #[derive(Clone, Default)]
            pub struct FloatContext {
                is_raw: <bool as Encode>::Context,
                is_int: <bool as Encode>::Context,
                integer: <Small as EncodingStrategy<i64>>::Context,
                mantissa: <Small as EncodingStrategy<i32>>::Context,
                exponent: <Small as EncodingStrategy<i8>>::Context,
            }

            impl Encode for $t {
                type Context = FloatContext;
                #[inline]
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    use super::super::bit_context::BitContext;
                    let intvalue = *self as i64;
                    // Decimal first, so round/small integers *fold* their
                    // trailing zeros into the power (`5000 -> (5,3)`) and share a
                    // small, compressible mantissa. `Small<i64>` (`is_int`) is
                    // only for large integers `to_decimal` can't fold to `i32`.
                    let decimal = to_decimal(*self);
                    let big_int = if decimal.is_none() && intvalue as $t == *self {
                        Some(intvalue)
                    } else {
                        None
                    };

                    if ctx.is_raw == BitContext::SATURATED_TRUE {
                        writer.encode_incompressible_bytes(&self.to_le_bytes());
                        return;
                    }
                    // `is_int` saturated: committed to the large-integer branch;
                    // anything else falls back to raw.
                    if ctx.is_int == BitContext::SATURATED_TRUE {
                        if let Some(iv) = big_int {
                            false.encode(writer, &mut ctx.is_raw);
                            <Small as EncodingStrategy<i64>>::encode(&iv, writer, &mut ctx.integer);
                        } else {
                            true.encode(writer, &mut ctx.is_raw);
                            writer.encode_incompressible_bytes(&self.to_le_bytes());
                        }
                        return;
                    }
                    let is_raw = decimal.is_none() && big_int.is_none();
                    is_raw.encode(writer, &mut ctx.is_raw);
                    if is_raw {
                        writer.encode_incompressible_bytes(&self.to_le_bytes());
                        return;
                    }
                    let is_int = big_int.is_some();
                    is_int.encode(writer, &mut ctx.is_int);
                    if let Some(iv) = big_int {
                        <Small as EncodingStrategy<i64>>::encode(&iv, writer, &mut ctx.integer);
                    } else {
                        let (mantissa, power) = decimal.unwrap();
                        <Small as EncodingStrategy<i32>>::encode(
                            &mantissa,
                            writer,
                            &mut ctx.mantissa,
                        );
                        <Small as EncodingStrategy<i8>>::encode(&power, writer, &mut ctx.exponent);
                    }
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    use super::super::bit_context::BitContext;
                    let raw = ctx.is_raw == BitContext::SATURATED_TRUE
                        || bool::decode(reader, &mut ctx.is_raw)?;
                    if raw {
                        let mut bytes = [0u8; $bits / 8];
                        reader.decode_incompressible_bytes(&mut bytes)?;
                        return Ok($t::from_le_bytes(bytes));
                    }
                    let is_int = ctx.is_int == BitContext::SATURATED_TRUE
                        || bool::decode(reader, &mut ctx.is_int)?;
                    if is_int {
                        let intvalue =
                            <Small as EncodingStrategy<i64>>::decode(reader, &mut ctx.integer)?;
                        Ok(intvalue as $t)
                    } else {
                        let mantissa =
                            <Small as EncodingStrategy<i32>>::decode(reader, &mut ctx.mantissa)?;
                        let power =
                            <Small as EncodingStrategy<i8>>::decode(reader, &mut ctx.exponent)?;
                        Ok(decimal_value(mantissa, power))
                    }
                }
            }

            // `value = ±mantissa · 10^power`, computed in f64 and narrowed.
            // Negative powers divide (`mantissa / 10^|power|`) rather than
            // multiply by a rounded `10^-|power|`, which is what lets a value
            // produced as `k / 10^n` round-trip exactly. `to_decimal`'s search
            // verifies candidates against this same function (encode's
            // round-trip check and decode both call it), so they always agree.
            fn decimal_value(mantissa: i32, power: i8) -> $t {
                let scale = POW10[power.unsigned_abs() as usize];
                let v = if power < 0 {
                    mantissa as f64 / scale
                } else {
                    mantissa as f64 * scale
                };
                v as $t
            }
            // Express `value` as `mantissa · 10^power` with an `i32` mantissa
            // and `i8` power, or `None` for the incompressible path. Trailing
            // factors of ten are folded into the power so that a run of
            // multiples of `10^k` shares a small, highly-compressible mantissa
            // (e.g. `1_000, 2_000, 1_000_000` become `(1,3), (2,3), (1,6)`).
            fn to_decimal(value: $t) -> Option<(i32, i8)> {
                let as_f64 = value as f64;

                // Integral values that fit `i64`: strip trailing zeros with
                // exact integer division — no float rounding, no ±1 probing.
                // A large non-round integer (mantissa still `> i32` once the
                // zeros are gone) returns `None` for the incompressible path.
                let as_i64 = as_f64 as i64;
                if as_i64 as f64 == as_f64 {
                    let mut mantissa = as_i64;
                    let mut power = 0i8;
                    while mantissa % 10 == 0 && mantissa != 0 && power < i8::MAX {
                        mantissa /= 10;
                        power += 1;
                    }
                    return i32::try_from(mantissa).ok().map(|m| (m, power));
                }

                // Integral values beyond `i64` reducible to an `i32` mantissa
                // via trailing zeros (e.g. `1e30`). Bigger `power` means fewer
                // significant digits, so keep the last (largest-power) match;
                // stop once dividing has shrunk the candidate to nothing.
                if as_f64 == as_f64.trunc() {
                    let mut best = None;
                    for power in 1..=i8::MAX {
                        let scale = POW10[power as usize];
                        let base = (as_f64 / scale).round();
                        if base == 0.0 {
                            break;
                        }
                        if (i32::MIN as f64..=i32::MAX as f64).contains(&base) {
                            for cand in [base, base - 1.0, base + 1.0] {
                                let mantissa = cand as i32;
                                if (mantissa as f64 * scale) as $t == value {
                                    best = Some((mantissa, power));
                                    break;
                                }
                            }
                        }
                    }
                    return best;
                }

                // Fractional: `value = mantissa / 10^places`, fewest places.
                // Once the scaled value overflows an `i32` it only grows
                // further (`POW10` climbs monotonically with `places`), so no
                // later place can match. Each place looks up `POW10` once and
                // checks the three ±1 candidates against that shared scale.
                for places in 1..=i8::MAX {
                    let scale = POW10[places as usize];
                    let base = (as_f64 * scale).round();
                    if !(i32::MIN as f64..=i32::MAX as f64).contains(&base) {
                        break;
                    }
                    for cand in [base, base - 1.0, base + 1.0] {
                        let mantissa = cand as i32;
                        if (mantissa as f64 / scale) as $t == value {
                            return Some((mantissa, -places));
                        }
                    }
                }

                None
            }

            #[derive(Clone, Default)]
            pub struct DecimalContext {
                is_decimal: <bool as Encode>::Context,
                mantissa: <Small as EncodingStrategy<i32>>::Context,
                exponent: <Small as EncodingStrategy<i8>>::Context,
            }
            impl EncodingStrategy<$t> for Decimal {
                type Context = DecimalContext;
                fn encode<E: super::EntropyCoder>(
                    value: &$t,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    // Encode as `mantissa * 10^power` whenever the value is a
                    // short decimal — round integers fold their trailing zeros
                    // into the power (e.g. `1e6 -> (1,6)`), giving small
                    // mantissas. Anything else (long mantissas, large non-round
                    // integers, extreme magnitudes) stores its raw bits
                    // incompressibly.
                    let decimal = to_decimal(*value);
                    decimal.is_some().encode(writer, &mut ctx.is_decimal);
                    if let Some((mantissa, power)) = decimal {
                        <Small as EncodingStrategy<i32>>::encode(
                            &mantissa,
                            writer,
                            &mut ctx.mantissa,
                        );
                        <Small as EncodingStrategy<i8>>::encode(&power, writer, &mut ctx.exponent);
                    } else {
                        writer.encode_incompressible_bytes(&value.to_le_bytes());
                    }
                }

                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    if bool::decode(reader, &mut ctx.is_decimal)? {
                        let mantissa =
                            <Small as EncodingStrategy<i32>>::decode(reader, &mut ctx.mantissa)?;
                        let power =
                            <Small as EncodingStrategy<i8>>::decode(reader, &mut ctx.exponent)?;
                        return Ok(decimal_value(mantissa, power));
                    }
                    let mut bytes = [0u8; $bits / 8];
                    reader.decode_incompressible_bytes(&mut bytes)?;
                    Ok($t::from_le_bytes(bytes))
                }
            }

            #[test]
            fn test_to_decimal() {
                // Values exact in both `f32` and `f64`: the fractions and the
                // integers below `2^24`.
                assert_eq!(to_decimal(0.1), Some((1, -1)));
                assert_eq!(to_decimal(0.02), Some((2, -2)));
                assert_eq!(to_decimal(100.5), Some((1005, -1)));
                assert_eq!(to_decimal(12345.0), Some((12345, 0)));
                assert_eq!(to_decimal(1.0), Some((1, 0)));
                assert_eq!(to_decimal(32.0), Some((32, 0)));
                // Round integers fold their trailing zeros into the power, so
                // a run of multiples of `10^k` shares a small mantissa.
                assert_eq!(to_decimal(20.0), Some((2, 1)));
                assert_eq!(to_decimal(1_000.0), Some((1, 3)));
                assert_eq!(to_decimal(2_000.0), Some((2, 3)));
                assert_eq!(to_decimal(1e6), Some((1, 6)));
                assert_eq!(to_decimal(-8_000.0), Some((-8, 3)));

                // Extremes whose canonical form is precision-dependent: `5e9`
                // and the 14-digit integer aren't exact in `f32`, and `f32`'s
                // coarser rounding lets `1 * 10^30` reconstruct (so it folds to
                // `(1,30)` where `f64` can only reach `(100,28)`). Pin these on
                // `f64` only.
                if std::mem::size_of::<$t>() == 8 {
                    assert_eq!(to_decimal(1e-30), Some((1, -30)));
                    // `5e9` exceeds `i32` but folds to a one-digit mantissa.
                    assert_eq!(to_decimal(5e9), Some((5, 9)));
                    // Large non-round integers don't fit an `i32` mantissa and
                    // fall through to `None` (the incompressible path).
                    assert_eq!(to_decimal(12_345_678_901_234.0), None);
                    // `1e30` exceeds `i64`, so it takes the float positive-power
                    // path. The mult-built `POW10[30]` is 1 ULP below the true
                    // `1e30`, so `1 * POW10[30] != 1e30`; the largest power that
                    // still reconstructs exactly is `100 * POW10[28]`. Either
                    // form round-trips identically.
                    assert_eq!(to_decimal(1e30), Some((100, 28)));
                }
            }
        }
    };
}
impl_float!(f64, u64, 64, f64_mod);
impl_float!(f32, u32, 32, f32_mod);

#[test]
fn float_roundtrip() {
    use super::{decode, encode};
    // Exercises all three default-float tiers — integer, decimal, raw — plus
    // negatives, subnormals, -0.0, inf/NaN, and pseudo-random full-mantissa
    // values (which take the raw path).
    let mut values: Vec<f64> = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        3.0,
        -3.0,
        8.0,
        1e6,
        -1e6,
        1e15,
        1e30,
        0.1,
        -0.1,
        0.5,
        -0.5,
        0.25,
        0.125,
        1.5,
        2.5,
        1000.5,
        1234.25,
        -9999.75,
        12.34,
        128.332,
        0.001,
        1e-5,
        12345678901234.0, // large int -> raw
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 2.0,
        f64::from_bits(1),
        std::f64::consts::PI,
        std::f64::consts::E,
        f64::MAX,
        f64::MIN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    let mut x = 0x2545f4914f6cdd1du64;
    for _ in 0..200 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b = if (x >> 52) & 0x7ff == 0x7ff {
            x & !(1 << 52)
        } else {
            x
        };
        values.push(f64::from_bits(b));
    }
    for &v in &values {
        let got: f64 = decode(&encode(&v)).unwrap();
        // Value equality (exact for these); -0.0 collapses to +0.0 through the
        // integer tier, as it always has. NaN is excluded from the set.
        assert_eq!(got, v, "f64 {v} did not round-trip");
    }
    let f32s: Vec<f32> = vec![
        0.0,
        -0.0,
        1.0,
        -3.0,
        8.0,
        1e6,
        0.1,
        0.5,
        0.25,
        1.5,
        -1234.25,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        std::f32::consts::PI,
        f32::MAX,
        f32::MIN,
    ];
    for &v in &f32s {
        let got: f32 = decode(&encode(&v)).unwrap();
        assert_eq!(got, v, "f32 {v} did not round-trip");
    }
}

#[test]
fn float_saturation_roundtrip() {
    use super::{decode, encode};
    // A long run of one tier saturates its selector; values of another tier
    // appended afterward must still round-trip (forced to raw once is_int has
    // committed). Build Vecs so the whole stream shares adapting contexts.
    fn check(values: Vec<f64>) {
        let decoded: Vec<f64> = decode(&encode(&values)).unwrap();
        assert_eq!(decoded.len(), values.len());
        for (g, w) in decoded.iter().zip(&values) {
            assert_eq!(g.to_bits(), w.to_bits());
        }
    }
    // 300 integers (saturates is_int) then decimals + a raw value.
    let mut ints: Vec<f64> = (0..300).map(|i| (i * 7 - 500) as f64).collect();
    ints.extend_from_slice(&[0.5, 1.25, 0.1, std::f64::consts::PI, 42.0]);
    check(ints);
    // 300 high-entropy values (saturates is_raw) then structured ones.
    let mut x = 0x9e3779b97f4a7c15u64;
    let mut raws: Vec<f64> = (0..300)
        .map(|_| {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            f64::from_bits(x)
        })
        .filter(|v| v.is_finite())
        .collect();
    raws.extend_from_slice(&[1.0, 0.5, 0.1, 100.0]);
    check(raws);
}

#[test]
fn decimal_float() {
    use crate::Encoded;

    fn sizes(v: f64) -> String {
        println!("Testing {v}.");
        format!(
            "decimal: {} bits, binary: {} bits",
            super::estimated_bits!(Encoded::<f64, Decimal>::from(v)),
            super::estimated_bits!(v)
        )
    }
    fn sizes32(v: f32) -> String {
        println!("Testing {v}.");
        format!(
            "decimal: {} bits, binary: {} bits",
            super::estimated_bits!(Encoded::<f32, Decimal>::from(v)),
            super::estimated_bits!(v)
        )
    }

    expect!["decimal: 13 bits, binary: 14 bits"].assert_eq(&sizes(1.1));
    expect!["decimal: 8 bits, binary: 9 bits"].assert_eq(&sizes(0.1));
    expect!["decimal: 13 bits, binary: 14 bits"].assert_eq(&sizes(0.9));
    expect!["decimal: 30 bits, binary: 31 bits"].assert_eq(&sizes(128.332));
    expect!["decimal: 65 bits, binary: 65 bits"].assert_eq(&sizes(1.0_f64.exp()));
    expect!["decimal: 8 bits, binary: 9 bits"].assert_eq(&sizes(0.0));
    expect!["decimal: 13 bits, binary: 14 bits"].assert_eq(&sizes(8.0));
    expect!["decimal: 65 bits, binary: 65 bits"].assert_eq(&sizes(8e200));
    expect!["decimal: 65 bits, binary: 65 bits"].assert_eq(&sizes(8e300));

    expect!["decimal: 39 bits, binary: 40 bits"].assert_eq(&sizes32(1.0_f32.exp()));
    expect!["decimal: 8 bits, binary: 9 bits"].assert_eq(&sizes32(0.1));
    expect!["decimal: 8 bits, binary: 9 bits"].assert_eq(&sizes32(0.0));
    expect!["decimal: 13 bits, binary: 14 bits"].assert_eq(&sizes32(8.0));
}

#[test]
fn decimal_roundtrip() {
    use super::{decode, encode};
    use crate::Encoded;

    // Named values exercising the fast path (small fractions, many-digit
    // irrationals), the string fallback (large integers with a positive power,
    // extreme magnitudes), and the integer/special cases.
    let mut vals: Vec<f64> = vec![
        1.1,
        0.1,
        0.9,
        128.332,
        std::f64::consts::E,
        std::f64::consts::PI,
        0.0,
        -0.0,
        8.0,
        -8.0,
        8e200,
        8e300,
        1e-300,
        0.001,
        0.5,
        -1.5,
        1234.25,
        -9999.75,
        100.25,
        1e-5,
        1.23456789,
        196821348019255.03,
        12345678901234.5,
        f64::MAX,
        f64::MIN,
    ];
    // Quantized decimals (the fast path's target) and arbitrary bit patterns
    // (which stress the fallback and the round-trip check).
    let mut x = 0x2545f4914f6cdd1du64;
    for _ in 0..20000 {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        vals.push((x % 100_000_000) as f64 / 100.0);
        vals.push((x % 1_000_000) as f64 / 1000.0);
        vals.push(f64::from_bits(x));
    }
    for v in vals {
        if !v.is_finite() {
            continue;
        }
        let e = Encoded::<f64, Decimal>::from(v);
        let got: Encoded<f64, Decimal> = decode(&encode(&e)).unwrap();
        // Value equality (exact for representable decimals); `Decimal` collapses
        // `-0.0` to `+0.0` via its integer tier, as it always has.
        assert_eq!(got, e, "decimal {v} did not round-trip");
    }
}
