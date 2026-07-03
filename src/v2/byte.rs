use super::{Encode, EncodingStrategy};
use crate::{Incompressible, Small, Sorted};

#[derive(Clone)]
pub struct ByteContext([<bool as Encode>::Context; 256]);
impl Default for ByteContext {
    #[inline]
    fn default() -> Self {
        ByteContext([Default::default(); 256])
    }
}

impl Encode for u8 {
    type Context = ByteContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        writer.encode_tree(&mut ctx.0, *self as usize)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(reader.decode_tree(&mut ctx.0) as u8)
    }
}

macro_rules! small_num {
    ($t:ty, $nbits:literal, $maxval:literal, $doublemax:literal, $testname:ident) => {
        mod $testname {
            use super::{Encode, UBits};

            #[derive(Clone)]
            pub struct Context([<bool as Encode>::Context; $doublemax]);

            impl Default for Context {
                #[inline]
                fn default() -> Self {
                    Self([Default::default(); $doublemax])
                }
            }

            impl Encode for $t {
                type Context = Context;
                #[inline]
                fn encode<E: super::super::EntropyCoder>(
                    &self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    writer.encode_tree(&mut ctx.0, u8::from(*self) as usize)
                }
                #[inline]
                fn decode<D: super::super::EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    Ok(((reader.decode_tree(&mut ctx.0)) as u8)
                        .try_into()
                        .unwrap())
                }
            }

            #[test]
            fn test() {
                for value in 0u8..=$maxval {
                    println!("Testing {value}");
                    let v: $t = value.try_into().unwrap();
                    let encoded = super::super::encode(&v);
                    let decoded = super::super::decode::<$t>(&encoded).unwrap();
                    assert_eq!(v, decoded);
                    assert_eq!(v.millibits(), super::super::Millibits::bits($nbits));
                }
            }
        }
    };
}

/// An N-Bit unsigned number that fits into a `u8`.
///
/// This number is tracked precisely, like `u8` itself.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UBits<const N: u8>(u8);

impl<const N: u8> From<UBits<N>> for u8 {
    fn from(value: UBits<N>) -> u8 {
        value.0
    }
}

impl<const N: u8> UBits<N> {
    #[inline]
    pub(crate) fn new(value: u8) -> Self {
        debug_assert!(N == 8 || value >> N == 0);
        Self(value)
    }
}

impl<const N: u8> TryFrom<u8> for UBits<N> {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if N == 8 || value >> N == 0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

small_num!(UBits<1>, 1, 1, 2, ub1);
small_num!(UBits<2>, 2, 3, 4, ub2);
small_num!(UBits<3>, 3, 7, 8, ub3);
small_num!(UBits<4>, 4, 15, 16, ub4);
small_num!(UBits<5>, 5, 31, 32, ub5);
small_num!(UBits<6>, 6, 63, 64, ub6);
small_num!(UBits<7>, 7, 127, 128, ub7);
small_num!(UBits<8>, 8, 255, 256, ub8);

impl Encode for i8 {
    type Context = <u8 as Encode>::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        (*self as u8).encode(writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        <u8 as Encode>::decode(reader, ctx).map(|v| v as i8)
    }
}

#[derive(Default, Clone)]
pub struct SmallContext {
    nonzero: <UBits<3> as Encode>::Context,
    b1: <UBits<1> as Encode>::Context,
    b2: <UBits<2> as Encode>::Context,
    b3: <UBits<3> as Encode>::Context,
    b4: <UBits<4> as Encode>::Context,
    b5: <UBits<5> as Encode>::Context,
    need_seven_bits: <bool as Encode>::Context,
    b6: <UBits<6> as Encode>::Context,
    b7: <UBits<7> as Encode>::Context,
}

impl EncodingStrategy<u8> for Small {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &u8, writer: &mut E, ctx: &mut Self::Context) {
        let nonzero: UBits<3>;
        match *value {
            0 => {
                nonzero = 0.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero)
            }
            1 => {
                nonzero = 1.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero)
            }
            2..4 => {
                nonzero = 2.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b1: UBits<1> = (*value - 2).try_into().unwrap();
                b1.encode(writer, &mut ctx.b1)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b2: UBits<2> = (*value - 4).try_into().unwrap();
                b2.encode(writer, &mut ctx.b2)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b3: UBits<3> = (*value - 8).try_into().unwrap();
                b3.encode(writer, &mut ctx.b3)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b4: UBits<4> = (*value - 16).try_into().unwrap();
                b4.encode(writer, &mut ctx.b4)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b5: UBits<5> = (*value - 32).try_into().unwrap();
                b5.encode(writer, &mut ctx.b5)
            }
            64..128 => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                false.encode(writer, &mut ctx.need_seven_bits);
                let b6: UBits<6> = (*value - 64).try_into().unwrap();
                b6.encode(writer, &mut ctx.b6)
            }
            128..=255 => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                true.encode(writer, &mut ctx.need_seven_bits);
                let b7: UBits<7> = (*value - 128).try_into().unwrap();
                b7.encode(writer, &mut ctx.b7)
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let nonzero: u8 = <UBits<3> as Encode>::decode(reader, &mut ctx.nonzero)?.into();
        match nonzero {
            0 => Ok(0),
            1 => Ok(1),
            2 => {
                let rest: u8 = <UBits<1> as Encode>::decode(reader, &mut ctx.b1)?.into();
                Ok(rest + 2)
            }
            3 => {
                let rest: u8 = <UBits<2> as Encode>::decode(reader, &mut ctx.b2)?.into();
                Ok(rest + 4)
            }
            4 => {
                let rest: u8 = <UBits<3> as Encode>::decode(reader, &mut ctx.b3)?.into();
                Ok(rest + 8)
            }
            5 => {
                let rest: u8 = <UBits<4> as Encode>::decode(reader, &mut ctx.b4)?.into();
                Ok(rest + 16)
            }
            6 => {
                let rest: u8 = <UBits<5> as Encode>::decode(reader, &mut ctx.b5)?.into();
                Ok(rest + 32)
            }
            7 => {
                if <bool as Encode>::decode(reader, &mut ctx.need_seven_bits)? {
                    let rest: u8 = <UBits<7> as Encode>::decode(reader, &mut ctx.b7)?.into();
                    Ok(rest + 128)
                } else {
                    let rest: u8 = <UBits<6> as Encode>::decode(reader, &mut ctx.b6)?.into();
                    Ok(rest + 64)
                }
            }
            _ => unreachable!(),
        }
    }
}

impl EncodingStrategy<i8> for Small {
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
        let z = <Small as EncodingStrategy<u8>>::decode(reader, ctx)?;
        Ok(((z >> 1) as i8) ^ (-((z & 1) as i8)))
    }
}

impl EncodingStrategy<u8> for Incompressible {
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
}

#[derive(Default, Clone)]
pub struct SortedU8Context {
    previous: Option<u8>,
    delta: <Small as EncodingStrategy<i8>>::Context,
}

impl EncodingStrategy<u8> for Sorted {
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
}

impl EncodingStrategy<i8> for Sorted {
    type Context = SortedU8Context;
    fn encode<E: super::EntropyCoder>(value: &i8, writer: &mut E, ctx: &mut Self::Context) {
        Sorted::encode(&(*value as u8), writer, ctx)
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<i8, std::io::Error> {
        <Sorted as EncodingStrategy<u8>>::decode(reader, ctx).map(|v| v as i8)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(u8::MAX, 3);
    assert_bits!(0_u8, 8);
    for b in 3_u8..255 {
        println!("Byte {b}");
        assert_bits!(b, 8);
    }
    assert_bits!(*b"hello", 31);
    assert_bits!(*b"hello world", 68);
    assert_bits!(*b"hello world, hello world", 129);
    assert_bits!(*b"hello hello, hello hello", 111);
    assert_bits!(*b"hello hello, hello hello, hello hello, hello hello", 195);
    assert_bits!(*b"hhhhhhhhhhhhhhhhhhhhhhhh", 37);
    assert_bits!(*b"hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh", 44);
    assert_bits!(*b"\0", 8);
    assert_bits!(*b"\x01", 8);
    assert_bits!(*b"\x01\x01", 13);
    assert_bits!(*b"\x01\x01\x01\x01", 19);
    assert_bits!(*b"\x01\x01\x01\x01\x01", 21);
    assert_bits!(*b"\x01\x01\x01\x01\x01\x01", 22);
    assert_bits!(*b"\x01\x02\x03\x04", 25);
    assert_bits!(*b"\x01\x02\x03\x04\x05", 30);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06", 36);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06\x07", 40);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06\x07\x08", 47);

    assert_bits!(i8::MAX, 8);
    assert_bits!(0_i8, 8);
}

#[test]
fn small() {
    use super::{assert_bits, Small};
    use crate::Encoded;
    fn check_size(v: u8, expected: usize) {
        println!("Checking {v}");
        assert_eq!(
            Encoded::<u8, Small>::new(v).millibits(),
            super::Millibits::bits(expected)
        );
        assert_bits!(Encoded::<u8, Small>::new(v), expected);
    }

    for x in 0..2 {
        check_size(x, 3);
    }
    for x in 2..4 {
        check_size(x, 4);
    }
    for x in 4..8 {
        check_size(x, 5);
    }
    for x in 8..16 {
        check_size(x, 6);
    }
    for x in 16..32 {
        check_size(x, 7);
    }
    for x in 32..64 {
        check_size(x, 8);
    }
    for x in 64..128 {
        check_size(x, 10);
    }
    for x in 128..255 {
        check_size(x, 11);
    }
    assert_eq!(
        Encoded::<u8, Small>::new(255u8).millibits(),
        super::Millibits::bits(11)
    );
}

#[test]
fn small_i8() {
    use super::{assert_bits, Small};
    use crate::Encoded;

    // Round-trip every i8 value.
    for v in i8::MIN..=i8::MAX {
        let enc = super::encode(&Encoded::<i8, Small>::new(v));
        let dec = super::decode::<Encoded<i8, Small>>(&enc).unwrap().value();
        assert_eq!(v, dec, "round-trip failed for {v}");
    }

    fn check_size(v: i8, expected: usize) {
        assert_bits!(
            Encoded::<i8, Small>::new(v),
            expected,
            "unexpected size for {v}"
        );
    }

    // Zig-zag mapping → same bit ranges as Small u8:
    // zigzag {0,1} → 3 bits: i8 values {0, -1}
    check_size(0, 3);
    check_size(-1, 3);
    // zigzag {2,3} → 4 bits: {1, -2}
    check_size(1, 4);
    check_size(-2, 4);
    // zigzag {4..7} → 5 bits: {2, 3, -3, -4}
    for v in [2i8, 3, -3, -4] {
        check_size(v, 5);
    }
    // zigzag {8..15} → 6 bits: {4..7, -5..-8}
    for v in [4i8, 7, -5, -8] {
        check_size(v, 6);
    }
    // zigzag {16..31} → 7 bits: {8..15, -9..-16}
    for v in [8i8, 15, -9, -16] {
        check_size(v, 7);
    }
    // zigzag {32..63} → 8 bits: {16..31, -17..-32}
    for v in [16i8, 31, -17, -32] {
        check_size(v, 8);
    }
    // zigzag {64..127} → 10 bits: {32..63, -33..-64}
    for v in [32i8, 63, -33, -64] {
        check_size(v, 10);
    }
    // zigzag {128..255} → 11 bits: {64..127, -65..-128}
    for v in [64i8, 127, -65] {
        check_size(v, 11);
    }
    // -128 → zigzag 255 → all-ones bit pattern (nonzero=7=111, need_seven=1, b7=127=1111111).
    // The Range coder compresses all-ones sequences well, same as assert_bits!(u8::MAX, 3).
    // Mirror the small_u8 test for u8=255: only verify entropy, not actual coded size.
    assert_eq!(
        crate::Encoded::<i8, Small>::new(-128).millibits(),
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
    use super::assert_bits;
    use crate::Encoded;
    assert_bits!(
        [
            Encoded::<u8, Sorted>::new(b'h'),
            Encoded::<u8, Sorted>::new(b'e'),
            Encoded::<u8, Sorted>::new(b'l'),
            Encoded::<u8, Sorted>::new(b'l'),
            Encoded::<u8, Sorted>::new(b'o'),
        ],
        29
    );
}
