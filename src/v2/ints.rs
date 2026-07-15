use super::atmost::geometric::geometric_seeded;
use super::atmost::{AtMost, AtMostContext};
use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder, Small};
use crate::{Incompressible, Sorted};

#[cfg(test)]
use expect_test::expect;

macro_rules! impl_uint {
    ($t:ident, $mod:ident, $bits:literal) => {
        mod $mod {
            use super::*;

            #[derive(Default, Clone)]
            pub struct SortedContext {
                previous: Option<$t>,
                not_sorted: <bool as Encode>::Context,
                value: <Small as EncodingStrategy<$t>>::Context,
                difference: <Small as EncodingStrategy<$t>>::Context,
            }

            impl EncodingStrategy<$t> for Sorted {
                type Context = SortedContext;
                fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted);
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value);
                        } else {
                            Small::encode(&(*value - previous), writer, &mut ctx.difference);
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value);
                    }
                    ctx.previous = Some(*value);
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let out = if let Some(previous) = ctx.previous.take() {
                        let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::decode(reader, &mut ctx.value)?
                        } else {
                            previous
                                + <Small as EncodingStrategy<$t>>::decode(
                                    reader,
                                    &mut ctx.difference,
                                )?
                        }
                    } else {
                        Small::decode(reader, &mut ctx.value)?
                    };
                    ctx.previous = Some(out);
                    Ok(out)
                }
            }

            impl EncodingStrategy<$t> for Incompressible {
                type Context = ();
                fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, _ctx: &mut Self::Context) {
                    writer.encode_incompressible_bytes(&value.to_le_bytes())
                }
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    _ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let mut b = [0; std::mem::size_of::<$t>()];
                    reader.decode_incompressible_bytes(&mut b)?;
                    Ok($t::from_le_bytes(b))
                }
            }
        }
    };
}
impl_uint!(u128, u128_mod, 128);
impl_uint!(u64, u64_mod, 64);
impl_uint!(u32, u32_mod, 32);
impl_uint!(u16, u16_mod, 16);

// The default integer `Encode` is now variable-length (the geometric-seeded
// `Small` algorithm), so a fresh context costs ~`bits` bits for an
// arbitrary/incompressible value but far fewer for small values — unlike the
// old fixed per-bit-position encoding, where every value in a range cost the
// same. These tests therefore probe individual representative values rather
// than asserting a whole range shares one size.

#[test]
fn size_u64() {
    use super::estimated_bits;
    expect!["10"].assert_eq(&estimated_bits!(0_u64));
    expect!["10"].assert_eq(&estimated_bits!(1_u64));
    expect!["17"].assert_eq(&estimated_bits!(255_u64));
    expect!["18"].assert_eq(&estimated_bits!(256_u64));
    expect!["28"].assert_eq(&estimated_bits!(1_000_000_u64));
    expect!["66"].assert_eq(&estimated_bits!(u64::MAX));
    expect!["83"].assert_eq(&estimated_bits!([0_u64; 128]));
    expect!["16"].assert_eq(&estimated_bits!([1_u64; 2]));
    expect!["51"].assert_eq(&estimated_bits!([1_u64; 19]));
    expect!["61"].assert_eq(&estimated_bits!([
        0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u32() {
    use super::estimated_bits;
    expect!["8"].assert_eq(&estimated_bits!(0_u32));
    expect!["8"].assert_eq(&estimated_bits!(1_u32));
    expect!["16"].assert_eq(&estimated_bits!(255_u32));
    expect!["16"].assert_eq(&estimated_bits!(256_u32));
    expect!["26"].assert_eq(&estimated_bits!(1_000_000_u32));
    expect!["33"].assert_eq(&estimated_bits!(u32::MAX));
    expect!["70"].assert_eq(&estimated_bits!([0_u32; 128]));
    expect!["3148"].assert_eq(&estimated_bits!([u32::MAX; 128]));
    expect!["14"].assert_eq(&estimated_bits!([1_u32; 2]));
    expect!["43"].assert_eq(&estimated_bits!([1_u32; 19]));
    expect!["53"].assert_eq(&estimated_bits!([
        0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u16() {
    use super::estimated_bits;
    expect!["7"].assert_eq(&estimated_bits!(0_u16));
    expect!["7"].assert_eq(&estimated_bits!(1_u16));
    expect!["13"].assert_eq(&estimated_bits!(255_u16));
    expect!["13"].assert_eq(&estimated_bits!(256_u16));
    expect!["17"].assert_eq(&estimated_bits!(u16::MAX));
    expect!["58"].assert_eq(&estimated_bits!([0_u16; 128]));
    expect!["1095"].assert_eq(&estimated_bits!([u16::MAX; 128]));
    expect!["11"].assert_eq(&estimated_bits!([1_u16; 2]));
    expect!["35"].assert_eq(&estimated_bits!([1_u16; 19]));
    expect!["44"].assert_eq(&estimated_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u128() {
    use super::estimated_bits;
    expect!["11"].assert_eq(&estimated_bits!(0_u128));
    expect!["11"].assert_eq(&estimated_bits!(1_u128));
    expect!["18"].assert_eq(&estimated_bits!(255_u128));
    expect!["130"].assert_eq(&estimated_bits!(u128::MAX));
    expect!["55"].assert_eq(&estimated_bits!([1_u128; 19]));
}

macro_rules! impl_compact {
    ($t:ident, $context:ident, $default_context:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            // AtMost<$bits - 1>, i.e. log2($bits) bits: code = lz.saturating_sub(1), so
            // lz=0 and lz=1 both map to code 0 (distinguished by lz_is_one), while
            // lz=$bits maps to code $bits-1 (all-TRUE, cheapest in fresh context) with
            // no extra bool needed.
            leading_zeros: <AtMost<{ $bits - 1 }> as Encode>::Context,
            lz_is_one: <bool as Encode>::Context,
            // partial[lz][i]: context for bit i of the partial top byte, given lz leading zeros.
            // Only partial_bits = (sig_bits % 8) bits are used per lz, where sig_bits = $bits-1-lz.
            partial: [[<bool as Encode>::Context; 8]; $bits],
        }
        impl Default for $context {
            fn default() -> Self {
                Self {
                    leading_zeros: Default::default(),
                    lz_is_one: Default::default(),
                    partial: [[Default::default(); 8]; $bits],
                }
            }
        }

        impl EncodingStrategy<$t> for Small {
            type Context = $context;
            #[inline]
            fn encode<E: EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                let lz = value.leading_zeros() as usize;
                // lz=0,1 → code 0 (+bool); lz=k≥2 → code k-1; lz=$bits → code $bits-1 (all-TRUE)
                let afewbits_val = lz.saturating_sub(1);
                AtMost::<{ $bits - 1 }>::new(afewbits_val).encode(writer, &mut ctx.leading_zeros);
                if afewbits_val == 0 {
                    (lz == 1).encode(writer, &mut ctx.lz_is_one);
                }
                if lz >= $bits - 1 {
                    return;
                }
                let sig_bits = $bits - 1 - lz;
                let full_bytes = sig_bits / 8;
                let partial_bits = sig_bits % 8;
                let value_bytes = value.to_le_bytes();
                if full_bytes > 0 {
                    writer.encode_incompressible_bytes(&value_bytes[..full_bytes]);
                }
                for i in 0..partial_bits {
                    let bit = (value_bytes[full_bytes] >> i) & 1 == 1;
                    bit.encode(writer, &mut ctx.partial[lz][i]);
                }
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                let afewbits_val = usize::from(AtMost::<{ $bits - 1 }>::decode(
                    reader,
                    &mut ctx.leading_zeros,
                )?);
                let lz = if afewbits_val == 0 {
                    if bool::decode(reader, &mut ctx.lz_is_one)? {
                        1
                    } else {
                        0
                    }
                } else {
                    afewbits_val + 1
                };
                if lz >= $bits - 1 {
                    return if lz == $bits { Ok(0) } else { Ok(1) };
                }
                let sig_bits = $bits - 1 - lz;
                let full_bytes = sig_bits / 8;
                let partial_bits = sig_bits % 8;
                let mut value_bytes = [0u8; std::mem::size_of::<$t>()];
                if full_bytes > 0 {
                    reader.decode_incompressible_bytes(&mut value_bytes[..full_bytes])?;
                }
                for i in 0..partial_bits {
                    if bool::decode(reader, &mut ctx.partial[lz][i])? {
                        value_bytes[full_bytes] |= 1 << i;
                    }
                }
                // Restore the implicit leading 1.
                value_bytes[full_bytes] |= 1 << partial_bits;
                Ok($t::from_le_bytes(value_bytes.try_into().unwrap()))
            }
        }

        // The default `Encode` for `$t` reuses `Small`'s exact encode/decode
        // logic (there is exactly one compiled copy, shared by both) via a
        // context that wraps `Small`'s own context type and only overrides
        // its `Default`: the leading-zero-count sub-context is seeded with a
        // geometric (Elias-gamma) prior — `geometric_seeded` — instead of the
        // flat, count-based `AtMostContext::SEEDED`. See `atmost::geometric`
        // for why: a fresh context then costs close to `$bits` bits for an
        // arbitrary/incompressible value (as a plain literal encoding would),
        // while `Small`'s flat seed pays ~`log2($bits)` extra fresh bits. Both
        // share one adaptive tree, so this default still learns skewed
        // distributions quickly, unlike a fixed per-bit-position encoding.
        #[derive(Clone)]
        pub struct $default_context($context);

        impl Default for $default_context {
            fn default() -> Self {
                Self($context {
                    leading_zeros: AtMostContext {
                        bits: geometric_seeded::<{ $bits - 1 }>(),
                    },
                    lz_is_one: Default::default(),
                    partial: [[Default::default(); 8]; $bits],
                })
            }
        }

        impl Encode for $t {
            type Context = $default_context;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                Small::encode(self, writer, &mut ctx.0)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                Small::decode(reader, &mut ctx.0)
            }
        }
    };
}

impl_compact!(u128, U128Compact, U128Default, 128);
impl_compact!(u64, U64Compact, U64Default, 64);
impl_compact!(u32, U32Compact, U32Default, 32);
impl_compact!(u16, U16Compact, U16Default, 16);

#[test]
fn compact_u16() {
    use super::estimated_bits;
    use crate::{Encoded, Small};
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_u16)));
    expect!["4"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_u16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(2_u16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(3_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(4_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(5_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(6_u16)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(7_u16)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(8_u16)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(u16::MAX)));
    expect!["27"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(0_u16); 128]));
    expect!["1104"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(u16::MAX); 128]));
    expect!["6"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u16); 2]));
    expect!["17"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u16); 19]));
    expect!["24"].assert_eq(&estimated_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]
    .map(Encoded::<_, Small>::new)));
}

#[test]
fn compact_u32() {
    use super::estimated_bits;
    use crate::{Encoded, Small};
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_u32)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(2_u32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(3_u32)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(4_u32)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(5_u32)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(6_u32)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(7_u32)));
    expect!["8"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(8_u32)));
    expect!["37"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(u32::MAX)));
    expect!["34"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(0_u32); 128]));
    expect!["3159"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(u32::MAX); 128]));
    expect!["8"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u32); 2]));
    expect!["21"].assert_eq(&estimated_bits!([Encoded::<_, Small>::new(1_u32); 19]));
    expect!["28"].assert_eq(&estimated_bits!([
        0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]
    .map(Encoded::<_, Small>::new)));

    for i in 0_u32..4096 {
        assert_eq!(
            super::encode(&Encoded::<_, Small>::new(i)),
            super::encode_with(Small, &i)
        );
    }
}

macro_rules! impl_signed {
    ($signed:ident, $unsigned:ident, $bits:literal, $mod:ident) => {
        mod $mod {
            use super::*;

            #[derive(Clone)]
            pub struct NormalContext {
                is_negative: <bool as Encode>::Context,
                // Magnitude is in [0, 2^($bits-1)-1]: effectively a ($bits-1)-bit unsigned.
                // The MSB of $unsigned is always 0, so we use $bits-1 leading_zero slots.
                leading_zero: [<bool as Encode>::Context; $bits - 1],
                u8_ctx: <u8 as Encode>::Context,
                partial: [[<bool as Encode>::Context; 8]; $bits - 1],
            }
            impl Default for NormalContext {
                fn default() -> Self {
                    Self {
                        is_negative: Default::default(),
                        leading_zero: [Default::default(); $bits - 1],
                        u8_ctx: Default::default(),
                        partial: [[Default::default(); 8]; $bits - 1],
                    }
                }
            }

            impl Encode for $signed {
                type Context = NormalContext;
                #[inline]
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    let is_neg = *self < 0;
                    is_neg.encode(writer, &mut ctx.is_negative);
                    let mag: $unsigned = if is_neg {
                        self.abs_diff(-1)
                    } else {
                        self.abs_diff(0)
                    };
                    // Encode magnitude as a ($bits-1)-bit value.
                    // mag < 2^($bits-1) so mag.leading_zeros() >= 1; adjusted_lz = leading_zeros - 1.
                    const MBITS: usize = $bits - 1;
                    let lz = mag.leading_zeros() as usize - 1;
                    if lz >= MBITS - 8 {
                        for i in (8..MBITS).rev() {
                            false.encode(writer, &mut ctx.leading_zero[i]);
                        }
                        (mag as u8).encode(writer, &mut ctx.u8_ctx);
                        return;
                    }
                    for i in (MBITS - lz..MBITS).rev() {
                        false.encode(writer, &mut ctx.leading_zero[i]);
                    }
                    true.encode(writer, &mut ctx.leading_zero[MBITS - 1 - lz]);
                    let sig_bits = MBITS - 1 - lz;
                    let full_bytes = sig_bits / 8;
                    let partial_bits = sig_bits % 8;
                    let value_bytes = mag.to_le_bytes();
                    if full_bytes > 0 {
                        writer.encode_incompressible_bytes(&value_bytes[..full_bytes]);
                    }
                    for i in 0..partial_bits {
                        let bit = (value_bytes[full_bytes] >> i) & 1 == 1;
                        bit.encode(writer, &mut ctx.partial[lz][i]);
                    }
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let is_neg = bool::decode(reader, &mut ctx.is_negative)?;
                    const MBITS: usize = $bits - 1;
                    let mut lz = 0usize;
                    let mag: $unsigned = loop {
                        if lz >= MBITS - 8 {
                            let v = u8::decode(reader, &mut ctx.u8_ctx)?;
                            break v as $unsigned;
                        }
                        if bool::decode(reader, &mut ctx.leading_zero[MBITS - 1 - lz])? {
                            let sig_bits = MBITS - 1 - lz;
                            let full_bytes = sig_bits / 8;
                            let partial_bits = sig_bits % 8;
                            let mut value_bytes = [0u8; std::mem::size_of::<$unsigned>()];
                            if full_bytes > 0 {
                                reader
                                    .decode_incompressible_bytes(&mut value_bytes[..full_bytes])?;
                            }
                            for i in 0..partial_bits {
                                if bool::decode(reader, &mut ctx.partial[lz][i])? {
                                    value_bytes[full_bytes] |= 1 << i;
                                }
                            }
                            value_bytes[full_bytes] |= 1 << partial_bits;
                            break $unsigned::from_le_bytes(value_bytes.try_into().unwrap());
                        }
                        lz += 1;
                    };
                    if is_neg {
                        Ok(-1 - (mag as $signed))
                    } else {
                        Ok(mag as $signed)
                    }
                }
            }

            #[derive(Clone)]
            pub struct Context {
                is_negative: <bool as Encode>::Context,
                positive: <Small as EncodingStrategy<$unsigned>>::Context,
                negative: <Small as EncodingStrategy<$unsigned>>::Context,
            }
            impl Default for Context {
                #[inline]
                fn default() -> Self {
                    Self {
                        is_negative: Default::default(),
                        positive: Default::default(),
                        negative: Default::default(),
                    }
                }
            }

            impl EncodingStrategy<$signed> for Small {
                type Context = Context;
                #[inline]
                fn encode<E: EntropyCoder>(
                    value: &$signed,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    (*value < 0).encode(writer, &mut ctx.is_negative);
                    if *value < 0 {
                        Small::encode(&value.abs_diff(-1), writer, &mut ctx.negative)
                    } else {
                        Small::encode(&value.abs_diff(0), writer, &mut ctx.positive)
                    }
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$signed, std::io::Error> {
                    if bool::decode(reader, &mut ctx.is_negative)? {
                        let p = <Small as EncodingStrategy<$unsigned>>::decode(
                            reader,
                            &mut ctx.negative,
                        )?;
                        Ok(-1 - (p as $signed))
                    } else {
                        let p = <Small as EncodingStrategy<$unsigned>>::decode(
                            reader,
                            &mut ctx.positive,
                        )?;
                        Ok(p as $signed)
                    }
                }
            }
            #[derive(Default, Clone)]
            pub struct SortedContext {
                previous: Option<$signed>,
                not_sorted: <bool as Encode>::Context,
                value: <Small as EncodingStrategy<$signed>>::Context,
                difference: <Small as EncodingStrategy<$unsigned>>::Context,
            }

            impl EncodingStrategy<$signed> for Sorted {
                type Context = SortedContext;
                fn encode<E: EntropyCoder>(
                    value: &$signed,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted);
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value);
                        } else {
                            Small::encode(&(value.abs_diff(previous)), writer, &mut ctx.difference);
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value);
                    }
                    ctx.previous = Some(*value);
                }
                fn decode<E: EntropyDecoder>(
                    reader: &mut E,
                    ctx: &mut Self::Context,
                ) -> Result<$signed, std::io::Error> {
                    let out = if let Some(previous) = ctx.previous.take() {
                        let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::decode(reader, &mut ctx.value)?
                        } else {
                            previous
                                .checked_add_unsigned(
                                    <Small as EncodingStrategy<$unsigned>>::decode(
                                        reader,
                                        &mut ctx.difference,
                                    )?,
                                )
                                .ok_or_else(|| std::io::Error::other("invalid addition"))?
                        }
                    } else {
                        Small::decode(reader, &mut ctx.value)?
                    };
                    ctx.previous = Some(out);
                    Ok(out)
                }
            }
        }
    };
}

impl_signed!(i128, u128, 128, mod_i128);
impl_signed!(i16, u16, 16, mod_i16);
impl_signed!(i32, u32, 32, mod_i32);
impl_signed!(i64, u64, 64, mod_i64);

#[test]
fn signed() {
    use super::{assert_bits_all, assert_millibits, estimated_bits};
    use crate::{Encoded, Small};
    use std::collections::BTreeSet;

    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i32)));
    expect!["6"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i32)));
    expect!["37"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i32::MAX)));
    expect!["37"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i32::MIN)));
    assert_bits_all!([0i32, 1, 7, 137, -1i32], expect!["32"]);
    expect!["32"].assert_eq(&estimated_bits!(i32::MIN));
    assert_bits_all!([i32::MAX, i32::MAX - 1], expect!["32"]);

    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i16)));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i16)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i16::MAX)));
    expect!["20"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i16::MIN)));
    expect!["16"].assert_eq(&estimated_bits!(i16::MIN));
    assert_bits_all!([i16::MAX, 0, 1, 7, 137, i16::MAX - 1], expect!["16"]);

    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(0_i64)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(1_i64)));
    expect!["7"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(-1_i64)));
    expect!["70"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i64::MAX)));
    expect!["70"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(i64::MIN)));
    assert_bits_all!([0i64, 1, 7, 137, -1i64], expect!["64"]);
    expect!["64"].assert_eq(&estimated_bits!(i64::MIN));
    expect!["64"].assert_eq(&estimated_bits!(i64::MAX - 1));

    assert_millibits!(
        BTreeSet::from([-1i16, 0, 1, 2]),
        expect!["Millibits(21986)"]
    );
    assert_millibits!(
        BTreeSet::from([-1i64, 0, 1, 2]),
        expect!["Millibits(27983)"]
    );
    assert_millibits!(BTreeSet::from([i16::MIN, i16::MAX]), expect!["44 bits"]);
    assert_millibits!(BTreeSet::from([i64::MIN, i64::MAX]), expect!["144 bits"]);
}
