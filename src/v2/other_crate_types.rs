#[cfg(feature = "nonmax")]
mod nonmax {
    use super::super::Encode;

    macro_rules! impl_encode_nonmax {
        ($ty:ty, $equiv:ty) => {
            impl Encode for $ty {
                type Context = <$equiv as Encode>::Context;
                #[inline]
                fn encode<E: super::super::EntropyCoder>(
                    &self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    self.get().encode(writer, ctx)
                }
                #[inline]
                fn decode<D: super::super::EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let v = <$equiv as Encode>::decode(reader, ctx)?;
                    <$ty>::new(v).ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Decoded value {v} is out of range"),
                        )
                    })
                }
            }
        };
    }

    macro_rules! impl_decode_async_nonmax {
        ($ty:ty, $equiv:ty) => {
            impl crate::v2::DecodeAsync<$ty> for crate::Normal {
                /// Exactly the equivalent integer.
                const MAX_BYTES: usize =
                    <crate::Normal as crate::v2::DecodeAsync<$equiv>>::MAX_BYTES;

                #[inline]
                async fn decode_awaiting<D: crate::v2::AsyncEntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$ty, std::io::Error> {
                    let v = <crate::Normal as crate::v2::DecodeAsync<$equiv>>::decode_async(
                        reader, ctx,
                    )
                    .await?;
                    <$ty>::new(v).ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Decoded value {v} is out of range"),
                        )
                    })
                }
            }
        };
    }

    impl_encode_nonmax!(nonmax::NonMaxI8, i8);
    impl_decode_async_nonmax!(nonmax::NonMaxI8, i8);
    impl_encode_nonmax!(nonmax::NonMaxI16, i16);
    impl_decode_async_nonmax!(nonmax::NonMaxI16, i16);
    impl_encode_nonmax!(nonmax::NonMaxI32, i32);
    impl_decode_async_nonmax!(nonmax::NonMaxI32, i32);
    impl_encode_nonmax!(nonmax::NonMaxI64, i64);
    impl_decode_async_nonmax!(nonmax::NonMaxI64, i64);
    impl_encode_nonmax!(nonmax::NonMaxU8, u8);
    impl_decode_async_nonmax!(nonmax::NonMaxU8, u8);
    impl_encode_nonmax!(nonmax::NonMaxU16, u16);
    impl_decode_async_nonmax!(nonmax::NonMaxU16, u16);
    impl_encode_nonmax!(nonmax::NonMaxU32, u32);
    impl_decode_async_nonmax!(nonmax::NonMaxU32, u32);
    impl_encode_nonmax!(nonmax::NonMaxU64, u64);
    impl_decode_async_nonmax!(nonmax::NonMaxU64, u64);
}

#[cfg(feature = "uuid")]
mod uuid {
    use super::super::Encode;
    use uuid::Uuid;

    impl Encode for Uuid {
        type Context = <(u64, u64) as Encode>::Context;
        #[inline]
        fn encode<E: super::super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
            self.as_u64_pair().encode(writer, ctx)
        }
        #[inline]
        fn decode<D: super::super::EntropyDecoder>(
            reader: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Self, std::io::Error> {
            let (high, low) = <(u64, u64) as Encode>::decode(reader, ctx)?;
            Ok(Uuid::from_u64_pair(high, low))
        }
    }

    impl crate::v2::DecodeAsync<Uuid> for crate::Normal {
        /// A pair of `u64`s.
        const MAX_BYTES: usize = <crate::Normal as crate::v2::DecodeAsync<(u64, u64)>>::MAX_BYTES;

        #[inline]
        async fn decode_awaiting<D: crate::v2::AsyncEntropyDecoder>(
            reader: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Uuid, std::io::Error> {
            let (high, low) =
                <crate::Normal as crate::v2::DecodeAsync<(u64, u64)>>::decode_async(reader, ctx)
                    .await?;
            Ok(Uuid::from_u64_pair(high, low))
        }
    }
}

/// R8: these impls were compiled by CI but never executed by any test.
///
/// Both are thin adapters over an existing impl, which is exactly why they are
/// easy to get subtly wrong and never notice: `Uuid`'s two halves could be
/// transposed and still round-trip in *shape*, and `NonMax*`'s validation
/// rejects one specific value an arbitrary test vector is unlikely to hit.
#[cfg(test)]
mod tests {
    /// Round-trip through both coders and, with `stream` on, through a stream
    /// chopped at one byte per chunk — which suspends constantly and so drives
    /// `decode_awaiting`/`MAX_BYTES` rather than the sync fast path.
    #[allow(unused)]
    fn round_trips<T>(value: T)
    where
        T: crate::v2::Encode + PartialEq + std::fmt::Debug,
        crate::Normal: crate::v2::DecodeAsync<T>,
    {
        let range = crate::v2::Range::encode(&value);
        assert_eq!(crate::v2::Range::decode::<T>(&range).as_ref(), Some(&value));
        let ans = crate::v2::Ans::encode(&value);
        assert_eq!(crate::v2::Ans::decode::<T>(&ans).as_ref(), Some(&value));

        #[cfg(feature = "stream")]
        {
            use crate::v2::stream::tests::Chunks;
            use futures_executor::block_on;
            let decoded: T =
                block_on(crate::v2::Range::decode_stream(Chunks::new(&range, 1))).unwrap();
            assert_eq!(decoded, value, "Range::decode_stream disagreed");
            let decoded: T = block_on(crate::v2::Ans::decode_stream(Chunks::new(&ans, 1))).unwrap();
            assert_eq!(decoded, value, "Ans::decode_stream disagreed");
        }
    }

    #[cfg(feature = "uuid")]
    #[test]
    fn uuid_round_trips() {
        // Asymmetric halves: a transposed `as_u64_pair` would survive a
        // palindromic value, so these deliberately differ high from low.
        for value in [
            ::uuid::Uuid::nil(),
            ::uuid::Uuid::max(),
            ::uuid::Uuid::from_u64_pair(0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210),
            ::uuid::Uuid::from_u64_pair(1, 0),
            ::uuid::Uuid::from_u64_pair(0, 1),
        ] {
            round_trips(value);
        }
    }

    #[cfg(feature = "nonmax")]
    #[test]
    fn nonmax_round_trips() {
        macro_rules! check {
            ($ty:ty, $prim:ty) => {
                // Including the values either side of the forbidden one, where a
                // `new` returning `None` would show up.
                for raw in [0 as $prim, 1 as $prim, <$prim>::MAX - 1, <$prim>::MIN] {
                    if let Some(v) = <$ty>::new(raw) {
                        round_trips(v);
                    }
                }
            };
        }
        check!(::nonmax::NonMaxU8, u8);
        check!(::nonmax::NonMaxU16, u16);
        check!(::nonmax::NonMaxU32, u32);
        check!(::nonmax::NonMaxU64, u64);
        check!(::nonmax::NonMaxI8, i8);
        check!(::nonmax::NonMaxI16, i16);
        check!(::nonmax::NonMaxI32, i32);
        check!(::nonmax::NonMaxI64, i64);
    }
}
