use std::marker::{PhantomData, PhantomPinned};

use super::Encode;

impl<T> Encode for PhantomData<T> {
    /// Carries no runtime information, so nothing is coded.
    const MAX_BYTES: usize = 0;
    type Context = ();

    #[inline]
    fn encode<E: super::EntropyCoder>(&self, _encoder: &mut E, _ctx: &mut Self::Context) {
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
}

impl Encode for PhantomPinned {
    /// Carries no runtime information, so nothing is coded.
    const MAX_BYTES: usize = 0;
    type Context = ();

    #[inline]
    fn encode<E: super::EntropyCoder>(&self, _encoder: &mut E, _ctx: &mut Self::Context) {
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
}
