use super::atmost::{walks, AtMost, AtMostContext};
use super::model::{Probability, SymbolCoder, SymbolDecoder, SymbolRange, SHIFT};
use super::{EntropyCoder, EntropyDecoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithState {
    lo: u64,
    hi: u64,
}

impl Default for ArithState {
    #[inline]
    fn default() -> Self {
        ArithState {
            lo: 0,
            hi: u64::MAX,
        }
    }
}

impl ArithState {
    #[inline]
    fn ready_bytes(&mut self) -> Bytes {
        let mut bytes = Bytes::default();
        if self.lo == self.hi {
            for b in self.lo.to_be_bytes() {
                bytes.push(b);
            }
            self.lo = 0;
            self.hi = u64::MAX;
        } else {
            for _ in 0..8 {
                let lo_byte = (self.lo >> 56) as u8;
                let hi_byte = (self.hi >> 56) as u8;
                // #[cfg(test)]
                // {
                //     let width = self.hi - self.lo;
                //     println!("width = {width:016x}");
                //     println!("  min = {:016x}", u64::MAX >> 8);
                //     println!("lo_byte {lo_byte:02x}");
                //     println!("hi_byte {hi_byte:02x}");
                // }
                if lo_byte == hi_byte {
                    self.lo <<= 8;
                    self.hi <<= 8;
                    // #[cfg(test)]
                    // {
                    //     println!("next_byte resetting to {self:x?}");
                    // }
                    bytes.push(lo_byte);
                } else {
                    return bytes;
                }
            }
        }
        bytes
    }

    /// The single byte that finalizes the stream: the top byte of the interval,
    /// which the decoder pulls to disambiguate the last coded value.
    #[inline]
    pub fn last_byte(self) -> u8 {
        (self.hi >> 56) as u8
    }

    /// Returns a set of bytes to be written out.
    #[must_use]
    #[inline]
    pub fn encode(&mut self, prob: Probability, value: bool) -> Bytes {
        if self.hi == self.lo + 1 {
            // special case that we need to handle differently.
            let bytes = if value {
                self.hi.to_be_bytes()
            } else {
                self.lo.to_be_bytes()
            };
            self.lo = 0;
            self.hi = u64::MAX;
            return Bytes { bytes, count: 8 };
        }
        let split = self.split(prob);
        debug_assert!(split < self.hi, "{self:x?} {prob:?}");
        debug_assert!(split >= self.lo);
        debug_assert!(self.hi > self.lo);
        if value {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        self.ready_bytes()
        // println!("encoding {prob} {shift} {value:?}   with split {split:016x} gives {self:x?}");
    }

    /// Returns bit and the number of bytes that need to be read.
    #[inline]
    pub fn decode(&mut self, prob: Probability, value: u64) -> (bool, usize) {
        if self.hi == self.lo + 1 {
            let bit = value == self.hi;
            self.hi = u64::MAX;
            self.lo = 0;
            return (bit, 8);
        }
        let split = self.split(prob);
        let b = value > split;
        // Branchless: compute both lo/hi updates and select via CMOV.
        self.lo = if b { split + 1 } else { self.lo };
        self.hi = if b { self.hi } else { split };
        (b, self.consume_decoded_bytes())
    }

    /// Normalize state after decode and return number of compressed bytes consumed.
    /// Uses leading_zeros to avoid a branch-heavy loop, eliminating ~12.5% mispredictions.
    #[inline]
    fn consume_decoded_bytes(&mut self) -> usize {
        let diff = self.lo ^ self.hi;
        if diff == 0 {
            self.lo = 0;
            self.hi = u64::MAX;
            return 8;
        }
        let n = (diff.leading_zeros() / 8) as usize;
        self.lo <<= n * 8;
        self.hi <<= n * 8;
        n
    }

    #[inline]
    fn split(self, Probability { prob }: Probability) -> u64 {
        // debug_assert!(prob < 1 << SHIFT);
        debug_assert!(self.hi > self.lo);
        let width = self.hi - self.lo;
        debug_assert!(self.lo >> 56 != self.hi >> 56);
        self.lo + (width >> SHIFT) * prob.get() as u64
    }

    /// Minimum interval width required before coding a whole tree symbol in
    /// one step. Guarantees every one of the `M` slots spans at least `M`
    /// values, so slot boundaries are exact and the top-slot rounding waste is
    /// at most a `1/M` fraction of the interval. The per-bit path tolerates
    /// arbitrarily narrow intervals, so this is only enforced (via
    /// [`ArithState::clamp_for_symbol`]) on the symbol path. (Must stay below
    /// `2^56` for `clamp_for_symbol`'s single-boundary argument to hold.)
    const MIN_SYMBOL_WIDTH: u64 = (SymbolRange::M as u64) * (SymbolRange::M as u64);

    /// Carry-less clamp renormalization (Subbotin-style): if the interval is
    /// too narrow for a symbol step, it must straddle exactly one top-byte
    /// boundary (byte-wise renormalization would otherwise have shifted it
    /// out). Discard the smaller side of that boundary so renormalization can
    /// proceed; the encoder simply never codes into the discarded part, at a
    /// cost of at most one bit per (rare) clamp. The choice depends only on
    /// `(lo, hi)`, which encoder and decoder share, so they always agree.
    ///
    /// Returns whether it clamped; the caller must then renormalize
    /// (`ready_bytes` / `consume_decoded_bytes`) and call this again.
    #[inline]
    fn clamp_for_symbol(&mut self) -> bool {
        if self.hi - self.lo >= Self::MIN_SYMBOL_WIDTH {
            return false;
        }
        // width < 2^56 with unequal top bytes ⟹ exactly one multiple of 2^56
        // lies in (lo, hi]: hi rounded down to its top byte.
        let boundary = self.hi & (0xFF << 56);
        debug_assert!(self.lo < boundary && boundary <= self.hi);
        if boundary - self.lo > self.hi - boundary + 1 {
            self.hi = boundary - 1;
        } else {
            self.lo = boundary;
        }
        true
    }

    /// Narrow the interval to `range`'s slots. Requires
    /// `width >= MIN_SYMBOL_WIDTH` (see [`ArithState::clamp_for_symbol`]).
    /// The top slot absorbs the sub-slot rounding remainder, mirroring how the
    /// per-bit `encode` gives the true branch everything above `split`.
    #[inline]
    fn narrow_symbol(&mut self, range: SymbolRange) {
        let step = (self.hi - self.lo) >> SymbolRange::BITS;
        let end = range.start() + range.width();
        self.hi = if end == SymbolRange::M {
            self.hi
        } else {
            self.lo + step * end as u64 - 1
        };
        self.lo += step * range.start() as u64;
        debug_assert!(self.hi > self.lo);
    }

    /// Which slot the decoder's window `value` falls in. Values in the
    /// top-slot remainder (and garbage past the end of the stream) clamp to
    /// the top slot.
    #[inline]
    fn symbol_slot(&self, value: u64) -> u32 {
        let step = (self.hi - self.lo) >> SymbolRange::BITS;
        (value.wrapping_sub(self.lo) / step).min((SymbolRange::M - 1) as u64) as u32
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Bytes {
    bytes: [u8; 8],
    count: usize,
}

impl Bytes {
    #[inline]
    fn push(&mut self, byte: u8) {
        self.bytes[self.count] = byte;
        self.count += 1;
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.bytes[..self.count]
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = std::iter::Take<std::array::IntoIter<u8, 8>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().take(self.count)
    }
}

/// Delay-interleave constant: the decoder's window `value` is a `u64` filled
/// from the first 8 stream bytes and pulled one byte per renorm in lockstep
/// with the encoder, so the decoder's cumulative pull-count stays exactly this
/// many bytes ahead of the encoder's emit-count. An incompressible run is
/// therefore spliced into the stream `W_DELAY` entropy bytes after the point it
/// was produced, so it lands exactly at the decoder's read cursor when the
/// decode logic reaches that field. See plans/streaming-io-api.md.
const W_DELAY: usize = 8;

/// The v2 range coder: arithmetic/range coding, with incompressible bytes
/// delay-interleaved into a single flat stream. It is the coder behind
/// [`encode`](super::encode)/[`decode`](super::decode) and
/// [`encode_to`](super::encode_to)/[`decode_from`](super::decode_from), which
/// produce identical bytes whether the value is buffered whole or streamed.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Range::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 4);
/// assert_eq!(compactly::v2::Range::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Default, Debug)]
pub struct Range(RangeEncoder<Vec<u8>>);

impl EntropyCoder for Range {
    type Writer = Vec<u8>;
    #[inline]
    fn new(writer: Vec<u8>) -> Self {
        Range(RangeEncoder::new(writer))
    }
    #[inline]
    fn finish(self) -> std::io::Result<Vec<u8>> {
        self.0.finish()
    }
    #[inline]
    fn encode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        bits: [bool; N],
    ) {
        self.0.encode_bits(contexts, bits)
    }

    #[inline]
    fn encode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut AtMostContext<MAX>,
        value: AtMost<MAX>,
    ) {
        self.0.encode_atmost(ctx, value)
    }

    #[inline]
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        self.0.encode_incompressible_bytes(bytes)
    }
}

impl SymbolCoder for Range {
    #[inline]
    fn encode_symbol(&mut self, range: SymbolRange) {
        self.0.encode_symbol(range)
    }
}

impl Range {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        let mut reader = Decoder::new(bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
    /// Whether `Range`'s decoder asks [`Walk::production`](super::Walk::production)
    /// to speculate on a non-power-of-two value count (see
    /// [`SymbolDecoder::SPECULATES`]). Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    #[cfg(feature = "benchmarking")]
    pub const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;
    /// Encode `values` using an explicitly forced tree walk, bypassing
    /// [`Walk::production`](super::Walk::production)'s usual choice for
    /// `MAX`. `WHICH_WALK` indexes [`WALKS`](super::WALKS). Benchmark support
    /// for `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    #[cfg(any(test, feature = "benchmarking"))]
    pub fn encode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        values: &[super::AtMost<MAX>],
    ) -> Vec<u8> {
        walks::encode_atmost_batch::<Self, MAX, WHICH_WALK>(Self::default(), values).into_vec()
    }
    /// The decode side of [`Self::encode_atmost_batch`]: decode `n` values
    /// with the same forced walk. Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    #[cfg(any(test, feature = "benchmarking"))]
    pub fn decode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        bytes: &[u8],
        n: usize,
    ) -> Vec<super::AtMost<MAX>> {
        walks::decode_atmost_batch::<Decoder, MAX, WHICH_WALK>(Decoder::new(bytes), n)
    }
    /// Finish encoding and return the bytes.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.0.finish().expect("writing to a Vec<u8> is infallible")
    }
    /// Encode `value` straight into a [`Write`](std::io::Write), streaming settled
    /// bytes out as they emerge rather than buffering the whole compressed output.
    /// The bytes are **identical** to [`Range::encode`].
    ///
    /// No buffering is applied, and `Range` writes **one byte per `write` call**
    /// (settled bytes as they emerge), so wrap an unbuffered sink like a `File` in
    /// a [`BufWriter`](std::io::BufWriter) yourself or performance will suffer. The
    /// returned writer is flushed before return, so a final flush error surfaces
    /// here rather than being lost in a wrapping `BufWriter`'s `Drop`.
    pub fn encode_to<T: super::Encode, W: std::io::Write>(
        value: &T,
        writer: W,
    ) -> std::io::Result<()> {
        super::stream_encode::<T, RangeEncoder<W>>(value, writer)
            .and_then(|mut w| std::io::Write::flush(&mut w))
    }
    /// Decode a value straight from a [`Read`](std::io::Read), pulling bytes on
    /// demand rather than requiring the whole compressed input in memory. Accepts
    /// the same bytes [`Range::encode`]/[`Range::encode_to`] produce.
    ///
    /// No buffering is applied, and `Range` issues **one `read` call per entropy
    /// byte** for the whole stream, so wrap an unbuffered source like a `File` in a
    /// [`BufReader`](std::io::BufReader) yourself or performance will suffer.
    pub fn decode_from<T: super::Encode, R: std::io::Read>(reader: R) -> std::io::Result<T> {
        super::stream_decode::<T, _>(RangeDecoder::new(reader))
    }

    /// Decode a value from an async stream of [`Bytes`](bytes::Bytes) chunks,
    /// decoding each chunk as it arrives rather than waiting for the whole
    /// input — so the decode overlaps the wait for the next chunk instead of
    /// following it. Accepts the same bytes [`Range::encode`] produces.
    ///
    /// A stream that delivers everything in one chunk is decoded by the sync
    /// slice [`Decoder`] instead: there is nothing to overlap in that case, so
    /// the async decoder would be pure cost. Which path a value takes is
    /// unobservable in the result — both read the same format, and the async
    /// decoder is an alternative implementation of it rather than a variant.
    #[cfg(feature = "stream")]
    pub async fn decode_stream<T, S, E>(stream: S) -> std::io::Result<T>
    where
        T: super::DecodeAsync,
        S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut source = super::stream::ChunkSource::new(stream).await;
        if let Some(whole) = source.take_if_single_chunk().await {
            let value = super::stream_decode::<T, _>(Decoder::new(&whole));
            return match source.take_error() {
                Some(e) => Err(e),
                None => value,
            };
        }
        super::stream_decode_async::<T, _>(AsyncRangeDecoder::from_source(source).await).await
    }
}
impl From<Range> for Vec<u8> {
    fn from(value: Range) -> Self {
        value.into_vec()
    }
}

/// Streaming range encoder: the same coding and delay-interleave splice as
/// [`Range`], but settled bytes are written to `W` as they emerge, so peak
/// memory is bounded rather than the whole `Vec`. Produces **byte-identical**
/// output to [`Range`] for the same value (enforced by
/// `streaming_matches_in_memory`). IO errors are latched and surfaced by
/// [`RangeEncoder::finish`], keeping the infallible [`EntropyCoder`] hot path
/// branch-free.
///
/// [`Range`] is the in-memory case, `RangeEncoder<Vec<u8>>`; the two share this
/// single implementation.
#[derive(Default)]
pub(crate) struct RangeEncoder<W: std::io::Write> {
    writer: W,
    state: ArithState,
    /// Count of entropy bytes written (excludes spliced runs) — schedules the
    /// `W_DELAY` splice.
    entropy_written: usize,
    /// Ring of not-yet-spliced incompressible runs.
    withheld: [Vec<u8>; W_DELAY],
    error: Option<std::io::Error>,
}

/// Summarizes rather than dumping the sink. A derived `Debug` would format the
/// whole accumulated output — megabytes on an in-progress encode, from an
/// incidental `dbg!()` — and would need `W: Debug`, which a sink need not be.
/// Progress is reported as `entropy_written`, which is the useful number anyway.
impl<W: std::io::Write> std::fmt::Debug for RangeEncoder<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeEncoder")
            .field("state", &self.state)
            .field("entropy_written", &self.entropy_written)
            .field(
                "withheld_bytes",
                &self.withheld.iter().map(Vec::len).sum::<usize>(),
            )
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<W: std::io::Write> RangeEncoder<W> {
    #[inline]
    fn write_out(&mut self, bytes: &[u8]) {
        if self.error.is_none() {
            if let Err(e) = self.writer.write_all(bytes) {
                self.error = Some(e);
            }
        }
    }

    /// Write settled entropy bytes, splicing each withheld incompressible run
    /// in as its target byte is written (the `W_DELAY` delay-interleave).
    #[inline]
    fn push_entropy(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.write_out(&[b]);
            self.entropy_written += 1;
            let slot = self.entropy_written % W_DELAY;
            // Splice this slot's withheld run (a no-op write when empty) and
            // clear it for reuse. `write_all`
            // fast-returns on an empty slice, so no guard is needed; the write
            // is inlined (rather than `write_out`) to keep the `writer`,
            // `error`, and `withheld` borrows disjoint.
            if self.error.is_none() {
                if let Err(e) = self.writer.write_all(&self.withheld[slot]) {
                    self.error = Some(e);
                }
            }
            self.withheld[slot].clear();
        }
    }
}

impl<W: std::io::Write> EntropyCoder for RangeEncoder<W> {
    type Writer = W;

    fn new(writer: W) -> Self {
        Self {
            writer,
            state: ArithState::default(),
            entropy_written: 0,
            withheld: Default::default(),
            error: None,
        }
    }

    /// Finish encoding: append the coder's final byte and flush any tail runs,
    /// then return the sink or the latched IO error. [`Range::into_vec`] is the
    /// in-memory caller.
    fn finish(mut self) -> std::io::Result<W> {
        let last = self.state.last_byte();
        self.push_entropy(&[last]);
        let mut remaining = self.withheld.iter().filter(|r| !r.is_empty()).count();
        while remaining > 0 {
            self.write_out(&[0]);
            self.entropy_written += 1;
            let slot = self.entropy_written % W_DELAY;
            if !self.withheld[slot].is_empty() {
                let run = std::mem::take(&mut self.withheld[slot]);
                self.write_out(&run);
                remaining -= 1;
            }
        }
        match self.error.take() {
            Some(e) => Err(e),
            None => Ok(self.writer),
        }
    }

    #[inline]
    fn encode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        bits: [bool; N],
    ) {
        for (value, ctx) in bits.into_iter().zip(contexts.iter_mut()) {
            let ready = self.state.encode(ctx.probability(), value);
            self.push_entropy(&ready);
            *ctx = ctx.adapt(value);
        }
    }

    #[inline]
    fn encode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut AtMostContext<MAX>,
        value: AtMost<MAX>,
    ) {
        walks::encode_symbol_or_bitwise(self, ctx, value)
    }

    #[inline]
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        let slot = self.entropy_written % W_DELAY;
        self.withheld[slot].extend_from_slice(bytes);
    }
}

impl<W: std::io::Write> SymbolCoder for RangeEncoder<W> {
    #[inline]
    fn encode_symbol(&mut self, range: SymbolRange) {
        while self.state.clamp_for_symbol() {
            let ready = self.state.ready_bytes();
            self.push_entropy(&ready);
        }
        self.state.narrow_symbol(range);
        let ready = self.state.ready_bytes();
        self.push_entropy(&ready);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder<'a> {
    /// The single flat delay-interleave stream: entropy bytes with each
    /// incompressible run spliced in. Entropy `pull`s and
    /// `decode_incompressible_bytes` both advance this one cursor; the W_DELAY
    /// splice guarantees a run sits exactly at the cursor when its field is
    /// reached.
    bytes: &'a [u8],
    state: ArithState,
    value: u64,
}

/// One range-decode bit step, operating on locals so the caller can keep `state`,
/// the decode window `value`, and the input cursor `bytes` register-resident
/// across a whole batch instead of round-tripping them through the `Decoder`.
#[inline(always)]
fn decode_step(
    state: &mut ArithState,
    value: &mut u64,
    bytes: &mut &[u8],
    probability: Probability,
) -> bool {
    let (out, sz) = state.decode(probability, *value);
    for _ in 0..sz {
        let byte = if let Some((&b, r)) = bytes.split_first() {
            *bytes = r;
            b
        } else {
            0
        };
        *value = (*value << 8) + byte as u64;
    }
    out
}

impl<'a> SymbolDecoder for Decoder<'a> {
    /// Unlike `Ans`, `Range` asks for the speculating walk — its u64-division
    /// symbol step provides the latency shadow that absorbs the speculation's
    /// extra instructions (measured −4…−17% at value counts ≥ 4); see the
    /// walk inventory in `atmost::walks`.
    const SPECULATES: bool = true;

    /// Whole-symbol decode step, the inverse of `Range::encode_symbol`:
    /// recover the slot with one division, let `walk` recover the value and
    /// interval (adapting its contexts), then do a single narrowing +
    /// renormalization. State is kept in locals across the whole symbol
    /// (register-resident), as in `decode_bits`.
    #[inline]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        let mut state = self.state;
        let mut value = self.value;
        let mut bytes = self.bytes;
        let pull = |state: &mut ArithState, value: &mut u64, bytes: &mut &[u8]| {
            let n = state.consume_decoded_bytes();
            for _ in 0..n {
                let byte = if let Some((&b, r)) = bytes.split_first() {
                    *bytes = r;
                    b
                } else {
                    0
                };
                *value = (*value << 8) + byte as u64;
            }
        };
        while state.clamp_for_symbol() {
            pull(&mut state, &mut value, &mut bytes);
        }
        let slot = state.symbol_slot(value);
        let (range, decoded) = walk(slot);
        state.narrow_symbol(range);
        pull(&mut state, &mut value, &mut bytes);
        self.state = state;
        self.value = value;
        self.bytes = bytes;
        decoded
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    type Reader = &'a [u8];

    fn new(bytes: &'a [u8]) -> Self {
        let (value, bytes) = if let Some((&first, rest)) = bytes.split_first_chunk() {
            (u64::from_be_bytes(first), rest)
        } else {
            let mut b = [0; 8];
            b[..bytes.len()].copy_from_slice(bytes);
            (u64::from_be_bytes(b), [].as_slice())
        };
        Self {
            bytes,
            state: ArithState::default(),
            value,
        }
    }

    /// The slice decoder never latches an IO error, so the decode result stands.
    #[inline]
    fn into_result<T>(self, value: Result<T, std::io::Error>) -> std::io::Result<T> {
        value
    }

    /// Whole `AtMost` symbol decode; see [`SymbolDecoder::decode_symbol_step`].
    #[inline]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    /// Adaptive batch decode, fused into a single pass (mirrors the `Ans`
    /// override). We keep `state`/`value`/`bytes` in locals and do lookup → decode
    /// → adapt in one pass, touching each independent context once, rather than
    /// re-reading the decoder fields every bit.
    #[inline]
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        let mut state = self.state;
        let mut value = self.value;
        let mut bytes = self.bytes;
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut value, &mut bytes, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        self.state = state;
        self.value = value;
        self.bytes = bytes;
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        // By the W_DELAY splice, the run sits at the cursor right now: the
        // entropy `pull`s that fill the window read exactly W_DELAY bytes ahead,
        // so the cursor lands on this run's first byte. Read it straight off.
        if self.bytes.len() < bytes.len() {
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                self.bytes.len(),
                bytes.len()
            )));
        }
        let (b, rest) = self.bytes.split_at(bytes.len());
        bytes.copy_from_slice(b);
        self.bytes = rest;
        Ok(())
    }
}

/// Read one byte from `reader`; return 0 at a clean EOF (matching the slice
/// [`Decoder`]'s zero-padding past the end of its byte slice), retry on
/// `Interrupted`, and latch any other error into `error` (returning 0 so
/// decoding can run to a validated stop rather than panicking mid-stream).
///
/// Once `error` is latched we never touch `reader` again — every later byte is a
/// fabricated 0. This keeps a transiently-failing reader (one that errors once
/// then recovers) from splicing genuine post-error bytes into the fabricated
/// zeros, which would desynchronize the arithmetic coder and could decode a
/// bogus length into a downstream `Vec::with_capacity`.
#[inline]
fn read_one_byte<R: std::io::Read>(reader: &mut R, error: &mut Option<std::io::Error>) -> u8 {
    if error.is_some() {
        return 0;
    }
    let mut buf = [0u8; 1];
    loop {
        // A 1-byte buffer means `Ok(0)` is EOF (the `Read` contract) and any
        // other success is exactly one byte; we never lose a partial read.
        match reader.read(&mut buf) {
            Ok(0) => return 0,
            Ok(_) => return buf[0],
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => {
                *error = Some(e);
                return 0;
            }
        }
    }
}

/// Streaming range decoder: pulls bytes from `R` on demand rather than indexing
/// a slice, so decoding a large value need not hold the whole compressed input
/// in memory. Reads the same bytes [`Range`]/[`RangeEncoder`] produce and
/// recovers identical values (the decode arithmetic is the slice [`Decoder`]'s;
/// only the byte source differs). IO errors are latched and surfaced by
/// [`RangeDecoder::into_result`]; a clean EOF yields zero bytes, which the
/// higher-level `Encode::decode` validation catches.
pub(crate) struct RangeDecoder<R: std::io::Read> {
    reader: R,
    state: ArithState,
    value: u64,
    error: Option<std::io::Error>,
}

impl<R: std::io::Read> RangeDecoder<R> {
    /// Pull `n` entropy bytes into the window.
    #[inline]
    fn refill(&mut self, n: usize) {
        for _ in 0..n {
            let byte = read_one_byte(&mut self.reader, &mut self.error);
            self.value = (self.value << 8) + byte as u64;
        }
    }
}

impl<R: std::io::Read> SymbolDecoder for RangeDecoder<R> {
    const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;

    #[inline]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        while self.state.clamp_for_symbol() {
            let n = self.state.consume_decoded_bytes();
            self.refill(n);
        }
        let slot = self.state.symbol_slot(self.value);
        let (range, decoded) = walk(slot);
        self.state.narrow_symbol(range);
        let n = self.state.consume_decoded_bytes();
        self.refill(n);
        decoded
    }
}

impl<R: std::io::Read> EntropyDecoder for RangeDecoder<R> {
    type Reader = R;

    fn new(mut reader: R) -> Self {
        // Fill the 8-byte window, matching `Decoder::new`'s initial `u64`.
        let mut error = None;
        let mut value = 0u64;
        for _ in 0..8 {
            value = (value << 8) + read_one_byte(&mut reader, &mut error) as u64;
        }
        Self {
            reader,
            state: ArithState::default(),
            value,
            error,
        }
    }

    /// Return `value` unless a read error was latched during decoding — the
    /// latched IO error wins even when `value` is itself `Err` (see the trait
    /// method's contract).
    fn into_result<T>(mut self, value: Result<T, std::io::Error>) -> std::io::Result<T> {
        match self.error.take() {
            Some(e) => Err(e),
            None => value,
        }
    }

    #[inline]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    #[inline]
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let (out, sz) = self.state.decode(context.probability(), self.value);
            self.refill(sz);
            *context = context.adapt(out);
            *b = out;
        }
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(&mut self, out: &mut [u8]) -> Result<(), std::io::Error> {
        // If an IO error was already latched (e.g. mid entropy stream), surface
        // that first, most-informative error here and abort — rather than
        // reading fabricated zeros as data, which for a corrupt length could
        // otherwise drive an unbounded read loop. Returning the latched error
        // keeps it the one `decode_from` reports (R6).
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        // By the W_DELAY splice the run sits at the reader cursor, so read it
        // straight off; `read_exact` errors on a truncated stream, stopping the
        // decode cleanly instead of returning silently-short data.
        self.reader.read_exact(out)
    }
}

/// Range decoder over an async source: the arithmetic is the sync decoders',
/// but every byte comes from a [`ChunkSource`], so running out of buffered
/// chunk suspends rather than blocks.
///
/// Reads the same bytes [`Range`]/[`RangeEncoder`] produce and recovers
/// identical values, including at the edges — a stream error is latched and
/// surfaced by [`into_result`](super::AsyncEntropyDecoder::into_result), and a
/// clean end of stream yields zero bytes, exactly as [`RangeDecoder`] does.
#[cfg(feature = "stream")]
pub struct AsyncRangeDecoder<S> {
    source: super::stream::ChunkSource<S>,
    state: ArithState,
    value: u64,
}

#[cfg(feature = "stream")]
impl<S> std::fmt::Debug for AsyncRangeDecoder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncRangeDecoder")
            .field("source", &self.source)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "stream")]
impl<S, E> AsyncRangeDecoder<S>
where
    S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    /// Build a decoder over `stream`, filling the 8-byte window to match
    /// [`Decoder::new`]'s and [`RangeDecoder::new`]'s initial `u64`.
    pub async fn new(stream: S) -> Self {
        Self::from_source(super::stream::ChunkSource::new(stream).await).await
    }

    /// [`Self::new`] over a source that has already been read from — used by
    /// `decode_stream`, whose single-chunk look-ahead has consumed a poll or two
    /// before deciding to decode asynchronously after all.
    pub(crate) async fn from_source(mut source: super::stream::ChunkSource<S>) -> Self {
        let mut value = 0u64;
        for _ in 0..W_DELAY {
            value = (value << 8) + source.next_byte().await as u64;
        }
        Self {
            source,
            state: ArithState::default(),
            value,
        }
    }

    /// Run `f` against the **sync** slice [`Decoder`], positioned exactly here,
    /// and adopt whatever position it reaches. `None` when the input is still
    /// arriving, in which case the caller must stay on the async path.
    ///
    /// This is what lets a multi-chunk decode stop paying for async once the
    /// last chunk is in hand. The frames already in flight stay async — their
    /// state lives in the async call stack and cannot be moved — but any
    /// sub-value that has *not started yet* can be decoded synchronously, with
    /// those frames acting as a shell. The deeper the handoff, the more of the
    /// remaining work runs at sync speed; handing over a whole loop tail rather
    /// than one element at a time is strictly better, since the sync decoder
    /// then keeps `state`/`value`/`bytes` register-resident across all of it.
    ///
    /// Only the three words of coder state need translating: `Decoder` holds
    /// the same `ArithState` and window `value`, and a `&[u8]` cursor where
    /// this holds a chunk plus offset. Nothing is re-read or re-derived, so the
    /// handoff is exact — the sync decoder resumes mid-stream rather than
    /// starting one.
    ///
    /// **Precondition:** `f` must not need more than
    /// [`can_sync`](super::AsyncEntropyDecoder::can_sync) reported available —
    /// i.e. either the input is complete, or every value `f` decodes has a
    /// `MAX_BYTES` that fits in what is buffered. Running the slice decoder past
    /// the end of a *non*-final chunk zero-pads and returns plausible, wrong
    /// values, and the position it reports back would be wrong too.
    ///
    /// That cannot be checked up front — it depends on `MAX_BYTES` being a true
    /// bound — so it is checked afterwards instead: consuming the whole buffer
    /// without the stream having ended means the decode wanted more than was
    /// there, which is exactly the symptom of an understated `MAX_BYTES`.
    #[inline]
    pub(crate) fn with_sync<R>(&mut self, f: impl FnOnce(&mut Decoder) -> R) -> R {
        let rest = self.source.buffered();
        let mut sync = Decoder {
            bytes: &rest,
            state: self.state,
            value: self.value,
        };
        let result = f(&mut sync);
        debug_assert!(
            !sync.bytes.is_empty() || self.source.is_final_chunk(),
            "sync decode consumed every buffered byte without reaching end of \
             stream: some type's MAX_BYTES is too small"
        );
        let consumed = rest.len() - sync.bytes.len();
        self.state = sync.state;
        self.value = sync.value;
        self.source.advance(consumed);
        result
    }

    /// Pull `n` entropy bytes into the window; the async twin of
    /// [`RangeDecoder::refill`].
    #[inline]
    async fn refill(&mut self, n: usize) {
        for _ in 0..n {
            let byte = self.source.next_byte().await;
            self.value = (self.value << 8) + byte as u64;
        }
    }
}

#[cfg(feature = "stream")]
impl<S, E> super::model::AsyncSymbolDecoder for AsyncRangeDecoder<S>
where
    S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    /// Must equal [`RangeDecoder`]'s, so both decoders pick the same `Walk` and
    /// therefore make the same coder steps.
    const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;

    #[inline]
    async fn decode_symbol_step(
        &mut self,
        walk: impl FnOnce(u32) -> (SymbolRange, usize),
    ) -> usize {
        while self.state.clamp_for_symbol() {
            let n = self.state.consume_decoded_bytes();
            self.refill(n).await;
        }
        let slot = self.state.symbol_slot(self.value);
        let (range, decoded) = walk(slot);
        self.state.narrow_symbol(range);
        let n = self.state.consume_decoded_bytes();
        self.refill(n).await;
        decoded
    }
}

#[cfg(feature = "stream")]
impl<S, E> super::AsyncEntropyDecoder for AsyncRangeDecoder<S>
where
    S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Sync<'a>
        = Decoder<'a>
    where
        Self: 'a;

    #[inline]
    fn can_sync(&self, max_bytes: usize) -> bool {
        self.source.can_sync(max_bytes)
    }

    #[inline]
    fn ready_bytes(&self) -> usize {
        self.source.ready_bytes()
    }

    #[inline]
    fn is_final(&self) -> bool {
        self.source.is_final_chunk()
    }

    #[inline]
    fn with_sync<R>(&mut self, f: impl FnOnce(&mut Decoder<'_>) -> R) -> R {
        AsyncRangeDecoder::with_sync(self, f)
    }

    /// A latched stream error wins over a downstream validation error, for the
    /// reason given on the trait method.
    fn into_result<T>(mut self, value: Result<T, std::io::Error>) -> std::io::Result<T> {
        match self.source.take_error() {
            Some(e) => Err(e),
            None => value,
        }
    }

    /// Overrides the trait's per-bit default with the fused whole-symbol walk,
    /// exactly as [`RangeDecoder`] does. Not an optimization: the two walks
    /// narrow the coder state differently and so read different bytes, and this
    /// must match what the encoder did.
    #[inline]
    fn decode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut AtMostContext<MAX>,
    ) -> impl std::future::Future<Output = AtMost<MAX>> {
        walks::decode_symbol_or_bitwise_async(self, ctx)
    }

    #[inline]
    async fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let (out, sz) = self.state.decode(context.probability(), self.value);
            self.refill(sz).await;
            *context = context.adapt(out);
            *b = out;
        }
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(
        &mut self,
        out: &mut [u8],
    ) -> impl std::future::Future<Output = Result<(), std::io::Error>> {
        // By the W_DELAY splice the run sits at the source cursor, so read it
        // straight off, exactly as `RangeDecoder` does.
        self.source.read_exact(out)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use rand::Rng;

    use super::*;

    fn rand_prob() -> (Probability, bool) {
        let value_bool = rand::random::<bool>();
        (rand::random::<Probability>(), value_bool)
    }

    #[test]
    fn encode_decode_last_byte() {
        fn test_state(original_s: ArithState) {
            assert_eq!(
                original_s.clone().ready_bytes().count,
                0,
                "state should already be regularized!"
            );
            assert!(original_s.hi > original_s.lo);
            // println!("\noriginal_s is {original_s:x?}");
            // println!("================================");
            for value_bool in [false, true] {
                let (p, _) = rand_prob();

                let mut s = original_s;
                let encoded_bytes = s.encode(p, value_bool);
                // println!("state after encoding {value_bool:?} is {s:x?}");

                let split = original_s.split(p);

                let values = if value_bool {
                    let rand_value = || rand::thread_rng().gen_range(split + 1..=original_s.hi);
                    vec![split + 1, original_s.hi, rand_value(), rand_value()]
                } else {
                    let rand_value = || rand::thread_rng().gen_range(original_s.lo..=split);
                    vec![original_s.lo, split, rand_value(), rand_value()]
                };
                // println!("\nsplit is {split:x} and choice is {value_bool:?}");
                for value in values {
                    // println!("\n  value={value:x} for {original_s:x?} and {value_bool:?}");
                    let mut decoding_s = original_s;
                    let (decoded, sz) = decoding_s.decode(p, value);
                    // println!("  after decoding {decoded:?} from {value:x} is {decoding_s:x?}");
                    assert_eq!(sz, encoded_bytes.count);
                    assert_eq!(decoded, value_bool);
                    assert_eq!(s, decoding_s);
                }
            }
        }

        test_state(ArithState {
            lo: u64::MAX / 2,
            hi: u64::MAX / 2 + 1,
        });

        let mut s = ArithState::default();
        for _ in 0..10_000 {
            // create a valid state
            s.lo = rand::random();
            if s.lo == u64::MAX {
                s.lo = 0;
            }
            s.hi = s.lo + 1 + (rand::random::<u64>() % (u64::MAX - s.lo));
            println!("initially s is {s:x?}");
            assert!(s.hi > s.lo);
            s.ready_bytes();
            println!("after regularization s is {s:x?}");
            test_state(s);
        }
    }

    #[test]
    fn zero_byte() {
        let mut s = ArithState::default();
        for _ in 0..7 {
            assert_eq!(
                s.encode(
                    Probability {
                        prob: NonZeroU8::new(127).unwrap()
                    },
                    false,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: NonZeroU8::new(127).unwrap(),
            },
            false,
        );
        assert_eq!(bytes.count, 1);
        assert_eq!(bytes.bytes, [0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn one_byte() {
        let mut s = ArithState::default();
        assert_eq!(
            s.split(Probability {
                prob: NonZeroU8::new(128).unwrap()
            }) >> 8,
            (u64::MAX / 2) >> 8
        );
        for _ in 0..8 {
            assert_eq!(
                s.encode(
                    Probability {
                        prob: NonZeroU8::new(127).unwrap()
                    },
                    true,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: NonZeroU8::new(127).unwrap(),
            },
            true,
        );
        assert_eq!(bytes.count, 1);
        assert_eq!(bytes.bytes, [255, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn symbol_state_roundtrip() {
        // Symbol narrowing must round-trip from every reachable state,
        // including adversarially narrow straddled intervals that force the
        // clamp renormalization.
        fn test_state(s: ArithState) {
            // encoder side: clamp until wide enough (bytes are emitted by the
            // caller in the real coder; here we only track state agreement)
            let mut enc = s;
            let mut enc_clamp_bytes = Vec::new();
            while enc.clamp_for_symbol() {
                enc_clamp_bytes.extend_from_slice(&enc.ready_bytes());
            }
            // decoder side must clamp identically, consuming the same count
            let mut dec = s;
            let mut dec_consumed = 0;
            while dec.clamp_for_symbol() {
                dec_consumed += dec.consume_decoded_bytes();
            }
            assert_eq!(enc, dec, "clamp must be deterministic in (lo, hi)");
            assert_eq!(enc_clamp_bytes.len(), dec_consumed);
            assert!(enc.hi - enc.lo >= ArithState::MIN_SYMBOL_WIDTH);

            // a random symbol interval
            let start = rand::random::<u32>() % SymbolRange::M;
            let width = 1 + rand::random::<u32>() % (SymbolRange::M - start);
            let range = SymbolRange::test_new(start, width);
            let mut narrowed = enc;
            narrowed.narrow_symbol(range);
            assert!(narrowed.lo >= enc.lo && narrowed.hi <= enc.hi);
            // every value in the narrowed interval must recover a slot inside
            // the coded range
            for value in [
                narrowed.lo,
                narrowed.hi,
                narrowed.lo + (narrowed.hi - narrowed.lo) / 2,
            ] {
                let slot = enc.symbol_slot(value);
                assert!(
                    slot >= start && slot < start + width,
                    "slot {slot} outside [{start}, {}) for {enc:x?} value {value:x}",
                    start + width
                );
            }
        }

        // the canonical narrowest straddle
        test_state(ArithState {
            lo: u64::MAX / 2,
            hi: u64::MAX / 2 + 1,
        });
        for _ in 0..10_000 {
            let mut s = ArithState::default();
            if rand::random::<bool>() {
                // adversarial: tiny width straddling a top-byte boundary
                let boundary = ((rand::random::<u64>() % 255) + 1) << 56;
                let below = rand::random::<u64>() % (1 << (rand::random::<u32>() % 40));
                let above = rand::random::<u64>() % (1 << (rand::random::<u32>() % 40));
                s.lo = boundary - 1 - below;
                s.hi = boundary + above;
            } else {
                s.lo = rand::random();
                if s.lo == u64::MAX {
                    s.lo = 0;
                }
                s.hi = s.lo + 1 + (rand::random::<u64>() % (u64::MAX - s.lo));
            }
            s.ready_bytes();
            if s.hi > s.lo {
                test_state(s);
            }
        }
    }

    #[test]
    fn encode_decode_symbols_and_bits() {
        super::super::check_mixed_bits_and_symbols!(Range, Decoder::new);
    }

    #[test]
    fn encode_decode() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 32 * 8;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_prob());
            }
            println!("\n\ntesting {probs:?}");
            // `ArithState::encode` is the coder's bit primitive at an arbitrary
            // probability (the `Range` type only offers context-driven
            // encoding). With no incompressible bytes the finished stream is
            // just the entropy bytes plus the final `last_byte`, so we drive the
            // state directly rather than through `Range`.
            let mut state = ArithState::default();
            let mut bytes = Vec::new();
            for &(p, bit) in &probs {
                let ready = state.encode(p, bit);
                bytes.extend_from_slice(&ready);
            }
            bytes.push(state.last_byte());
            println!("\n\nEncoded random as: {bytes:02x?}\n");
            let mut decoder = Decoder::new(&bytes);
            for &(p, bit) in &probs {
                println!("Decoding {p:?} {bit:?}");
                // `decode_step` is the coder's bit primitive at an arbitrary
                // probability (the trait only exposes context-driven decoding).
                let decoded = decode_step(
                    &mut decoder.state,
                    &mut decoder.value,
                    &mut decoder.bytes,
                    p,
                );
                assert_eq!(decoded, bit);
            }
        }
    }

    /// Adversarial round-trip for the delay-interleave splice (`W_DELAY`):
    /// coded values interleaved with incompressible runs of every length
    /// straddling `W_DELAY`, consecutive short runs, and runs at the very end
    /// (the tail edge case). A wrong delay, off-by-one at a splice, or botched
    /// tail is silent corruption, so this hammers the boundaries.
    #[test]
    fn delay_interleave_roundtrip_adversarial() {
        use crate::{Encoded, Incompressible};
        type Item = (u8, Encoded<Vec<u8>, Incompressible>);
        let mut x = 0x1234_5678_9abc_def0u64;
        let mut rng = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            x
        };
        for trial in 0..300u64 {
            let n = (rng() % 60) as usize;
            let mut items: Vec<Item> = Vec::new();
            for i in 0..n {
                // Cover 0..=20 deterministically on some trials (straddles
                // W_DELAY = 8), random on others.
                let len = if trial % 3 == 0 {
                    i % 21
                } else {
                    (rng() % 21) as usize
                };
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }
            // Force a non-empty raw run as the final element on half the trials
            // (the tail case, within W_DELAY of the end).
            if trial % 2 == 0 {
                let len = (trial as usize % 12) + 1;
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }
            let encoded = super::super::encode(&items);
            let decoded: Vec<Item> = super::super::decode(&encoded).unwrap();
            assert_eq!(decoded, items, "trial {trial}");
        }
    }

    /// The streaming and in-memory paths must be freely mix-and-matchable: the
    /// byte stream is identical however it was produced, and either decoder
    /// reads either encoder's output. Checks all four combinations on the same
    /// adversarial coded+incompressible data as `delay_interleave...`.
    #[test]
    fn streaming_matches_in_memory() {
        use crate::{Encoded, Incompressible};
        type Item = (u8, Encoded<Vec<u8>, Incompressible>);
        let mut x = 0xfeed_face_dead_beefu64;
        let mut rng = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            x
        };
        for trial in 0..300u64 {
            let n = (rng() % 60) as usize;
            let mut items: Vec<Item> = Vec::new();
            for i in 0..n {
                let len = if trial % 3 == 0 {
                    i % 21
                } else {
                    (rng() % 21) as usize
                };
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }
            if trial % 2 == 0 {
                let len = (trial as usize % 12) + 1;
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }

            let in_memory = super::super::encode(&items);

            // encode_to (streaming) into a Vec must be byte-identical to encode.
            let mut streamed = Vec::new();
            super::super::encode_to(&items, &mut streamed).unwrap();
            assert_eq!(streamed, in_memory, "trial {trial}: encode_to != encode");

            // Every reader/writer pairing round-trips:
            let d1: Vec<Item> = super::super::decode_from(in_memory.as_slice()).unwrap();
            assert_eq!(d1, items, "trial {trial}: decode_from(encode)");
            let d2: Vec<Item> = super::super::decode(&streamed).unwrap();
            assert_eq!(d2, items, "trial {trial}: decode(encode_to)");
            let d3: Vec<Item> = super::super::decode_from(streamed.as_slice()).unwrap();
            assert_eq!(d3, items, "trial {trial}: decode_from(encode_to)");
        }
    }

    /// A value type with both entropy-coded and incompressible bytes, big enough
    /// that the streamed output is a few hundred bytes — enough for a mid-stream
    /// reader/writer failure to land inside the coded region.
    fn sample_items() -> Vec<(u8, crate::Encoded<Vec<u8>, crate::Incompressible>)> {
        use crate::Encoded;
        let mut x = 0x0123_4567_89ab_cdefu64;
        let mut rng = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            x
        };
        (0..50)
            .map(|i| {
                let run: Vec<u8> = (0..(i % 7)).map(|_| rng() as u8).collect();
                (rng() as u8, Encoded::new(run))
            })
            .collect()
    }

    /// `encode_to` to a real file on disk, then `decode_from` it. Exercises the
    /// `Read`/`Write` paths against a real OS file (not a `Vec`/slice), where
    /// reads genuinely return in chunks and can hit EOF.
    #[test]
    fn roundtrip_through_a_real_file() {
        type Item = (u8, crate::Encoded<Vec<u8>, crate::Incompressible>);
        let items = sample_items();
        let path = std::env::temp_dir().join(format!(
            "compactly-stream-roundtrip-{}.bin",
            std::process::id()
        ));

        let file = std::fs::File::create(&path).unwrap();
        super::super::encode_to(&items, file).unwrap();

        // Bytes on disk are identical to the in-memory encoding.
        let on_disk = std::fs::read(&path).unwrap();
        assert_eq!(on_disk, super::super::encode(&items));

        let file = std::fs::File::open(&path).unwrap();
        let decoded: Vec<Item> = super::super::decode_from(file).unwrap();
        assert_eq!(decoded, items);

        std::fs::remove_file(&path).ok();
    }

    /// A `Write` that accepts `fail_after` bytes and then errors on every
    /// further write.
    struct FailingWriter {
        written: usize,
        fail_after: usize,
    }
    impl std::io::Write for FailingWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            if self.written >= self.fail_after {
                return Err(std::io::Error::other("disk full"));
            }
            let take = buf.len().min(self.fail_after - self.written);
            self.written += take;
            Ok(take)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    /// `encode_to` must surface a writer error as a clean `Err`, never panic and
    /// never silently succeed.
    #[test]
    fn encode_to_surfaces_writer_errors() {
        let items = sample_items();
        // Fail immediately (byte 0) and partway through (byte 64); both must be
        // reported through the `Result`, whether the failure hits during
        // encoding or during the final `BufWriter` flush.
        for fail_after in [0usize, 64] {
            let writer = FailingWriter {
                written: 0,
                fail_after,
            };
            let result = super::super::encode_to(&items, writer);
            assert!(
                result.is_err(),
                "encode_to should report the writer failure (fail_after={fail_after})"
            );
        }
    }

    /// R1: `encode_to` docs tell callers to wrap an unbuffered sink in a
    /// `BufWriter`. `BufWriter`'s `Drop` silently swallows its final flush error,
    /// so `encode_to` must flush the returned writer itself — otherwise a caller
    /// following that advice loses the tail of the stream on a disk-full at flush
    /// time and still gets `Ok(())`.
    #[test]
    fn encode_to_surfaces_buffered_flush_error() {
        /// Accepts every write (into a `BufWriter`'s buffer it never fills) but
        /// fails on flush — i.e. the error only appears at the final flush.
        struct FlushFails;
        impl std::io::Write for FlushFails {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(buf.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Err(std::io::Error::other("flush failed (e.g. ENOSPC)"))
            }
        }
        let items = sample_items();
        let writer = std::io::BufWriter::new(FlushFails);
        let result = super::super::encode_to(&items, writer);
        assert!(
            result.is_err(),
            "encode_to must surface the wrapped BufWriter's final flush error, \
             not swallow it in Drop"
        );
    }

    /// A `Read` over `data` that yields at most one byte per call and returns a
    /// transient error on the call indices in `fail_at` — modelling a flaky
    /// socket/disk that errors once and would then recover. `calls` is shared
    /// with the test so it can assert the reader is never touched again once an
    /// error has been latched.
    struct FlakyReader {
        data: Vec<u8>,
        pos: usize,
        calls: std::rc::Rc<std::cell::Cell<usize>>,
        fail_at: Vec<usize>,
    }
    impl std::io::Read for FlakyReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let call = self.calls.get();
            self.calls.set(call + 1);
            if self.fail_at.contains(&call) {
                return Err(std::io::Error::other("transient read failure"));
            }
            if self.pos >= self.data.len() || buf.is_empty() {
                return Ok(0);
            }
            buf[0] = self.data[self.pos];
            self.pos += 1;
            Ok(1)
        }
    }

    /// `decode_from` must surface a reader error as a clean `Err`, never panic
    /// (the R5 corruption path) and never return silently-wrong data. Once the
    /// error is latched the reader is never touched again, so a would-be
    /// recovery cannot splice genuine bytes into the fabricated zeros.
    #[test]
    fn decode_from_surfaces_reader_errors() {
        type Item = (u8, crate::Encoded<Vec<u8>, crate::Incompressible>);
        let items = sample_items();
        let encoded = super::super::encode(&items);
        assert!(
            encoded.len() > 40,
            "need a stream long enough to fail inside"
        );

        // Fail on call 20, and again on call 30 — the second must never be
        // reached, because the latch is supposed to stop us touching the reader
        // at all after the first failure.
        let calls = std::rc::Rc::new(std::cell::Cell::new(0));
        let reader = FlakyReader {
            data: encoded.clone(),
            pos: 0,
            calls: std::rc::Rc::clone(&calls),
            fail_at: vec![20, 30],
        };
        let result: std::io::Result<Vec<Item>> = super::super::decode_from(reader);
        assert!(
            result.is_err(),
            "decode_from should report the latched reader error"
        );
        // The assertion that actually pins the R5 guard: `is_err()` alone would
        // hold even without it (the first error still gets latched and
        // surfaced). What the guard buys is that the reader is untouched
        // afterwards, so a reader that recovers cannot splice genuine bytes into
        // the fabricated zeros and silently desynchronize the coder.
        assert_eq!(
            calls.get(),
            21,
            "reader must not be called again after the first error is latched \
             (called {} times; expected to stop at the failing call 20)",
            calls.get()
        );

        // Sanity: with no injected failures the same reader decodes correctly,
        // confirming the one-byte-at-a-time reader is otherwise well-behaved.
        let reader = FlakyReader {
            data: encoded,
            pos: 0,
            calls: std::rc::Rc::new(std::cell::Cell::new(0)),
            fail_at: vec![],
        };
        let decoded: Vec<Item> = super::super::decode_from(reader).unwrap();
        assert_eq!(decoded, items);
    }

    /// R2: once a read error is latched, [`EntropyDecoder::into_result`] must
    /// surface *that* IO error even when `T::decode` itself returned an `Err`.
    /// Coder decode is infallible, so a mid-stream failure zero-pads and the
    /// fabricated bits often trip an unrelated downstream validation (a zero
    /// `NonZero`, a bad `char`); returning that symptom would silently drop the
    /// real root cause. Before the fix `Range::decode_from` did `T::decode(..)?`,
    /// propagating the downstream error and losing the latched one.
    #[test]
    fn into_result_prefers_latched_error_over_downstream() {
        // Fails on the very first read, so an IO error latches during
        // construction and the whole stream is fabricated zeros thereafter.
        let reader = FlakyReader {
            data: vec![0u8; 16],
            pos: 0,
            calls: std::rc::Rc::new(std::cell::Cell::new(0)),
            fail_at: vec![0],
        };
        let decoder = RangeDecoder::new(reader);
        let downstream: std::io::Result<u8> =
            Err(std::io::Error::other("downstream validation symptom"));
        let err = decoder
            .into_result(downstream)
            .expect_err("a latched read error must surface as Err");
        assert!(
            err.to_string().contains("transient read failure"),
            "the latched IO error (root cause) must win over the downstream \
             validation error, got: {err}"
        );

        // With nothing latched, both an Ok and an Err pass through unchanged.
        let clean = RangeDecoder::new(std::io::Cursor::new(vec![0u8; 16]));
        assert_eq!(clean.into_result::<u8>(Ok(42)).unwrap(), 42);
        let clean = RangeDecoder::new(std::io::Cursor::new(vec![0u8; 16]));
        assert_eq!(
            clean
                .into_result::<u8>(Err(std::io::Error::other("kept")))
                .unwrap_err()
                .to_string(),
            "kept"
        );
    }
}

#[test]
fn range_debug_summarizes_rather_than_dumping() {
    use super::Encode;
    // A large in-progress encode must not format its whole output buffer.
    let big: Vec<u64> = (0..50_000).collect();
    let mut coder = Range::default();
    big.encode(&mut coder, &mut <Vec<u64> as Encode>::Context::default());
    let shown = format!("{coder:?}");
    assert!(
        shown.len() < 300,
        "Debug should summarize, not dump the buffer; got {} chars: {shown}",
        shown.len()
    );
    assert!(shown.contains("entropy_written"), "got {shown}");
}
