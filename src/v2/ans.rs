use super::atmost::{walks, AtMost, AtMostContext};
use super::bit_context::BitContext;
use super::model::{Probability, SymbolCoder, SymbolDecoder, SymbolRange};
use super::{EntropyCoder, EntropyDecoder};
mod bytes;
use bytes::Bytes;

type State = u32;
const STATE_BYTES: usize = std::mem::size_of::<State>();

impl From<Probability> for State {
    fn from(value: Probability) -> Self {
        Self::from(value.prob.get())
    }
}

/// Append `v` as an unsigned LEB128 varint (chunk framing).
fn push_varint(out: &mut Vec<u8>, mut v: usize) {
    while v >= 0x80 {
        out.push((v as u8) | 0x80);
        v >>= 7;
    }
    out.push(v as u8);
}

/// Read an unsigned LEB128 varint from the front of `bytes`, advancing it.
///
/// Malformed input must not panic: this is reached from `Ans::decode`, which is
/// `Option`-returning, for every chunk header. A continuation-bit run longer
/// than a `usize` would otherwise shift out of range (a debug-build panic), so
/// the shift is bounded exactly as in [`read_varint_io`].
fn read_varint(bytes: &mut &[u8]) -> usize {
    let mut v = 0usize;
    let mut shift = 0u32;
    while let Some((&b, rest)) = bytes.split_first() {
        *bytes = rest;
        v |= ((b & 0x7f) as usize) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= usize::BITS {
            break; // corrupt varint: stop rather than shift out of range
        }
    }
    v
}

/// ANS entropy encoding.
///
/// Can be used to encode data.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Ans::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 9);
/// assert_eq!(compactly::v2::Ans::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default)]
pub struct Ans(AnsEncoder<Vec<u8>>);

/// Streaming ANS encoder: records deferred coding ops and, once the buffer fills
/// a chunk (`CHUNK_OPS` ops), flushes a self-contained rANS chunk to `W` — so
/// peak encoder memory is bounded regardless of value size. Produces
/// **byte-identical** output to the in-memory [`Ans`] (which is just
/// `AnsEncoder<Vec<u8>>`) for the same value. IO errors are latched and surfaced
/// by [`AnsEncoder::finish`], keeping the infallible [`EntropyCoder`] hot path
/// branch-free.
#[derive(Debug, Default)]
pub(crate) struct AnsEncoder<W: std::io::Write> {
    /// Ops recorded for the *current* chunk (flushed once they reach
    /// `CHUNK_OPS`); the contexts adapt across chunk boundaries during recording.
    ops: Vec<Op>,
    /// Raw incompressible bytes recorded for the current chunk, in order.
    incompressible_bytes: Vec<u8>,
    /// Sink for flushed chunk frames — a `Vec<u8>` in memory, or any `Write`.
    writer: W,
    error: Option<std::io::Error>,
}

/// The number of ops (bits + symbols + incompressible runs) per chunk. Each
/// chunk is an independent rANS unit, so this bounds the encoder's op buffer and
/// the decoder's per-chunk memory; the contexts adapt continuously across chunk
/// boundaries so there is no compression loss beyond one state-flush per chunk.
/// A value fitting in one chunk emits a single (final) chunk. Small enough to
/// bound memory, large enough that the per-chunk overhead is negligible.
const CHUNK_OPS: usize = 1 << 16;

/// One deferred coding operation. rANS runs the coder backwards over each chunk
/// of the buffer in [`Ans::into_vec`], so symbols are recorded here next to bits
/// to preserve their interleaving. The symbol interval is stored packed
/// (`width` is in `1..=M`, so `width - 1` fits a `u16`) to keep the buffer entry
/// small. `Incompressible` marks the *position* of a raw run in the op sequence
/// so chunking partitions the raw bytes with the coded ops; it carries no
/// payload, since the bytes themselves live in `AnsEncoder::incompressible_bytes`
/// and both sides drain them in lockstep with these markers. Carrying a length
/// here would also widen the whole enum to `usize` alignment, inflating every
/// `Bit`/`Symbol` entry to no purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Bit(bool, Probability),
    Symbol { start: u16, width_minus_1: u16 },
    Incompressible,
}

impl<W: std::io::Write> EntropyCoder for AnsEncoder<W> {
    #[inline]
    fn encode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N], bits: [bool; N]) {
        self.ops
            .extend(bits.into_iter().zip(contexts.iter_mut()).map(|(b, ctx)| {
                let probability = ctx.probability();
                *ctx = ctx.adapt(b);
                Op::Bit(b, probability)
            }));
        self.maybe_flush();
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
        self.ops.push(Op::Incompressible);
        self.incompressible_bytes.extend_from_slice(bytes);
        self.maybe_flush();
    }
}

impl<W: std::io::Write> SymbolCoder for AnsEncoder<W> {
    /// Record one deferred whole-symbol op, packed like the bit ops.
    #[inline]
    fn encode_symbol(&mut self, range: SymbolRange) {
        self.ops.push(Op::Symbol {
            start: range.start() as u16,
            width_minus_1: (range.width() - 1) as u16,
        });
        self.maybe_flush();
    }
}

impl EntropyCoder for Ans {
    #[inline]
    fn encode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N], bits: [bool; N]) {
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

impl SymbolCoder for Ans {
    #[inline]
    fn encode_symbol(&mut self, range: SymbolRange) {
        self.0.encode_symbol(range)
    }
}

impl<W: std::io::Write> AnsEncoder<W> {
    #[inline]
    pub(crate) fn new(writer: W) -> Self {
        Self {
            ops: Vec::new(),
            incompressible_bytes: Vec::new(),
            writer,
            error: None,
        }
    }

    #[inline]
    fn write_out(&mut self, bytes: &[u8]) {
        if self.error.is_none() {
            if let Err(e) = self.writer.write_all(bytes) {
                self.error = Some(e);
            }
        }
    }

    /// Flush a non-final chunk once the op buffer reaches `CHUNK_OPS`. Called
    /// after each recorded batch, so chunk boundaries always land between batches
    /// (a `decode_bits<N>` never straddles two chunks' separate rANS streams).
    #[inline]
    fn maybe_flush(&mut self) {
        if self.ops.len() >= CHUNK_OPS {
            self.flush_chunk(false);
        }
    }

    /// Reverse-encode the current chunk's ops into a self-contained rANS stream
    /// and write the framed chunk to `writer`, then clear the chunk buffers. The
    /// contexts are *not* reset (they adapt across chunks). `is_final` writes an
    /// op-count of 0 (the decoder then decodes until the value is complete).
    fn flush_chunk(&mut self, is_final: bool) {
        let mut coder = Encoder::new();
        let mut entropy = Vec::new();
        for op in self.ops.iter().rev() {
            match *op {
                Op::Bit(b, probability) => {
                    if let Some(byte) = coder.encode(b, probability) {
                        entropy.push(byte);
                    }
                }
                Op::Symbol {
                    start,
                    width_minus_1,
                } => {
                    let (bytes, state) = coder
                        .state
                        .encode_symbol(start as State, width_minus_1 as State + 1);
                    coder.state = state;
                    entropy.extend(bytes);
                }
                Op::Incompressible => {} // raw bytes bypass the coder
            }
        }
        entropy.extend(coder.finish_encoding());
        entropy.reverse();

        let op_count = if is_final { 0 } else { self.ops.len() };
        let mut header = Vec::new();
        push_varint(&mut header, op_count);
        push_varint(&mut header, entropy.len());
        push_varint(&mut header, self.incompressible_bytes.len());
        self.write_out(&header);
        self.write_out(&entropy);
        // `incompressible_bytes` is cleared for the next chunk anyway, so take it
        // out to write (avoids borrowing `self` while `write_out` needs `&mut`).
        let incompressible = std::mem::take(&mut self.incompressible_bytes);
        self.write_out(&incompressible);
        self.ops.clear();
    }

    /// Finish encoding: flush the final chunk, then return the sink or the
    /// latched IO error. [`Ans::into_vec`] is the in-memory caller.
    pub(crate) fn finish(mut self) -> std::io::Result<W> {
        self.flush_chunk(true);
        match self.error.take() {
            Some(e) => Err(e),
            None => Ok(self.writer),
        }
    }
}

impl Ans {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Encode `value` straight into a [`Write`](std::io::Write), streaming chunks
    /// out as they fill rather than buffering the whole compressed output; the
    /// bytes are **identical** to [`Ans::encode`]. `writer` is wrapped in a
    /// [`BufWriter`](std::io::BufWriter) internally.
    pub fn encode_to<T: super::Encode, W: std::io::Write>(
        value: &T,
        writer: W,
    ) -> std::io::Result<()> {
        let mut encoder = AnsEncoder::new(std::io::BufWriter::new(writer));
        value.encode(&mut encoder, &mut T::Context::default());
        let buffered = encoder.finish()?;
        // Surface a deferred flush error from the BufWriter, if any.
        buffered.into_inner().map_err(|e| e.into_error())?;
        Ok(())
    }
    /// Decode a value straight from a [`Read`](std::io::Read), pulling one chunk
    /// at a time rather than requiring the whole compressed input in memory.
    /// Accepts the same bytes [`Ans::encode`]/[`Ans::encode_to`] produce.
    /// `reader` is wrapped in a [`BufReader`](std::io::BufReader) internally.
    pub fn decode_from<T: super::Encode, R: std::io::Read>(reader: R) -> std::io::Result<T> {
        let mut decoder = AnsDecoder::new(std::io::BufReader::new(reader));
        match T::decode(&mut decoder, &mut T::Context::default()) {
            Ok(value) => decoder.into_result(value),
            // Prefer a latched IO error over whatever `T::decode` reported.
            // Coder decode is infallible, so a mid-stream IO failure zero-pads
            // instead of erroring; the fabricated bits then often trip some
            // unrelated validation (a zero `NonZero`, a bad `char`) deeper in
            // `T::decode`. Returning that would report a symptom and silently
            // drop the actual root cause.
            Err(e) => Err(decoder.into_result(()).err().unwrap_or(e)),
        }
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        // Peek the first frame's op-count: 0 means it is the *final* chunk, so
        // the whole value lives in this one chunk and needs no boundary
        // tracking. That is the common case, and skipping the per-batch
        // `ops_left` bookkeeping is worth up to 21% of decode time — see
        // [`Decoder`]. Multi-chunk streams take the tracking decoder.
        let single_chunk = read_varint(&mut { bytes }) == 0;
        if single_chunk {
            Self::decode_with::<T, false>(bytes)
        } else {
            Self::decode_with::<T, true>(bytes)
        }
    }

    /// One arm of [`Self::decode`]'s dispatch.
    ///
    /// Deliberately **not** inlined: each instantiation monomorphizes the whole
    /// of `T::decode`, and letting both arms inline into one function makes an
    /// unoptimized build reserve stack for both call trees at once (the arms are
    /// exclusive, but debug builds do not overlap their slots).
    ///
    /// That doubled frame size overflowed `windows-test`'s 2 MiB test-thread
    /// stack on `crash_from_bench` — whose *own* data is shallow (`["Al",
    /// "Aïr"]`); the depth comes from `encoded_bits!` wrapping it in an 8x8
    /// nested tuple. Measured with `RUST_MIN_STACK`: 1792 KiB before the const
    /// generic, 2048 KiB with it inlined, 1792 KiB again out of line. Note that
    /// margin is thin — this test sat at 1792 KiB against 2 MiB even before any
    /// of this work, so it is a sensitive canary for debug frame growth.
    ///
    /// Out of line, only one arm's frame is ever live. The call costs nothing
    /// measurable — it happens once per decoded value, not per op.
    #[inline(never)]
    fn decode_with<T: super::Encode, const CHUNKED: bool>(bytes: &[u8]) -> Option<T> {
        let mut reader = Decoder::<CHUNKED>::from(bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
    /// Whether `Ans`'s decoder asks [`Walk::production`](super::Walk::production)
    /// to speculate on a non-power-of-two value count (see
    /// [`SymbolDecoder::SPECULATES`]). Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;
    /// Encode `values` using an explicitly forced tree walk, bypassing
    /// [`Walk::production`](super::Walk::production)'s usual choice for
    /// `MAX`. `WHICH_WALK` indexes [`WALKS`](super::WALKS). Benchmark support
    /// for `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub fn encode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        values: &[super::AtMost<MAX>],
    ) -> Vec<u8> {
        walks::encode_atmost_batch::<Self, MAX, WHICH_WALK>(Self::default(), values).into_vec()
    }
    /// The decode side of [`Self::encode_atmost_batch`]: decode `n` values
    /// with the same forced walk. Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub fn decode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        bytes: &[u8],
        n: usize,
    ) -> Vec<super::AtMost<MAX>> {
        // Mirror `Ans::decode`'s dispatch — including keeping the arms out of
        // line — so the benchmark measures the decoder production would
        // actually pick for these bytes.
        if read_varint(&mut { bytes }) == 0 {
            Self::decode_atmost_batch_with::<MAX, WHICH_WALK, false>(bytes, n)
        } else {
            Self::decode_atmost_batch_with::<MAX, WHICH_WALK, true>(bytes, n)
        }
    }

    /// One arm of [`Self::decode_atmost_batch`]'s dispatch; see
    /// [`Self::decode_with`] for why this is kept out of line.
    #[inline(never)]
    fn decode_atmost_batch_with<const MAX: usize, const WHICH_WALK: usize, const CHUNKED: bool>(
        bytes: &[u8],
        n: usize,
    ) -> Vec<super::AtMost<MAX>> {
        walks::decode_atmost_batch::<Decoder<CHUNKED>, MAX, WHICH_WALK>(
            Decoder::<CHUNKED>::from(bytes),
            n,
        )
    }
    /// Whether this encoder is still on its first chunk, i.e. nothing has been
    /// flushed and `self.0.ops` is a complete record of the value.
    /// [`Self::replay_entropy_decode`] requires this; benchmarks use it to size
    /// their input. Benchmark support, not part of the stable API.
    #[doc(hidden)]
    pub fn is_single_chunk(&self) -> bool {
        self.0.writer.is_empty()
    }

    /// Benchmark helper: replay only the entropy-decode steps against
    /// `encoded`, using this op buffer (from encoding the same value) as an
    /// oracle for the probabilities and symbol intervals that the adaptive
    /// contexts would supply. This isolates the rANS state/byte work from the
    /// model (context adaptation) and value construction; see
    /// `src/bin/ans-decode-phases.rs`. Panics if a decoded bit disagrees with
    /// the recorded one. Returns a checksum so callers can `black_box` it.
    ///
    /// Only valid for a **single-chunk** value: once encoding flushes a chunk
    /// (at [`CHUNK_OPS`] ops) it clears the flushed ops from `self.ops`, so the
    /// buffer would no longer be a complete oracle. A single-chunk value has
    /// written nothing to its sink yet, so `self.0.writer` is still empty; we
    /// assert that, failing loudly rather than measuring a truncated replay.
    /// Keep the benchmark input under `CHUNK_OPS` ops.
    #[doc(hidden)]
    pub fn replay_entropy_decode(&self, encoded: &[u8]) -> u32 {
        assert!(
            self.0.writer.is_empty(),
            "replay_entropy_decode requires a single-chunk stream (input exceeded CHUNK_OPS ops)"
        );
        let mut decoder = Decoder::<false>::from(encoded);
        let mut checksum = 0u32;
        for op in &self.0.ops {
            match *op {
                Op::Bit(b, probability) => {
                    let bit =
                        decode_step(&mut decoder.state.state, &mut decoder.bytes, probability);
                    assert_eq!(bit, b);
                    checksum = checksum.wrapping_add(bit as u32);
                }
                Op::Symbol {
                    start,
                    width_minus_1,
                } => {
                    let mut state = decoder.state.state;
                    let slot = state & (SymbolRange::M - 1);
                    state = (width_minus_1 as State + 1) * (state >> SymbolRange::BITS)
                        + (slot - start as State);
                    while state < 1 << (State::BITS - 8) {
                        let Some((&byte, rest)) = decoder.bytes.split_first() else {
                            break;
                        };
                        decoder.bytes = rest;
                        state = (state << 8) | byte as State;
                    }
                    decoder.state.state = state;
                    checksum = checksum.wrapping_add(slot);
                }
                Op::Incompressible => {} // raw bytes bypass the coder
            }
        }
        checksum
    }
    /// Finish encoding: flush the final chunk and return the framed stream.
    ///
    /// The stream is a sequence of chunks. Non-final chunks (flushed during
    /// recording once the op buffer reaches [`CHUNK_OPS`]) carry their real
    /// op-count; the final chunk carries op-count 0 ("decode until the value is
    /// complete"). Each chunk is
    /// `[op-count][entropy-len][incompressible-len][entropy][incompressible]`,
    /// all varints; `entropy` is that chunk's self-contained rANS stream (state
    /// then body, in decode order) and `incompressible` its raw bytes. The
    /// contexts adapt continuously across chunk boundaries, so chunking costs
    /// only one rANS state-flush (plus the tiny frame header) per chunk.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.0.finish().expect("writing to a Vec<u8> is infallible")
    }
}
impl From<Ans> for Vec<u8> {
    fn from(value: Ans) -> Self {
        value.into_vec()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Encoder {
    state: StateOnly,
}

impl Encoder {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: StateOnly { state: 0 },
        }
    }

    /// Encode a bit using distribution Bernoulli(probability).
    #[inline(always)]
    fn encode(&mut self, b: bool, probability: Probability) -> Option<u8> {
        let (out, state) = self.state.encode(b, probability);
        self.state = state;
        out
    }

    #[inline(always)]
    pub fn finish_encoding(&mut self) -> Bytes {
        let mut bytes = Bytes::default();
        while self.state.state != 0 {
            bytes.push(self.state.state as u8);
            self.state.state >>= 8;
        }
        bytes
    }
}

/// The in-memory (slice) decoder.
///
/// `CHUNKED` selects how chunk boundaries are tracked. A stream whose *first*
/// frame is the final one (op-count 0) is a single chunk, which is the common
/// case; decoding it needs no boundary tracking at all, so `CHUNKED = false`
/// compiles the `ops_left` check and decrement out of the per-batch hot path
/// entirely. That is worth real time: keeping the bookkeeping unconditionally
/// measured **+21%** on a cache-resident `Vec<u64>` decode and +6% on a
/// memory-bound one. [`Ans::decode`] peeks the first frame and picks.
#[derive(Eq, PartialEq, Debug)]
pub struct Decoder<'a, const CHUNKED: bool = true> {
    state: StateOnly,
    /// The current chunk's rANS body (entropy bytes after the initial state).
    bytes: &'a [u8],
    /// The current chunk's raw incompressible bytes, in order.
    incompressible: &'a [u8],
    /// The rest of the stream: chunks not yet entered. Unused when `!CHUNKED`.
    rest: &'a [u8],
    /// Ops left in the current chunk before the next chunk must be loaded;
    /// `usize::MAX` for the final chunk. Only read/written when `CHUNKED`.
    ops_left: usize,
}

impl<'a, const CHUNKED: bool> From<&'a [u8]> for Decoder<'a, CHUNKED> {
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        // Enter the first chunk; `load_next_chunk` parses its frame (see
        // `Ans::flush_chunk`). A single-chunk (final) value leaves `ops_left`
        // at `usize::MAX`, so even under `CHUNKED` the check never fires.
        let mut decoder = Decoder {
            state: StateOnly { state: 0 },
            bytes: &[],
            incompressible: &[],
            rest: bytes,
            ops_left: 0,
        };
        decoder.load_next_chunk();
        decoder
    }
}

impl<'a, const CHUNKED: bool> Decoder<'a, CHUNKED> {
    /// Enter the next chunk from `self.rest`: parse its frame, initialize the
    /// rANS state from the entropy body, point `incompressible` at its raw run,
    /// advance `self.rest`, and set `ops_left`. Each chunk is an independent
    /// rANS stream, so the state restarts here; the model contexts (owned by the
    /// caller) carry over, exactly as they did across the boundary on encode.
    #[inline]
    fn load_next_chunk(&mut self) {
        let mut rest = self.rest;
        let op_count = read_varint(&mut rest);
        let entropy_len = read_varint(&mut rest);
        let incompressible_len = read_varint(&mut rest);
        let (entropy, rest) = rest.split_at(entropy_len.min(rest.len()));
        let (incompressible, rest) = rest.split_at(incompressible_len.min(rest.len()));
        self.rest = rest;
        self.incompressible = incompressible;
        self.ops_left = if op_count == 0 { usize::MAX } else { op_count };
        if entropy.len() < STATE_BYTES {
            let mut state: State = 0;
            for &b in entropy.iter() {
                state = state << 8 | State::from(b);
            }
            self.state = StateOnly { state };
            self.bytes = &[];
        } else {
            let state = State::from_be_bytes(entropy[0..STATE_BYTES].try_into().unwrap());
            self.state = StateOnly { state };
            self.bytes = &entropy[STATE_BYTES..];
        }
    }
}

/// One rANS bit-decode step, operating on locals so the caller can keep `state`
/// and the input cursor `bytes` register-resident across a whole batch.
#[inline(always)]
fn decode_step(state: &mut State, bytes: &mut &[u8], probability: Probability) -> bool {
    let ones = State::from(probability);
    let zeros = 256 - ones;
    let z = *state & 255;
    let b = z >= ones;
    let s = *state >> 8;
    // Branchless: compute both paths and select via CMOV.
    let state_b = (s * zeros).wrapping_add(z.wrapping_sub(ones));
    let state_nb = s * ones + z;
    let mut new_s = if b { state_b } else { state_nb };
    if new_s < (1 << (State::BITS - 8)) {
        if let Some((&byte, rest)) = bytes.split_first() {
            *bytes = rest;
            new_s = (new_s << 8) | byte as State;
        }
    }
    *state = new_s;
    b
}

impl<'a, const CHUNKED: bool> SymbolDecoder for Decoder<'a, CHUNKED> {
    /// `Ans` always takes the plain walk: its lean symbol step leaves
    /// speculative work exposed — measured slower at every value count
    /// (+4…+22%); see the walk inventory in `atmost::walks`.
    const SPECULATES: bool = false;

    /// Whole-symbol decode step: peek the low [`SymbolRange::BITS`]
    /// bits of the state as the slot, let `walk` recover the value and
    /// interval (adapting its contexts), then do a single rANS advance +
    /// renormalization instead of one per bit.
    ///
    /// The bit steps (total 256) and symbol steps (total `M = 2^16`) share the
    /// same normalization interval `[2^24, 2^32)`, so they can interleave
    /// freely in one state/stream; a symbol step may need to pull up to two
    /// bytes where a bit step pulls at most one.
    #[inline(always)]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        if CHUNKED && self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state.state;
        let mut bytes = self.bytes;
        let slot = state & (SymbolRange::M - 1);
        let (range, value) = walk(slot);
        state = range.width() * (state >> SymbolRange::BITS) + (slot - range.start());
        while state < (1 << (State::BITS - 8)) {
            let Some((&byte, rest)) = bytes.split_first() else {
                break;
            };
            bytes = rest;
            state = (state << 8) | byte as State;
        }
        self.state.state = state;
        self.bytes = bytes;
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(1);
        }
        value
    }
}

impl<'a, const CHUNKED: bool> EntropyDecoder for Decoder<'a, CHUNKED> {
    /// Whole `AtMost` symbol decode; see [`SymbolDecoder::decode_symbol_step`].
    #[inline(always)]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    /// Adaptive batch decode, fused into a single pass.
    ///
    /// We pull `state`/`bytes` into locals and do probability-lookup, decode, and
    /// `adapt` in one pass, keeping the coder state register-resident across the
    /// run rather than re-reading the `Decoder` every bit. The contexts are
    /// independent, so adapting bit `i` never changes bit `j`'s probability — the
    /// result is identical to the per-bit default.
    #[inline(always)]
    fn decode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N]) -> [bool; N] {
        if CHUNKED && self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state.state;
        let mut bytes = self.bytes;
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut bytes, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        self.state.state = state;
        self.bytes = bytes;
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(N);
        }
        bits
    }

    #[inline(always)]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        if CHUNKED && self.ops_left == 0 {
            self.load_next_chunk();
        }
        if self.incompressible.len() < bytes.len() {
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                self.incompressible.len(),
                bytes.len()
            )));
        }
        let (b, incompressible) = self.incompressible.split_at(bytes.len());
        self.incompressible = incompressible;
        bytes.copy_from_slice(b);
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(1);
        }
        Ok(())
    }
}

/// Read one byte from `reader`, returning 0 at a clean EOF and — once an error is
/// latched — never touching `reader` again (fabricating 0s), mirroring the slice
/// decoder's zero-padding past the end of a chunk.
#[inline]
fn read_one_byte_io<R: std::io::Read>(reader: &mut R, error: &mut Option<std::io::Error>) -> u8 {
    if error.is_some() {
        return 0;
    }
    let mut buf = [0u8; 1];
    loop {
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

/// Read a LEB128 varint (the chunk-frame header encoding) from `reader`.
fn read_varint_io<R: std::io::Read>(reader: &mut R, error: &mut Option<std::io::Error>) -> usize {
    let mut v = 0usize;
    let mut shift = 0u32;
    loop {
        let b = read_one_byte_io(reader, error);
        v |= ((b & 0x7f) as usize) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= usize::BITS {
            break; // corrupt varint: stop rather than shift out of range
        }
    }
    v
}

/// Read `len` bytes from `reader` into a fresh buffer, in bounded increments so a
/// corrupt/huge declared length can't drive one giant allocation. A short read
/// latches the error and returns the partial buffer; the rANS decode then
/// zero-pads / reports insufficient bytes, and `into_result` surfaces the error.
fn read_region<R: std::io::Read>(
    reader: &mut R,
    len: usize,
    error: &mut Option<std::io::Error>,
) -> Vec<u8> {
    if error.is_some() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut remaining = len;
    while remaining > 0 {
        let chunk = remaining.min(1 << 16);
        let start = out.len();
        out.resize(start + chunk, 0);
        if let Err(e) = reader.read_exact(&mut out[start..]) {
            *error = Some(e);
            out.truncate(start);
            break;
        }
        remaining -= chunk;
    }
    out
}

/// Streaming ANS decoder: pulls one chunk frame at a time from `R` rather than
/// indexing a whole slice, so decoding a large value need only hold one chunk's
/// entropy + incompressible bytes at once. Reads the same bytes
/// [`Ans`]/[`AnsEncoder`] produce and recovers identical values (the per-chunk
/// arithmetic is the slice [`Decoder`]'s; only the byte source differs). IO
/// errors are latched and surfaced by [`AnsDecoder::into_result`].
pub(crate) struct AnsDecoder<R: std::io::Read> {
    reader: R,
    state: State,
    /// Current chunk's entropy region (its leading `STATE_BYTES` seed the state).
    entropy: Vec<u8>,
    epos: usize,
    /// Current chunk's raw incompressible region.
    incompressible: Vec<u8>,
    ipos: usize,
    /// Ops left in the current chunk; `usize::MAX` for the final chunk.
    ops_left: usize,
    error: Option<std::io::Error>,
}

impl<R: std::io::Read> AnsDecoder<R> {
    pub(crate) fn new(reader: R) -> Self {
        let mut decoder = AnsDecoder {
            reader,
            state: 0,
            entropy: Vec::new(),
            epos: 0,
            incompressible: Vec::new(),
            ipos: 0,
            ops_left: 0,
            error: None,
        };
        decoder.load_next_chunk();
        decoder
    }

    /// Read and enter the next chunk frame: parse the header, pull the entropy
    /// and incompressible regions into owned buffers, seed the rANS state from
    /// the entropy's leading bytes, and set `ops_left`.
    fn load_next_chunk(&mut self) {
        let op_count = read_varint_io(&mut self.reader, &mut self.error);
        let entropy_len = read_varint_io(&mut self.reader, &mut self.error);
        let incompressible_len = read_varint_io(&mut self.reader, &mut self.error);
        self.entropy = read_region(&mut self.reader, entropy_len, &mut self.error);
        self.incompressible = read_region(&mut self.reader, incompressible_len, &mut self.error);
        self.ipos = 0;
        self.ops_left = if op_count == 0 { usize::MAX } else { op_count };
        if self.entropy.len() < STATE_BYTES {
            let mut state: State = 0;
            for &b in self.entropy.iter() {
                state = state << 8 | State::from(b);
            }
            self.state = state;
            self.epos = self.entropy.len();
        } else {
            self.state = State::from_be_bytes(self.entropy[0..STATE_BYTES].try_into().unwrap());
            self.epos = STATE_BYTES;
        }
    }

    /// Return `value` unless a read error was latched during decoding.
    pub(crate) fn into_result<T>(mut self, value: T) -> std::io::Result<T> {
        match self.error.take() {
            Some(e) => Err(e),
            None => Ok(value),
        }
    }
}

impl<R: std::io::Read> SymbolDecoder for AnsDecoder<R> {
    const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;

    #[inline]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        if self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state;
        let mut slice: &[u8] = &self.entropy[self.epos.min(self.entropy.len())..];
        let before = slice.len();
        let slot = state & (SymbolRange::M - 1);
        let (range, value) = walk(slot);
        state = range.width() * (state >> SymbolRange::BITS) + (slot - range.start());
        while state < (1 << (State::BITS - 8)) {
            let Some((&byte, rest)) = slice.split_first() else {
                break;
            };
            slice = rest;
            state = (state << 8) | byte as State;
        }
        let consumed = before - slice.len();
        self.epos += consumed;
        self.state = state;
        self.ops_left = self.ops_left.saturating_sub(1);
        value
    }
}

impl<R: std::io::Read> EntropyDecoder for AnsDecoder<R> {
    #[inline]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    #[inline]
    fn decode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N]) -> [bool; N] {
        if self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state;
        let mut slice: &[u8] = &self.entropy[self.epos.min(self.entropy.len())..];
        let before = slice.len();
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut slice, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        let consumed = before - slice.len();
        self.epos += consumed;
        self.state = state;
        self.ops_left = self.ops_left.saturating_sub(N);
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(&mut self, out: &mut [u8]) -> Result<(), std::io::Error> {
        if self.ops_left == 0 {
            self.load_next_chunk();
        }
        let start = self.ipos.min(self.incompressible.len());
        let avail_len = self.incompressible.len() - start;
        if avail_len < out.len() {
            // Prefer a latched IO error (the informative root cause) over the
            // generic truncation message.
            if let Some(e) = self.error.take() {
                return Err(e);
            }
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                avail_len,
                out.len()
            )));
        }
        out.copy_from_slice(&self.incompressible[start..start + out.len()]);
        self.ipos = start + out.len();
        self.ops_left = self.ops_left.saturating_sub(1);
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct StateOnly {
    state: State,
}
impl StateOnly {
    #[inline(always)]
    fn encode(mut self, b: bool, probability: Probability) -> (Option<u8>, Self) {
        let mut out = None;
        let ones = State::from(probability);
        let zeros = 256 - ones;
        // we use uniform of size matching the bit value to decode from state first
        let freq = if b { zeros } else { ones };
        // shift data from state to bulk when it grows too much
        if self.state >> (State::BITS - 8) >= freq {
            out = Some(self.state as u8);
            self.state >>= 8;
        }
        // the code really starts here, decode digit from freq base
        let mut z = self.state % freq;
        if b {
            z += ones;
        }
        // now encode new digit from 256 base
        (
            out,
            Self {
                state: (self.state / freq) * 256 + z,
            },
        )
    }

    /// Encode a whole tree symbol occupying `[start, start + width)` of the
    /// total `M = 2^16`. Same rANS scheme as [`StateOnly::encode`] but with a
    /// 16-bit total instead of 8, so renormalization can emit up to two bytes.
    /// Shares the bit steps' normalization interval `[2^24, 2^32)`.
    #[inline(always)]
    fn encode_symbol(mut self, start: State, width: State) -> (Bytes, Self) {
        let mut bytes = Bytes::default();
        // Emit while state >= width << 16 (kept shift-free: width can be 2^16).
        while self.state >> SymbolRange::BITS >= width {
            bytes.push(self.state as u8);
            self.state >>= 8;
        }
        self.state = ((self.state / width) << SymbolRange::BITS) | (self.state % width + start);
        (bytes, self)
    }

    /// The decode counterpart to [`StateOnly::encode_symbol`], for tests; the
    /// trait's `decode_atmost_tree` inlines this same logic.
    #[cfg(test)]
    fn decode_symbol(
        mut self,
        start: State,
        width: State,
        mut next_byte: impl FnMut() -> Option<u8>,
    ) -> Self {
        let slot = self.state & (SymbolRange::M - 1);
        debug_assert!(slot >= start && slot < start + width);
        self.state = width * (self.state >> SymbolRange::BITS) + (slot - start);
        while self.state < 1 << (State::BITS - 8) {
            let Some(byte) = next_byte() else { break };
            self.state = (self.state << 8) | State::from(byte);
        }
        self
    }

    /// The decode counterpart to [`StateOnly::encode`]. The `Ans` decoder's
    /// `decode_step` inlines this same logic; this stand-alone version exists so
    /// `check_state_only` can unit-test the encode/decode round-trip directly.
    #[cfg(test)]
    #[inline(always)]
    fn decode(
        mut self,
        probability: Probability,
        next_byte: impl FnOnce() -> Option<u8>,
    ) -> (bool, Self) {
        let ones = State::from(probability);
        let zeros = 256 - ones;
        let z = self.state & 255;
        let b = z >= ones;
        self.state >>= 8;
        // Branchless: compute both paths and select via CMOV.
        // z.wrapping_sub(ones) is only used when b=true (z >= ones), so no actual underflow.
        let state_b = (self.state * zeros).wrapping_add(z.wrapping_sub(ones));
        let state_nb = self.state * ones + z;
        self.state = if b { state_b } else { state_nb };
        if self.state < 1 << (State::BITS - 8) {
            if let Some(u) = next_byte() {
                self.state = (self.state << 8) | State::from(u);
            }
        }
        (b, self)
    }
}

#[test]
fn check_state_only() {
    for probability in (1..255).map(|i| Probability {
        prob: i.try_into().unwrap(),
    }) {
        for state in (0 as State..u16::MAX as State)
            // .chain((0..u16::MAX as State).map(|i| u32::MAX as State - i))
            // .chain((0..u16::MAX as State).map(|i| u32::MAX as State + i))
            .chain((0..u16::MAX as State).map(|i| State::MAX - i))
        {
            for b in [true, false] {
                // println!("Testing with state={state:x} probability={probability:?} bool={b}");
                let (mut next_byte, s) = StateOnly { state }.encode(b, probability);
                let next = || next_byte.take();
                let (bout, again) = s.decode(probability, next);
                assert_eq!(bout, b);
                assert_eq!(again.state, state);
                // If encoding produced a byte, then decoding must consume it.
                assert!(next_byte.is_none());
            }
        }
    }
}

#[test]
fn check_state_only_symbol() {
    // Symbol steps must round-trip from every reachable state region, for
    // every interval shape including extreme widths (the reserve-clamped
    // trees produce widths from 1 up to M/2 and starts across all of M).
    let mut cases: Vec<(State, State)> = vec![
        (0, 1),
        (65535, 1),
        (0, 65536),
        (0, 32768),
        (32768, 32768),
        (255, 256),
    ];
    for _ in 0..200 {
        let start = rand::random::<u32>() % SymbolRange::M;
        let width = 1 + rand::random::<u32>() % (SymbolRange::M - start);
        cases.push((start, width));
    }
    for &(start, width) in &cases {
        for state in (0 as State..u16::MAX as State)
            .chain((0..u16::MAX as State).map(|i| State::MAX - i))
            .step_by(97)
        {
            let (bytes, s) = StateOnly { state }.encode_symbol(start, width);
            // The encoded state's low bits are the slot, inside the interval.
            assert!(s.state & (SymbolRange::M - 1) >= start);
            let mut emitted: Vec<u8> = bytes.iter().copied().collect();
            // decode pulls in the reverse order of emission
            let again = s.decode_symbol(start, width, || emitted.pop());
            assert_eq!(
                again.state, state,
                "symbol round-trip failed for start={start} width={width} state={state:#x}"
            );
            assert!(emitted.is_empty(), "decode must consume all emitted bytes");
        }
    }
}

#[test]
fn check_ans_mixed_bits_and_symbols() {
    // Both decoder instantiations must agree on these (single-chunk) streams:
    // `false` is the fast path production picks, `true` the tracking one.
    super::check_mixed_bits_and_symbols!(Ans, Decoder::<false>::from);
    super::check_mixed_bits_and_symbols!(Ans, Decoder::<true>::from);
}

#[test]
fn check_ans_coder() {
    for size in (0..32).chain([100, 1_000, 10_000]) {
        println!("testing with size {size}");
        for _ in 0..size.min(1000) + 1000 {
            let mut data = Vec::new();
            data.resize_with(size, rand::random::<bool>);
            let mut distros = Vec::new();
            distros.resize_with(size, rand::random::<Probability>);
            let mut writer = Ans::default();
            for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
                // `Op::Bit` is the coder's bit primitive at an arbitrary
                // probability (the trait only offers context-driven encoding).
                writer.0.ops.push(Op::Bit(b, probability));
            }
            let bytes = writer.into_vec();
            let mut decoder = Decoder::<false>::from(bytes.as_slice());
            for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
                // println!("checking {b} {probability}");
                // `decode_step` is the coder's bit primitive at an arbitrary
                // probability (the trait only exposes context-driven decoding).
                let bit = decode_step(&mut decoder.state.state, &mut decoder.bytes, probability);
                assert_eq!(bit, b);
            }
            assert_eq!(decoder.state.state, 0);
        }
    }
}

#[test]
fn ans_is_reasonable() {
    let data = vec![true; 1024 * 8];
    // 8192 elements draws one collection sentinel (see `v2::sentinel`). On this
    // maximally degenerate stream the interval barely narrows per bool, so the
    // one improbable marker forces a disproportionate renormalization flush:
    // 10 -> 17 bytes. Ordinary data pays the marker's entropy and nothing more
    // (100k `u64` takes 24 markers for +16 bytes, ~0.67 B each, as predicted).
    assert_eq!(super::Range::encode(&data).len(), 17);
    assert_eq!(Ans::decode::<Vec<bool>>(&Ans::encode(&data)).unwrap(), data);
    // `Ans` pays for both, independently: 18 at the merge base, +1 for the
    // sentinel marker and +1 for this PR's chunk frame header. Note both
    // branches happened to assert 19 here for those two different reasons, so
    // the textual merge agreed on a value that was wrong for the combination.
    assert_eq!(Ans::encode(&data).len(), 20);
}

/// Count the chunk frames in an `Ans` stream (see [`Ans::flush_chunk`]), so a
/// test can confirm it actually exercised more than the single final chunk.
#[cfg(test)]
fn count_chunks(mut bytes: &[u8]) -> usize {
    let mut chunks = 0;
    while !bytes.is_empty() {
        let op_count = read_varint(&mut bytes);
        let entropy_len = read_varint(&mut bytes);
        let incompressible_len = read_varint(&mut bytes);
        bytes = &bytes[(entropy_len + incompressible_len).min(bytes.len())..];
        chunks += 1;
        if op_count == 0 {
            break; // the final chunk
        }
    }
    chunks
}

/// A value big enough to span several `CHUNK_OPS` chunks must round-trip, with
/// bits, whole symbols, and incompressible runs all crossing chunk boundaries
/// while the model contexts adapt continuously across them.
#[test]
fn multi_chunk_round_trips() {
    use crate::{Encoded, Incompressible};
    type Item = (u64, Encoded<Vec<u8>, Incompressible>);
    let mut x = 0x1234_5678_9abc_def0u64;
    let mut rng = || {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        x
    };
    // ~5 ops per item (a `u64` plus a short incompressible run) over 60k items
    // is several times `CHUNK_OPS`.
    let items: Vec<Item> = (0..60_000)
        .map(|_| {
            let len = (rng() % 5) as usize;
            let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
            (rng() % 1000, Encoded::new(run))
        })
        .collect();

    let encoded = Ans::encode(&items);
    assert!(
        count_chunks(&encoded) >= 2,
        "test should exercise multiple chunks, got {}",
        count_chunks(&encoded)
    );

    let decoded: Vec<Item> = Ans::decode(&encoded).unwrap();
    assert_eq!(decoded, items);
}

/// The streaming (`encode_to`/`decode_from`) and in-memory (`encode`/`decode`)
/// paths must be freely mix-and-matchable: byte-identical output, and either
/// decoder reads either encoder's bytes — across single- and multi-chunk sizes.
#[test]
fn streaming_matches_in_memory() {
    use crate::{Encoded, Incompressible};
    type Item = (u64, Encoded<Vec<u8>, Incompressible>);
    let mut x = 0xdead_beef_0000_1111u64;
    let mut rng = || {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        x
    };
    for &n in &[0usize, 1, 10, 60_000] {
        let items: Vec<Item> = (0..n)
            .map(|_| {
                let len = (rng() % 6) as usize;
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                (rng() % 5000, Encoded::new(run))
            })
            .collect();

        let in_memory = Ans::encode(&items);

        // encode_to (streaming) into a Vec must be byte-identical to encode.
        let mut streamed = Vec::new();
        Ans::encode_to(&items, &mut streamed).unwrap();
        assert_eq!(streamed, in_memory, "n={n}: encode_to != encode");

        // Every reader/writer pairing round-trips.
        let d1: Vec<Item> = Ans::decode(&in_memory).unwrap();
        assert_eq!(d1, items, "n={n}: decode(encode)");
        let d2: Vec<Item> = Ans::decode_from(in_memory.as_slice()).unwrap();
        assert_eq!(d2, items, "n={n}: decode_from(encode)");
        let d3: Vec<Item> = Ans::decode(&streamed).unwrap();
        assert_eq!(d3, items, "n={n}: decode(encode_to)");
        let d4: Vec<Item> = Ans::decode_from(streamed.as_slice()).unwrap();
        assert_eq!(d4, items, "n={n}: decode_from(encode_to)");
    }
}

#[cfg(test)]
mod test {
    use super::super::bit_context::BitContext;
    use super::*;

    fn rand_context() -> (BitContext, bool) {
        let value_bool = rand::random::<bool>();
        (rand::random::<BitContext>(), value_bool)
    }

    #[test]
    fn normal() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 256;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_context());
            }
            println!("\n\ntesting {probs:?}");
            let mut encoder = Ans::default();

            for &(p, bit) in &probs {
                encoder.encode_bit(&mut p.clone(), bit);
            }

            let bytes = encoder.into_vec();

            let mut decoder = Decoder::<true>::from(bytes.as_slice());

            for &(p, bit) in &probs {
                println!("Decoding before {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
        }
    }

    #[test]
    fn incompressible() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 256;
            let mut probs = Vec::new();
            let mut after_probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_context());
                after_probs.push(rand_context());
            }
            let num_inc = rand::random::<usize>() % 9;
            let mut inc = Vec::new();
            for _ in 0..num_inc {
                // Attempt to get random bytes with a wide distribution of
                // number of bits required.
                let mut num_bytes = rand::random::<usize>() % 9;
                if num_bytes == 8 {
                    num_bytes = rand::random::<usize>() % 512;
                    if num_bytes > 500 {
                        num_bytes = rand::random::<usize>() % 512_000;
                    }
                }
                let mut bytes: Vec<u8> = Vec::new();
                for _ in 0..num_bytes {
                    bytes.push(rand::random());
                }
                inc.push(bytes);
            }
            println!("\n\ntesting {probs:?}\n\n{inc:?}");
            let mut encoder = Ans::default();

            for &(p, bit) in &probs {
                encoder.encode_bit(&mut p.clone(), bit);
            }
            for bytes in &inc {
                encoder.encode_incompressible_bytes(bytes);
            }
            for &(p, bit) in &after_probs {
                encoder.encode_bit(&mut p.clone(), bit);
            }

            let bytes = encoder.into_vec();
            println!("\n\nEncoded random as: {bytes:02x?}\n");

            println!(
                "encoded ends with incompressible {:?}",
                &bytes[bytes.len() - inc.iter().map(|x| x.len()).sum::<usize>()..]
            );

            let mut decoder = Decoder::<false>::from(bytes.as_slice());

            for &(p, bit) in &probs {
                println!("Decoding before {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
            for b in &inc {
                println!("decoding {b:?}");
                let mut v = vec![0u8; b.len()];
                decoder.decode_incompressible_bytes(&mut v).unwrap();
                assert_eq!(&v, b);
            }
            for &(p, bit) in &after_probs {
                println!("Decoding after {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
        }
    }
}

/// `Op` fills the encoder's per-chunk buffer, so its size is a real memory cost.
/// A payload on `Incompressible` would force `usize` alignment and inflate every
/// `Bit`/`Symbol` entry too.
#[test]
fn r6_op_is_compact() {
    assert_eq!(std::mem::size_of::<Op>(), 6, "Op should stay 6 bytes");
    assert_eq!(
        std::mem::align_of::<Op>(),
        2,
        "Op should stay 2-byte aligned"
    );
}

#[test]
fn r1_malformed_header_must_not_panic() {
    // `Ans::decode` is `Option`-returning: malformed bytes must yield `None`
    // (or any value), never panic.
    for pattern in [0xffu8, 0x80, 0xfe] {
        for len in [1usize, 4, 16, 64] {
            let bytes = vec![pattern; len];
            let _ = Ans::decode::<u64>(&bytes);
            let _ = Ans::decode::<Vec<u64>>(&bytes);
            let _ = Ans::decode::<String>(&bytes);
        }
    }
}
