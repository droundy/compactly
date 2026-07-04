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
mod nonzero;
mod maps;
mod markers;
mod net;
mod millibits;
mod option;
mod other_crate_types;
mod raw;
mod sets;
mod string;
mod symbol;
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
    fn encode_bits<const N: usize>(&mut self, bits_with_probabilities: [(bool, ans::Probability); N]);

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

    /// Encode one whole `log2(N)`-bit tree symbol.
    ///
    /// `contexts` is the heap-shaped context array walked as
    /// `node = (node << 1) + 1 + bit` (the `u8`/`Bits<N>`/`UBits<N>` codes);
    /// its length `N` is exactly `1 << n_bits`, so the bit count is derived
    /// from the array type and the walk fully unrolls after monomorphization.
    ///
    /// The default implementation codes the tree bit-by-bit, identical to the
    /// historical per-bit walk (`Raw` keeps it, preserving its bit-packed
    /// format). Coders with a whole-symbol primitive (`Range`, `Ans`,
    /// `Millibits`) override it via [`symbol::SymbolRange`]: same contexts,
    /// same adaptation, but a single coding step instead of `log2(N)`.
    #[inline]
    fn encode_tree<const N: usize>(
        &mut self,
        contexts: &mut [bit_context::BitContext; N],
        value: usize,
    ) {
        let n_bits = N.ilog2();
        debug_assert_eq!(1 << n_bits, N);
        debug_assert!(value < N);
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let bit = (value >> i) & 1 == 1;
            let context = &mut contexts[node];
            self.encode_bit(context.probability(), bit);
            *context = context.adapt(bit);
            node = (node << 1) + 1 + bit as usize;
        }
    }

    /// Encode one *escaped-tree* symbol: a `root` bit for "is a value
    /// present", then — only when present — the `log2(N)`-bit tree for the
    /// value. This is the `bool`-guarded-tree pattern (`char`'s `is_ascii` +
    /// 7-bit ASCII tree) fused so single-step coders pay one step for the
    /// common present case instead of two; `None` is the escape and costs
    /// just the root bit.
    ///
    /// The default implementation is the unfused bit-by-bit coding (kept by
    /// `Raw`); `Range`/`Ans`/`Millibits` override it with a single
    /// [`symbol::SymbolRange`] step. Adaptation is identical either way.
    #[inline]
    fn encode_escaped_tree<const N: usize>(
        &mut self,
        root: &mut bit_context::BitContext,
        contexts: &mut [bit_context::BitContext; N],
        value: Option<usize>,
    ) {
        let present = value.is_some();
        self.encode_bit(root.probability(), present);
        *root = root.adapt(present);
        if let Some(value) = value {
            self.encode_tree(contexts, value);
        }
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
    fn decode_bits<const N: usize>(&mut self, contexts: &mut [bit_context::BitContext; N]) -> [bool; N];

    /// Decode a given bit, adapting its probability context. The `N == 1` case of
    /// [`Self::decode_bits`]; `array::from_mut` reinterprets the `&mut BitContext`
    /// as a `&mut [BitContext; 1]` for free (no copy).
    #[inline(always)]
    fn decode_bit(&mut self, context: &mut bit_context::BitContext) -> bool {
        let [bit] = self.decode_bits(std::array::from_mut(context));
        bit
    }

    /// Decode one whole tree symbol; the inverse of
    /// [`EntropyCoder::encode_tree`]. Returns the decoded value in `0..N`.
    ///
    /// Infallible like [`Self::decode_bits`]: running past the encoded data
    /// yields arbitrary (but in-range) values, which higher-level
    /// `Encode::decode` impls validate.
    #[inline]
    fn decode_tree<const N: usize>(
        &mut self,
        contexts: &mut [bit_context::BitContext; N],
    ) -> usize {
        let n_bits = N.ilog2();
        debug_assert_eq!(1 << n_bits, N);
        let mut node = 0usize;
        for _ in 0..n_bits {
            let bit = self.decode_bit(&mut contexts[node]);
            node = (node << 1) + 1 + bit as usize;
        }
        node - (N - 1)
    }

    /// Decode one escaped-tree symbol; the inverse of
    /// [`EntropyCoder::encode_escaped_tree`]. Returns `None` for the escape,
    /// or the decoded value in `0..N`.
    #[inline]
    fn decode_escaped_tree<const N: usize>(
        &mut self,
        root: &mut bit_context::BitContext,
        contexts: &mut [bit_context::BitContext; N],
    ) -> Option<usize> {
        if self.decode_bit(root) {
            Some(self.decode_tree(contexts))
        } else {
            None
        }
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

/// Compare an actual encoded size against the expected one recorded in a
/// test. Normally panics on mismatch like `assert_eq!`, but when the
/// `COLLECT_SIZE_DIFFS` environment variable is set it prints a `SIZEDIFF`
/// line and keeps going — so after an intentional encoding change, one
/// `COLLECT_SIZE_DIFFS=1 cargo test` run lists every expected number that
/// needs updating (and shows the compression impact) instead of stopping at
/// the first.
#[cfg(test)]
#[track_caller]
pub(crate) fn check_encoded_size<T: PartialEq + std::fmt::Debug>(actual: T, expected: T, what: &str) {
    if actual == expected {
        return;
    }
    if std::env::var_os("COLLECT_SIZE_DIFFS").is_some() {
        let loc = std::panic::Location::caller();
        println!(
            "SIZEDIFF {}:{}: {expected:?} -> {actual:?} ({what})",
            loc.file(),
            loc.line()
        );
    } else {
        panic!("unexpected encoded size ({what}): expected {expected:?}, got {actual:?}");
    }
}

#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:expr) => {
        let v = $v;
        let bytes = super::encode(&v);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        super::check_encoded_size(bytes.len(), $size, "bytes");
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
        super::check_encoded_size((bytes.len() + 4) / 8, $size, "bits");
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
        super::check_encoded_size((bytes.len() + 4) / 8, $size, &format!("bits: {}", $msg));
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
        super::check_encoded_size(bits, $size, "raw bits");
        super::check_encoded_size(entropy, super::Millibits::bits($size), "entropy");
    };
    ($v:expr, $size:expr, $millibits:expr) => {
        let (bits, entropy) = super::Raw::sizes(&$v);
        super::check_encoded_size(bits, $size, "raw bits");
        super::check_encoded_size(entropy, $millibits, "entropy");
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
        super::check_encoded_size((bytes.len() + 4) / 8, $size, "ans bits");
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
