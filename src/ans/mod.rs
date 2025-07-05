//! The `ans` format of compactly.
//!
//! This format should be unmodified after the 1.0 release, except for addition
//! of support for new strategies, which won't change the binary format of types
//! that don't use those strategies.
pub use compactly_derive::EncodeAns as Encode;
use std::io::Read;

mod adapt;
mod ans;
mod arc;
mod arith;
mod array;
mod bit_context;
mod bits;
mod bools;
mod byte;
mod bytes;
mod floats;
#[cfg(feature = "generate_bit_context")]
pub mod generate_bit_context;
mod ints;
mod low_cardinality;
mod maps;
mod option;
mod sets;
mod string;
mod tuples;
mod ulessthan;
mod usizes;
mod vecs;

use crate::{LowCardinality, Small};
pub use adapt::{Reader, Writer};
pub use ulessthan::ULessThan;

/// A place where we can put bits where we have estimated the probabilities.
pub trait EntropyCoder {
    /// Encode a given bit with its probability
    fn encode(&mut self, probability: ans::Probability, bit: bool) -> Result<(), std::io::Error>;
    /// Finish the encoding
    fn finish(&mut self) -> Result<(), std::io::Error>;
}

/// Trait for types that can be compactly encoded.
///
/// Normally you will derive this for your own types, although it can be
/// implemented manually.
pub trait Encode: Sized {
    /// Context storing probability model for this type.
    type Context: Default + Clone;

    /// Encode this value to the [`Writer<W>`].
    fn encode<E: EntropyCoder>(
        &self,
        encoder: &mut E,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    /// Extimate the number of millibits required for this value.
    ///
    /// Returns `None` if this estimation has not been implemented.
    #[expect(unused_variables)]
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        // let mut counter = Writer::new(adapt::Counter::default());
        // self.encode(&mut counter, ctx).ok();
        // counter.len() * 8000
        None
    }

    /// Decode value from ['Reader<R>`].
    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;
}

/// Encode the `value` into a `Vec<u8>` of bytes.`
pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    {
        let mut writer = Writer::new(&mut out);
        value
            .encode(&mut writer, &mut T::Context::default())
            .unwrap();
        writer.finish().unwrap();
    }
    out
}

/// Decode a value of this type from `bytes`.
///
/// Returns `None` if the bytes do not encode a valid value.
pub fn decode<T: Encode>(mut bytes: &[u8]) -> Option<T> {
    let mut reader = Reader::new(&mut bytes).unwrap();
    T::decode(&mut reader, &mut T::Context::default()).ok()
}

/// An encoding strategy for type `T`.
///
/// You *can* implement this for your own types, if you want them to support
/// e.g. `Small` encodings.  But I expect this to be unusual.  It would be
/// possible to create a `Derive` macro for this, but I don't think it is
/// needed.  If you want such a macro file an issue.
///
/// Note that besides implementing existing strategies for your own types, you
/// can also create entirely new strategies in your crates.  If you do that, you
/// can use full paths in your derive macros, e.g.
/// `#[compactly(your_crate::SuperCoolEncodingStratgy]`.
pub trait EncodingStrategy<T> {
    /// The conext (i.e. probability model) for this encoding strategy applied to this type.
    type Context: Default + Clone;

    /// Encode the value with this strategy.
    fn encode<E: EntropyCoder>(
        value: &T,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error>;

    /// Estimate the size of the encoded value using this stratgy.
    #[expect(unused_variables)]
    fn millibits(value: &T, ctx: &mut Self::Context) -> Option<usize> {
        None
    }

    /// Decode the value using this strategy.
    fn decode<R: Read>(
        reader: &mut Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error>;
}

/// Encode a value with a specific strategy (into a `Vec<u8>`).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn encode_with<T: Encode, S: EncodingStrategy<T>>(_: S, value: &T) -> Vec<u8> {
    let mut out = Vec::with_capacity(8);
    {
        let mut writer = Writer::<&mut Vec<u8>>::new(&mut out);
        S::encode(value, &mut writer, &mut S::Context::default()).unwrap();
        writer.finish().unwrap();
    }
    out
}

/// Decode a value with a specific strategy (from a bytes slice).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn decode_with<T: Encode, S: EncodingStrategy<T>>(_: S, mut bytes: &[u8]) -> Option<T> {
    let mut reader = Reader::new(&mut bytes).unwrap();
    S::decode(&mut reader, &mut S::Context::default()).ok()
}

impl<T, S: EncodingStrategy<T>> Encode for crate::Encoded<T, S> {
    type Context = S::Context;
    #[inline]
    fn encode<E: EntropyCoder>(
        &self,
        writer: &mut E,
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
    fn encode<E: EntropyCoder>(
        value: &T,
        writer: &mut E,
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
        let ans = $v;
        let bytes = super::encode(&ans);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(ans), "decoded value is incorrect");
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
        let ans = $v;
        let bytes = super::encode(&ans);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(ans), "decoded value is incorrect: {}", $msg);
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
