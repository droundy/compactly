use super::afewbits::{AFewBits, AFewBitsContext};
use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder, Small};
use crate::{Incompressible, Sorted};

macro_rules! impl_uint {
    ($t:ident, $mod:ident, $bits:literal) => {
        mod $mod {
            use super::*;

            #[derive(Clone, Copy)]
            pub struct Context {
                leading_zero: [<bool as Encode>::Context; $bits],
                context: [<bool as Encode>::Context; $bits],
            }
            impl Default for Context {
                #[inline]
                fn default() -> Self {
                    Self {
                        leading_zero: [Default::default(); $bits],
                        context: [Default::default(); $bits],
                    }
                }
            }

            impl Encode for $t {
                type Context = Context;
                #[inline]
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    let mut am_leading = true;
                    for i in (0..$bits).rev() {
                        let bit = (*self & (1 << i)) != 0;
                        if am_leading {
                            bit.encode(writer, &mut ctx.leading_zero[i]);
                            am_leading = !bit;
                        } else {
                            bit.encode(writer, &mut ctx.context[i]);
                        }
                    }
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let mut v = 0;
                    let mut am_leading = true;
                    for i in (0..$bits).rev() {
                        let bit = if am_leading {
                            let bit = bool::decode(reader, &mut ctx.leading_zero[i])?;
                            am_leading = !bit;
                            bit
                        } else {
                            bool::decode(reader, &mut ctx.context[i])?
                        };
                        if bit {
                            v |= 1 << i;
                        }
                    }
                    Ok(v)
                }
            }

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
impl_uint!(u64, u64_mod, 64);
impl_uint!(u32, u32_mod, 32);
impl_uint!(u16, u16_mod, 16);

#[test]
fn size_u64() {
    use super::assert_bits;
    for sz in 0..1024_u64 {
        println!("Trying with {sz}");
        assert_bits!(sz, 64);
    }
    for sz in [1_000_000_u64] {
        println!("Trying with {sz}");
        assert_bits!(sz, 64);
    }
    for sz in [u64::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 25);
    }
    assert_bits!([0_u64; 128], 430);
    assert_bits!([1_u64; 2], 101);
    assert_bits!([1_u64; 19], 274);
    assert_bits!(
        [0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        306
    );
}

#[test]
fn size_u32() {
    use super::assert_bits;
    for sz in [u32::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 12);
    }
    assert_bits!([0_u32; 128], 215);
    assert_bits!([u32::MAX; 128], 175);
    assert_bits!([1_u32; 2], 51);
    assert_bits!([1_u32; 19], 137);
    assert_bits!(
        [0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        155
    );
    for sz in 0..32768_u32 {
        println!("Trying with {sz}");
        assert_bits!(sz, 32);
    }
    for sz in 999_990_u32..1_000_000 {
        println!("Trying with {sz}");
        assert_bits!(sz, 32);
    }
}

#[test]
fn size_u16() {
    use super::assert_bits;
    for sz in 0..1_u16 {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    for sz in 1..21845_u16 {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    for sz in [u16::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 7);
    }
    assert_bits!([0_u16; 128], 108);
    assert_bits!([u16::MAX; 128], 98);
    assert_bits!([1_u16; 2], 25);
    assert_bits!([1_u16; 19], 69);
    assert_bits!(
        [0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        80
    );
}

macro_rules! impl_compact {
    ($t:ident, $context:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            // AFewBits<$bits-1>: code = lz.saturating_sub(1), so lz=0 and lz=1 both map to
            // code 0 (distinguished by lz_is_one), while lz=$bits maps to code $bits-1
            // (all-TRUE, cheapest in fresh context) with no extra bool needed.
            leading_zeros: AFewBitsContext<{ $bits - 1 }>,
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
                let afewbits_val = lz.saturating_sub(1) as u8;
                AFewBits::<{ $bits - 1 }>::new(afewbits_val).encode(writer, &mut ctx.leading_zeros);
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
                let afewbits_val =
                    u8::from(AFewBits::<{ $bits - 1 }>::decode(reader, &mut ctx.leading_zeros)?)
                        as usize;
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
    };
}

impl_compact!(u64, U64Compact, 64);
impl_compact!(u32, U32Compact, 32);
impl_compact!(u16, U16Compact, 16);

#[test]
fn compact_u16() {
    use super::assert_bits;
    use crate::{Encoded, Small};
    assert_bits!(Encoded::<_, Small>::new(0_u16), 1);
    assert_bits!(Encoded::<_, Small>::new(1_u16), 4);
    assert_bits!(Encoded::<_, Small>::new(2_u16), 5);
    assert_bits!(Encoded::<_, Small>::new(3_u16), 5);
    assert_bits!(Encoded::<_, Small>::new(4_u16), 6);
    assert_bits!(Encoded::<_, Small>::new(5_u16), 6);
    assert_bits!(Encoded::<_, Small>::new(6_u16), 6);
    assert_bits!(Encoded::<_, Small>::new(7_u16), 6);
    assert_bits!(Encoded::<_, Small>::new(8_u16), 7);
    assert_bits!(Encoded::<_, Small>::new(u16::MAX), 20);
    assert_bits!([Encoded::<_, Small>::new(0_u16); 128], 25);
    assert_bits!([Encoded::<_, Small>::new(u16::MAX); 128], 1105);
    assert_bits!([Encoded::<_, Small>::new(1_u16); 2], 6);
    assert_bits!([Encoded::<_, Small>::new(1_u16); 19], 17);
    assert_bits!(
        [0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            .map(Encoded::<_, Small>::new),
        24
    );
}

#[test]
fn compact_u32() {
    use super::assert_bits;
    use crate::{Encoded, Small};
    assert_bits!(Encoded::<_, Small>::new(0_u32), 2);
    assert_bits!(Encoded::<_, Small>::new(1_u32), 5);
    assert_bits!(Encoded::<_, Small>::new(2_u32), 6);
    assert_bits!(Encoded::<_, Small>::new(3_u32), 6);
    assert_bits!(Encoded::<_, Small>::new(4_u32), 7);
    assert_bits!(Encoded::<_, Small>::new(5_u32), 7);
    assert_bits!(Encoded::<_, Small>::new(6_u32), 7);
    assert_bits!(Encoded::<_, Small>::new(7_u32), 7);
    assert_bits!(Encoded::<_, Small>::new(8_u32), 8);
    assert_bits!(Encoded::<_, Small>::new(u32::MAX), 38);
    assert_bits!([Encoded::<_, Small>::new(0_u32); 128], 30);
    assert_bits!([Encoded::<_, Small>::new(u32::MAX); 128], 3160);
    assert_bits!([Encoded::<_, Small>::new(1_u32); 2], 8);
    assert_bits!([Encoded::<_, Small>::new(1_u32); 19], 22);
    assert_bits!(
        [0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            .map(Encoded::<_, Small>::new),
        28
    );

    for i in 0_u32..4096 {
        assert_eq!(
            super::encode(&Encoded::<_, Small>::new(i)),
            super::encode_with(Small, &i)
        );
    }
}

macro_rules! impl_signed {
    ($signed:ident, $unsigned:ident, $mod:ident) => {
        mod $mod {
            use super::*;
            impl Encode for $signed {
                type Context = <$unsigned as Encode>::Context;
                #[inline]
                fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                    $unsigned::from_le_bytes(self.to_le_bytes()).encode(writer, ctx)
                }
                #[inline]
                fn decode<D: EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let v = $unsigned::decode(reader, ctx)?;
                    Ok($signed::from_le_bytes(v.to_le_bytes()))
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

impl_signed!(i16, u16, mod_i16);
impl_signed!(i32, u32, mod_i32);
impl_signed!(i64, u64, mod_i64);

#[test]
fn signed() {
    use super::{assert_bits, raw_bits};
    use crate::{Encoded, Small};
    use std::collections::BTreeSet;

    assert_bits!(Encoded::<_, Small>::new(0_i32), 6);
    assert_bits!(Encoded::<_, Small>::new(1_i32), 6);
    assert_bits!(Encoded::<_, Small>::new(-1_i32), 3);
    assert_bits!(Encoded::<_, Small>::new(i32::MAX), 38);
    assert_bits!(Encoded::<_, Small>::new(i32::MIN), 38);
    for v in [i32::MIN, i32::MAX, 0, 1, 7, 137, i32::MAX - 1] {
        println!("testing {v}");
        assert_bits!(v, 32);
    }
    for v in [-1i32] {
        println!("testing {v}");
        assert_bits!(v, 12);
    }

    assert_bits!(Encoded::<_, Small>::new(0_i16), 5);
    assert_bits!(Encoded::<_, Small>::new(1_i16), 5);
    assert_bits!(Encoded::<_, Small>::new(-1_i16), 2);
    assert_bits!(Encoded::<_, Small>::new(i16::MAX), 20);
    assert_bits!(Encoded::<_, Small>::new(i16::MIN), 20);
    for v in [i16::MIN, i16::MAX, 0, 1, 7, 137, i16::MAX - 1] {
        println!("testing {v}");
        assert_bits!(v, 16);
    }

    assert_bits!(Encoded::<_, Small>::new(0_i64), 7);
    assert_bits!(Encoded::<_, Small>::new(1_i64), 7);
    assert_bits!(Encoded::<_, Small>::new(-1_i64), 3);
    assert_bits!(Encoded::<_, Small>::new(i64::MAX), 71);
    assert_bits!(Encoded::<_, Small>::new(i64::MIN), 71);
    for v in [i64::MIN, 0, 1, 7, 137, i64::MAX - 1] {
        println!("testing {v}");
        assert_bits!(v, 64);
    }
    for v in [-1i64] {
        println!("testing {v}");
        assert_bits!(v, 25);
    }

    use super::Millibits;
    raw_bits!(BTreeSet::from([-1i16, 0, 1, 2]), 27, Millibits::new(21985));
    raw_bits!(BTreeSet::from([-1i64, 0, 1, 2]), 35, Millibits::new(27979));
    raw_bits!(BTreeSet::from([i16::MIN, i16::MAX]), 44);
    raw_bits!(BTreeSet::from([i64::MIN, i64::MAX]), 144);
}
