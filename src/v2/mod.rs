//! The `v2` format of compactly: adaptive entropy coding with two
//! interchangeable coders (`Range`, the default, and the faster-decoding
//! `Ans`).
//!
//! # How v2 fits together
//!
//! Three layers, each blind to the ones above it:
//!
//! 1. **Entropy coders** — [`EntropyCoder`]/[`EntropyDecoder`]
//!    implementations that turn a sequence of probability-weighted coding
//!    decisions into bytes and back. A coder knows nothing about types:
//!
//!    | Coder | Purpose |
//!    |-------|---------|
//!    | [`Range`] | Default; arithmetic/range coding; what [`encode`]/[`decode`] use |
//!    | [`Ans`] | rANS; same interface, decodes faster (runs the stream backwards, so encoding buffers ops) |
//!    | [`Millibits`] | Size estimation only; accumulates fractional bits, produces no bytes |
//!    | [`Raw`] | Bit-packing with the probabilities ignored; the coder traits' default method bodies *are* its format |
//!
//! 2. **The probability model** — `BitContext` (in `bit_context.rs`) is a
//!    small adaptive state machine (a generated 675-state table): ask it
//!    `probability()`, tell it what happened with `adapt(bit)`. `model.rs`
//!    holds the vocabulary the coders actually consume — `Probability` for
//!    one bit, `SymbolRange` for one whole tree symbol — plus `BitModel`,
//!    a context's hot-path data fused into a single table load.
//!
//! 3. **Codecs** — [`Encode`] impls (and [`EncodingStrategy`] variants) for
//!    each type decide *which* bits and symbols to code under *which*
//!    contexts. The derive macro generates a `Context` struct with one field
//!    per struct field, so every field's model adapts independently.
//!
//! ## The unit of coding is a sub-interval
//!
//! Every adaptive thing a coder accepts means "narrow your state to the
//! sub-interval `[start, start + width)` of `[0, 2^k)`":
//!
//! - a bit with `Probability` `p` is the two-slot case, `k = 8`:
//!   `[0, 256p)` for false, `[256p, 256)` for true;
//! - a whole [`AtMost`] symbol with a `SymbolRange` is the general case,
//!   `k = 16`, built by the tree walks in `atmost::walks`.
//!
//! Conceptually a bool *is* an `AtMost<1>`. They stay separate primitives on
//! purpose: the coders give bits and symbols deliberately different
//! renormalization regimes (`Ans` bit steps refill at most one byte against
//! a base-256 total; symbol steps up to two bytes against base-2^16), bits
//! dominate the coded traffic, and the batched [`EntropyDecoder::decode_bits`]
//! fast path is a bits-only concept. Merging them was measured and rejected;
//! see OPTIMIZING.md.
//!
//! ## The lockstep contract
//!
//! The decoder can only recover values because it reproduces, bit for bit,
//! every probability the encoder used. So encode and decode of each codec
//! must read and adapt *the same contexts in the same order*, and the tree
//! walks guarantee that the whole-symbol and bit-by-bit paths adapt
//! identically (tested against a reference walk in `atmost::walks`). Coder
//! decode is deliberately infallible — running past the end of the stream
//! yields arbitrary in-range values — and validation happens once, in
//! `Encode::decode` impls.
//!
//! ## Performance doctrine (invariants, not suggestions)
//!
//! Decode is **latency-bound** (measured IPC ≈ 1.4): the next coding step
//! cannot start until the previous bit resolves, so cycles — never
//! instruction counts — decide. Standing invariants, each backed by
//! measurement:
//!
//! - tree walks fully unroll (compile-time trip counts via `const` tree
//!   depth) and inline into the coder's symbol step;
//! - work moves *off* the serial bit-resolution chain (speculating on both
//!   children) only where a coder's symbol step can absorb the extra
//!   instructions — the choice is per-coder and measured, recorded in the
//!   walk inventory in `atmost::walks`;
//! - probability priors belong in seeded initial contexts
//!   (`AtMostContext::SEEDED`), never in the coding split.
//!
//! Benchmarking discipline, results, and the graveyard of measured dead ends
//! live in OPTIMIZING.md.
//!
//! # Stability
//!
//! This format should be unmodified after the 1.0 release, except for addition
//! of support for new strategies, which won't change the binary format of types
//! that don't use those strategies.
pub use compactly_derive::EncodeV2 as Encode;

mod ans;
mod arc;
mod arith;
mod array;
mod atmost;
mod bit_context;
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
mod model;
mod net;
mod nonzero;
mod option;
mod other_crate_types;
mod raw;
mod sets;
mod string;
mod tuples;
mod usizes;
mod vecs;

use crate::{LowCardinality, Small};
pub use ans::Ans;
pub use arith::Range;
pub use atmost::AtMost;
pub use millibits::Millibits;
pub use raw::Raw;

/// The default `encode_incompressible_bytes` relies on a fresh context
/// meaning "no information": its probability must be exactly one half.
#[test]
fn default_context_is_fifty_percent() {
    assert_eq!(
        bit_context::BitContext::default().probability(),
        model::Probability::new(127, 127)
    );
}

/// A place where we can put bits where we have estimated the probabilities.
pub trait EntropyCoder: Default {
    /// Encode `N` bits, each with its own independent adaptive context —
    /// symmetric with [`EntropyDecoder::decode_bits`]: the coder reads each
    /// context's probability and adapts it, so encode- and decode-side
    /// context bookkeeping cannot drift apart.
    ///
    /// This is the primitive encode operation. Because `N` is a constant,
    /// implementations may keep state in registers across the batch and
    /// specialize for fixed widths, and `Encode` impls can encode and decode in
    /// the same batched shape.
    fn encode_bits<const N: usize>(
        &mut self,
        contexts: &mut [bit_context::BitContext; N],
        bits: [bool; N],
    );

    /// Encode a given bit, adapting its probability context. The `N == 1`
    /// case of [`Self::encode_bits`].
    #[inline(always)]
    fn encode_bit(&mut self, context: &mut bit_context::BitContext, bit: bool) {
        self.encode_bits(std::array::from_mut(context), [bit]);
    }

    /// Encode the `value` into a `Vec<u8>` of bytes.`
    fn encode<T: Encode>(value: &T) -> Self {
        let mut writer = Self::default();
        value.encode(&mut writer, &mut T::Context::default());
        writer
    }

    /// Encode one whole [`AtMost<MAX>`](AtMost) symbol — the adaptive
    /// primitive for "one of `MAX + 1` values", as [`Self::encode_bits`] is
    /// for bits. The tree search over `0..=MAX` and the per-node context
    /// bookkeeping are `AtMost`'s implementation details (see
    /// `atmost::walks`); the coder only chooses how to *pay* for the walk.
    ///
    /// The default implementation codes the search bit-by-bit through
    /// [`Self::encode_bit`], identical to the historical walk (`Raw` keeps
    /// it, preserving its bit-packed format). Coders with a whole-symbol
    /// primitive (`Range`, `Ans`, `Millibits` — the internal
    /// `model::SymbolCoder` trait) override it with a one-liner into
    /// `atmost::walks::encode_symbol_or_bitwise`, paying a single coding
    /// step (one renormalization) for the whole symbol.
    #[inline]
    fn encode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut atmost::AtMostContext<MAX>,
        value: AtMost<MAX>,
    ) {
        atmost::walks::encode_bitwise(self, &mut ctx.bits, value.into())
    }

    /// Encode a given slice of incompressible bytes.
    ///
    /// Note that ideally implementations will do something more efficient than
    /// just omitting to track probabilities, but the default implementation
    /// should suffice for correctness.
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        for mut b in bytes.iter().copied() {
            for _ in 0..8 {
                // A throwaway default context is exactly a 50/50 probability
                // (checked by `default_context_is_fifty_percent`), and
                // discarding it after one bit keeps it that way.
                self.encode_bit(&mut bit_context::BitContext::default(), (b & 1) == 1);
                b >>= 1;
            }
        }
    }
}

/// The read-side counterpart of [`EntropyCoder`]: decodes the bits, symbols,
/// and incompressible bytes in the same order they were encoded, adapting the
/// same contexts identically.
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

    /// Decode one whole [`AtMost<MAX>`](AtMost) symbol; the inverse of
    /// [`EntropyCoder::encode_atmost`].
    ///
    /// Infallible like [`Self::decode_bits`]: running past the encoded data
    /// yields arbitrary (but in-range) values, which higher-level
    /// `Encode::decode` impls validate.
    #[inline]
    fn decode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut atmost::AtMostContext<MAX>,
    ) -> AtMost<MAX>
    where
        Self: Sized,
    {
        AtMost::new(atmost::walks::decode_bitwise(self, &mut ctx.bits))
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

    /// Encode this value with the given [`EntropyCoder`].
    fn encode<E: EntropyCoder>(&self, encoder: &mut E, ctx: &mut Self::Context);

    /// Decode a value with the given [`EntropyDecoder`].
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
/// and evaluates to a `String` holding the number of bits (rounded) needed to
/// encode the 64 copies, ready to pass to `expect![...].assert_eq(...)`.
/// Uses the default `Range` coder unless another coder type is given as the
/// first argument.
#[cfg(test)]
macro_rules! encoded_bits {
    ($v:expr) => {
        crate::v2::encoded_bits!(crate::v2::Range, $v)
    };
    ($coder:ty, $v:expr) => {{
        let one = $v;
        let bytes = <$coder>::encode(&one);
        println!("Bytes are {bytes:?} for {one:?}");
        let decoded = <$coder>::decode(&bytes);
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
        let bytes = <$coder>::encode(&v);
        let decoded = <$coder>::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded tuple value is incorrect");
        ((bytes.len() + 4) / 8).to_string()
    }};
}
#[cfg(test)]
pub(crate) use encoded_bits;

/// Round-trips the value once (encode → decode → assert equal) and evaluates to
/// a `String` holding the estimated size in bits according to the [`Millibits`]
/// entropy estimator, ready to pass to `expect![...].assert_eq(...)`.
///
/// Prefer this over [`encoded_bits!`] when the test is about how compactly a
/// format encodes a value: it measures the format's entropy directly, free of
/// the range coder's rounding and per-copy amortization. Reach for
/// [`encoded_bits!`] only when the actual coded output is what's under test
/// (e.g. comparing the range coder against `Ans`, or checking that the coder
/// achieves its `millibits` estimate).
#[cfg(test)]
macro_rules! estimated_bits {
    ($v:expr) => {{
        let v = $v;
        let bits = crate::v2::Encode::millibits(&v).as_bits();
        let bytes = super::encode(&v);
        let decoded = super::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        bits
    }};
}
#[cfg(test)]
pub(crate) use estimated_bits;

/// Takes an iterator of values (optionally mapped through a function) that are
/// all expected to have the same [`estimated_bits!`] count, and checks that
/// count against the expected value.
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
        let bits = crate::v2::estimated_bits!(f(first));
        for v in iter {
            let other = crate::v2::estimated_bits!(f(v));
            assert_eq!(other, bits, "encoded size differs for {v:?}");
        }
        $expected.assert_eq(&bits);
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

/// Round-trips randomly interleaved context-driven bits and whole-tree byte
/// symbols through a real coder: bits (total 256) and tree symbols
/// (total 2^16) share one state and stream, and encode and decode must adapt
/// identical context state throughout. `$make_decoder` builds the coder's
/// decoder from a `&[u8]`.
#[cfg(test)]
macro_rules! check_mixed_bits_and_symbols {
    ($coder:ty, $make_decoder:expr) => {{
        use crate::v2::bit_context::BitContext;
        use crate::v2::{EntropyCoder, EntropyDecoder};
        for trial in 0..2000 {
            let n_ops = rand::random::<usize>() % 200;
            #[derive(Debug, Clone, Copy)]
            enum Planned {
                Bit(bool),
                Byte(u8),
            }
            let mut plan = Vec::new();
            for _ in 0..n_ops {
                if rand::random::<bool>() {
                    plan.push(Planned::Bit(rand::random()));
                } else {
                    plan.push(Planned::Byte(rand::random()));
                }
            }
            // Bits draw round-robin from a bank of contexts starting in
            // random states, so the coder sees a wide range of probabilities.
            let mut bit_bank = [BitContext::default(); 8];
            for ctx in bit_bank.iter_mut() {
                *ctx = rand::random();
            }
            let mut enc_bits = bit_bank;
            let mut enc_bytes = crate::v2::atmost::AtMostContext::<255>::default();
            let mut writer = <$coder>::default();
            let mut which = 0usize;
            for op in &plan {
                match *op {
                    Planned::Bit(b) => {
                        writer.encode_bit(&mut enc_bits[which % 8], b);
                        which += 1;
                    }
                    Planned::Byte(b) => {
                        writer.encode_atmost(&mut enc_bytes, crate::v2::AtMost::new(b as usize))
                    }
                }
            }
            let encoded: Vec<u8> = writer.into_vec();
            #[allow(clippy::redundant_closure_call)]
            let mut decoder = ($make_decoder)(encoded.as_slice());
            let mut dec_bits = bit_bank;
            let mut dec_bytes = crate::v2::atmost::AtMostContext::<255>::default();
            let mut which = 0usize;
            for (i, op) in plan.iter().enumerate() {
                match *op {
                    Planned::Bit(b) => {
                        let bit = decoder.decode_bit(&mut dec_bits[which % 8]);
                        which += 1;
                        assert_eq!(bit, b, "bit {i} of trial {trial}");
                    }
                    Planned::Byte(b) => {
                        let v = decoder.decode_atmost(&mut dec_bytes);
                        assert_eq!(usize::from(v), b as usize, "byte {i} of trial {trial}");
                    }
                }
            }
            assert_eq!(enc_bits, dec_bits, "bit contexts must adapt identically");
            assert_eq!(enc_bytes, dec_bytes, "byte contexts must adapt identically");
        }
    }};
}
#[cfg(test)]
pub(crate) use check_mixed_bits_and_symbols;
