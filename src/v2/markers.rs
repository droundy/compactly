use std::marker::{PhantomData, PhantomPinned};

use super::Encode;

impl<T> Encode for PhantomData<T> {
    type Context = ();

    #[inline]
    fn encode<E: super::EntropyCoder>(_value: &Self, _encoder: &mut E, _ctx: &mut Self::Context) {
        // PhantomData carries no runtime information, so encoding is a no-op
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        _decoder: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        // PhantomData can always be constructed without decoding anything
        Ok(PhantomData)
    }

    /// Carries no runtime information, so nothing is coded.
    const MAX_BYTES: usize = 0;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        _decoder: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(PhantomData)
    }
}

impl Encode for PhantomPinned {
    type Context = ();

    #[inline]
    fn encode<E: super::EntropyCoder>(_value: &Self, _encoder: &mut E, _ctx: &mut Self::Context) {
        // PhantomData carries no runtime information, so encoding is a no-op
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        _decoder: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        // PhantomData can always be constructed without decoding anything
        Ok(PhantomPinned)
    }

    /// Carries no runtime information, so nothing is coded.
    const MAX_BYTES: usize = 0;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        _decoder: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(PhantomPinned)
    }
}
