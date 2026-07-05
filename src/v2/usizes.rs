use crate::Sorted;

use super::{
    byte::UBits, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder, Small, ULessThan,
};

#[derive(Default, Clone)]
pub struct UsizeContext {
    less_than_four: <bool as Encode>::Context,
    small: <ULessThan<4> as Encode>::Context,
    big: <Small as EncodingStrategy<u64>>::Context,
}

impl Encode for usize {
    type Context = UsizeContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        if let Ok(r) = ULessThan::<4>::try_from(*self) {
            true.encode(writer, &mut ctx.less_than_four);
            r.encode(writer, &mut ctx.small)
        } else {
            false.encode(writer, &mut ctx.less_than_four);
            Small::encode(&((*self - 4) as u64), writer, &mut ctx.big)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.less_than_four)? {
            ULessThan::<4>::decode(reader, &mut ctx.small).map(usize::from)
        } else {
            let v: u64 = Small::decode(reader, &mut ctx.big)?;
            usize::try_from(v + 4).map_err(std::io::Error::other)
        }
    }
}

#[derive(Clone, Default)]
pub struct SmallContext {
    small_nonzero: <UBits<3> as Encode>::Context,
    b1: <UBits<1> as Encode>::Context,
    b2: <UBits<2> as Encode>::Context,
    b3: <UBits<3> as Encode>::Context,
    b4: <UBits<4> as Encode>::Context,
    b5: <UBits<5> as Encode>::Context,
    // Values >= 64 are delegated to Small<u64> (UBits + incompressible bytes).
    large: <Small as EncodingStrategy<u64>>::Context,
}

impl EncodingStrategy<usize> for Small {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &usize, writer: &mut E, ctx: &mut Self::Context) {
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
                nonzero.encode(writer, &mut ctx.small_nonzero);
                let b1: UBits<1> = (*value as u8 - 2).try_into().unwrap();
                b1.encode(writer, &mut ctx.b1)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero);
                let b2: UBits<2> = (*value as u8 - 4).try_into().unwrap();
                b2.encode(writer, &mut ctx.b2)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero);
                let b3: UBits<3> = (*value as u8 - 8).try_into().unwrap();
                b3.encode(writer, &mut ctx.b3)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero);
                let b4: UBits<4> = (*value as u8 - 16).try_into().unwrap();
                b4.encode(writer, &mut ctx.b4)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero);
                let b5: UBits<5> = (*value as u8 - 32).try_into().unwrap();
                b5.encode(writer, &mut ctx.b5)
            }
            _ => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.small_nonzero);
                Small::encode(&(*value as u64 - 64), writer, &mut ctx.large);
            }
        }
    }

    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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
                let v: u64 = Small::decode(reader, &mut ctx.large)?;
                Ok(v as usize + 64)
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
    fn encode<E: super::EntropyCoder>(value: &usize, writer: &mut E, ctx: &mut Self::Context) {
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
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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
    use super::raw_bits;
    use crate::Encoded;
    raw_bits!(Encoded::<_, Small>::new(0_u64), @"6 bits");
    raw_bits!(0_usize, @"3 bits");
    raw_bits!(Encoded::<_, Small>::new(1_u64), @"6 bits");
    raw_bits!(1_usize, @"3 bits");
    raw_bits!(Encoded::<_, Small>::new(2_u64), @"7 bits");
    raw_bits!(2_usize, @"3 bits");
    raw_bits!(3_usize, @"3 bits");
    raw_bits!(4_usize, @"7 bits");
    raw_bits!(5_usize, @"7 bits");
    raw_bits!(6_usize, @"8 bits");
    raw_bits!(7_usize, @"8 bits");
    raw_bits!(8_usize, @"9 bits");
    raw_bits!(Encoded::<_, Small>::new(16_u64), @"10 bits");
    raw_bits!(16_usize, @"10 bits");
    raw_bits!(Encoded::<_, Small>::new(32_u64), @"11 bits");
    raw_bits!(32_usize, @"11 bits");
    raw_bits!(Encoded::<_, Small>::new(64_u64), @"12 bits");
    raw_bits!(64_usize, @"12 bits");
    raw_bits!(Encoded::<_, Small>::new(128_u64), @"13 bits");
    raw_bits!(128_usize, @"13 bits");
    raw_bits!(Encoded::<_, Small>::new(256_u64), @"14 bits");
    raw_bits!(256_usize, @"14 bits");
    raw_bits!(512_usize, @"15 bits");
    raw_bits!(Encoded::<_, Small>::new(1024_u64), @"16 bits");
    raw_bits!(1024_usize, @"16 bits");
    raw_bits!(Encoded::<_, Small>::new(1024_u64 * 1024), @"26 bits");
    raw_bits!(1024_usize * 1024, @"26 bits");
    raw_bits!(1024_usize * 1024 * 1024, @"36 bits");
    raw_bits!(u32::MAX as usize, @"38 bits");
    // Note the code will work for u32, but the following two tests will fail.
    raw_bits!(1024_usize * 1024 * 1024 * 1024, @"46 bits");
    raw_bits!(1024_usize * 1024 * 1024 * 1024 * 1024, @"56 bits");
    raw_bits!([0_usize; 128], @"384 bits, entropy Millibits(20013)");
    raw_bits!([1_usize; 19], @"57 bits, entropy Millibits(12834)");
    raw_bits!([0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], @"78 bits, entropy Millibits(18846)");
}

#[test]
fn small() {
    use crate::Encoded;
    fn small_size(vals: impl IntoIterator<Item = usize>) -> usize {
        let mut sizes = vals.into_iter().map(|v| {
            println!("Checking {v}");
            let val = Encoded::<_, Small>::new(v);
            let encoded = super::Raw::encode(&val);
            let decoded = super::Raw::decode(&encoded);
            assert_eq!(
                decoded,
                Some(Encoded::<_, Small>::new(v)),
                "raw round-trip failed for {v}"
            );
            let (bits, entropy) = super::Raw::sizes(&val);
            assert_eq!(
                entropy,
                super::Millibits::bits(bits),
                "entropy disagrees with raw size for {v}"
            );
            assert_eq!(
                val.millibits(),
                super::Millibits::bits(bits),
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
        let encoded = super::Raw::encode(&v);
        let decoded = super::Raw::decode(&encoded);
        assert_eq!(decoded, Some(v), "raw round-trip failed for {v}");
        let (bits, entropy) = super::Raw::sizes(&v);
        assert_eq!(
            entropy,
            super::Millibits::bits(bits),
            "entropy disagrees with raw size for {v}"
        );
        assert_eq!(
            v.millibits(),
            super::Millibits::bits(bits),
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
    insta::assert_snapshot!(both_sizes(4), @"small: 5 bits, normal: 7 bits");
    insta::assert_snapshot!(both_sizes(5), @"small: 5 bits, normal: 7 bits");
    insta::assert_snapshot!(both_sizes(23), @"small: 7 bits, normal: 11 bits");
    insta::assert_snapshot!(both_sizes(37), @"small: 8 bits, normal: 12 bits");
    insta::assert_snapshot!(both_sizes(63), @"small: 8 bits, normal: 12 bits");
    insta::assert_snapshot!(both_sizes(117), @"small: 14 bits, normal: 13 bits");
    insta::assert_snapshot!(both_sizes(u32::MAX as usize), @"small: 40 bits, normal: 38 bits");
    insta::assert_snapshot!(small_size(0..2), @"3");
    insta::assert_snapshot!(small_size(2..4), @"4");
    insta::assert_snapshot!(small_size(4..8), @"5");
    insta::assert_snapshot!(small_size(8..16), @"6");
    insta::assert_snapshot!(small_size(16..32), @"7");
    insta::assert_snapshot!(small_size(32..64), @"8");
    insta::assert_snapshot!(small_size(64..66), @"9");
    insta::assert_snapshot!(small_size(66..68), @"10");
    insta::assert_snapshot!(small_size(68..72), @"11");
    insta::assert_snapshot!(small_size(72..80), @"12");
    insta::assert_snapshot!(small_size(80..96), @"13");
    insta::assert_snapshot!(small_size(96..128), @"14");
    insta::assert_snapshot!(small_size(128..192), @"15");
    insta::assert_snapshot!(small_size(192..320), @"16");
    insta::assert_snapshot!(small_size(320..512), @"17");
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

        let encoded = super::Ans::encode(&v);
        let decoded = super::Ans::decode(&encoded);
        assert_eq!(decoded, Some(v));

        let encoded = super::Range::encode(&v);
        let decoded = super::Range::decode(&encoded);
        assert_eq!(decoded, Some(v));
    }
}

#[derive(Default, Clone)]
pub struct IsizeContext {
    is_negative: <bool as Encode>::Context,
    magnitude: <usize as Encode>::Context,
}

impl Encode for isize {
    type Context = IsizeContext;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let is_neg = *self < 0;
        is_neg.encode(writer, &mut ctx.is_negative);
        let mag: usize = if is_neg {
            self.abs_diff(-1)
        } else {
            self.abs_diff(0)
        };
        mag.encode(writer, &mut ctx.magnitude);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let is_neg = bool::decode(reader, &mut ctx.is_negative)?;
        let mag = usize::decode(reader, &mut ctx.magnitude)?;
        if is_neg {
            Ok(-1 - mag as isize)
        } else {
            Ok(mag as isize)
        }
    }
}

#[derive(Default, Clone)]
pub struct IsizeSmallContext {
    is_negative: <bool as Encode>::Context,
    positive: <Small as EncodingStrategy<usize>>::Context,
    negative: <Small as EncodingStrategy<usize>>::Context,
}

impl EncodingStrategy<isize> for Small {
    type Context = IsizeSmallContext;
    #[inline]
    fn encode<E: EntropyCoder>(value: &isize, writer: &mut E, ctx: &mut Self::Context) {
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
    ) -> Result<isize, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_negative)? {
            let p: usize = Small::decode(reader, &mut ctx.negative)?;
            Ok(-1 - p as isize)
        } else {
            let p: usize = Small::decode(reader, &mut ctx.positive)?;
            Ok(p as isize)
        }
    }
}

#[derive(Default, Clone)]
pub struct IsizeSortedContext {
    previous: Option<isize>,
    not_sorted: <bool as Encode>::Context,
    value: <Small as EncodingStrategy<isize>>::Context,
    difference: <Small as EncodingStrategy<usize>>::Context,
}

impl EncodingStrategy<isize> for Sorted {
    type Context = IsizeSortedContext;
    fn encode<E: EntropyCoder>(value: &isize, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(previous) = ctx.previous.take() {
            let not_sorted = *value < previous;
            not_sorted.encode(writer, &mut ctx.not_sorted);
            if not_sorted {
                Small::encode(value, writer, &mut ctx.value);
            } else {
                Small::encode(&value.abs_diff(previous), writer, &mut ctx.difference);
            }
        } else {
            Small::encode(value, writer, &mut ctx.value);
        }
        ctx.previous = Some(*value);
    }
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<isize, std::io::Error> {
        let out = if let Some(previous) = ctx.previous.take() {
            let not_sorted = bool::decode(reader, &mut ctx.not_sorted)?;
            if not_sorted {
                Small::decode(reader, &mut ctx.value)?
            } else {
                let diff: usize = Small::decode(reader, &mut ctx.difference)?;
                previous.wrapping_add(diff as isize)
            }
        } else {
            Small::decode(reader, &mut ctx.value)?
        };
        ctx.previous = Some(out);
        Ok(out)
    }
}

#[test]
fn isize_correctness() {
    use crate::Encoded;
    for v in (0..u16::MAX as isize)
        .chain((0..u16::MAX as isize).map(|i| -(i + 1)))
        .chain((0..u16::MAX as isize).map(|i| isize::MAX - i))
        .chain((0..u16::MAX as isize).map(|i| isize::MIN + i))
    {
        let encoded = super::encode(&v);
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(v));

        let encoded = super::encode(&Encoded::<_, Small>::new(v));
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(Encoded::<_, Small>::new(v)));
    }
}
