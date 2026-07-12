use crate::Sorted;

use super::{AtMost, Encode, EncodingStrategy, EntropyCoder, EntropyDecoder, Small};

#[cfg(test)]
use expect_test::expect;

#[derive(Default, Clone)]
pub struct UsizeContext {
    less_than_four: <bool as Encode>::Context,
    small: <AtMost<3> as Encode>::Context,
    big: <Small as EncodingStrategy<u64>>::Context,
}

impl Encode for usize {
    type Context = UsizeContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        if let Ok(r) = AtMost::<3>::try_from(*self) {
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
            AtMost::<3>::decode(reader, &mut ctx.small).map(usize::from)
        } else {
            let v: u64 = Small::decode(reader, &mut ctx.big)?;
            usize::try_from(v + 4).map_err(std::io::Error::other)
        }
    }
}

#[derive(Clone, Default)]
pub struct SmallContext {
    small_nonzero: <AtMost<7> as Encode>::Context,
    b1: <AtMost<1> as Encode>::Context,
    b2: <AtMost<3> as Encode>::Context,
    b3: <AtMost<7> as Encode>::Context,
    b4: <AtMost<15> as Encode>::Context,
    b5: <AtMost<31> as Encode>::Context,
    // Values >= 64 are delegated to Small<u64>.
    large: <Small as EncodingStrategy<u64>>::Context,
}

impl EncodingStrategy<usize> for Small {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &usize, writer: &mut E, ctx: &mut Self::Context) {
        // A 3-bit bucket code, then the value's offset into the bucket.
        let bucket = |code: usize| AtMost::<7>::new(code);
        match *value {
            0 => bucket(0).encode(writer, &mut ctx.small_nonzero),
            1 => bucket(1).encode(writer, &mut ctx.small_nonzero),
            2..4 => {
                bucket(2).encode(writer, &mut ctx.small_nonzero);
                AtMost::<1>::new(*value - 2).encode(writer, &mut ctx.b1)
            }
            4..8 => {
                bucket(3).encode(writer, &mut ctx.small_nonzero);
                AtMost::<3>::new(*value - 4).encode(writer, &mut ctx.b2)
            }
            8..16 => {
                bucket(4).encode(writer, &mut ctx.small_nonzero);
                AtMost::<7>::new(*value - 8).encode(writer, &mut ctx.b3)
            }
            16..32 => {
                bucket(5).encode(writer, &mut ctx.small_nonzero);
                AtMost::<15>::new(*value - 16).encode(writer, &mut ctx.b4)
            }
            32..64 => {
                bucket(6).encode(writer, &mut ctx.small_nonzero);
                AtMost::<31>::new(*value - 32).encode(writer, &mut ctx.b5)
            }
            _ => {
                bucket(7).encode(writer, &mut ctx.small_nonzero);
                Small::encode(&(*value as u64 - 64), writer, &mut ctx.large);
            }
        }
    }

    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<usize, std::io::Error> {
        let nz = usize::from(AtMost::<7>::decode(reader, &mut ctx.small_nonzero)?);
        match nz {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(usize::from(AtMost::<1>::decode(reader, &mut ctx.b1)?) + 2),
            3 => Ok(usize::from(AtMost::<3>::decode(reader, &mut ctx.b2)?) + 4),
            4 => Ok(usize::from(AtMost::<7>::decode(reader, &mut ctx.b3)?) + 8),
            5 => Ok(usize::from(AtMost::<15>::decode(reader, &mut ctx.b4)?) + 16),
            6 => Ok(usize::from(AtMost::<31>::decode(reader, &mut ctx.b5)?) + 32),
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
    use super::assert_millibits;
    use crate::Encoded;
    assert_millibits!(Encoded::<_, Small>::new(0_u64), expect!["6 bits"]);
    assert_millibits!(0_usize, expect!["3 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(1_u64), expect!["6 bits"]);
    assert_millibits!(1_usize, expect!["3 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(2_u64), expect!["7 bits"]);
    assert_millibits!(2_usize, expect!["3 bits"]);
    assert_millibits!(3_usize, expect!["3 bits"]);
    assert_millibits!(4_usize, expect!["7 bits"]);
    assert_millibits!(5_usize, expect!["7 bits"]);
    assert_millibits!(6_usize, expect!["8 bits"]);
    assert_millibits!(7_usize, expect!["8 bits"]);
    assert_millibits!(8_usize, expect!["9 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(16_u64), expect!["10 bits"]);
    assert_millibits!(16_usize, expect!["10 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(32_u64), expect!["11 bits"]);
    assert_millibits!(32_usize, expect!["11 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(64_u64), expect!["12 bits"]);
    assert_millibits!(64_usize, expect!["12 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(128_u64), expect!["13 bits"]);
    assert_millibits!(128_usize, expect!["13 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(256_u64), expect!["14 bits"]);
    assert_millibits!(256_usize, expect!["14 bits"]);
    assert_millibits!(512_usize, expect!["15 bits"]);
    assert_millibits!(Encoded::<_, Small>::new(1024_u64), expect!["16 bits"]);
    assert_millibits!(1024_usize, expect!["16 bits"]);
    assert_millibits!(
        Encoded::<_, Small>::new(1024_u64 * 1024),
        expect!["26 bits"]
    );
    assert_millibits!(1024_usize * 1024, expect!["26 bits"]);
    assert_millibits!(1024_usize * 1024 * 1024, expect!["36 bits"]);
    assert_millibits!(u32::MAX as usize, expect!["38 bits"]);
    // Note the code will work for u32, but the following two tests will fail.
    assert_millibits!(1024_usize * 1024 * 1024 * 1024, expect!["46 bits"]);
    assert_millibits!(1024_usize * 1024 * 1024 * 1024 * 1024, expect!["56 bits"]);
    assert_millibits!([0_usize; 128], expect!["Millibits(20130)"]);
    assert_millibits!([1_usize; 19], expect!["Millibits(12844)"]);
    assert_millibits!(
        [0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        expect!["Millibits(18862)"]
    );
}

#[test]
fn small() {
    use crate::Encoded;
    fn small_size(vals: impl IntoIterator<Item = usize>) -> String {
        let mut sizes = vals.into_iter().map(|v| {
            println!("Checking {v}");
            let val = Encoded::<_, Small>::new(v);
            let encoded = super::encode(&val);
            let decoded = super::decode(&encoded);
            assert_eq!(
                decoded,
                Some(Encoded::<_, Small>::new(v)),
                "round-trip failed for {v}"
            );
            let entropy = val.millibits();
            let bits: usize = entropy.as_bits().parse().unwrap();
            assert_eq!(
                entropy,
                super::Millibits::bits(bits),
                "small size is not a whole number of bits for {v}"
            );
            (v, bits)
        });
        let (_, bits) = sizes.next().expect("small_size needs at least one value");
        for (v, other) in sizes {
            assert_eq!(other, bits, "encoded size differs for {v}");
        }
        bits.to_string()
    }
    fn normal_size(v: usize) -> usize {
        let encoded = super::encode(&v);
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(v), "round-trip failed for {v}");
        let entropy = v.millibits();
        let bits: usize = entropy.as_bits().parse().unwrap();
        assert_eq!(
            entropy,
            super::Millibits::bits(bits),
            "normal size is not a whole number of bits for {v}"
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

    expect!["small: 3 bits, normal: 3 bits"].assert_eq(&both_sizes(0));
    expect!["small: 3 bits, normal: 3 bits"].assert_eq(&both_sizes(1));
    expect!["small: 4 bits, normal: 3 bits"].assert_eq(&both_sizes(2));
    expect!["small: 5 bits, normal: 7 bits"].assert_eq(&both_sizes(4));
    expect!["small: 5 bits, normal: 7 bits"].assert_eq(&both_sizes(5));
    expect!["small: 7 bits, normal: 11 bits"].assert_eq(&both_sizes(23));
    expect!["small: 8 bits, normal: 12 bits"].assert_eq(&both_sizes(37));
    expect!["small: 8 bits, normal: 12 bits"].assert_eq(&both_sizes(63));
    expect!["small: 14 bits, normal: 13 bits"].assert_eq(&both_sizes(117));
    expect!["small: 40 bits, normal: 38 bits"].assert_eq(&both_sizes(u32::MAX as usize));
    expect!["3"].assert_eq(&small_size(0..2));
    expect!["4"].assert_eq(&small_size(2..4));
    expect!["5"].assert_eq(&small_size(4..8));
    expect!["6"].assert_eq(&small_size(8..16));
    expect!["7"].assert_eq(&small_size(16..32));
    expect!["8"].assert_eq(&small_size(32..64));
    expect!["9"].assert_eq(&small_size(64..66));
    expect!["10"].assert_eq(&small_size(66..68));
    expect!["11"].assert_eq(&small_size(68..72));
    expect!["12"].assert_eq(&small_size(72..80));
    expect!["13"].assert_eq(&small_size(80..96));
    expect!["14"].assert_eq(&small_size(96..128));
    expect!["15"].assert_eq(&small_size(128..192));
    expect!["16"].assert_eq(&small_size(192..320));
    expect!["17"].assert_eq(&small_size(320..512));
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
