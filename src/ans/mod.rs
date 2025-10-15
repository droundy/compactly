//! The `ans` format of compactly.
//!
//! This format should be unmodified after the 1.0 release, except for addition
//! of support for new strategies, which won't change the binary format of types
//! that don't use those strategies.
pub use compactly_derive::EncodeAns as Encode;

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
mod millibits;
mod option;
mod other_crate_types;
mod raw;
mod sets;
mod string;
mod tuples;
mod ulessthan;
mod usizes;
mod vecs;

use crate::{LowCardinality, Small};
pub use ans::Ans;
pub use arith::Range;
pub use millibits::Millibits;
pub use raw::Raw;
pub use ulessthan::ULessThan;

const FIFTY_PERCENT: ans::Probability = ans::Probability::new(127, 127);

/// A place where we can put bits where we have estimated the probabilities.
pub trait EntropyCoder: Default {
    /// Encode a given bit with its probability
    fn encode_bit(&mut self, probability: ans::Probability, bit: bool);

    /// Encode the `value` into a `Vec<u8>` of bytes.`
    fn encode<T: Encode>(value: &T) -> Self {
        let mut writer = Self::default();
        value.encode(&mut writer, &mut T::Context::default());
        writer
    }

    /// Encode a given slice of incompressible bytes.
    ///
    /// Note that ideall implementations will do something more efficient than
    /// just omitting to track probabilities, but the default implementation
    /// should suffice for correctness.
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        for mut b in bytes.iter().copied() {
            for _ in 0..8 {
                self.encode_bit(FIFTY_PERCENT, (b & 1) == 1);
                b >>= 1;
            }
        }
    }
}

/// A way .
pub trait EntropyDecoder {
    /// Decode a given bit with the given probability
    fn decode_bit_nonadaptive(
        &mut self,
        probability: ans::Probability,
    ) -> Result<bool, std::io::Error>;

    /// Encode a given bit with its probability
    #[inline(always)]
    fn decode_bit(
        &mut self,
        context: &mut bit_context::BitContext,
    ) -> Result<bool, std::io::Error> {
        let bit = self.decode_bit_nonadaptive(context.probability())?;
        *context = context.adapt(bit);
        Ok(bit)
    }

    /// Decode a fixed number of incompressible bytes into a slice.
    #[inline(always)]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        for v in bytes {
            let mut b = 0;
            for i in 0..8 {
                b = b | ((self.decode_bit_nonadaptive(FIFTY_PERCENT)? as u8) << i);
            }
            *v = b;
        }
        Ok(())
    }
}

/// Trait for types that can be compactly encoded.
///
/// Normally you will derive this for your own types, although it can be
/// implemented manually.
pub trait Encode: Sized {
    /// Context storing probability model for this type.
    type Context: Default + Clone;

    /// Encode this value to the [`Writer<W>`].
    fn encode<E: EntropyCoder>(&self, encoder: &mut E, ctx: &mut Self::Context);

    /// Decode value from ['Reader<R>`].
    fn decode<D: EntropyDecoder>(
        entropy_decoder: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;

    /// Estimate the size of this value
    fn millibits(&self) -> Millibits {
        let mut m = Millibits::default();
        self.encode(&mut m, &mut Self::Context::default());
        m
    }
}

/// Encode the `value` into a `Vec<u8>` of bytes.`
pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut writer = arith::Range::default();
    value.encode(&mut writer, &mut T::Context::default());
    writer.into_vec()
}

/// Decode a value of this type from `bytes`.
///
/// Returns `None` if the bytes do not encode a valid value.
pub fn decode<T: Encode>(mut bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(&mut bytes);
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
    fn encode<E: EntropyCoder>(value: &T, writer: &mut E, ctx: &mut Self::Context);

    /// Decode the value using this strategy.
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error>;
}

/// Encode a value with a specific strategy (into a `Vec<u8>`).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn encode_with<T: Encode, S: EncodingStrategy<T>>(_: S, value: &T) -> Vec<u8> {
    let mut writer = Range::default();
    S::encode(value, &mut writer, &mut S::Context::default());
    writer.into_vec()
}

/// Decode a value with a specific strategy (from a bytes slice).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn decode_with<T: Encode, S: EncodingStrategy<T>>(_: S, mut bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(&mut bytes);
    S::decode(&mut reader, &mut S::Context::default()).ok()
}

impl<T, S: EncodingStrategy<T>> Encode for crate::Encoded<T, S> {
    type Context = S::Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        S::encode(&self.value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
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
    fn encode<E: EntropyCoder>(value: &T, writer: &mut E, ctx: &mut Self::Context) {
        value.encode(writer, ctx)
    }
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
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
        println!("Bytes are {bytes:?} for {ans:?}");
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

#[cfg(test)]
macro_rules! raw_bits {
    ($v:expr, $size:expr) => {
        let v = $v;
        let encoded = super::Raw::encode(&v);
        let decoded = super::Raw::decode(&encoded);
        let (bits, entropy) = super::Raw::sizes(&v);
        assert_eq!(decoded, Some(v));
        assert_eq!(bits, $size, "unexpected number of raw bits");
        assert_eq!(entropy, super::Millibits::bits($size), "unexpected entropy");
    };
    ($v:expr, $size:expr, $millibits:expr) => {
        let (bits, entropy) = super::Raw::sizes(&$v);
        assert_eq!(bits, $size, "unexpected number of raw bits");
        assert_eq!(entropy, $millibits, "unexpected entropy");
    };
}
#[cfg(test)]
pub(crate) use raw_bits;

#[cfg(test)]
macro_rules! assert_ans_bits {
    ($v:expr, $size:expr) => {
        let ans = $v;
        let bytes = super::Ans::encode(&ans);
        let decoded = super::Ans::decode(&bytes);
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
        let bytes = super::Ans::encode(&v);
        let decoded = super::Ans::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded tuple value is incorrect");
        assert_eq!((bytes.len() + 4) / 8, $size, "unexpected number of bits");
    };
    ($v:expr, $size:expr, $msg:expr) => {
        let ans = $v;
        let bytes = super::Ans::encode(&ans);
        let decoded = super::Ans::decode(&bytes);
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
        let bytes = super::Ans::encode(&v);
        let decoded = super::Ans::decode(&bytes);
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
pub(crate) use assert_ans_bits;
