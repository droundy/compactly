use super::atmost::AtMost;
use super::{Encode, Strategy};
use crate::{Incompressible, Normal, Small, Sorted};

#[cfg(test)]
use super::millibits;
#[cfg(test)]
use expect_test::expect;

impl Encode for u8 {
    type Context = <AtMost<255> as Encode>::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&AtMost::<255>::new(*value as usize), writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(usize::from(AtMost::<255>::decode(reader, ctx)?) as u8)
    }

    const MAX_BYTES: usize = <AtMost<255> as Encode>::MAX_BYTES;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        Ok(usize::from(<AtMost<255> as Encode>::decode_async(reader, ctx).await?) as u8)
    }
}

impl Encode for i8 {
    type Context = <u8 as Encode>::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&(*value as u8), writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        <u8 as Encode>::decode(reader, ctx).map(|v| v as i8)
    }

    /// Reinterpreted as a `u8`.
    const MAX_BYTES: usize = <u8 as Encode>::MAX_BYTES;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        Ok(<u8 as Encode>::decode_async(reader, ctx).await? as i8)
    }
}

#[derive(Default, Clone)]
pub struct SmallContext {
    nonzero: <AtMost<7> as Encode>::Context,
    b1: <AtMost<1> as Encode>::Context,
    b2: <AtMost<3> as Encode>::Context,
    b3: <AtMost<7> as Encode>::Context,
    b4: <AtMost<15> as Encode>::Context,
    b5: <AtMost<31> as Encode>::Context,
    need_seven_bits: <bool as Encode>::Context,
    b6: <AtMost<63> as Encode>::Context,
    b7: <AtMost<127> as Encode>::Context,
}

impl Encode<Small> for u8 {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &u8, writer: &mut E, ctx: &mut Self::Context) {
        // A 3-bit bucket code, then the value's offset into the bucket.
        let bucket = |code: usize| AtMost::<7>::new(code);
        let rest = |first: u8| (*value - first) as usize;
        match *value {
            0 => Normal::encode(&bucket(0), writer, &mut ctx.nonzero),
            1 => Normal::encode(&bucket(1), writer, &mut ctx.nonzero),
            2..4 => {
                Normal::encode(&bucket(2), writer, &mut ctx.nonzero);
                Normal::encode(&AtMost::<1>::new(rest(2)), writer, &mut ctx.b1)
            }
            4..8 => {
                Normal::encode(&bucket(3), writer, &mut ctx.nonzero);
                Normal::encode(&AtMost::<3>::new(rest(4)), writer, &mut ctx.b2)
            }
            8..16 => {
                Normal::encode(&bucket(4), writer, &mut ctx.nonzero);
                Normal::encode(&AtMost::<7>::new(rest(8)), writer, &mut ctx.b3)
            }
            16..32 => {
                Normal::encode(&bucket(5), writer, &mut ctx.nonzero);
                Normal::encode(&AtMost::<15>::new(rest(16)), writer, &mut ctx.b4)
            }
            32..64 => {
                Normal::encode(&bucket(6), writer, &mut ctx.nonzero);
                Normal::encode(&AtMost::<31>::new(rest(32)), writer, &mut ctx.b5)
            }
            64..128 => {
                Normal::encode(&bucket(7), writer, &mut ctx.nonzero);
                Normal::encode(&false, writer, &mut ctx.need_seven_bits);
                Normal::encode(&AtMost::<63>::new(rest(64)), writer, &mut ctx.b6)
            }
            128..=255 => {
                Normal::encode(&bucket(7), writer, &mut ctx.nonzero);
                Normal::encode(&true, writer, &mut ctx.need_seven_bits);
                Normal::encode(&AtMost::<127>::new(rest(128)), writer, &mut ctx.b7)
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        fn rest<const MAX: usize, D: super::EntropyDecoder>(
            reader: &mut D,
            ctx: &mut <AtMost<MAX> as Encode>::Context,
        ) -> Result<u8, std::io::Error> {
            Ok(usize::from(AtMost::<MAX>::decode(reader, ctx)?) as u8)
        }
        match usize::from(AtMost::<7>::decode(reader, &mut ctx.nonzero)?) {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(rest::<1, D>(reader, &mut ctx.b1)? + 2),
            3 => Ok(rest::<3, D>(reader, &mut ctx.b2)? + 4),
            4 => Ok(rest::<7, D>(reader, &mut ctx.b3)? + 8),
            5 => Ok(rest::<15, D>(reader, &mut ctx.b4)? + 16),
            6 => Ok(rest::<31, D>(reader, &mut ctx.b5)? + 32),
            7 => {
                if <bool as Encode>::decode(reader, &mut ctx.need_seven_bits)? {
                    Ok(rest::<127, D>(reader, &mut ctx.b7)? + 128)
                } else {
                    Ok(rest::<63, D>(reader, &mut ctx.b6)? + 64)
                }
            }
            _ => unreachable!(),
        }
    }

    /// A bucket symbol, then either an offset symbol or (top bucket) a bool
    /// plus an offset symbol.
    const MAX_BYTES: usize = <AtMost<7> as Encode>::MAX_BYTES
        + <bool as Encode>::MAX_BYTES
        + <AtMost<127> as Encode>::MAX_BYTES;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        async fn rest<const MAX: usize, D: super::AsyncEntropyDecoder>(
            reader: &mut D,
            ctx: &mut <AtMost<MAX> as Encode>::Context,
        ) -> Result<u8, std::io::Error> {
            Ok(usize::from(<AtMost<MAX> as Encode>::decode_async(reader, ctx).await?) as u8)
        }
        let bucket = <AtMost<7> as Encode>::decode_async(reader, &mut ctx.nonzero).await?;
        match usize::from(bucket) {
            0 => Ok(0),
            1 => Ok(1),
            2 => Ok(rest::<1, D>(reader, &mut ctx.b1).await? + 2),
            3 => Ok(rest::<3, D>(reader, &mut ctx.b2).await? + 4),
            4 => Ok(rest::<7, D>(reader, &mut ctx.b3).await? + 8),
            5 => Ok(rest::<15, D>(reader, &mut ctx.b4).await? + 16),
            6 => Ok(rest::<31, D>(reader, &mut ctx.b5).await? + 32),
            7 => {
                if <bool as Encode>::decode_async(reader, &mut ctx.need_seven_bits).await? {
                    Ok(rest::<127, D>(reader, &mut ctx.b7).await? + 128)
                } else {
                    Ok(rest::<63, D>(reader, &mut ctx.b6).await? + 64)
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Encode<Small> for i8 {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &i8, writer: &mut E, ctx: &mut Self::Context) {
        let v = *value as u8;
        // Zig-zag: 0→0, -1→1, 1→2, -2→3, 2→4, …, 127→254, -128→255
        let zigzag = (v << 1) ^ (0u8.wrapping_sub(v >> 7));
        Small::encode(&zigzag, writer, ctx)
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        let z = <u8 as Encode<Small>>::decode(reader, ctx)?;
        Ok(((z >> 1) as i8) ^ (-((z & 1) as i8)))
    }

    /// Zig-zagged into `Small<u8>`.
    const MAX_BYTES: usize = <u8 as Encode<Small>>::MAX_BYTES;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        let z = <u8 as Encode<Small>>::decode_async(reader, ctx).await?;
        Ok(((z >> 1) as i8) ^ (-((z & 1) as i8)))
    }
}

impl Encode<Incompressible> for u8 {
    type Context = ();
    fn encode<E: super::EntropyCoder>(value: &u8, writer: &mut E, _ctx: &mut Self::Context) {
        writer.encode_incompressible_bytes(&[*value])
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let mut byte = [0u8];
        reader.decode_incompressible_bytes(&mut byte)?;
        Ok(byte[0])
    }

    /// One byte, straight through.
    const MAX_BYTES: usize = std::mem::size_of::<u8>();

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let mut byte = [0u8];
        reader.decode_incompressible_bytes(&mut byte).await?;
        Ok(byte[0])
    }
}

#[derive(Default, Clone)]
pub struct SortedU8Context {
    previous: Option<u8>,
    delta: <i8 as Encode<Small>>::Context,
}

impl Encode<Sorted> for u8 {
    type Context = SortedU8Context;
    fn encode<E: super::EntropyCoder>(value: &u8, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(previous) = ctx.previous.take() {
            // Wrapping delta always round-trips and always takes the short way
            // around the byte circle, so it fits in an i8 for every pair.
            Small::encode(
                &(value.wrapping_sub(previous) as i8),
                writer,
                &mut ctx.delta,
            );
        } else {
            // The first element has no `previous`; storing it raw is cheaper (no
            // adaptive context to allocate) and there is no neighbor to predict it.
            writer.encode_incompressible_bytes(&[*value]);
        }
        ctx.previous = Some(*value);
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let out = if let Some(previous) = ctx.previous.take() {
            let delta: i8 = Small::decode(reader, &mut ctx.delta)?;
            previous.wrapping_add(delta as u8)
        } else {
            let mut byte = [0u8];
            reader.decode_incompressible_bytes(&mut byte)?;
            byte[0]
        };
        ctx.previous = Some(out);
        Ok(out)
    }

    /// The first value is raw; every later one is a `Small<i8>` delta.
    const MAX_BYTES: usize = {
        let raw = std::mem::size_of::<u8>();
        let delta = <i8 as Encode<Small>>::MAX_BYTES;
        if delta > raw {
            delta
        } else {
            raw
        }
    };

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let out = if let Some(previous) = ctx.previous.take() {
            let delta: i8 = <i8 as Encode<Small>>::decode_async(reader, &mut ctx.delta).await?;
            previous.wrapping_add(delta as u8)
        } else {
            let mut byte = [0u8];
            reader.decode_incompressible_bytes(&mut byte).await?;
            byte[0]
        };
        ctx.previous = Some(out);
        Ok(out)
    }
}

impl Encode<Sorted> for i8 {
    type Context = SortedU8Context;
    fn encode<E: super::EntropyCoder>(value: &i8, writer: &mut E, ctx: &mut Self::Context) {
        Sorted::encode(&(*value as u8), writer, ctx)
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        <u8 as Encode<Sorted>>::decode(reader, ctx).map(|v| v as i8)
    }

    const MAX_BYTES: usize = <u8 as Encode<Sorted>>::MAX_BYTES;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        Ok(<u8 as Encode<Sorted>>::decode_async(reader, ctx).await? as i8)
    }
}

#[test]
fn size() {
    use super::{assert_bits_all, estimated_bits};
    expect!["8"].assert_eq(&estimated_bits!(u8::MAX));
    expect!["8"].assert_eq(&estimated_bits!(0_u8));
    assert_bits_all!(3_u8..255, expect!["8"]);
    expect!["31"].assert_eq(&estimated_bits!(*b"hello"));
    expect!["68"].assert_eq(&estimated_bits!(*b"hello world"));
    expect!["129"].assert_eq(&estimated_bits!(*b"hello world, hello world"));
    expect!["111"].assert_eq(&estimated_bits!(*b"hello hello, hello hello"));
    expect!["195"].assert_eq(&estimated_bits!(
        *b"hello hello, hello hello, hello hello, hello hello"
    ));
    expect!["37"].assert_eq(&estimated_bits!(*b"hhhhhhhhhhhhhhhhhhhhhhhh"));
    expect!["44"].assert_eq(&estimated_bits!(
        *b"hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
    ));
    expect!["8"].assert_eq(&estimated_bits!(*b"\0"));
    expect!["8"].assert_eq(&estimated_bits!(*b"\x01"));
    expect!["13"].assert_eq(&estimated_bits!(*b"\x01\x01"));
    expect!["19"].assert_eq(&estimated_bits!(*b"\x01\x01\x01\x01"));
    expect!["21"].assert_eq(&estimated_bits!(*b"\x01\x01\x01\x01\x01"));
    expect!["22"].assert_eq(&estimated_bits!(*b"\x01\x01\x01\x01\x01\x01"));
    expect!["25"].assert_eq(&estimated_bits!(*b"\x01\x02\x03\x04"));
    expect!["30"].assert_eq(&estimated_bits!(*b"\x01\x02\x03\x04\x05"));
    expect!["35"].assert_eq(&estimated_bits!(*b"\x01\x02\x03\x04\x05\x06"));
    expect!["40"].assert_eq(&estimated_bits!(*b"\x01\x02\x03\x04\x05\x06\x07"));
    expect!["47"].assert_eq(&estimated_bits!(*b"\x01\x02\x03\x04\x05\x06\x07\x08"));

    expect!["8"].assert_eq(&estimated_bits!(i8::MAX));
    expect!["8"].assert_eq(&estimated_bits!(0_i8));
}

#[test]
fn small() {
    use super::Small;
    use crate::Encoded;
    fn size_of(vals: impl IntoIterator<Item = u8>) -> String {
        let mut sizes = vals.into_iter().map(|v| {
            println!("Checking {v}");
            let bits = super::encoded_bits!(Encoded::<u8, Small>::new(v));
            assert_eq!(
                millibits(&Encoded::<u8, Small>::new(v)),
                super::Millibits::bits(bits.parse().unwrap()),
                "millibits estimate disagrees for {v}"
            );
            (v, bits)
        });
        let (_, bits) = sizes.next().expect("size_of needs at least one value");
        for (v, other) in sizes {
            assert_eq!(other, bits, "encoded size differs for {v}");
        }
        bits
    }

    expect!["3"].assert_eq(&size_of(0..2));
    expect!["4"].assert_eq(&size_of(2..4));
    expect!["5"].assert_eq(&size_of(4..8));
    expect!["6"].assert_eq(&size_of(8..16));
    expect!["7"].assert_eq(&size_of(16..32));
    expect!["8"].assert_eq(&size_of(32..64));
    expect!["10"].assert_eq(&size_of(64..128));
    expect!["11"].assert_eq(&size_of(128..255));
    assert_eq!(
        millibits(&Encoded::<u8, Small>::new(255u8)),
        super::Millibits::bits(11)
    );
}

#[test]
fn small_i8() {
    use super::Small;
    use crate::Encoded;

    // Round-trip every i8 value.
    for v in i8::MIN..=i8::MAX {
        let enc = super::encode(&Encoded::<i8, Small>::new(v));
        let dec = super::decode::<Encoded<i8, Small>>(&enc).unwrap().value();
        assert_eq!(v, dec, "round-trip failed for {v}");
    }

    fn size_of(vals: impl IntoIterator<Item = i8>) -> String {
        let mut sizes = vals.into_iter().map(|v| {
            println!("Checking {v}");
            (v, super::estimated_bits!(Encoded::<i8, Small>::new(v)))
        });
        let (_, bits) = sizes.next().expect("size_of needs at least one value");
        for (v, other) in sizes {
            assert_eq!(other, bits, "encoded size differs for {v}");
        }
        bits
    }

    // Zig-zag mapping → same bit ranges as Small u8:
    // zigzag {0,1} → 3 bits: i8 values {0, -1}
    expect!["3"].assert_eq(&size_of([0]));
    expect!["3"].assert_eq(&size_of([-1]));
    // zigzag {2,3} → 4 bits: {1, -2}
    expect!["4"].assert_eq(&size_of([1]));
    expect!["4"].assert_eq(&size_of([-2]));
    // zigzag {4..7} → 5 bits: {2, 3, -3, -4}
    expect!["5"].assert_eq(&size_of([2i8, 3, -3, -4]));
    // zigzag {8..15} → 6 bits: {4..7, -5..-8}
    expect!["6"].assert_eq(&size_of([4i8, 7, -5, -8]));
    // zigzag {16..31} → 7 bits: {8..15, -9..-16}
    expect!["7"].assert_eq(&size_of([8i8, 15, -9, -16]));
    // zigzag {32..63} → 8 bits: {16..31, -17..-32}
    expect!["8"].assert_eq(&size_of([16i8, 31, -17, -32]));
    // zigzag {64..127} → 10 bits: {32..63, -33..-64}
    expect!["10"].assert_eq(&size_of([32i8, 63, -33, -64]));
    // zigzag {128..255} → 11 bits: {64..127, -65..-128}
    expect!["11"].assert_eq(&size_of([64i8, 127, -65]));
    // -128 → zigzag 255 → all-ones bit pattern (nonzero=7=111, need_seven=1, b7=127=1111111).
    // Mirror the small_u8 test for u8=255: verify the millibits entropy estimate directly.
    assert_eq!(
        millibits(&crate::Encoded::<i8, Small>::new(-128)),
        super::Millibits::bits(11)
    );
}

#[test]
fn sorted_u8_roundtrip() {
    use crate::Encoded;
    // Every possible (previous, current) pair must round-trip correctly.
    for prev in 0u8..=255 {
        for cur in 0u8..=255 {
            let data = [
                Encoded::<u8, Sorted>::new(prev),
                Encoded::<u8, Sorted>::new(cur),
            ];
            let enc = super::encode(&data);
            let dec: [Encoded<u8, Sorted>; 2] = super::decode(&enc).unwrap();
            assert_eq!(
                [dec[0].value(), dec[1].value()],
                [prev, cur],
                "round-trip failed for [{prev}, {cur}]"
            );
        }
    }
    // Also verify single values.
    for v in 0u8..=255 {
        let enc = super::encode_with(Sorted, &v);
        let dec: u8 = super::decode_with(Sorted, &enc).unwrap();
        assert_eq!(dec, v);
    }
    // i8 round-trip via the same context.
    for v in i8::MIN..=i8::MAX {
        let enc = super::encode_with(Sorted, &v);
        let dec: i8 = super::decode_with(Sorted, &enc).unwrap();
        assert_eq!(dec, v);
    }
}

#[test]
fn sorted_u8_ascii() {
    use super::estimated_bits;
    use crate::Encoded;
    expect!["28"].assert_eq(&estimated_bits!([
        Encoded::<u8, Sorted>::new(b'h'),
        Encoded::<u8, Sorted>::new(b'e'),
        Encoded::<u8, Sorted>::new(b'l'),
        Encoded::<u8, Sorted>::new(b'l'),
        Encoded::<u8, Sorted>::new(b'o'),
    ]));
}
