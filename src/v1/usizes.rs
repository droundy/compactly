use crate::Sorted;

use super::{byte::UBits, Encode, EncodingStrategy, Small, ULessThan};
use std::io::{Read, Write};

#[derive(Default, Clone)]
pub struct UsizeContext {
    less_than_four: <bool as Encode>::Context,
    small: <ULessThan<4> as Encode>::Context,
    big: <Small as EncodingStrategy<u64>>::Context,
}

impl Encode for usize {
    type Context = UsizeContext;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if let Ok(r) = ULessThan::<4>::try_from(*self) {
            true.encode(writer, &mut ctx.less_than_four)?;
            r.encode(writer, &mut ctx.small)
        } else {
            false.encode(writer, &mut ctx.less_than_four)?;
            Small::encode(&((*self - 4) as u64), writer, &mut ctx.big)
        }
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.less_than_four)? {
            ULessThan::<4>::decode(reader, &mut ctx.small).map(usize::from)
        } else {
            let v: u64 = Small::decode(reader, &mut ctx.big)?;
            usize::try_from(v + 4).map_err(std::io::Error::other)
        }
    }

    #[inline]
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = 0;
        if let Ok(r) = ULessThan::<4>::try_from(*self) {
            tot += true.millibits(&mut ctx.less_than_four)?;
            tot += r.millibits(&mut ctx.small)?;
        } else {
            tot += false.millibits(&mut ctx.less_than_four)?;
            tot += Small::millibits(&(*self as u64 - 4), &mut ctx.big)?;
        }
        Some(tot)
    }
}

#[derive(Clone)]
pub struct SmallContext {
    small_nonzero: <UBits<3> as Encode>::Context,
    b1: <UBits<1> as Encode>::Context,
    b2: <UBits<2> as Encode>::Context,
    b3: <UBits<3> as Encode>::Context,
    b4: <UBits<4> as Encode>::Context,
    b5: <UBits<5> as Encode>::Context,
    bits_beyond_seven: <UBits<6> as Encode>::Context,
    bits: [<bool as Encode>::Context; 64],
}
impl Default for SmallContext {
    fn default() -> Self {
        SmallContext {
            small_nonzero: Default::default(),
            b1: Default::default(),
            b2: Default::default(),
            b3: Default::default(),
            b4: Default::default(),
            b5: Default::default(),
            bits_beyond_seven: Default::default(),
            bits: [Default::default(); 64],
        }
    }
}

impl EncodingStrategy<usize> for Small {
    type Context = SmallContext;
    fn encode<W: Write>(
        value: &usize,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let nonzero: UBits<3>;
        match *value {
            0 => {
                nonzero = 0.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)
            }
            1 => {
                nonzero = 1.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)
            }
            2..4 => {
                nonzero = 2.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let b1: UBits<1> = (*value as u8 - 2).try_into().unwrap();
                b1.encode(writer, &mut ctx.b1)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let b2: UBits<2> = (*value as u8 - 4).try_into().unwrap();
                b2.encode(writer, &mut ctx.b2)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let b3: UBits<3> = (*value as u8 - 8).try_into().unwrap();
                b3.encode(writer, &mut ctx.b3)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let b4: UBits<4> = (*value as u8 - 16).try_into().unwrap();
                b4.encode(writer, &mut ctx.b4)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let b5: UBits<5> = (*value as u8 - 32).try_into().unwrap();
                b5.encode(writer, &mut ctx.b5)
            }
            _ => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero)?;
                let value = *value as u64;
                let zeros = value.leading_zeros() as u8;
                let bits_beyond_seven = 64 - 7 - zeros;
                let bits_beyond_seven: UBits<6> = bits_beyond_seven.try_into().unwrap();
                bits_beyond_seven.encode(writer, &mut ctx.bits_beyond_seven)?;
                for off in 0..6 + u8::from(bits_beyond_seven) {
                    ((value >> off) & 1 == 1).encode(writer, &mut ctx.bits[off as usize])?;
                }
                Ok(())
            }
        }
    }
    fn millibits(value: &usize, ctx: &mut Self::Context) -> Option<usize> {
        let nonzero: UBits<3>;
        match *value {
            0 => {
                nonzero = 0.try_into().unwrap();
                nonzero.millibits(&mut ctx.small_nonzero)
            }
            1 => {
                nonzero = 1.try_into().unwrap();
                nonzero.millibits(&mut ctx.small_nonzero)
            }
            2..4 => {
                nonzero = 2.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let b1: UBits<1> = (*value as u8 - 2).try_into().unwrap();
                Some(tot + b1.millibits(&mut ctx.b1)?)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let b2: UBits<2> = (*value as u8 - 4).try_into().unwrap();
                Some(tot + b2.millibits(&mut ctx.b2)?)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let b3: UBits<3> = (*value as u8 - 8).try_into().unwrap();
                Some(tot + b3.millibits(&mut ctx.b3)?)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let b4: UBits<4> = (*value as u8 - 16).try_into().unwrap();
                Some(tot + b4.millibits(&mut ctx.b4)?)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let b5: UBits<5> = (*value as u8 - 32).try_into().unwrap();
                Some(tot + b5.millibits(&mut ctx.b5)?)
            }
            _ => {
                nonzero = 7.try_into().unwrap();
                let mut tot = nonzero.millibits(&mut ctx.small_nonzero)?;
                let value = *value as u64;
                let zeros = value.leading_zeros() as u8;
                let bits_beyond_seven = 64 - 7 - zeros;
                let bits_beyond_seven: UBits<6> = bits_beyond_seven.try_into().unwrap();
                tot += bits_beyond_seven.millibits(&mut ctx.bits_beyond_seven)?;
                for off in 0..6 + u8::from(bits_beyond_seven) {
                    tot += ((value >> off) & 1 == 1).millibits(&mut ctx.bits[off as usize])?;
                }
                Some(tot)
            }
        }
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<usize, std::io::Error> {
        let nz = u8::from(<UBits<3> as Encode>::decode(
            reader,
            &mut ctx.small_nonzero,
        )?);
        match nz {
            0 => Ok(0),
            1 => Ok(1),
            2 => {
                let rest: u8 = <UBits<1> as Encode>::decode(reader, &mut ctx.b1)?.into();
                Ok(rest as usize + 2)
            }
            3 => {
                let rest: u8 = <UBits<2> as Encode>::decode(reader, &mut ctx.b2)?.into();
                Ok(rest as usize + 4)
            }
            4 => {
                let rest: u8 = <UBits<3> as Encode>::decode(reader, &mut ctx.b3)?.into();
                Ok(rest as usize + 8)
            }
            5 => {
                let rest: u8 = <UBits<4> as Encode>::decode(reader, &mut ctx.b4)?.into();
                Ok(rest as usize + 16)
            }
            6 => {
                let rest: u8 = <UBits<5> as Encode>::decode(reader, &mut ctx.b5)?.into();
                Ok(rest as usize + 32)
            }
            7 => {
                // 7 means we have a large number, so we just have to store it as usual.
                let bits: u8 =
                    <UBits<6> as Encode>::decode(reader, &mut ctx.bits_beyond_seven)?.into();
                let bits = 6 + bits;
                let mut out = 1_usize << bits;
                for off in 0..bits {
                    let b = <bool as Encode>::decode(reader, &mut ctx.bits[off as usize])?;
                    out |= (b as usize) << off;
                }
                Ok(out)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Clone)]
pub struct SortedContext {
    previous: Option<usize>,
    not_sorted: <bool as Encode>::Context,
    value: <Small as EncodingStrategy<usize>>::Context,
    difference: <Small as EncodingStrategy<usize>>::Context,
}

impl EncodingStrategy<usize> for Sorted {
    type Context = SortedContext;
    fn encode<W: Write>(
        value: &usize,
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
    ) -> Result<usize, std::io::Error> {
        let out = if let Some(previous) = ctx.previous.take() {
            let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
            if not_sorted {
                Small::decode(reader, &mut ctx.value)?
            } else {
                previous + <Small as EncodingStrategy<usize>>::decode(reader, &mut ctx.difference)?
            }
        } else {
            Small::decode(reader, &mut ctx.value)?
        };
        ctx.previous = Some(out);
        Ok(out)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    use crate::Encoded;
    assert_bits!(Encoded::<_, Small>::new(0_u64), @"3");
    assert_bits!(0_usize, @"3");
    assert_bits!(Encoded::<_, Small>::new(1_u64), @"7");
    assert_bits!(1_usize, @"3");
    assert_bits!(Encoded::<_, Small>::new(2_u64), @"7");
    assert_bits!(2_usize, @"3");
    assert_bits!(3_usize, @"1");
    assert_bits!(4_usize, @"8");
    assert_bits!(5_usize, @"8");
    assert_bits!(6_usize, @"8");
    assert_bits!(7_usize, @"8");
    assert_bits!(8_usize, @"9");
    assert_bits!(Encoded::<_, Small>::new(16_u64), @"10");
    assert_bits!(16_usize, @"10");
    assert_bits!(Encoded::<_, Small>::new(32_u64), @"11");
    assert_bits!(32_usize, @"11");
    assert_bits!(Encoded::<_, Small>::new(64_u64), @"12");
    assert_bits!(64_usize, @"12");
    assert_bits!(Encoded::<_, Small>::new(128_u64), @"13");
    assert_bits!(128_usize, @"13");
    assert_bits!(Encoded::<_, Small>::new(256_u64), @"14");
    assert_bits!(256_usize, @"14");
    assert_bits!(512_usize, @"15");
    assert_bits!(Encoded::<_, Small>::new(1024_u64), @"16");
    assert_bits!(1024_usize, @"16");
    assert_bits!(Encoded::<_, Small>::new(1024_u64 * 1024), @"26");
    assert_bits!(1024_usize * 1024, @"26");
    assert_bits!(1024_usize * 1024 * 1024, @"36");
    assert_bits!(u32::MAX as usize, @"38");
    // Note the code will work for u32, but the following two tests will fail.
    assert_bits!(1024_usize * 1024 * 1024 * 1024, @"46");
    assert_bits!(1024_usize * 1024 * 1024 * 1024 * 1024, @"56");
    assert_bits!([0_usize; 128], @"20");
    assert_bits!([1_usize; 19], @"13");
    assert_bits!([0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], @"19");
}

#[test]
fn correctness() {
    use crate::Encoded;
    for v in (0..u16::MAX as usize)
        .chain((0..u16::MAX as usize).map(|i| u32::MAX as usize - i))
        .chain((0..u16::MAX as usize).map(|i| u32::MAX as usize + i))
        .chain((0..u16::MAX as usize).map(|i| usize::MAX - i))
    {
        let encoded = super::encode(&v);
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(v));
        let encoded = super::encode(&Encoded::<_, Small>::new(v));
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(Encoded::<_, Small>::new(v)));
    }
}

#[test]
fn small() {
    use crate::Encoded;
    fn small_size(vals: impl IntoIterator<Item = usize>) -> usize {
        let mut sizes = vals.into_iter().map(|v| {
            println!("Checking {v}");
            let bits = super::encoded_bits!(Encoded::<_, Small>::new(v));
            assert_eq!(
                Encoded::<_, Small>::new(v).millibits(&mut Default::default()),
                Some(1000 * bits),
                "small wrong size"
            );
            (v, bits)
        });
        let (_, bits) = sizes.next().expect("small_size needs at least one value");
        for (v, other) in sizes {
            assert_eq!(other, bits, "encoded size differs for {v}");
        }
        bits
    }
    fn normal_size(v: usize) -> usize {
        let bits = super::encoded_bits!(v);
        assert_eq!(
            v.millibits(&mut Default::default()),
            Some(1000 * bits),
            "normal wrong size"
        );
        bits
    }
    fn both_sizes(v: usize) -> String {
        format!(
            "small: {} bits, normal: {} bits",
            small_size([v]),
            normal_size(v)
        )
    }

    insta::assert_snapshot!(both_sizes(0), @"small: 3 bits, normal: 3 bits");
    insta::assert_snapshot!(both_sizes(1), @"small: 3 bits, normal: 3 bits");
    insta::assert_snapshot!(both_sizes(2), @"small: 4 bits, normal: 3 bits");
    insta::assert_snapshot!(both_sizes(4), @"small: 5 bits, normal: 8 bits");
    insta::assert_snapshot!(both_sizes(5), @"small: 5 bits, normal: 8 bits");
    insta::assert_snapshot!(both_sizes(23), @"small: 7 bits, normal: 11 bits");
    insta::assert_snapshot!(both_sizes(37), @"small: 8 bits, normal: 12 bits");
    insta::assert_snapshot!(both_sizes(63), @"small: 8 bits, normal: 12 bits");
    insta::assert_snapshot!(both_sizes(117), @"small: 15 bits, normal: 13 bits");
    insta::assert_snapshot!(both_sizes(u32::MAX as usize), @"small: 40 bits, normal: 38 bits");
    insta::assert_snapshot!(small_size(0..2), @"3");
    insta::assert_snapshot!(small_size(2..4), @"4");
    insta::assert_snapshot!(small_size(4..8), @"5");
    insta::assert_snapshot!(small_size(8..16), @"6");
    insta::assert_snapshot!(small_size(16..32), @"7");
    insta::assert_snapshot!(small_size(32..64), @"8");
    insta::assert_snapshot!(small_size(64..128), @"15");
    insta::assert_snapshot!(small_size(128..256), @"16");
    insta::assert_snapshot!(small_size(256..512), @"17");
}
