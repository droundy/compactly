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

/// Derive [`Encode`](trait@Encode): both the sync impl and the async-decode
/// members ([`Encode::MAX_BYTES`], [`Encode::decode_awaiting`]) that
/// [`decode_stream`] needs.
///
/// There is only ever one derive: [`Encode`](trait@Encode) is one trait with
/// no opt-out, so a type reachable through `decode_stream` needs every field's
/// type to implement it too — which every type in this crate does.
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
#[cfg(test)]
mod max_bytes;
mod millibits;
mod model;
mod net;
mod nonzero;
mod option;
mod other_crate_types;
mod sentinel;
mod sets;
#[cfg(feature = "stream")]
mod stream;
mod string;
mod tuples;
mod usizes;
mod vecs;

use crate::{LowCardinality, Normal, Small};
pub use ans::Ans;
#[cfg(feature = "stream")]
pub use arith::AsyncRangeDecoder;
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

    /// Finish decoding: `Ok(())` if the value just decoded can be trusted, or the
    /// reason it cannot. The decode-side twin of [`EntropyCoder::finish`].
    ///
    /// Consumes the decoder because the answer is not known until decoding has
    /// stopped. Three things can make a decode untrustworthy: an IO error latched
    /// mid-read, (for framed formats) a chunk frame that never fully arrived, and
    /// empty input, which no encoding can produce — even `()` costs a byte. All
    /// are reported here rather than from the decoding methods because coder
    /// decode is **infallible** — running past the data zero-pads rather than
    /// erroring — so nothing downstream is in a position to notice. An in-memory
    /// slice decoder over a complete buffer can only ever report the last of the
    /// three.
    ///
    /// [`Self::decode_value`] applies it, and is where the precedence rule lives.
    fn finish(self) -> std::io::Result<()>;

    /// Decode a whole value with strategy `S` and finish the decoder — **the**
    /// way to use one, and why [`Self::finish`] is rarely called by hand.
    ///
    /// Decoding and finishing are necessarily two steps: [`Encode::decode`]
    /// recurses on a borrowed `&mut Self`, while finishing consumes the decoder,
    /// because what it reports is not known until decoding has stopped. Pairing
    /// them here means no call site has to, and a caller who reached for the
    /// pieces separately could still forget the second.
    ///
    /// **`finish`'s error wins** even when the decode itself returned `Err` — note
    /// the `?` below runs first. The fabricated bits from a short read or a
    /// missing frame routinely trip some unrelated validation (a zero `NonZero`,
    /// a bad `char`) deeper in `Encode::decode`, and returning that downstream
    /// symptom would silently drop the root cause that produced it.
    ///
    /// For a type's default encoding use [`Normal`] as `S`.
    #[inline]
    fn decode_value<T: Encode<S>, S>(mut self) -> std::io::Result<T>
    where
        Self: Sized,
    {
        // Bound to a `let` so the borrow of `self` ends before it is consumed.
        let value = <T as Encode<S>>::decode(&mut self, &mut <T as Encode<S>>::Context::default());
        self.finish()?;
        value
    }

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

/// The async twin of [`EntropyDecoder`], for decoding a value as its bytes
/// arrive from an async source rather than from a buffer that is already whole.
///
/// Every read point can suspend, which is what lets the decode overlap the wait
/// for the next chunk instead of following it. That is the *only* difference:
/// the bits, symbols, and adaptation are identical to [`EntropyDecoder`]'s, and
/// both decoders read the same bytes.
///
/// Deliberately **not** a subtrait of [`EntropyDecoder`]: an async decoder
/// cannot supply the sync methods without blocking, and the sync decoders
/// cannot suspend. They are separate implementations of the same format.
///
/// The methods are written desugared, as `fn … -> impl Future`, rather than as
/// `async fn`. They mean the same thing, but `async fn` in a *public* trait
/// trips rustc's `async_fn_in_trait` lint, and this crate builds warning-free.
/// The lint's own suggestion is to desugar **and** add `+ Send`; we take only
/// the first half. A `Send` bound here would propagate to `T` — every decoded
/// value would have to be `Send`, forever, since it cannot be relaxed later
/// without a breaking change. Left off, auto traits still leak through the
/// opaque return type, so the future *is* `Send` whenever the decoder, the
/// contexts, and the value all are, which is what `tokio::spawn` needs.
pub trait AsyncEntropyDecoder {
    /// The sync decoder this one hands off to once no more input can arrive.
    ///
    /// No `where Self: 'a`: an implementor's sync decoder is *positioned* at the
    /// async decoder's cursor but does not borrow from it — `Range` hands it a
    /// local `Bytes`, `Ans` a borrowed frame buffer — and requiring the bound
    /// forces every implementor to prove `S: 'a` for a lifetime the associated
    /// type need not mention.
    type Sync<'a>: EntropyDecoder;

    /// Bytes that may be emitted but not yet accounted for by the information
    /// coded so far — the margin [`Self::sync_capacity`] holds back.
    const SETTLING_BYTES: usize;

    /// Bytes already buffered, decodable without awaiting.
    ///
    /// **Only meaningful through [`Self::sync_capacity`]**, which is the sole
    /// caller — it is an input to the byte-counted handoff test, not a report on
    /// the buffer. An implementor whose real safe-handoff condition is not a byte
    /// count is expected to return `0` here to opt out of that test and gate on
    /// [`Self::is_final`] instead, which is exactly what `Ans` does: nothing in
    /// one of its frames is decodable until all of it has arrived, so it has no
    /// meaningful partial count to report. Do not read this as "how much is
    /// buffered".
    fn ready_bytes(&self) -> usize;

    /// Whether no more input can arrive, so any amount may be consumed.
    fn is_final(&self) -> bool;

    /// How many items, each accounting for at most `info_bytes` of information,
    /// can certainly be decoded from what is already buffered.
    ///
    /// Pass a sum of [`MAX_BYTES`](Encode::MAX_BYTES) values and nothing else:
    /// this adds [`Self::SETTLING_BYTES`] itself, **once**, which is the only
    /// correct number of times. That is the whole reason callers are given a
    /// capacity rather than the raw margin — there is no way to forget it, and
    /// no way to add it twice.
    ///
    /// `usize::MAX` once no more input can arrive: past true end of stream the
    /// sync decoder zero-pads, which is what it should do there.
    #[inline]
    fn sync_capacity(&self, info_bytes: usize) -> usize {
        if self.is_final() {
            return usize::MAX;
        }
        self.ready_bytes().saturating_sub(Self::SETTLING_BYTES) / info_bytes.max(1)
    }

    /// Decode one whole value with the sync decoder if there is certainly room
    /// for it, else `None` and the caller must stay async.
    ///
    /// The safe default for a single value; [`Self::with_sync`] is for handing
    /// over several at once, which is faster when a caller can compute how many
    /// fit (see [`Self::sync_capacity`]).
    #[inline]
    fn sync_decode_if_there_is_room<T: Encode<S>, S>(
        &mut self,
        ctx: &mut T::Context,
    ) -> Option<Result<T, std::io::Error>> {
        // A comparison rather than [`Self::sync_capacity`]'s division: this is a
        // per-*value* gate, and `T::MAX_BYTES` is a constant, so it folds to one
        // compare against a constant. `sync_capacity` keeps the division for
        // batch loops, where it is amortised over the whole batch.
        if !self.is_final()
            && self.ready_bytes() < Self::SETTLING_BYTES.saturating_add(<T as Encode<S>>::MAX_BYTES)
        {
            return None;
        }
        Some(self.with_sync(|sync| <T as Encode<S>>::decode(sync, ctx)))
    }

    /// Decode with the sync decoder, positioned exactly here.
    ///
    /// The point is that an async decode need only stay async for as long as it
    /// is actually waiting on bytes. Frames already in flight cannot move — they
    /// live in the async call stack — but a sub-value that has *not started* can
    /// run entirely synchronously beneath them. Hand over as much at a time as
    /// possible: a batch of elements beats one at a time, since the sync decoder
    /// then keeps its state register-resident across all of them.
    ///
    /// **Only call within the budget [`Self::sync_capacity`] reports** — it must
    /// cover everything `f` decodes. [`Self::sync_decode_if_there_is_room`] is
    /// the safe single-value form that applies that check for you.
    fn with_sync<R>(&mut self, f: impl FnOnce(&mut Self::Sync<'_>) -> R) -> R;

    /// Finish decoding, reporting why the value cannot be trusted if it cannot;
    /// the async twin of [`EntropyDecoder::finish`], covering the same three
    /// failure modes plus an error latched by the chunk source itself.
    fn finish(self) -> std::io::Result<()>;

    /// Decode a whole value with strategy `S` and finish the decoder; the async
    /// twin of [`EntropyDecoder::decode_value`], pairing the two steps and
    /// applying the same precedence rule for the same reason.
    #[inline]
    fn decode_value<T: Encode<S>, S>(
        mut self,
    ) -> impl std::future::Future<Output = std::io::Result<T>>
    where
        Self: Sized,
    {
        async move {
            // Bound to a `let` so the borrow of `self` ends before it is consumed.
            let value = <T as Encode<S>>::decode_async(
                &mut self,
                &mut <T as Encode<S>>::Context::default(),
            )
            .await;
            self.finish()?;
            value
        }
    }

    /// Decode `N` bits, each with its own context. The async twin of
    /// [`EntropyDecoder::decode_bits`], and likewise the core required
    /// primitive: infallible, because running past the encoded data yields
    /// arbitrary bits that higher-level `decode_async` impls validate.
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [bit_context::BitContext; N],
    ) -> impl std::future::Future<Output = [bool; N]>;

    /// The `N == 1` case of [`Self::decode_bits`].
    #[inline(always)]
    fn decode_bit(
        &mut self,
        context: &mut bit_context::BitContext,
    ) -> impl std::future::Future<Output = bool> {
        async {
            let [bit] = self.decode_bits(std::array::from_mut(context)).await;
            bit
        }
    }

    /// Decode one whole [`AtMost<MAX>`](AtMost); the async twin of
    /// [`EntropyDecoder::decode_atmost`].
    ///
    /// **This default always walks bitwise, and that must match what the encoder
    /// did.** The two walks are not interchangeable: a symbol step narrows the
    /// coder over the whole `MAX`-slot interval, while bit steps narrow it one
    /// `Probability` split per level, so they consume
    /// *different bytes* for the same value and a mismatch desyncs the coder
    /// silently. Both shipped implementations override this to follow
    /// [`Walk::production`](Walk::production), exactly as their sync
    /// counterparts do; a coder whose encoder ever symbol-codes an `AtMost`
    /// must override it too rather than inherit this.
    #[inline]
    fn decode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut atmost::AtMostContext<MAX>,
    ) -> impl std::future::Future<Output = AtMost<MAX>>
    where
        Self: Sized,
    {
        async { AtMost::new(atmost::walks::decode_bitwise_async(self, &mut ctx.bits).await) }
    }

    /// Decode a fixed number of incompressible bytes; the async twin of
    /// [`EntropyDecoder::decode_incompressible_bytes`].
    fn decode_incompressible_bytes(
        &mut self,
        bytes: &mut [u8],
    ) -> impl std::future::Future<Output = Result<(), std::io::Error>>;
}

/// Information one whole-symbol step can account for, in bytes.
///
/// A [`SymbolRange`](model::SymbolRange) slot is at least 1 of `M = 2^16`, so a
/// symbol costs at most 16 bits — two bytes — plus one byte of margin for the
/// interval `clamp_for_symbol` discards, which is a few bits per step but is not
/// tightly derived. Margin is nearly free here (it widens a `u64`'s bound from
/// 18 to 20) and covers the one part of the derivation that is not airtight.
pub(crate) const MAX_INFO_BYTES_PER_SYMBOL: usize = 3;

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

    /// The most **information** one value coded with this strategy can account
    /// for, in bytes, or [`usize::MAX`] when there is no bound (anything
    /// length-driven, like a collection or a `String`).
    ///
    /// Deliberately *excludes* the coder's settling margin — bytes emitted but
    /// not yet accounted for by the information coded so far. That margin is
    /// bounded per *span*, not per value, so adding it here would count it once
    /// per value and make sums badly wrong: seven coded bits cost seven bytes
    /// plus one margin, not seven margins. The decoder adds its own
    /// `SETTLING_BYTES` exactly once, in
    /// [`sync_capacity`](AsyncEntropyDecoder::sync_capacity); callers never add
    /// it themselves.
    ///
    /// So this composes cleanly: **sum** over the parts a value codes in
    /// sequence, **max** over branches it might take. Build it from
    /// `<bool as Encode>::MAX_BYTES` (one adaptive bit),
    /// [`MAX_INFO_BYTES_PER_SYMBOL`], and one byte per incompressible byte —
    /// derived, not measured. A loose bound only costs batching headroom; a
    /// **wrong** one decodes past the end of the buffer and returns plausible
    /// garbage, so prefer margin. `v2::max_bytes` property-tests every bound
    /// against real decodes.
    ///
    /// # Correctness
    ///
    /// **An understated bound is silent data corruption, not a panic.** It lets
    /// the sync handoff read past what has actually arrived, where the coder
    /// zero-pads and yields a plausible wrong value — no `Err`, no crash. The
    /// backstop in
    /// [`with_sync`](AsyncEntropyDecoder::with_sync) is a `debug_assert!`, so it
    /// is **compiled out in release**, and `v2::max_bytes` can only cover the
    /// types listed in it. This is a public trait you are invited to implement,
    /// so if you write one: derive the bound from the coding schedule rather
    /// than measuring a sample, round *up* when unsure, and add a case to
    /// `v2::max_bytes` — a sample tells you what one value happened to cost, not
    /// what the worst one can.
    ///
    /// Required, with no default to fall through: an omitted bound must not
    /// silently become "unbounded".
    const MAX_BYTES: usize;

    /// Decode a value with this strategy, from a source that may run dry
    /// part-way through; the async twin of [`Self::decode`].
    ///
    /// **Implement this, but call [`Self::decode_async`]**, which wraps it with
    /// the fast path below. Nothing stops a caller reaching for this one
    /// directly; it would simply be slower.
    fn decode_awaiting<D: AsyncEntropyDecoder>(
        decoder: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<Self, std::io::Error>>;

    /// Decode a value with this strategy — the method callers should use.
    ///
    /// Hands the whole value to the *sync* decoder whenever [`Self::MAX_BYTES`]
    /// of it is certainly buffered already, and only falls back to
    /// [`Self::decode_awaiting`] when it might have to wait. Being the default
    /// rather than something each call site opens by hand is the point: a site
    /// that forgot would still be *correct*, just permanently slow, so no test
    /// would catch the omission.
    #[inline]
    fn decode_async<D: AsyncEntropyDecoder>(
        decoder: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<Self, std::io::Error>> {
        async {
            // Bound to a `let` so the borrows of `decoder` and `ctx` end before
            // the fallback needs them again.
            let attempt = decoder.sync_decode_if_there_is_room::<Self, S>(ctx);
            match attempt {
                Some(result) => result,
                None => Self::decode_awaiting(decoder, ctx).await,
            }
        }
    }
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
/// use compactly::v2::{AsyncEntropyDecoder, Encode, EntropyCoder, EntropyDecoder, Strategy};
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
///
///     const MAX_BYTES: usize = <u8 as Encode>::MAX_BYTES;
///     async fn decode_awaiting<D: AsyncEntropyDecoder>(r: &mut D, ctx: &mut Self::Context)
///         -> Result<u8, std::io::Error> {
///         <u8 as Encode>::decode_async(r, ctx).await
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

    /// Decode a value using this strategy, from a source that may run dry
    /// part-way through. The async twin of [`Self::decode`].
    #[inline]
    fn decode_async<T: Encode<Self>, D: AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut <T as Encode<Self>>::Context,
    ) -> impl std::future::Future<Output = Result<T, std::io::Error>> {
        <T as Encode<Self>>::decode_async(reader, ctx)
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
    arith::Decoder::new(bytes).decode_value::<T, Normal>().ok()
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

/// Decode a value from an async stream of [`Bytes`](::bytes::Bytes) chunks,
/// decoding each chunk as it arrives rather than waiting for the whole input.
///
/// Accepts the same bytes [`encode`]/[`encode_to`] produce. Uses the default
/// [`Range`] coder; [`Range::decode_stream`] selects explicitly.
///
/// The input bound is what the ecosystem already speaks — `aws_sdk_s3`'s
/// `ByteStream`, `object_store`'s `GetResult::into_stream()`, and `axum`'s
/// `Body::into_data_stream()` all match it directly.
#[cfg(feature = "stream")]
pub async fn decode_stream<T, S, E>(stream: S) -> std::io::Result<T>
where
    T: Encode,
    S: futures_core::Stream<Item = Result<::bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    Range::decode_stream::<T, _, _>(stream).await
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

    /// Exactly the wrapped strategy's.
    const MAX_BYTES: usize = <T as Encode<S>>::MAX_BYTES;

    #[inline]
    async fn decode_awaiting<D: AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            value: <T as Encode<S>>::decode_async(reader, ctx).await?,
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
