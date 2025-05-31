pub use compactly_derive::EncodeV1 as Encode;
use std::io::{Read, Write};

pub mod adapt;
mod arc;
pub mod arith;
mod array;
mod bit_context;
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

use crate::{LowCardinality, Small};
pub use adapt::{Reader, Writer};
pub use arith::Probability;
pub use urange::URange;

pub trait Encode: Sized {
    type Context: Default + Clone;

    fn encode<W: Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    #[expect(unused_variables)]
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        // let mut counter = Writer::new(adapt::Counter::default());
        // self.encode(&mut counter, ctx).ok();
        // counter.len() * 8000
        None
    }

    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;
}

pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    let mut writer = Writer::new(&mut out);
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
    type Context: Default + Clone;

    fn encode<W: Write>(
        value: &T,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    #[expect(unused_variables)]
    fn millibits(value: &T, ctx: &mut Self::Context) -> Option<usize> {
        None
    }

    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error>;
}

pub fn encode_with<T: Encode, S: EncodingStrategy<T>>(_: S, value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    let mut writer = Writer::<&mut Vec<u8>>::new(&mut out);
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
impl<T> Compact<T> {
    pub fn new(value: T) -> Self {
        Compact(value)
    }
}
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
impl<T> Encode for Compact<T>
where
    Small: EncodingStrategy<T>,
{
    type Context = <Small as EncodingStrategy<T>>::Context;
    fn encode<W: Write>(
        &self,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        <Small as EncodingStrategy<T>>::encode(self, writer, ctx)
    }
    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        <Small as EncodingStrategy<T>>::decode(reader, ctx).map(Compact)
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        <Small as EncodingStrategy<T>>::millibits(self, ctx)
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
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        S::millibits(&self.value, ctx)
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

impl<T: Encode> EncodingStrategy<T> for crate::Normal {
    type Context = <T as Encode>::Context;
    #[inline]
    fn encode<W: Write>(
        value: &T,
        writer: &mut Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.encode(writer, ctx)
    }
    fn millibits(value: &T, ctx: &mut Self::Context) -> Option<usize> {
        value.millibits(ctx)
    }
    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error> {
        T::decode(reader, ctx)
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
    ($v:expr, $size:expr, $msg:expr) => {
        let v1 = $v;
        let bytes = super::encode(&v1);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v1), "decoded value is incorrect: {}", $msg);
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
        assert_eq!(
            decoded,
            Some(v),
            "decoded tuple value is incorrect: {}",
            $msg
        );
        assert_eq!(
            (bytes.len() + 4) / 8,
            $size,
            "unexpected number of bits: {}",
            $msg
        );
    };
}
#[cfg(test)]
pub(crate) use assert_bits;
