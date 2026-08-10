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
