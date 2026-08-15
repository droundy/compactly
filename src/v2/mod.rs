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
//!
//! 2. **The probability model** — `BitContext` (in `bit_context.rs`) is a
//!    small adaptive state machine (a generated 675-state table): ask it
//!    `probability()`, tell it what happened with `adapt(bit)`. `model.rs`
//!    holds the vocabulary the coders actually consume — `Probability` for
//!    one bit, `SymbolRange` for one whole tree symbol — plus `BitModel`,
//!    a context's hot-path data fused into a single table load.
//!
//! 3. **Codecs** — [`Encode`] impls for each type decide *which* bits and
//!    symbols to code under *which* contexts. `Encode<S>` is parameterized by
//!    the [strategy](Strategy) `S` (defaulting to [`Normal`](crate::Normal)),
//!    so `Encode<Small> for u64` is the same type coded a different way. The
//!    derive macro generates a `Context` struct with one field per struct
//!    field, so every field's model adapts independently.
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
mod sentinel;
mod sets;
mod string;
mod tuples;
mod usizes;
mod vecs;

use crate::{LowCardinality, Small};
pub use ans::Ans;
pub use arith::Range;
/// Benchmark-support surface for `benches/atmost.rs`; not part of the stable API.
///
/// `atmost` is a private module, so gating this re-export is what removes
/// `Walk`/`WALKS` from the public API — the items themselves stay `pub(crate)`
/// because production code (`Walk::production`) and the walk unit tests use
/// them either way.
#[doc(hidden)]
#[cfg(feature = "benchmarking")]
pub use atmost::walks::{Walk, WALKS};
pub use atmost::AtMost;
pub use millibits::Millibits;

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
///
/// `Default` is not a supertrait: a coder that owns an output sink (as the
/// streaming encoder behind [`encode_to`] does) cannot be `Default`. The
/// in-memory [`Self::encode`] constructor requires `Default` per-method
/// instead. `Sized` is kept so the symbol walks can pass `self`.
pub trait EntropyCoder: Sized {
    /// The sink this coder finalizes into (`W` for the streaming `XEncoder<W>`,
    /// `Vec<u8>` for the in-memory coders, `()` for [`Millibits`]).
    ///
    /// Intentionally **unbounded**: neither this trait nor the `stream_encode`
    /// helper ever calls [`Write`](std::io::Write) methods on it — only `new`,
    /// `finish`, and `Encode::encode`. The real `W: Write` bound lives on the
    /// `impl<W: Write> EntropyCoder for XEncoder<W>` blocks, where the writing
    /// happens. Leaving it unbounded lets [`Millibits`] use `type Writer = ()`.
    type Writer;

    /// Build a coder that writes into `writer`.
    ///
    /// No buffering is applied: wrap `writer` in a [`BufWriter`](std::io::BufWriter)
    /// yourself if it is an unbuffered sink like a `File` (an in-memory `Vec<u8>`
    /// needs none). Streaming coders write settled bytes as they emerge.
    fn new(writer: Self::Writer) -> Self;

    /// Flush all remaining coder state and return the sink, or the first IO error
    /// latched during encoding. Returning the sink (rather than `()`) is what lets
    /// `into_vec` be `finish().unwrap()` and lets a `File` caller recover its
    /// handle.
    fn finish(self) -> std::io::Result<Self::Writer>;

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

    /// Encode the `value` into a `Vec<u8>` of bytes.
    fn encode<T: Encode>(value: &T) -> Self
    where
        Self: Default,
    {
        let mut writer = Self::default();
        <T as Encode>::encode(value, &mut writer, &mut T::Context::default());
        writer
    }

    /// Encode one whole [`AtMost<MAX>`](AtMost) value — the adaptive
    /// primitive for "one of `MAX + 1` values", as [`Self::encode_bits`] is
    /// for bits.
    ///
    /// A default implementation is provided in terms of [`Self::encode_bit`],
    /// so a coder need only override this if it can code a whole value more
    /// efficiently than one bit at a time.
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
    /// The source this decoder pulls from (`R` for the streaming `XDecoder<R>`,
    /// `&[u8]` for the in-memory slice decoders). **Unbounded** for the same
    /// reason as [`EntropyCoder::Writer`]: the real `R: Read` bound is on the
    /// streaming impl blocks.
    type Reader;

    /// Build a decoder over `reader`.
    ///
    /// No buffering is applied: wrap `reader` in a [`BufReader`](std::io::BufReader)
    /// yourself for an unbuffered source (a `&[u8]` needs none).
    fn new(reader: Self::Reader) -> Self;

    /// Fold a decode result together with any IO error latched while reading.
    ///
    /// A latched read error **wins** even when `value` is itself `Err`: coder
    /// decode is infallible, so a mid-stream IO failure zero-pads instead of
    /// erroring, and the fabricated bits then often trip some unrelated validation
    /// (a zero `NonZero`, a bad `char`) deeper in `Encode::decode`. Returning that
    /// downstream symptom would silently drop the real root cause. In-memory slice
    /// decoders never latch, so they just return `value`.
    fn into_result<T>(self, value: Result<T, std::io::Error>) -> std::io::Result<T>;

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

    /// Decode one whole [`AtMost<MAX>`](AtMost) value; the inverse of
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
    /// build one on; every coder copies bytes wholesale (`Ans`/`Range`).
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error>;
}

/// Trait for types that can be compactly encoded.
///
/// Normally you will derive this for your own types, although it can be
/// implemented manually.
///
/// The parameter `S` selects the *encoding strategy* — the way this type is
/// turned into bits. It defaults to [`Normal`](crate::Normal), so the bound
/// `T: Encode` means "`T` has the default encoding", and implementing
/// `Encode<Small> for T` is what makes `#[compactly(Small)]` work on a field of
/// type `T`. A type may implement as many strategies as make sense for it.
///
/// Because the strategy is a parameter rather than a separate trait, the
/// methods here are associated functions taking `value: &Self`, not `&self`
/// methods — a type implementing several strategies would make `value.encode(…)`
/// ambiguous. Call a codec through its strategy instead — [`Strategy`] gives
/// every strategy `Normal::encode(&value, …)` / `Small::decode(reader, …)`, so
/// the default is spelled the same way as every other strategy.
#[diagnostic::on_unimplemented(
    message = "`{Self}` cannot be encoded with the `{S}` strategy",
    label = "no `{S}` encoding for `{Self}`",
    note = "add `#[derive(Encode)]` to `{Self}` if it is your own type, or pick a strategy it supports"
)]
pub trait Encode<S = crate::Normal>: Sized {
    /// Context storing probability model for this type.
    type Context: Default + Clone;

    /// Encode `value` with the given [`EntropyCoder`].
    fn encode<E: EntropyCoder>(value: &Self, encoder: &mut E, ctx: &mut Self::Context);

    /// Decode a value with the given [`EntropyDecoder`].
    fn decode<D: EntropyDecoder>(
        entropy_decoder: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error>;
}

/// Estimate the encoded size of `value` under its default encoding, without
/// producing any bytes.
///
/// Crate-private: this exists for the size assertions in the codec unit tests.
/// If it ever becomes user-facing it should be a `pub fn` alongside [`encode`],
/// not a method — a method would only be able to sugar the default strategy,
/// leaving `Small`, `Compressible`, … spelled a different way.
#[cfg(test)]
pub(crate) fn millibits<T: Encode>(value: &T) -> Millibits {
    let mut m = Millibits::default();
    <T as Encode>::encode(value, &mut m, &mut <T as Encode>::Context::default());
    m
}

/// Marker for the strategy types, giving them `Small::encode(&value, …)` syntax.
///
/// This is pure sugar over [`Encode`]: `Small::encode(&v, coder, ctx)` is
/// `<T as Encode<Small>>::encode(&v, coder, ctx)`. It is opt-in per strategy
/// rather than blanket-implemented, so these names never land on the types
/// being *encoded*.
///
/// You can define entirely new strategies in your own crate: declare the marker
/// type, implement `Encode<YourStrategy>` for the types it should apply to, and
/// implement this trait to get the calling syntax. Name it by full path in a
/// derive attribute — `#[compactly(your_crate::SuperCoolStrategy)]`.
///
/// ```
/// use compactly::v2::{Encode, EntropyCoder, EntropyDecoder, Strategy};
///
/// pub struct SuperCoolStrategy;
/// impl Strategy for SuperCoolStrategy {}
///
/// impl Encode<SuperCoolStrategy> for u8 {
///     type Context = <u8 as Encode>::Context;
///     fn encode<E: EntropyCoder>(value: &u8, w: &mut E, ctx: &mut Self::Context) {
///         <u8 as Encode>::encode(value, w, ctx)
///     }
///     fn decode<D: EntropyDecoder>(r: &mut D, ctx: &mut Self::Context)
///         -> Result<u8, std::io::Error> {
///         <u8 as Encode>::decode(r, ctx)
///     }
/// }
///
/// // and it lifts through the transparent wrappers for free:
/// let bytes = compactly::v2::encode_with(SuperCoolStrategy, &Some(Box::new(7u8)));
/// assert_eq!(
///     compactly::v2::decode_with(SuperCoolStrategy, &bytes),
///     Some(Some(Box::new(7u8))),
/// );
/// ```
pub trait Strategy: Sized {
    /// Encode `value` with this strategy.
    #[inline]
    fn encode<T: Encode<Self>, E: EntropyCoder>(
        value: &T,
        writer: &mut E,
        ctx: &mut <T as Encode<Self>>::Context,
    ) {
        <T as Encode<Self>>::encode(value, writer, ctx)
    }

    /// Decode a value using this strategy.
    #[inline]
    fn decode<T: Encode<Self>, D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut <T as Encode<Self>>::Context,
    ) -> Result<T, std::io::Error> {
        <T as Encode<Self>>::decode(reader, ctx)
    }
}

impl Strategy for crate::Normal {}
impl Strategy for crate::Small {}
impl Strategy for crate::Compressible {}
impl Strategy for crate::Incompressible {}
impl Strategy for crate::Sorted {}
impl Strategy for crate::LowCardinality {}
impl Strategy for crate::Decimal {}
impl<K, V> Strategy for crate::Mapping<K, V> {}
impl<V> Strategy for crate::Values<V> {}

/// Encode the `value` into a `Vec<u8>` of bytes.
pub fn encode<T: Encode>(value: &T) -> Vec<u8> {
    let mut writer = arith::Range::default();
    <T as Encode>::encode(value, &mut writer, &mut T::Context::default());
    writer.into_vec()
}

/// Decode a value of this type from `bytes`.
///
/// Returns `None` if the bytes do not encode a valid value.
pub fn decode<T: Encode>(bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(bytes);
    T::decode(&mut reader, &mut T::Context::default()).ok()
}

/// Eager pre-allocation size for a length decoded from untrusted input.
///
/// A corrupt or truncated stream can decode an absurd length; passing it
/// straight to `Vec::with_capacity` (etc.) panics with a capacity overflow — or
/// speculatively allocates gigabytes — *before* the decode loop can reach the
/// error and return `Err`. Capping the eager reservation at roughly 1 MiB
/// (regardless of element size) avoids that immediate allocation failure; the
/// container still grows to the true length for a valid stream, at most a few
/// extra reallocations for genuinely large collections.
///
/// This bounds the *eager* allocation only, not total decode work — the two are
/// separate defenses and both are needed. Total work is bounded by the periodic
/// marker in [`sentinel`](self::sentinel), which every length-driven loop codes
/// and which a corrupt stream cannot forge; that catches an absurd claimed length
/// within one marker interval even for element types where every bit pattern is
/// legal (`u8`) and so nothing else rejects it. This cap is what keeps the
/// allocation from failing *before* the loop gets far enough to check.
#[inline]
pub(crate) fn capacity_for<T>(len: usize) -> usize {
    let elem = std::mem::size_of::<T>().max(1);
    len.min((1 << 20) / elem)
}

/// Shared encode plumbing: build a coder over `writer`, encode `value`, and
/// finish. One monomorphized copy serves every coder's `encode_to` (and the
/// free [`encode_to`] below), so there is no per-coder duplication and no `fn`
/// pointer between the entry point and the coder.
fn stream_encode<T: Encode, E: EntropyCoder>(
    value: &T,
    writer: E::Writer,
) -> std::io::Result<E::Writer> {
    let mut encoder = E::new(writer);
    <T as Encode>::encode(value, &mut encoder, &mut T::Context::default());
    encoder.finish()
}

/// Shared decode plumbing: run `T::decode` and fold the result with any latched
/// IO error (the [`EntropyDecoder::into_result`] rule). Takes an **already-built**
/// decoder rather than constructing one, because decoder construction is not
/// uniform across coders — `Ans::decode_from` first peeks the leading chunk tag
/// to choose its single-chunk fast path — while this tail (decode + `into_result`)
/// is identical for both, and is where the latched-error correctness lives.
fn stream_decode<T: Encode, D: EntropyDecoder>(mut decoder: D) -> std::io::Result<T> {
    let value = T::decode(&mut decoder, &mut T::Context::default());
    decoder.into_result(value)
}

/// Encode `value` straight into a [`Write`](std::io::Write), streaming bytes out
/// as they are produced rather than buffering the whole compressed output. The
/// bytes are **identical** to [`encode(value)`](encode) — streaming only bounds
/// peak memory, which matters when the value is a large fraction of RAM.
///
/// Uses the default [`Range`] coder; [`Range::encode_to`] / [`Ans::encode_to`]
/// select explicitly. No buffering is applied — wrap an unbuffered sink like a
/// `File` in a [`BufWriter`](std::io::BufWriter) yourself.
pub fn encode_to<T: Encode, W: std::io::Write>(value: &T, writer: W) -> std::io::Result<()> {
    Range::encode_to(value, writer)
}

/// Decode a value straight from a [`Read`](std::io::Read), pulling bytes on
/// demand rather than requiring the whole compressed input in memory. Accepts
/// the same bytes [`encode`]/[`encode_to`] produce.
///
/// Uses the default [`Range`] coder; [`Range::decode_from`] / [`Ans::decode_from`]
/// select explicitly. No buffering is applied — wrap an unbuffered source in a
/// [`BufReader`](std::io::BufReader) yourself.
pub fn decode_from<T: Encode, R: std::io::Read>(reader: R) -> std::io::Result<T> {
    Range::decode_from(reader)
}

/// Encode a value with a specific strategy (into a `Vec<u8>`).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn encode_with<S: Strategy, T: Encode<S>>(_: S, value: &T) -> Vec<u8> {
    let mut writer = Range::default();
    S::encode(
        value,
        &mut writer,
        &mut <T as Encode<S>>::Context::default(),
    );
    writer.into_vec()
}

/// Decode a value with a specific strategy (from a bytes slice).
///
/// I don't expect this to be used in practice, but it can be helpful for
/// testing.
pub fn decode_with<S: Strategy, T: Encode<S>>(_: S, bytes: &[u8]) -> Option<T> {
    let mut reader = arith::Decoder::new(bytes);
    S::decode(&mut reader, &mut <T as Encode<S>>::Context::default()).ok()
}

impl<T: Encode<S>, S> Encode<crate::Normal> for crate::Encoded<T, S> {
    type Context = <T as Encode<S>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        <T as Encode<S>>::encode(&value.value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            value: <T as Encode<S>>::decode(reader, ctx)?,
            _phantom: std::marker::PhantomData,
        })
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
        let bits = crate::v2::millibits(&v).as_bits();
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

/// Round-trips the value through the default `Range` coder, and checks it
/// against a string describing the estimated size according to [`Millibits`]:
/// an exact `"N bits"` when the estimate lands on a whole bit, else the raw
/// `Millibits` debug form.
#[cfg(test)]
macro_rules! assert_millibits {
    ($v:expr, $expected:expr) => {{
        let v = $v;
        let entropy = crate::v2::millibits(&v);
        let encoded = super::encode(&v);
        let decoded = super::decode(&encoded);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        let bits: usize = entropy.as_bits().parse().unwrap();
        let s = if entropy == super::Millibits::bits(bits) {
            format!("{bits} bits")
        } else {
            format!("{entropy:?}")
        };
        $expected.assert_eq(&s);
    }};
}
#[cfg(test)]
pub(crate) use assert_millibits;

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
