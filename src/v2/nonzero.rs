use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
use crate::Small;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

// Unsigned NonZero types encode as (value - 1), shifting the "missing" slot from
// zero to the type max, matching the nonmax pattern.
macro_rules! impl_nonzero_uint {
    ($nz:ty, $uint:ty) => {
        impl Encode for $nz {
            type Context = <$uint as Encode>::Context;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                (self.get() - 1).encode(writer, ctx)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let v = <$uint>::decode(reader, ctx)?;
                <$nz>::new(v.wrapping_add(1))
                    .ok_or_else(|| std::io::Error::other("decoded NonZero value is zero"))
            }
        }

        impl EncodingStrategy<$nz> for Small {
            type Context = <Small as EncodingStrategy<$uint>>::Context;
            #[inline]
            fn encode<E: EntropyCoder>(value: &$nz, writer: &mut E, ctx: &mut Self::Context) {
                Small::encode(&(value.get() - 1), writer, ctx)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$nz, std::io::Error> {
                let v: $uint = Small::decode(reader, ctx)?;
                <$nz>::new(v.wrapping_add(1))
                    .ok_or_else(|| std::io::Error::other("decoded NonZero value is zero"))
            }
        }
    };
}

// Signed NonZero types use a nonzero-zigzag: negative values interleave with
// positive ones, mapping to the corresponding unsigned type.
//
//   encode: v > 0  =>  v*2 - 1   (so 1→1, 2→3, 3→5, …)
//           v < 0  =>  -(v+1)*2  (so -1→0, -2→2, -3→4, …)
//
// The "missing" slot is the unsigned max, so NonZeroI8 encodes to u8 0..=254.
// This lets Small work optimally since small-magnitude values map to small
// unsigned values.
macro_rules! impl_nonzero_int {
    ($nz:ty, $int:ty, $uint:ty) => {
        impl Encode for $nz {
            type Context = <$uint as Encode>::Context;
            #[inline]
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let v = self.get();
                let encoded: $uint = if v > 0 {
                    (v as $uint) * 2 - 1
                } else {
                    ((-(v + 1)) as $uint) * 2
                };
                encoded.encode(writer, ctx)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let u = <$uint>::decode(reader, ctx)?;
                let v: $int = if u & 1 == 1 {
                    ((u + 1) / 2) as $int
                } else {
                    -((u / 2) as $int) - 1
                };
                <$nz>::new(v)
                    .ok_or_else(|| std::io::Error::other("decoded NonZero value is zero"))
            }
        }

        impl EncodingStrategy<$nz> for Small {
            type Context = <Small as EncodingStrategy<$uint>>::Context;
            #[inline]
            fn encode<E: EntropyCoder>(value: &$nz, writer: &mut E, ctx: &mut Self::Context) {
                let v = value.get();
                let encoded: $uint = if v > 0 {
                    (v as $uint) * 2 - 1
                } else {
                    ((-(v + 1)) as $uint) * 2
                };
                Small::encode(&encoded, writer, ctx)
            }
            #[inline]
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$nz, std::io::Error> {
                let u: $uint = Small::decode(reader, ctx)?;
                let v: $int = if u & 1 == 1 {
                    ((u + 1) / 2) as $int
                } else {
                    -((u / 2) as $int) - 1
                };
                <$nz>::new(v)
                    .ok_or_else(|| std::io::Error::other("decoded NonZero value is zero"))
            }
        }
    };
}

impl_nonzero_uint!(NonZeroU8, u8);
impl_nonzero_uint!(NonZeroU16, u16);
impl_nonzero_uint!(NonZeroU32, u32);
impl_nonzero_uint!(NonZeroU64, u64);
impl_nonzero_uint!(NonZeroU128, u128);
impl_nonzero_uint!(NonZeroUsize, usize);

impl_nonzero_int!(NonZeroI8, i8, u8);
impl_nonzero_int!(NonZeroI16, i16, u16);
impl_nonzero_int!(NonZeroI32, i32, u32);
impl_nonzero_int!(NonZeroI64, i64, u64);
impl_nonzero_int!(NonZeroI128, i128, u128);
impl_nonzero_int!(NonZeroIsize, isize, usize);

#[test]
fn nonzero_uint_roundtrip() {
    use super::{decode, encode};
    use crate::Encoded;

    for v in [1u8, 2, 127, 128, 254, 255] {
        let nz = NonZeroU8::new(v).unwrap();
        assert_eq!(decode::<NonZeroU8>(&encode(&nz)), Some(nz));
        let sm = Encoded::<_, Small>::new(nz);
        assert_eq!(decode::<Encoded<_, Small>>(&encode(&sm)), Some(sm));
    }
    for v in [1u64, 2, u32::MAX as u64, u64::MAX] {
        let nz = NonZeroU64::new(v).unwrap();
        assert_eq!(decode::<NonZeroU64>(&encode(&nz)), Some(nz));
    }
}

#[test]
fn nonzero_int_roundtrip() {
    use super::{decode, encode};
    use crate::Encoded;

    for v in [-128i8, -2, -1, 1, 2, 126, 127] {
        let nz = NonZeroI8::new(v).unwrap();
        assert_eq!(decode::<NonZeroI8>(&encode(&nz)), Some(nz));
        let sm = Encoded::<_, Small>::new(nz);
        assert_eq!(decode::<Encoded<_, Small>>(&encode(&sm)), Some(sm));
    }
    for v in [i64::MIN, i64::MIN + 1, -1i64, 1, i64::MAX - 1, i64::MAX] {
        let nz = NonZeroI64::new(v).unwrap();
        assert_eq!(decode::<NonZeroI64>(&encode(&nz)), Some(nz));
    }
}

