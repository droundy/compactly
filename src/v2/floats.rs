use super::{Encode, EncodingStrategy};
use crate::{Decimal, Small};

#[cfg(test)]
use expect_test::expect;

macro_rules! impl_float {
    ($t:ident, $intty:ident, $sint:ident, $context:ident, $decimal:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            is_int: <bool as Encode>::Context,
            int_context: <Small as EncodingStrategy<$intty>>::Context,
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
            #[inline]
            fn default() -> Self {
                Self {
                    is_int: Default::default(),
                    int_context: Default::default(),
                    context: [Default::default(); $bits],
                }
            }
        }

        impl Encode for $t {
            type Context = $context;
            #[inline]
            fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let intvalue = *self as $intty;
                let is_int = intvalue as $t == *self;
                is_int.encode(writer, &mut ctx.is_int);
                if is_int {
                    <Small as EncodingStrategy<$intty>>::encode(
                        &intvalue,
                        writer,
                        &mut ctx.int_context,
                    )
                } else {
                    // The $bits raw bits are independent (one context per position),
                    // so encode them as a single batch.
                    let bits = $intty::from_le_bytes(self.to_le_bytes());
                    let values = std::array::from_fn::<_, $bits, _>(|i| (bits >> i) & 1 == 1);
                    writer.encode_bits(&mut ctx.context, values);
                }
            }
            #[inline]
            fn decode<D: super::EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$intty>>::decode(reader, &mut ctx.int_context)?;
                    Ok(intvalue as $t)
                } else {
                    // The $bits raw bits are independent (one context per position),
                    // so decode them as a single register-resident batch.
                    let decoded = reader.decode_bits(&mut ctx.context);
                    let mut bits: $intty = 0;
                    for i in 0..$bits {
                        if decoded[i] {
                            bits |= (1 << i);
                        }
                    }
                    Ok($t::from_le_bytes(bits.to_le_bytes()))
                }
            }
        }

        #[derive(Clone, Default)]
        pub struct $decimal {
            is_int: <bool as Encode>::Context,
            integer: <Small as EncodingStrategy<$sint>>::Context,
            is_decimal: <bool as Encode>::Context,
            mantissa: <Small as EncodingStrategy<i32>>::Context,
            exponent: <Small as EncodingStrategy<i8>>::Context,
        }
        impl EncodingStrategy<$t> for Decimal {
            type Context = $decimal;
            fn encode<E: super::EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                // `value = ±mantissa · 10^power`, computed in f64 and narrowed:
                // a `u32` mantissa (< 2^32) and `10^|power|` for `|power| <= 22`
                // are both exact, so the mul/div is correctly rounded; the cast
                // to `$t` is identity for f64 and correct-by-consistency for f32
                // (encode's round-trip check and decode share this function).
                fn decimal_value(mantissa: i32, power: i8) -> $t {
                    let scale = 10f64.powi(power.unsigned_abs() as i32);
                    let v = if power < 0 {
                        mantissa as f64 / scale
                    } else {
                        mantissa as f64 * scale
                    };
                    v as $t
                }

                // Find the fewest decimal places whose rounded (signed) `i32`
                // mantissa reconstructs `value` exactly (probing ±1 for the
                // large-magnitude off-by-one). `None` — a value not representable
                // as `d·10^p` with `d` in `i32` and `|p| <= 22` (17-digit
                // mantissas, large integers with a positive power, extreme
                // magnitudes) — takes the incompressible path, no string
                // formatting anywhere.
                fn to_decimal(value: $t) -> Option<(i32, i8)> {
                    for places in 1..=22i8 {
                        let scale = 10f64.powi(places as i32);
                        let base = (value as f64 * scale).round();
                        for cand in [base, base - 1.0, base + 1.0] {
                            if (i32::MIN as f64..=i32::MAX as f64).contains(&cand) {
                                let mantissa = cand as i32;
                                if decimal_value(mantissa, -places) == value {
                                    return Some((mantissa, -places));
                                }
                            }
                        }
                    }
                    None
                }

                let intvalue = *value as $sint;
                let is_int = intvalue as $t == *value;
                is_int.encode(writer, &mut ctx.is_int);
                if is_int {
                    <Small as EncodingStrategy<$sint>>::encode(&intvalue, writer, &mut ctx.integer);
                    return;
                }

                let decimal = to_decimal(*value);
                decimal.is_some().encode(writer, &mut ctx.is_decimal);
                match decimal {
                    Some((mantissa, power)) => {
                        <Small as EncodingStrategy<i32>>::encode(
                            &mantissa,
                            writer,
                            &mut ctx.mantissa,
                        );
                        <Small as EncodingStrategy<i8>>::encode(&power, writer, &mut ctx.exponent);
                    }
                    None => {
                        // Not a short decimal — store the bits incompressibly.
                        writer.encode_incompressible_bytes(&value.to_le_bytes());
                    }
                }
            }

            fn decode<D: super::EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                fn decimal_value(mantissa: i32, power: i8) -> $t {
                    let scale = 10f64.powi(power.unsigned_abs() as i32);
                    let v = if power < 0 {
                        mantissa as f64 / scale
                    } else {
                        mantissa as f64 * scale
                    };
                    v as $t
                }

                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$sint>>::decode(reader, &mut ctx.integer)?;
                    return Ok(intvalue as $t);
                }
                if bool::decode(reader, &mut ctx.is_decimal)? {
                    let mantissa =
                        <Small as EncodingStrategy<i32>>::decode(reader, &mut ctx.mantissa)?;
                    let power = <Small as EncodingStrategy<i8>>::decode(reader, &mut ctx.exponent)?;
                    return Ok(decimal_value(mantissa, power));
                }
                let mut bytes = [0u8; $bits / 8];
                reader.decode_incompressible_bytes(&mut bytes)?;
                Ok($t::from_le_bytes(bytes))
            }
        }
    };
}
impl_float!(f64, u64, i64, F64Context, F64Decimal, 64);
impl_float!(f32, u32, i32, F32Context, F32Decimal, 32);

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

    expect!["decimal: 14 bits, binary: 65 bits"].assert_eq(&sizes(1.1));
    expect!["decimal: 9 bits, binary: 65 bits"].assert_eq(&sizes(0.1));
    expect!["decimal: 14 bits, binary: 65 bits"].assert_eq(&sizes(0.9));
    expect!["decimal: 31 bits, binary: 65 bits"].assert_eq(&sizes(128.332));
    expect!["decimal: 66 bits, binary: 65 bits"].assert_eq(&sizes(1.0_f64.exp()));
    expect!["decimal: 5 bits, binary: 4 bits"].assert_eq(&sizes(0.0));
    expect!["decimal: 10 bits, binary: 9 bits"].assert_eq(&sizes(8.0));
    expect!["decimal: 66 bits, binary: 65 bits"].assert_eq(&sizes(8e200));
    expect!["decimal: 66 bits, binary: 65 bits"].assert_eq(&sizes(8e300));

    expect!["decimal: 40 bits, binary: 33 bits"].assert_eq(&sizes32(1.0_f32.exp()));
    expect!["decimal: 9 bits, binary: 33 bits"].assert_eq(&sizes32(0.1));
    expect!["decimal: 5 bits, binary: 4 bits"].assert_eq(&sizes32(0.0));
    expect!["decimal: 10 bits, binary: 9 bits"].assert_eq(&sizes32(8.0));
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
