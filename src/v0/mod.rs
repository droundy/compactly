use cabac::traits::CabacWriter;
use std::io::{Read, Write};

pub use compactly_derive::EncodeV0 as Encode;

mod array;
mod bits;
mod bools;
mod byte;
mod floats;
mod ints;
mod low_cardinality;
mod maps;
mod option;
mod sets;
mod string;
mod tuples;
mod urange;
mod usizes;
mod vecs;

use crate::{Decimal, LowCardinality, Small};
pub use cabac;
pub use urange::URange;

pub type Writer<W> = cabac::vp8::VP8Writer<W>;
pub type Reader<R> = cabac::vp8::VP8Reader<R>;

pub trait Encode: Sized {
    type Context: Default;

    fn encode<W: Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;
}

pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    let mut writer = Writer::new(&mut out).unwrap();
    value
        .encode(&mut writer, &mut T::Context::default())
        .unwrap();
    writer.finish().unwrap();
    out
}

pub fn decode<T: Encode>(mut bytes: &[u8]) -> Option<T> {
    let mut reader = Reader::new(&mut bytes).unwrap();
    T::decode(&mut reader, &mut T::Context::default()).ok()
}

pub trait EncodingStrategy<T> {
    type Context: Default;

    fn encode<W: Write>(
        value: &T,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoded<T, S: EncodingStrategy<T>> {
    value: T,
    _phantom: std::marker::PhantomData<S>,
}

pub fn encode_with<T: Encode, S: EncodingStrategy<T>>(_: S, value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    let mut writer = Writer::<&mut Vec<u8>>::new(&mut out).unwrap();
    S::encode(value, &mut writer, &mut S::Context::default()).unwrap();
    writer.finish().unwrap();
    out
}

pub fn decode_with<T: Encode, S: EncodingStrategy<T>>(_: S, mut bytes: &[u8]) -> Option<T> {
    let mut reader = Reader::new(&mut bytes).unwrap();
    S::decode(&mut reader, &mut S::Context::default()).ok()
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]

pub struct Compact<T>(T);
impl<T> std::ops::Deref for Compact<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> std::ops::DerefMut for Compact<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S: EncodingStrategy<T>> Encode for crate::Encoded<T, S> {
    type Context = S::Context;
    #[inline]
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        S::encode(&self.value, writer, ctx)
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            value: S::decode(reader, ctx)?,
            _phantom: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:expr) => {
        let v = $v;
        let bytes = super::encode(&v);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        assert_eq!(bytes.len(), $size, "unexpected size");
    };
}
#[cfg(test)]
pub(crate) use assert_size;

#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $size:expr) => {
        let v1 = $v;
        let bytes = super::encode(&v1);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v1), "decoded value is incorrect");
        let v = (
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
        );
        let bytes = super::encode(&v);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded tuple value is incorrect");
        assert_eq!((bytes.len() + 4) / 8, $size, "unexpected number of bits");
    };
}
#[cfg(test)]
pub(crate) use assert_bits;
