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

    impl_encode_nonmax!(nonmax::NonMaxI8, i8);
    impl_encode_nonmax!(nonmax::NonMaxI16, i16);
    impl_encode_nonmax!(nonmax::NonMaxI32, i32);
    impl_encode_nonmax!(nonmax::NonMaxI64, i64);
    impl_encode_nonmax!(nonmax::NonMaxU8, u8);
    impl_encode_nonmax!(nonmax::NonMaxU16, u16);
    impl_encode_nonmax!(nonmax::NonMaxU32, u32);
    impl_encode_nonmax!(nonmax::NonMaxU64, u64);
}
