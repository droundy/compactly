use super::{Encode, EncodingStrategy, Reader, Small, ULessThan, Writer};
use crate::{Incompressible, Sorted};
use std::io::{Read, Write};

#[cfg(test)]
use expect_test::expect;

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
                fn encode<W: Write>(
                    &self,
                    writer: &mut Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    let mut am_leading = true;
                    for i in (0..$bits).rev() {
                        let bit = (*self & (1 << i)) != 0;
                        if am_leading {
                            bit.encode(writer, &mut ctx.leading_zero[i])?;
                            am_leading = !bit;
                        } else {
                            bit.encode(writer, &mut ctx.context[i])?;
                        }
                    }
                    Ok(())
                }
                #[inline]
                fn decode<R: Read>(
                    reader: &mut Reader<R>,
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

            impl EncodingStrategy<$t> for Incompressible {
                type Context = Context;
                fn encode<W: Write>(
                    value: &$t,
                    writer: &mut super::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    value.encode(writer, ctx)
                }
                fn decode<R: Read>(
                    reader: &mut super::Reader<R>,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    <$t as Encode>::decode(reader, ctx)
                }
            }
            #[derive(Default, Clone)]
            pub struct SortedIntContext {
                previous: Option<$t>,
                not_sorted: <bool as Encode>::Context,
                value: <Small as EncodingStrategy<$t>>::Context,
                difference: <Small as EncodingStrategy<$t>>::Context,
            }

            impl EncodingStrategy<$t> for Sorted {
                type Context = SortedIntContext;
                fn encode<W: Write>(
                    value: &$t,
                    writer: &mut super::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value)?;
                        } else {
                            Small::encode(&(*value - previous), writer, &mut ctx.difference)?;
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value)?;
                    }
                    ctx.previous = Some(*value);
                    Ok(())
                }
                fn decode<R: Read>(
                    reader: &mut super::Reader<R>,
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
        }
    };
}
impl_uint!(u64, u64_mod, 64);
impl_uint!(u32, u32_mod, 32);
impl_uint!(u16, u16_mod, 16);

#[test]
fn size_u64() {
    use super::{assert_bits_all, encoded_bits};
    assert_bits_all!(0..1024_u64, expect!["64"]);
    expect!["64"].assert_eq(&encoded_bits!(1_000_000_u64));
    expect!["25"].assert_eq(&encoded_bits!(u64::MAX));
    expect!["430"].assert_eq(&encoded_bits!([0_u64; 128]));
    expect!["101"].assert_eq(&encoded_bits!([1_u64; 2]));
    expect!["274"].assert_eq(&encoded_bits!([1_u64; 19]));
    expect!["306"].assert_eq(&encoded_bits!([
        0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

#[test]
fn size_u32() {
    use super::{assert_bits_all, encoded_bits};
    expect!["12"].assert_eq(&encoded_bits!(u32::MAX));
    expect!["215"].assert_eq(&encoded_bits!([0_u32; 128]));
    expect!["175"].assert_eq(&encoded_bits!([u32::MAX; 128]));
    expect!["51"].assert_eq(&encoded_bits!([1_u32; 2]));
    expect!["137"].assert_eq(&encoded_bits!([1_u32; 19]));
    expect!["155"].assert_eq(&encoded_bits!([
        0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
    assert_bits_all!(0..32768_u32, expect!["32"]);
    assert_bits_all!(999_990_u32..1_000_000, expect!["32"]);
}

#[test]
fn size_u16() {
    use super::{assert_bits_all, encoded_bits};
    assert_bits_all!(0..1_u16, expect!["16"]);
    assert_bits_all!(1..21845_u16, expect!["16"]);
    expect!["7"].assert_eq(&encoded_bits!(u16::MAX));
    expect!["108"].assert_eq(&encoded_bits!([0_u16; 128]));
    expect!["98"].assert_eq(&encoded_bits!([u16::MAX; 128]));
    expect!["25"].assert_eq(&encoded_bits!([1_u16; 2]));
    expect!["69"].assert_eq(&encoded_bits!([1_u16; 19]));
    expect!["80"].assert_eq(&encoded_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]));
}

macro_rules! impl_compact {
    ($t:ident, $context:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            leading_zeros: <ULessThan<{ $bits + 1 }> as Encode>::Context,
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
            fn default() -> Self {
                Self {
                    leading_zeros: Default::default(),
                    context: [Default::default(); $bits],
                }
            }
        }

        impl EncodingStrategy<$t> for Small {
            type Context = $context;
            fn encode<W: Write>(
                value: &$t,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let uleading = value.leading_zeros() as usize;
                let leading_zeros = ULessThan::<{ $bits + 1 }>::new(uleading);
                leading_zeros.encode(writer, &mut ctx.leading_zeros)?;
                if uleading >= $bits - 1 {
                    return Ok(());
                }
                for i in 0..($bits - 1) - uleading {
                    ((value >> i) & 1 == 1).encode(writer, &mut ctx.context[i])?;
                }
                Ok(())
            }
            fn millibits(value: &$t, ctx: &mut Self::Context) -> Option<usize> {
                let uleading = value.leading_zeros() as usize;
                let leading_zeros = ULessThan::<{ $bits + 1 }>::new(uleading);
                let mut tot = leading_zeros.millibits(&mut ctx.leading_zeros)?;
                if uleading >= $bits - 1 {
                    return Some(tot);
                }
                for i in 0..($bits - 1) - uleading {
                    tot += ((value >> i) & 1 == 1).millibits(&mut ctx.context[i])?;
                }
                Some(tot)
            }
            fn decode<R: Read>(
                reader: &mut super::Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                let leading_zeros =
                    ULessThan::<{ $bits + 1 }>::decode(reader, &mut ctx.leading_zeros)?;
                let uleading = usize::from(leading_zeros);
                if uleading >= $bits {
                    return Ok(0);
                }
                let mut out = 1 << ($bits - 1 - uleading);
                for i in 0..($bits - 1) - uleading {
                    if bool::decode(reader, &mut ctx.context[i])? {
                        out |= 1 << i;
                    }
                }
                Ok(out)
            }
        }
    };
}

impl_compact!(u64, U64Compact, 64);
impl_compact!(u32, U32Compact, 32);
impl_compact!(u16, U16Compact, 16);

#[test]
fn compact_u16() {
    use super::encoded_bits;
    use crate::{Encoded, Small};
    expect!["2"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(0_u16)));
    expect!["5"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(1_u16)));
    expect!["5"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(2_u16)));
    expect!["5"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(3_u16)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(4_u16)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(5_u16)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(6_u16)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(7_u16)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(8_u16)));
    expect!["19"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(u16::MAX)));
    expect!["30"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(0_u16); 128]));
    expect!["128"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(u16::MAX); 128]));
    expect!["8"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(1_u16); 2]));
    expect!["22"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(1_u16); 19]));
    expect!["28"].assert_eq(&encoded_bits!([
        0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
    ]
    .map(Encoded::<_, Small>::new)));
}

#[test]
fn compact_u32() {
    use super::encoded_bits;
    use crate::{Encoded, Small};
    expect!["3"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(0_u32)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(1_u32)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(2_u32)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(3_u32)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(4_u32)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(5_u32)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(6_u32)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(7_u32)));
    expect!["8"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(8_u32)));
    expect!["36"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(u32::MAX)));
    expect!["40"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(0_u32); 128]));
    expect!["242"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(u32::MAX); 128]));
    expect!["10"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(1_u32); 2]));
    expect!["26"].assert_eq(&encoded_bits!([Encoded::<_, Small>::new(1_u32); 19]));
    expect!["33"].assert_eq(&encoded_bits!([
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
    ($signed:ident, $unsigned:ident, $mod:ident) => {
        mod $mod {
            use super::*;
            impl Encode for $signed {
                type Context = <$unsigned as Encode>::Context;
                #[inline]
                fn encode<W: Write>(
                    &self,
                    writer: &mut super::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    $unsigned::from_le_bytes(self.to_le_bytes()).encode(writer, ctx)
                }
                #[inline]
                fn decode<R: Read>(
                    reader: &mut super::Reader<R>,
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
                fn encode<W: Write>(
                    value: &$signed,
                    writer: &mut super::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    (*value < 0).encode(writer, &mut ctx.is_negative)?;
                    if *value < 0 {
                        Small::encode(&value.abs_diff(-1), writer, &mut ctx.negative)
                    } else {
                        Small::encode(&value.abs_diff(0), writer, &mut ctx.positive)
                    }
                }
                fn millibits(value: &$signed, ctx: &mut Self::Context) -> Option<usize> {
                    let mut tot = (*value < 0).millibits(&mut ctx.is_negative)?;
                    if *value < 0 {
                        tot += Small::millibits(&value.abs_diff(-1), &mut ctx.negative)?;
                    } else {
                        tot += Small::millibits(&value.abs_diff(0), &mut ctx.positive)?;
                    }
                    Some(tot)
                }
                #[inline]
                fn decode<R: Read>(
                    reader: &mut super::Reader<R>,
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
                fn encode<W: Write>(
                    value: &$signed,
                    writer: &mut super::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    if let Some(previous) = ctx.previous.take() {
                        let not_sorted = *value < previous;
                        not_sorted.encode(writer, &mut ctx.not_sorted)?;
                        if not_sorted {
                            Small::encode(value, writer, &mut ctx.value)?;
                        } else {
                            Small::encode(
                                &(value.abs_diff(previous)),
                                writer,
                                &mut ctx.difference,
                            )?;
                        }
                    } else {
                        Small::encode(value, writer, &mut ctx.value)?;
                    }
                    ctx.previous = Some(*value);
                    Ok(())
                }
                fn decode<R: Read>(
                    reader: &mut super::Reader<R>,
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
    use super::{assert_bits_all, assert_size, encoded_bits};
    use crate::{Encoded, Small};
    use std::collections::BTreeSet;

    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(0_i32)));
    expect!["7"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(1_i32)));
    expect!["3"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(-1_i32)));
    expect!["36"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i32::MAX)));
    expect!["36"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i32::MIN)));
    assert_bits_all!(
        [i32::MIN, i32::MAX, 0, 1, 7, 137, i32::MAX - 1],
        expect!["32"]
    );
    expect!["12"].assert_eq(&encoded_bits!(-1i32));

    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(0_i16)));
    expect!["6"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(1_i16)));
    expect!["3"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(-1_i16)));
    expect!["19"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i16::MAX)));
    expect!["19"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i16::MIN)));
    assert_bits_all!(
        [i16::MIN, i16::MAX, 0, 1, 7, 137, i16::MAX - 1],
        expect!["16"]
    );

    expect!["8"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(0_i64)));
    expect!["8"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(1_i64)));
    expect!["3"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(-1_i64)));
    expect!["68"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i64::MAX)));
    expect!["68"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(i64::MIN)));
    assert_bits_all!([i64::MIN, 0, 1, 7, 137, i64::MAX - 1], expect!["64"]);
    expect!["25"].assert_eq(&encoded_bits!(-1i64));

    assert_size!(BTreeSet::from([-1i16, 0, 1, 2]), expect!["4"]);
    assert_size!(BTreeSet::from([i16::MIN, i16::MAX]), expect!["6"]);
}
