//! The `ans` format of compactly.
//!
//! This format should be unmodified after the 1.0 release, except for addition
//! of support for new strategies, which won't change the binary format of types
//! that don't use those strategies.
pub use compactly_derive::EncodeV2 as Encode;

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
mod markers;
mod millibits;
mod net;
mod nonzero;
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
    /// Encode `N` bits, each paired with its probability.
    ///
    /// This is the primitive encode operation. Because `N` is a constant,
    /// implementations may keep state in registers across the batch and
    /// specialize for fixed widths, and `Encode` impls can encode and decode in
    /// the same batched shape.
    fn encode_bits<const N: usize>(
        &mut self,
        bits_with_probabilities: [(bool, ans::Probability); N],
    );

    /// Encode a given bit with its probability. The `N == 1` case.
    #[inline(always)]
    fn encode_bit(&mut self, probability: ans::Probability, bit: bool) {
        self.encode_bits([(bit, probability)]);
    }

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
    /// Decode `N` bits, each with its own independent probability context.
    ///
    /// This is the core required primitive — `decode_bit` is just the `N == 1`
    /// case. Taking the contexts as one `&mut [BitContext; N]` (rather than
    /// `[&mut BitContext; N]`) lets the coder index the array in place instead of
    /// receiving a materialized array of `N` pointers, which was measurable
    /// overhead. The contexts are independent, so the coder is free to keep its
    /// state register-resident across the whole batch.
    ///
    /// Decoding a bit is infallible: there is always a bit to produce from the
    /// coder state (running past the encoded data simply yields arbitrary bits,
    /// which higher-level `Encode::decode` impls validate). Returning `[bool; N]`
    /// rather than a `Result` keeps error edges out of the hot path.
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [bit_context::BitContext; N],
    ) -> [bool; N];

    /// Decode a given bit, adapting its probability context. The `N == 1` case of
    /// [`Self::decode_bits`]; `array::from_mut` reinterprets the `&mut BitContext`
    /// as a `&mut [BitContext; 1]` for free (no copy).
    #[inline(always)]
    fn decode_bit(&mut self, context: &mut bit_context::BitContext) -> bool {
        let [bit] = self.decode_bits(std::array::from_mut(context));
        bit
    }

    /// Decode a fixed number of incompressible bytes into a slice.
    ///
    /// Required (no default) because there is no single-bit no-adapt primitive to
    /// build one on; every coder either copies bytes wholesale (`Ans`/`Range`) or
    /// reads raw bits (`Raw`).
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error>;
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
pub fn decode<T: Encode>(bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(bytes);
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
pub fn decode_with<T: Encode, S: EncodingStrategy<T>>(_: S, bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(bytes);
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
    ($v:expr, $expected:expr) => {
        let v = $v;
        let bytes = super::encode(&v);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        $expected.assert_eq(&bytes.len().to_string());
    };
}
#[cfg(test)]
pub(crate) use assert_size;

/// Encodes the value once and as 64 copies, checking that both round-trip,
/// and evaluates to the number of bits (rounded) needed to encode the 64
/// copies.
#[cfg(test)]
macro_rules! encoded_bits {
    ($v:expr) => {{
        let one = $v;
        let bytes = super::encode(&one);
        println!("Bytes are {bytes:?} for {one:?}");
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(one), "decoded value is incorrect");
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
        (bytes.len() + 4) / 8
    }};
}
#[cfg(test)]
pub(crate) use encoded_bits;

#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $expected:expr) => {
        $expected.assert_eq(&crate::v2::encoded_bits!($v).to_string());
    };
}
#[cfg(test)]
pub(crate) use assert_bits;

/// Like [`assert_bits!`], but takes an iterator of values (optionally mapped
/// through a function) that are all expected to encode to the same number of
/// bits.
#[cfg(test)]
macro_rules! assert_bits_all {
    ($values:expr, $expected:expr) => {
        crate::v2::assert_bits_all!($values, |v| v, $expected);
    };
    ($values:expr, $f:expr, $expected:expr) => {
        let f = $f;
        let mut iter = ($values).into_iter();
        let first = iter
            .next()
            .expect("assert_bits_all! needs at least one value");
        let bits = crate::v2::encoded_bits!(f(first));
        for v in iter {
            let other = crate::v2::encoded_bits!(f(v));
            assert_eq!(other, bits, "encoded size differs for {v:?}");
        }
        $expected.assert_eq(&bits.to_string());
    };
}
#[cfg(test)]
pub(crate) use assert_bits_all;

/// Checks that the value round-trips through the `Raw` coder, and evaluates
/// to a string describing the raw size in bits and the estimated entropy.
#[cfg(test)]
macro_rules! raw_size {
    ($v:expr) => {{
        let v = $v;
        let encoded = super::Raw::encode(&v);
        let decoded = super::Raw::decode(&encoded);
        let (bits, entropy) = super::Raw::sizes(&v);
        assert_eq!(decoded, Some(v), "raw decoded value is incorrect");
        if entropy == super::Millibits::bits(bits) {
            format!("{bits} bits")
        } else {
            format!("{bits} bits, entropy {entropy:?}")
        }
    }};
}
#[cfg(test)]
pub(crate) use raw_size;

#[cfg(test)]
macro_rules! raw_bits {
    ($v:expr, $expected:expr) => {
        $expected.assert_eq(&crate::v2::raw_size!($v));
    };
}
#[cfg(test)]
pub(crate) use raw_bits;

/// Like [`encoded_bits!`] but for the `Ans` coder.
#[cfg(test)]
macro_rules! ans_encoded_bits {
    ($v:expr) => {{
        let one = $v;
        let bytes = super::Ans::encode(&one);
        let decoded = super::Ans::decode(&bytes);
        assert_eq!(decoded, Some(one), "decoded value is incorrect");
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
        (bytes.len() + 4) / 8
    }};
}
#[cfg(test)]
pub(crate) use ans_encoded_bits;

#[cfg(test)]
macro_rules! assert_ans_bits {
    ($v:expr, $expected:expr) => {
        $expected.assert_eq(&crate::v2::ans_encoded_bits!($v).to_string());
    };
}
#[cfg(test)]
pub(crate) use assert_ans_bits;
