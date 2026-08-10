use super::Encode;

impl<T: Encode, const N: usize> Encode for [T; N] {
    type Context = T::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        for v in self {
            v.encode(writer, ctx);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut x = Vec::with_capacity(N);
        for _ in 0..N {
            x.push(T::decode(reader, ctx)?);
        }
        x.try_into()
            .map_err(|_| std::io::Error::other("impossible: x should have N values"))
    }
}

impl<T: Encode, const N: usize> super::DecodeAsync<[T; N]> for crate::Normal
where
    // The equality is what the blanket `EncodingStrategy for Normal` impl says,
    // but with `Normal: DecodeAsync<T>` in scope the compiler prefers that
    // param-env candidate and stops normalizing, so restate it.
    crate::Normal:
        super::DecodeAsync<T> + super::EncodingStrategy<T, Context = <T as Encode>::Context>,
{
    /// `N` elements and no length — `N` is known at compile time.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T>>::MAX_BYTES.saturating_mul(N);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<[T; N], std::io::Error> {
        let mut x = Vec::with_capacity(N);
        for _ in 0..N {
            x.push(<crate::Normal as super::DecodeAsync<T>>::decode_async(reader, ctx).await?);
        }
        x.try_into()
            .map_err(|_| std::io::Error::other("impossible: x should have N values"))
    }
}
