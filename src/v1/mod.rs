pub use compactly_derive::EncodeV1 as Encode;
use std::io::{Read, Write};

pub mod adapt;
pub mod arith;
mod array;
mod bit_context;
mod bits;
mod bools;
mod byte;
mod encoded;
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

pub use adapt::{Reader, Writer};
pub use arith::Probability;
pub use urange::URange;

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

/// A strategy for encoding values that are small.
///
/// e.g. if there are integers then they should be small integers.
pub use crate::v0::Small;

/// A strategy for encoding values that are often repeated.
///
/// This can be shockingly efficient when there are just a few values for e.g. a
/// string field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LowCardinality;

/// A strategy for encoding floating point values that have round decimal values.
pub use crate::v0::Decimal;

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
