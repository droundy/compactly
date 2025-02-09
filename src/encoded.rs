use crate::{Encode, Encoded, EncodingStrategy};

impl<T, S: EncodingStrategy<T>> Encode for Encoded<T, S> {
    type Context = S::Context;
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut crate::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        S::encode(&self.value, writer, ctx)
    }
    fn decode<R: std::io::Read>(
        reader: &mut crate::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            value: S::decode(reader, ctx)?,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T, S: EncodingStrategy<T>> Encoded<T, S> {
    pub fn new(value: T) -> Self {
        Self::from(value)
    }
}

impl<T, S: EncodingStrategy<T>> From<T> for Encoded<T, S> {
    fn from(value: T) -> Self {
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, S: EncodingStrategy<T>> std::ops::Deref for Encoded<T, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T, S: EncodingStrategy<T>> std::ops::DerefMut for Encoded<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
