use super::atmost::{walks, AtMost, AtMostContext};
use super::bit_context::BitContext;
use super::model::{Probability, SymbolCoder, SymbolDecoder, SymbolRange};
#[cfg(feature = "stream")]
use super::AsyncEntropyDecoder;
use super::{EntropyCoder, EntropyDecoder};
mod bytebuf;
use bytebuf::Bytes;

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
/// `None` if `bytes` ran out mid-varint — a truncated header.
///
/// Malformed input must not panic: this is reached from `Ans::decode`, which is
/// `Option`-returning, for every chunk header. A continuation-bit run longer
/// than a `usize` would otherwise shift out of range (a debug-build panic), so
/// the shift is bounded exactly as in [`read_varint_io`]. An over-long run is
/// corruption rather than truncation, so it yields `Some` and is left to the
/// decode to reject.
fn read_varint(bytes: &mut &[u8]) -> Option<usize> {
    let mut v = 0usize;
    let mut shift = 0u32;
    while let Some((&b, rest)) = bytes.split_first() {
        *bytes = rest;
        v |= ((b & 0x7f) as usize) << shift;
        if b & 0x80 == 0 {
            return Some(v);
        }
        shift += 7;
        if shift >= usize::BITS {
            return Some(v); // corrupt varint: stop rather than shift out of range
        }
    }
    None
}

/// ANS entropy encoding.
///
/// Can be used to encode data.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Ans::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 7);
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
#[derive(Default)]
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

/// Summarize rather than dump: the derived `Debug` would print every recorded
/// op (up to `CHUNK_OPS` of them), every raw incompressible byte, and — for the
/// in-memory `AnsEncoder<Vec<u8>>` behind [`Ans`] — every chunk flushed so far,
/// so debug-printing a coder mid-encode would echo the whole payload. Sizes are
/// what is actually useful when debugging chunking anyway. Written by hand
/// rather than derived also drops the `W: Debug` bound, so an encoder over any
/// writer stays printable.
impl<W: std::io::Write> std::fmt::Debug for AnsEncoder<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AnsEncoder")
            .field("buffered_ops", &self.ops.len())
            .field("incompressible_bytes", &self.incompressible_bytes.len())
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
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
    type Writer = W;

    #[inline]
    fn new(writer: W) -> Self {
        Self {
            ops: Vec::new(),
            incompressible_bytes: Vec::new(),
            writer,
            error: None,
        }
    }

    /// Finish encoding: flush the final chunk, then return the sink or the
    /// latched IO error. [`Ans::into_vec`] is the in-memory caller.
    fn finish(mut self) -> std::io::Result<W> {
        self.flush_chunk(true);
        match self.error.take() {
            Some(e) => Err(e),
            None => Ok(self.writer),
        }
    }

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
    type Writer = Vec<u8>;
    #[inline]
    fn new(writer: Vec<u8>) -> Self {
        Ans(AnsEncoder::new(writer))
    }
    #[inline]
    fn finish(self) -> std::io::Result<Vec<u8>> {
        self.0.finish()
    }
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
    /// contexts are *not* reset (they adapt across chunks).
    ///
    /// The frame's leading varint is a tag whose low bit says whether this is the
    /// final chunk, so the two shapes are distinguishable before either is read:
    ///
    /// | | tag | then | layout |
    /// |---|---|---|---|
    /// | final | `raw_len * 2` | — | `[raw][entropy…EOF]` |
    /// | non-final | `op_count * 2 + 1` | `entropy_len`, `raw_len` | `[entropy][raw]` |
    ///
    /// A final chunk therefore spends **one** varint where the obvious framing
    /// spends three, which matters because every stream has exactly one final
    /// chunk and small values are *only* that chunk. `op_count` loses a bit to
    /// the tag, but a full chunk's count is 3 varint bytes either way.
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

        // `incompressible_bytes` is cleared for the next chunk anyway, so take it
        // out to write (avoids borrowing `self` while `write_out` needs `&mut`).
        let incompressible = std::mem::take(&mut self.incompressible_bytes);
        let mut header = Vec::new();
        if is_final {
            // Nothing follows this chunk, so its last region runs to the end of
            // the stream and needs no length — only the *other* region's does.
            // Raw bytes go first (their length is usually 0, so the tag varint is
            // usually a single byte) and the entropy body runs to EOF.
            push_varint(&mut header, incompressible.len() * 2);
            self.write_out(&header);
            self.write_out(&incompressible);
            self.write_out(&entropy);
        } else {
            push_varint(&mut header, self.ops.len() * 2 + 1);
            push_varint(&mut header, entropy.len());
            push_varint(&mut header, incompressible.len());
            self.write_out(&header);
            self.write_out(&entropy);
            self.write_out(&incompressible);
        }
        self.ops.clear();
    }
}

impl Ans {
    /// Decode a value from an async stream of [`Bytes`](bytes::Bytes), decoding
    /// each chunk frame as it arrives rather than waiting for the whole input.
    /// Accepts the same bytes [`Ans::encode`] produces.
    ///
    /// Overlap stops at the final chunk, whose entropy region has no length and
    /// so cannot be read before end of stream; see [`AsyncAnsDecoder`].
    #[cfg(feature = "stream")]
    pub async fn decode_stream<T, S, E>(stream: S) -> std::io::Result<T>
    where
        crate::Normal: super::DecodeAsync<T>,
        S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let mut source = super::stream::ChunkSource::new(stream).await;
        // The same fast path `Range::decode_stream` takes, and it matters more
        // here: an input already in hand has no overlap to offer, and `Ans`'s
        // slice decoder is the one that does no copying at all.
        if let Some(whole) = source.take_if_single_chunk().await {
            // `Self::decode`'s dispatch, but keeping the error rather than
            // discarding it — a stream decode reports *why* it failed.
            let bytes: &[u8] = &whole;
            let value = if read_varint(&mut { bytes }).unwrap_or(0) & 1 == 0 {
                Decoder::<false>::from(bytes).decode_value::<T, crate::Normal>()
            } else {
                Decoder::<true>::from(bytes).decode_value::<T, crate::Normal>()
            };
            return match source.take_error() {
                Some(e) => Err(e),
                None => value,
            };
        }
        AsyncAnsDecoder::from_source(source)
            .await
            .decode_value::<T, crate::Normal>()
            .await
    }

    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Encode `value` straight into a [`Write`](std::io::Write), streaming chunks
    /// out as they fill rather than buffering the whole compressed output; the
    /// bytes are **identical** to [`Ans::encode`].
    ///
    /// No buffering is applied — wrap an unbuffered sink like a `File` in a
    /// [`BufWriter`](std::io::BufWriter) yourself. (`Ans` writes each chunk's body
    /// in bulk, so it is less syscall-bound than `Range`, which writes a byte at a
    /// time.) The returned writer is flushed before return, so a final flush error
    /// surfaces here rather than being lost in a wrapping `BufWriter`'s `Drop`.
    pub fn encode_to<T: super::Encode, W: std::io::Write>(
        value: &T,
        writer: W,
    ) -> std::io::Result<()> {
        super::stream_encode::<T, AnsEncoder<W>>(value, writer)
            .and_then(|mut w| std::io::Write::flush(&mut w))
    }
    /// Decode a value straight from a [`Read`](std::io::Read), pulling one chunk
    /// at a time rather than requiring the whole compressed input in memory.
    /// Accepts the same bytes [`Ans::encode`]/[`Ans::encode_to`] produce.
    ///
    /// No buffering is applied — wrap an unbuffered source in a
    /// [`BufReader`](std::io::BufReader) yourself. (`Ans` reads only the small
    /// chunk-header varints a byte at a time and pulls each chunk body in bulk via
    /// `read_exact`, so it is less syscall-bound than `Range`, which reads a byte
    /// at a time for the whole stream.)
    pub fn decode_from<T: super::Encode, R: std::io::Read>(mut reader: R) -> std::io::Result<T> {
        // Consume the first frame's tag to pick the decoder, exactly as
        // `Ans::decode` peeks it: an even tag marks the *final* chunk, so the
        // whole value is one chunk and needs no boundary tracking. Unlike the
        // slice case we cannot peek and re-read, so the tag is handed to the
        // constructor. A read error here latches and travels with it, so an
        // unreadable stream still reports the IO error rather than a decode one.
        let mut error = None;
        let tag = read_varint_io(&mut reader, &mut error);
        if tag & 1 == 0 {
            Self::decode_from_with::<T, _, false>(reader, tag, error)
        } else {
            Self::decode_from_with::<T, _, true>(reader, tag, error)
        }
    }

    /// One arm of [`Self::decode_from`]'s dispatch; kept out of line for the
    /// same reason as [`Self::decode_with`]. The latched-IO-error-wins rule (a
    /// mid-stream read failure must not be masked by a downstream `T::decode`
    /// validation error) lives in
    /// [`decode_value`](super::EntropyDecoder::decode_value) via [`AnsDecoder`]'s
    /// `finish`.
    #[inline(never)]
    fn decode_from_with<T: super::Encode, R: std::io::Read, const CHUNKED: bool>(
        reader: R,
        tag: usize,
        error: Option<std::io::Error>,
    ) -> std::io::Result<T> {
        AnsDecoder::<R, CHUNKED>::with_first_tag(reader, tag, error)
            .decode_value::<T, crate::Normal>()
    }
    /// [`Self::decode_from`] with the chunk-boundary tracking forced rather than
    /// chosen from the first tag, so a benchmark can measure both instantiations
    /// against the same bytes in the same build. `CHUNKED = false` is the
    /// single-chunk fast path; `true` is what a multi-chunk stream gets.
    ///
    /// Only valid for a **single-chunk** stream. Forcing `false` on a multi-chunk
    /// one does not stop at the boundary and does not error: with the `ops_left`
    /// check compiled out, `load_next_chunk` never fires, the exhausted entropy
    /// buffer simply stops renormalizing, and decode runs to completion emitting
    /// silently corrupted values. Benchmark support for
    /// `src/bin/just-decompress-stream.rs`, not part of the stable API.
    #[doc(hidden)]
    #[cfg(feature = "benchmarking")]
    pub fn decode_from_forced<T: super::Encode, R: std::io::Read, const CHUNKED: bool>(
        mut reader: R,
    ) -> std::io::Result<T> {
        let mut error = None;
        let tag = read_varint_io(&mut reader, &mut error);
        Self::decode_from_with::<T, _, CHUNKED>(reader, tag, error)
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        // Peek the first frame's tag: an even tag marks the *final* chunk, so the
        // whole value lives in this one chunk and needs no boundary tracking.
        // That is the common case, and skipping the per-batch `ops_left`
        // bookkeeping is worth up to 21% of decode time — see [`Decoder`].
        // Multi-chunk streams take the tracking decoder.
        // A truncated tag decodes as 0 here, i.e. single-chunk; the decoder it
        // picks then reports the truncation itself, so the peek need not.
        let single_chunk = read_varint(&mut { bytes }).unwrap_or(0) & 1 == 0;
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
        // Via `finish`, so the truncation check is the same one every other
        // route through this decoder gets.
        Decoder::<CHUNKED>::from(bytes)
            .decode_value::<T, crate::Normal>()
            .ok()
    }
    /// Whether `Ans`'s decoder asks [`Walk::production`](super::Walk::production)
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
        // Mirror `Ans::decode`'s dispatch — including keeping the arms out of
        // line — so the benchmark measures the decoder production would
        // actually pick for these bytes.
        if read_varint(&mut { bytes }).unwrap_or(0) & 1 == 0 {
            Self::decode_atmost_batch_with::<MAX, WHICH_WALK, false>(bytes, n)
        } else {
            Self::decode_atmost_batch_with::<MAX, WHICH_WALK, true>(bytes, n)
        }
    }

    /// One arm of [`Self::decode_atmost_batch`]'s dispatch; see
    /// [`Self::decode_with`] for why this is kept out of line.
    #[inline(never)]
    #[cfg(any(test, feature = "benchmarking"))]
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
    #[cfg(feature = "benchmarking")]
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
    #[cfg(feature = "benchmarking")]
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
    /// The stream is a sequence of chunks, each opening with a varint tag whose
    /// low bit distinguishes the two frame shapes (see
    /// [`AnsEncoder::flush_chunk`]). Non-final chunks — flushed during recording
    /// once the op buffer reaches [`CHUNK_OPS`] — are
    /// `[op_count * 2 + 1][entropy-len][incompressible-len][entropy][incompressible]`.
    /// The final chunk is `[raw-len * 2][incompressible][entropy]`, its entropy
    /// running to the end of the stream: nothing follows it, so that length would
    /// be redundant. `entropy` is the chunk's self-contained rANS stream (state
    /// then body, in decode order). The contexts adapt continuously across chunk
    /// boundaries, so chunking costs only one rANS state-flush (plus the frame
    /// header) per chunk.
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
/// frame's tag is even is a single (final) chunk, which is the common
/// case; decoding it needs no boundary tracking at all, so `CHUNKED = false`
/// compiles the `ops_left` check and decrement out of the per-batch hot path
/// entirely. That is worth real time: keeping the bookkeeping unconditionally
/// measured **+21%** on a cache-resident `Vec<u64>` decode and +6% on a
/// memory-bound one. [`Ans::decode`] peeks the first frame and picks.
#[derive(Eq, PartialEq)]
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
    /// Set when a frame declared a region longer than the bytes that remain —
    /// i.e. the stream is truncated. See [`Self::saw_whole_chunks`].
    truncated: bool,
}

/// Summarize rather than dump, for the same reason as [`AnsEncoder`]'s: `bytes`,
/// `incompressible`, and `rest` are slices *into the stream being decoded*, and
/// `rest` is everything not yet consumed — so the derived `Debug` would print
/// essentially the entire input. Remaining counts are the useful part.
impl<const CHUNKED: bool> std::fmt::Debug for Decoder<'_, CHUNKED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoder")
            .field("state", &self.state)
            .field("entropy_left", &self.bytes.len())
            .field("incompressible_left", &self.incompressible.len())
            .field("stream_left", &self.rest.len())
            .field("ops_left", &self.ops_left)
            .field("truncated", &self.truncated)
            .finish()
    }
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
            truncated: false,
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
    /// Read one header varint, noting truncation if the stream ran out inside
    /// it. That includes running out *before* it: a frame is only ever read
    /// where one is due, so an empty `rest` means the stream stopped on a frame
    /// boundary — the one truncation the region-length checks cannot see, since
    /// a cut there leaves every region that *was* delivered complete.
    #[inline]
    fn take_varint(&mut self, rest: &mut &'a [u8]) -> usize {
        match read_varint(rest) {
            Some(v) => v,
            None => {
                self.truncated = true;
                0
            }
        }
    }

    #[inline]
    fn load_next_chunk(&mut self) {
        let mut rest = self.rest;
        let tag = self.take_varint(&mut rest);
        let (entropy, incompressible, rest) = if tag & 1 == 0 {
            // Final chunk: `[raw][entropy…EOF]`, so the entropy body is whatever
            // is left once the raw run is taken off the front.
            let raw_len = tag >> 1;
            self.truncated |= raw_len > rest.len();
            let (incompressible, entropy) = rest.split_at(raw_len.min(rest.len()));
            self.ops_left = usize::MAX;
            (entropy, incompressible, &rest[rest.len()..])
        } else {
            let entropy_len = self.take_varint(&mut rest);
            let incompressible_len = self.take_varint(&mut rest);
            self.truncated |= entropy_len.saturating_add(incompressible_len) > rest.len();
            let (entropy, rest) = rest.split_at(entropy_len.min(rest.len()));
            let (incompressible, rest) = rest.split_at(incompressible_len.min(rest.len()));
            // A non-final frame claiming 0 ops would re-enter this on the very
            // next op; treat it as unbounded so corrupt input cannot spin here.
            let op_count = tag >> 1;
            self.ops_left = if op_count == 0 { usize::MAX } else { op_count };
            (entropy, incompressible, rest)
        };
        self.rest = rest;
        self.incompressible = incompressible;
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

    /// Whether every chunk entered so far was present in full.
    ///
    /// A frame header declares its region lengths, so a truncated stream is
    /// detectable: the declared bytes simply are not there. Without this check
    /// [`load_next_chunk`](Self::load_next_chunk) clamps each region to what is
    /// left and the rANS decode zero-pads past the end, which usually yields a
    /// *plausible* short value rather than a failure — so a half-delivered
    /// stream decodes quietly to the wrong thing.
    ///
    /// Chunks are entered lazily, so this reports on the prefix actually read,
    /// not on the whole input; trailing bytes past the decoded value are
    /// ignored, as they always have been.
    ///
    /// The one truncation this cannot see is a short tail of the **final**
    /// chunk's entropy body, which carries no length field (it runs to end of
    /// stream — see [`AnsEncoder::flush_chunk`]). The streaming decoder is blind
    /// to it for the same reason, so the two agree.
    #[inline]
    fn saw_whole_chunks(&self) -> bool {
        !self.truncated
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
    type Reader = &'a [u8];

    #[inline]
    fn new(bytes: &'a [u8]) -> Self {
        Self::from(bytes)
    }

    /// The slice decoder reads no IO, but it does notice a buffer that ran out
    /// inside a frame — see [`Self::saw_whole_chunks`], which is why the check
    /// lives here rather than in one caller: every route through this decoder
    /// gets it, including [`Ans::decode_stream`]'s fast path.
    ///
    /// Truncation takes precedence over whatever the decode itself returned,
    /// since the missing frame is what fabricated the bits that tripped it.
    #[inline]
    fn finish(self) -> std::io::Result<()> {
        if self.saw_whole_chunks() {
            Ok(())
        } else {
            Err(truncated_stream())
        }
    }

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

/// Read one byte from `reader`, reporting a clean EOF as `None` and — once an
/// error is latched — never touching `reader` again (fabricating 0s), mirroring
/// the slice decoder's zero-padding past the end of a chunk.
#[inline]
fn read_one_byte_io<R: std::io::Read>(
    reader: &mut R,
    error: &mut Option<std::io::Error>,
) -> Option<u8> {
    if error.is_some() {
        return Some(0);
    }
    let mut buf = [0u8; 1];
    loop {
        match reader.read(&mut buf) {
            Ok(0) => return None,
            Ok(_) => return Some(buf[0]),
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => {
                *error = Some(e);
                return Some(0);
            }
        }
    }
}

/// The error every decoder reports for a stream that ran out inside a chunk
/// frame — whether short of a declared region or short of the whole frame. Its
/// own function because three decoders — slice, reader, and async — have to
/// agree on it; see [`truncated_at_a_frame_boundary`].
fn truncated_stream() -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::UnexpectedEof,
        "truncated stream: a chunk frame is missing or incomplete",
    )
}

/// Read a LEB128 varint (the chunk-frame header encoding) from `reader`.
///
/// Every caller reads a header at a point where a frame is due, so end of stream
/// *here* is truncation rather than a clean finish — and it is the one
/// truncation [`read_region`]'s short-read check cannot see, because a stream cut
/// exactly on a frame boundary leaves every region it did deliver complete. The
/// slice decoder makes the same judgement in [`Decoder::load_next_chunk`].
fn read_varint_io<R: std::io::Read>(reader: &mut R, error: &mut Option<std::io::Error>) -> usize {
    let mut v = 0usize;
    let mut shift = 0u32;
    loop {
        let Some(b) = read_one_byte_io(reader, error) else {
            error.get_or_insert_with(truncated_stream);
            break;
        };
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
/// zero-pads / reports insufficient bytes, and `finish` surfaces the error.
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

/// The most entropy bytes a single chunk can legitimately hold.
///
/// A chunk is flushed once the op buffer reaches [`CHUNK_OPS`], and one
/// `encode_bits::<N>` batch can overshoot by at most `N`. Every `encode_bits`
/// call site in the repo passes `N = 1` — batching so far is decode-side only
/// (`micro-batch` goes up to 16, but only through `decode_bits`) — so today the
/// overshoot is zero; the slack below is deliberately generous against future
/// encode-side batching. A bit op emits at most one byte and a symbol op at
/// most two, plus the [`STATE_BYTES`] flush.
///
/// This exists because the *final* chunk's entropy body has no length field —
/// it runs to end of stream (see [`AnsEncoder::flush_chunk`]) — so the streaming
/// reader would otherwise buffer whatever a stream chose to append.
const MAX_CHUNK_ENTROPY: usize = 2 * (CHUNK_OPS + 256) + STATE_BYTES;

/// Read the rest of `reader` — the final chunk's entropy body, which carries no
/// length — capped at [`MAX_CHUNK_ENTROPY`] so a stream cannot make the decoder
/// buffer without bound. Exceeding the cap means the frame is corrupt.
fn read_final_region<R: std::io::Read>(
    reader: &mut R,
    error: &mut Option<std::io::Error>,
) -> Vec<u8> {
    use std::io::Read;
    if error.is_some() {
        return Vec::new();
    }
    let mut out = Vec::new();
    // One byte past the cap, so hitting it is distinguishable from filling it.
    if let Err(e) = reader
        .take(MAX_CHUNK_ENTROPY as u64 + 1)
        .read_to_end(&mut out)
    {
        *error = Some(e);
        return Vec::new();
    }
    if out.len() > MAX_CHUNK_ENTROPY {
        *error = Some(std::io::Error::other(
            "corrupt stream: final chunk exceeds the maximum entropy size",
        ));
        return Vec::new();
    }
    out
}

/// Streaming ANS decoder: pulls one chunk frame at a time from `R` rather than
/// indexing a whole slice, so decoding a large value need only hold one chunk's
/// entropy + incompressible bytes at once. Reads the same bytes
/// [`Ans`]/[`AnsEncoder`] produce and recovers identical values (the per-chunk
/// arithmetic is the slice [`Decoder`]'s; only the byte source differs). IO
/// errors are latched and surfaced by [`AnsDecoder::finish`].
///
/// `CHUNKED` plays exactly the role it does on [`Decoder`]: an even first tag
/// means the whole value is one final chunk, so `CHUNKED = false` compiles the
/// per-op `ops_left` check and decrement out of the hot paths.
/// [`Ans::decode_from`] reads the first tag and picks.
/// Public only because it names [`AsyncAnsDecoder`]'s `Sync` associated type;
/// every field is private and there is no way to build one from outside.
pub struct AnsDecoder<R: std::io::Read, const CHUNKED: bool = true> {
    reader: R,
    state: State,
    /// Current chunk's entropy region (its leading `STATE_BYTES` seed the state).
    entropy: Vec<u8>,
    /// How much of `entropy` is consumed.
    ///
    /// Invariant: `epos <= entropy.len()`, so the hot paths can index
    /// `entropy[epos..]` without clamping. `enter_chunk` sets it to at most the
    /// new region's length, and each step advances it to
    /// `entropy.len() - remaining.len()` for a suffix `remaining` — never past
    /// the end. Clamping here instead cost ~2.7% of streaming decode.
    epos: usize,
    /// Current chunk's raw incompressible region.
    incompressible: Vec<u8>,
    ipos: usize,
    /// Ops left in the current chunk; `usize::MAX` for the final chunk.
    /// Only read/written when `CHUNKED`.
    ops_left: usize,
    error: Option<std::io::Error>,
}

impl<R: std::io::Read, const CHUNKED: bool> AnsDecoder<R, CHUNKED> {
    /// Build a decoder positioned just past the first chunk's `tag`, which the
    /// caller has already consumed from `reader` in order to choose `CHUNKED`
    /// (and which may have latched `error` doing so).
    pub(crate) fn with_first_tag(reader: R, tag: usize, error: Option<std::io::Error>) -> Self {
        let mut decoder = AnsDecoder {
            reader,
            state: 0,
            entropy: Vec::new(),
            epos: 0,
            incompressible: Vec::new(),
            ipos: 0,
            ops_left: 0,
            error,
        };
        decoder.enter_chunk(tag);
        decoder
    }

    /// Read the next chunk frame's tag and enter it.
    ///
    /// Unreachable when `!CHUNKED` — the sole chunk is final, so `ops_left`
    /// stays `usize::MAX` and no caller ever asks for another chunk.
    fn load_next_chunk(&mut self) {
        let tag = read_varint_io(&mut self.reader, &mut self.error);
        self.enter_chunk(tag);
    }

    /// Enter the chunk whose frame `tag` opens: pull the entropy and
    /// incompressible regions into owned buffers, seed the rANS state from the
    /// entropy's leading bytes, and set `ops_left`.
    fn enter_chunk(&mut self, tag: usize) {
        if tag & 1 == 0 {
            // Final chunk: `[raw][entropy…EOF]` (see `AnsEncoder::flush_chunk`).
            self.incompressible = read_region(&mut self.reader, tag >> 1, &mut self.error);
            self.entropy = read_final_region(&mut self.reader, &mut self.error);
            self.ops_left = usize::MAX;
        } else {
            let entropy_len = read_varint_io(&mut self.reader, &mut self.error);
            let incompressible_len = read_varint_io(&mut self.reader, &mut self.error);
            self.entropy = read_region(&mut self.reader, entropy_len, &mut self.error);
            self.incompressible =
                read_region(&mut self.reader, incompressible_len, &mut self.error);
            // A non-final frame claiming 0 ops would re-enter this on the very
            // next op; treat it as unbounded so corrupt input cannot spin here.
            let op_count = tag >> 1;
            self.ops_left = if op_count == 0 { usize::MAX } else { op_count };
        }
        self.ipos = 0;
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
}

impl<R: std::io::Read, const CHUNKED: bool> SymbolDecoder for AnsDecoder<R, CHUNKED> {
    const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;

    #[inline]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        if CHUNKED && self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state;
        let mut slice: &[u8] = &self.entropy[self.epos..];
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
        self.epos = self.entropy.len() - slice.len();
        self.state = state;
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(1);
        }
        value
    }
}

impl<R: std::io::Read, const CHUNKED: bool> EntropyDecoder for AnsDecoder<R, CHUNKED> {
    type Reader = R;

    /// Read the leading chunk tag, then enter the first chunk. Note this does
    /// *not* choose `CHUNKED` from the tag (that is the caller's job in
    /// [`Ans::decode_from`], which peeks the tag before picking the type);
    /// constructing `AnsDecoder<R, CHUNKED>` directly commits to the given
    /// `CHUNKED`. See [`Self::with_first_tag`] for the pre-peeked path.
    fn new(mut reader: R) -> Self {
        let mut error = None;
        let tag = read_varint_io(&mut reader, &mut error);
        Self::with_first_tag(reader, tag, error)
    }

    /// Fails with any read error latched during decoding; a frame that ran out
    /// mid-read latches [`truncated_stream`] rather than setting a separate flag.
    fn finish(mut self) -> std::io::Result<()> {
        self.error.take().map_or(Ok(()), Err)
    }

    #[inline]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    #[inline]
    fn decode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N]) -> [bool; N] {
        if CHUNKED && self.ops_left == 0 {
            self.load_next_chunk();
        }
        let mut state = self.state;
        let mut slice: &[u8] = &self.entropy[self.epos..];
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut slice, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        self.epos = self.entropy.len() - slice.len();
        self.state = state;
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(N);
        }
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(&mut self, out: &mut [u8]) -> Result<(), std::io::Error> {
        if CHUNKED && self.ops_left == 0 {
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
        if CHUNKED {
            self.ops_left = self.ops_left.saturating_sub(1);
        }
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
    // 8192 elements draws collection sentinels (see `v2::sentinel`), but on
    // this stream they are nearly free: a marker is a `true` in a context that
    // has long since saturated, so it costs ~5 millibits (measured: 4096
    // elements price out 5 mb above 4095). These 10 bytes are essentially the
    // bools themselves — `Millibits` puts the whole vector at 73.2 bits, i.e.
    // 9.2 of the 10, the rest being the coder's terminating flush.
    //
    // Nor is the per-bool cost the floor: 73213/8192 averages 8.9 millibits,
    // well above `BitContext`'s 1/256 floor of ~5.6, because the context has
    // to climb the adaptation ramp first — the first 1000 values alone cost
    // 29.1 of the 73.2 bits.
    assert_eq!(super::Range::encode(&data).len(), 10);
    assert_eq!(Ans::decode::<Vec<bool>>(&Ans::encode(&data)).unwrap(), data);
    // `Ans` additionally carries a chunk frame (see `Ans::flush_chunk`), which
    // `Range` does not — that framing is the bulk of the gap between these two
    // numbers. Measured for this combination rather than derived: the deeper
    // `MAX_PRODUCT` floor and the chunk format each move it, so composing the
    // two branches' arithmetic gives the wrong answer.
    assert_eq!(Ans::encode(&data).len(), 12);
}

/// Count the chunk frames in an `Ans` stream (see [`Ans::flush_chunk`]), so a
/// test can confirm it actually exercised more than the single final chunk.
#[cfg(test)]
fn count_chunks(bytes: &[u8]) -> usize {
    frame_starts(bytes).len() - 1
}

/// Walk the frames of a well-formed `Ans` stream, returning the offset at which
/// each one starts — plus a last entry for where the final chunk's entropy body
/// begins, which is one past the end of the last frame's declared bytes.
///
/// That last offset is where truncation stops being detectable: every other
/// region has a declared length, so a short one is caught, but the final entropy
/// body runs to end of stream and a cut inside it is indistinguishable from a
/// shorter encoding. The earlier offsets are the frame boundaries, the cuts that
/// leave every delivered region complete and so need a check of their own.
#[cfg(test)]
fn frame_starts(bytes: &[u8]) -> Vec<usize> {
    let total = bytes.len();
    let mut starts = Vec::new();
    let mut bytes = bytes;
    while !bytes.is_empty() {
        starts.push(total - bytes.len());
        let tag = read_varint(&mut bytes).expect("truncated frame tag");
        if tag & 1 == 0 {
            // The final chunk: raw run, then entropy to end of stream.
            bytes = &bytes[(tag >> 1).min(bytes.len())..];
            break;
        }
        let entropy_len = read_varint(&mut bytes).expect("truncated entropy length");
        let incompressible_len = read_varint(&mut bytes).expect("truncated incompressible length");
        bytes = &bytes[(entropy_len + incompressible_len).min(bytes.len())..];
    }
    starts.push(total - bytes.len());
    starts
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

/// A stream cut short must be **rejected**, not quietly decoded into a plausible
/// shorter value — the failure mode a half-delivered download produces.
///
/// Every region but one declares its length in the frame header, so the decoder
/// can tell the bytes are missing. The exception is the final chunk's entropy
/// body, which runs to end of stream and so cannot be distinguished from a
/// shorter encoding; `walk_frames` says where that begins, and everything before
/// it must fail. Both decoders are checked, since `Decoder` and `AnsDecoder`
/// arrive at the answer by different routes (a `truncated` flag vs. a latched
/// short read) and it would be easy to fix one and not the other.
#[test]
fn truncation_is_rejected_rather_than_decoded_short() {
    let value: Vec<u64> = (0..20_000).map(|i| i * 2_654_435_761).collect();
    let encoded = Ans::encode(&value);
    let starts = frame_starts(&encoded);
    assert!(
        starts.len() >= 3,
        "test wants a multi-chunk stream, got {} chunk(s)",
        starts.len() - 1
    );
    let final_entropy = *starts.last().unwrap();
    assert!(final_entropy < encoded.len());

    // Sample rather than sweep: each cut is a whole decode. The three explicit
    // cuts pin the boundary itself, which is where an off-by-one would hide.
    let step = encoded.len() / 32;
    for cut in (0..encoded.len()).step_by(step).chain([
        final_entropy - 1,
        final_entropy,
        encoded.len() - 1,
    ]) {
        let prefix = &encoded[..cut];
        let slice = Ans::decode::<Vec<u64>>(prefix);
        let reader = Ans::decode_from::<Vec<u64>, _>(prefix);
        if cut < final_entropy {
            assert!(
                slice.is_none(),
                "cut={cut}: a truncated frame decoded to a value of length {:?}",
                slice.map(|v: Vec<u64>| v.len())
            );
            assert!(
                reader.is_err(),
                "cut={cut}: the reader decoder accepted a truncated frame"
            );
        } else {
            // Past the last declared length there is nothing left to check, so
            // the requirement is only that the two agree with each other.
            assert_eq!(
                slice.is_some(),
                reader.is_ok(),
                "cut={cut}: slice and reader decoders disagree"
            );
        }
    }

    assert_eq!(Ans::decode::<Vec<u64>>(&encoded), Some(value));
}

/// The truncation that the region-length checks alone cannot see: a stream cut
/// **exactly on a frame boundary**, so every region it delivered is complete and
/// only the absence of the next frame gives it away.
///
/// Worth its own test because it is the plausible one — frames are tens of
/// kilobytes, and a transport that stops between them is ordinary — and because
/// each decoder had to be taught it separately: an empty `rest` for the slice
/// decoder, `read_varint_io` hitting EOF for the reader one.
#[test]
fn truncated_at_a_frame_boundary() {
    let value: Vec<u64> = (0..20_000).map(|i| i * 2_654_435_761).collect();
    let encoded = Ans::encode(&value);
    let starts = frame_starts(&encoded);
    assert!(starts.len() >= 3, "test wants a multi-chunk stream");

    // Every boundary but offset 0 leaves at least one whole frame decoded, so
    // the decoder gets genuinely under way before running out. Offset 0 is the
    // degenerate case — an empty input, which is no encoding at all.
    for &cut in &starts[..starts.len() - 1] {
        let prefix = &encoded[..cut];
        assert!(
            Ans::decode::<Vec<u64>>(prefix).is_none(),
            "cut={cut}: the slice decoder accepted a stream ending on a frame boundary"
        );
        let err = Ans::decode_from::<Vec<u64>, _>(prefix)
            .expect_err("the reader decoder accepted a stream ending on a frame boundary");
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof, "{err}");
    }
}

/// The streaming (`encode_to`/`decode_from`) and in-memory (`encode`/`decode`)
/// paths must be freely mix-and-matchable: byte-identical output, and either
/// decoder reads either encoder's bytes — across single- and multi-chunk sizes.
///
/// The chunk-count assertion is what makes this cover all four decoder
/// instantiations: both `Decoder<CHUNKED>` and both `AnsDecoder<_, CHUNKED>`.
/// Without it, a change that pushed every `n` here to one side would silently
/// leave two of them untested.
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
        let chunks = count_chunks(&in_memory);
        if n == 60_000 {
            assert!(chunks >= 2, "n={n} should be multi-chunk, got {chunks}");
        } else {
            assert_eq!(chunks, 1, "n={n} should be single-chunk");
        }

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

/// The "latched IO error wins" rule lives in [`EntropyDecoder::decode_value`],
/// fed by [`AnsDecoder::finish`], where the round-trip tests above never
/// exercise it. This covers all three things the consolidation put on that path:
///  - a construction-time latch (first chunk tag unreadable) beating a downstream
///    `Err`, at the trait method directly;
///  - a read failure **mid-decode, inside `load_next_chunk`** of a genuine
///    multi-chunk stream — the exact path the old inline logic guarded; and
///  - that the surfaced error is **specifically the latched IO error**, not a
///    decode validation symptom fabricated from the zero-padded tail.
#[cfg(test)]
#[test]
fn ans_decode_from_surfaces_latched_read_error() {
    use crate::{Encoded, Incompressible};

    /// A `Read` that delivers `data[..fail_after]` and then errors on every
    /// further call, modelling a stream that dies partway through.
    struct FailAfter {
        data: Vec<u8>,
        pos: usize,
        fail_after: usize,
    }
    impl std::io::Read for FailAfter {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = buf.len().min(self.fail_after.saturating_sub(self.pos));
            if n == 0 {
                return Err(std::io::Error::other("transient read failure"));
            }
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }

    // (a) A construction-time latch is reported, at the trait method. That it
    // then beats a downstream `Err` is `decode_value`'s rule, covered by
    // `arith`'s `decode_value_prefers_latched_error_over_downstream`.
    let decoder = AnsDecoder::<FailAfter, true>::new(FailAfter {
        data: Vec::new(),
        pos: 0,
        fail_after: 0,
    });
    let err = EntropyDecoder::finish(decoder)
        .expect_err("a read error latched during construction must be reported");
    assert!(
        err.to_string().contains("transient read failure"),
        "the latched IO error must be the one reported, got: {err}"
    );

    // (b) Mid-decode failure inside `load_next_chunk` of a real multi-chunk
    // stream (same shape as `multi_chunk_round_trips`). Cutting the reader off
    // exactly at the first chunk boundary lets the first chunk decode cleanly and
    // lands the failure when the decoder reads the *second* chunk's frame.
    type Item = (u64, Encoded<Vec<u8>, Incompressible>);
    let mut x = 0x1234_5678_9abc_def0u64;
    let mut rng = || {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        x
    };
    let items: Vec<Item> = (0..60_000)
        .map(|_| {
            let len = (rng() % 5) as usize;
            let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
            (rng() % 1000, Encoded::new(run))
        })
        .collect();
    let encoded = Ans::encode(&items);
    assert!(count_chunks(&encoded) >= 2, "need a multi-chunk stream");

    // Byte length of the first (non-final) chunk: its 3-varint header plus both
    // region bodies (the frame walk `count_chunks` uses).
    let mut p: &[u8] = &encoded;
    let tag = read_varint(&mut p).expect("truncated frame tag");
    assert!(
        tag & 1 == 1,
        "first chunk must be non-final in a multi-chunk stream"
    );
    let entropy_len = read_varint(&mut p).expect("truncated entropy length");
    let incompressible_len = read_varint(&mut p).expect("truncated incompressible length");
    let header_len = encoded.len() - p.len();
    let first_chunk_end = header_len + entropy_len + incompressible_len;

    let reader = FailAfter {
        data: encoded.clone(),
        pos: 0,
        fail_after: first_chunk_end,
    };
    let err = Ans::decode_from::<Vec<Item>, _>(reader)
        .expect_err("a read failure entering the second chunk must surface as Err");
    assert!(
        err.to_string().contains("transient read failure"),
        "decode_from must report the latched IO error, not a fabricated decode \
         symptom, got: {err}"
    );
}

/// R4, `Ans` side: mirror of `arith`'s `encode_to_surfaces_buffered_flush_error`.
/// `Ans::encode_to` must flush the caller's writer so a wrapping `BufWriter`'s
/// final flush error surfaces here rather than being swallowed by its `Drop`.
#[cfg(test)]
#[test]
fn ans_encode_to_surfaces_buffered_flush_error() {
    struct FlushFails;
    impl std::io::Write for FlushFails {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Err(std::io::Error::other("flush failed (e.g. ENOSPC)"))
        }
    }
    let writer = std::io::BufWriter::new(FlushFails);
    let result = Ans::encode_to(&vec![5u64, 4, 3, 2, 1], writer);
    assert!(
        result.is_err(),
        "Ans::encode_to must surface the wrapped BufWriter's final flush error"
    );
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

/// Neither coder may echo its payload when debug-printed: the encoder buffers up
/// to `CHUNK_OPS` ops plus every flushed chunk, and the decoder holds slices of
/// the whole input. Both must summarize instead.
#[test]
fn debug_summarizes_rather_than_dumping() {
    use super::Encode;
    let big: Vec<u64> = (0..50_000).collect();

    let mut coder = Ans::default();
    big.encode(&mut coder, &mut <Vec<u64> as Encode>::Context::default());
    let shown = format!("{coder:?}");
    assert!(
        shown.len() < 300,
        "encoder Debug should summarize, not dump {} recorded ops; got {} chars: {shown}",
        coder.0.ops.len(),
        shown.len()
    );
    assert!(shown.contains("buffered_ops"), "got {shown}");

    let encoded = Ans::encode(&big);
    assert!(encoded.len() > 300, "want an input worth truncating");
    let decoder = Decoder::<true>::from(encoded.as_slice());
    let shown = format!("{decoder:?}");
    assert!(
        shown.len() < 300,
        "decoder Debug should summarize, not dump the {}-byte stream; got {} chars: {shown}",
        encoded.len(),
        shown.len()
    );
    assert!(shown.contains("stream_left"), "got {shown}");
}

#[test]
fn r1_malformed_header_must_not_panic() {
    // `Ans::decode` is `Option`-returning: malformed bytes must yield `None`
    // (or any value), never panic.
    //
    // `decode_from` is covered alongside it because the two decoders fail
    // differently. `AnsDecoder` indexes `entropy[epos..]` unclamped, relying on
    // the documented `epos <= entropy.len()` invariant, and derives its cursor
    // by subtracting lengths — so a break in that invariant surfaces here as a
    // slice-index panic or a subtraction overflow, on exactly the malformed
    // input most likely to expose it. It returns `Result`, so an `Err` is fine;
    // only a panic is a failure.
    for pattern in [0xffu8, 0x80, 0xfe] {
        for len in [1usize, 4, 16, 64] {
            let bytes = vec![pattern; len];
            let _ = Ans::decode::<u64>(&bytes);
            let _ = Ans::decode::<Vec<u64>>(&bytes);
            let _ = Ans::decode::<String>(&bytes);
            let _ = Ans::decode_from::<u64, _>(bytes.as_slice());
            let _ = Ans::decode_from::<Vec<u64>, _>(bytes.as_slice());
            let _ = Ans::decode_from::<String, _>(bytes.as_slice());
        }
    }
}

/// The whole point of the two-shape frame: a value with no raw run pays a single
/// tag byte of framing, not three varints. These are the smallest encodings the
/// format produces, so a regression here means the final-chunk frame grew back.
#[test]
fn final_frame_costs_one_byte() {
    // Tag only — the entropy body is empty and runs to end of stream.
    assert_eq!(Ans::encode(&false).len(), 1);
    assert_eq!(Ans::encode(&Vec::<u64>::new()).len(), 1);
    // Tag plus a one-byte body.
    assert_eq!(Ans::encode(&true).len(), 2);

    // ...and they still round trip, including through the streaming reader.
    for bytes in [Ans::encode(&false), Ans::encode(&true)] {
        let want = bytes.len() == 2;
        assert_eq!(Ans::decode::<bool>(&bytes), Some(want));
        assert_eq!(Ans::decode_from::<bool, _>(bytes.as_slice()).unwrap(), want);
    }
    assert_eq!(
        Ans::decode::<Vec<u64>>(&Ans::encode(&Vec::<u64>::new())),
        Some(vec![])
    );
}

/// The final chunk's entropy body has no length field, so the streaming reader
/// reads it to EOF — capped, or a stream could make it buffer without bound.
#[test]
fn oversized_final_chunk_is_rejected() {
    let mut bytes = vec![0u8]; // tag 0: final chunk, empty raw run
    bytes.resize(1 + MAX_CHUNK_ENTROPY + 16, 0xab);
    let err = Ans::decode_from::<Vec<u64>, _>(bytes.as_slice())
        .expect_err("a final chunk past the cap must be rejected, not buffered");
    // Assert on the *cap's* own message: this much garbage would make `Vec<u64>`
    // decode fail for unrelated reasons anyway, so a bare `is_err` would pass
    // even with no cap at all.
    assert!(
        err.to_string().contains("exceeds the maximum entropy size"),
        "expected the size cap to reject this, got: {err}"
    );
    // The slice decoder indexes rather than buffering, so it is not at risk and
    // must still merely fail to produce a value rather than panic.
    assert_eq!(Ans::decode::<Vec<u64>>(&bytes), None);
}

// ============================ async streaming decode ============================

/// A [`Read`](std::io::Read) over chunk frames the async layer has already
/// pulled from the stream whole.
///
/// [`AnsDecoder`] touches its reader **only** inside `enter_chunk`, so buffering
/// whole frames here lets the entire sync decoder — hot loops, walks and all —
/// run unchanged on the async path. Reads past the end return 0, which
/// `read_region`/`read_varint_io` already treat as truncation.
#[cfg(feature = "stream")]
#[derive(Default)]
/// Public for the same reason as [`AnsDecoder`], and equally opaque.
pub struct FrameBuffer {
    bytes: Vec<u8>,
    pos: usize,
    /// End offset of each appended frame, so the async side can tell how many
    /// complete frames the sync side has not entered yet.
    ends: std::collections::VecDeque<usize>,
}

#[cfg(feature = "stream")]
impl FrameBuffer {
    /// Whether a complete frame is buffered that `enter_chunk` has not consumed
    /// yet. Checked before every op, so it reads the last frame's end offset
    /// rather than scanning: frames are appended and entered in order, so one
    /// comparison answers it.
    #[inline]
    fn has_unentered(&self) -> bool {
        self.ends.back().is_some_and(|&end| end > self.pos)
    }

    /// Append one complete frame, first dropping whatever the sync side has
    /// finished with so the buffer does not grow with the stream.
    fn push_frame(&mut self, frame: &[u8]) {
        // `enter_chunk` reads a frame in full, so the cursor always sits on a
        // frame boundary and everything behind it is dead.
        let dead = self.pos;
        while self.ends.front().is_some_and(|&end| end <= dead) {
            self.ends.pop_front();
        }
        if dead > 0 {
            self.bytes.drain(..dead);
            self.pos = 0;
            for end in self.ends.iter_mut() {
                *end -= dead;
            }
        }
        self.bytes.extend_from_slice(frame);
        self.ends.push_back(self.bytes.len());
    }
}

#[cfg(feature = "stream")]
impl std::io::Read for FrameBuffer {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let n = out.len().min(self.bytes.len() - self.pos);
        out[..n].copy_from_slice(&self.bytes[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

/// How few ops may be left in the current chunk before [`AsyncAnsDecoder`] will
/// suspend to buffer the next frame.
///
/// It has to exceed the most ops one call through the decoder can consume,
/// because `read_ahead` runs once per call and not once per op.
///
/// **Today every call spends exactly one op, so this is insurance, not a live
/// constraint** — setting it to 0 passes the whole test suite. The two multi-op
/// calls are `decode_bits::<N>`, where every *encode* site passes `N = 1` and
/// the encoder flushes between batches, and a bitwise `AtMost` walk, which
/// `Walk::production` selects only for `MAX == 1` (one bit) or
/// `MAX >= SymbolRange::M` — and the latter needs an `[BitContext; MAX]` context
/// that does not survive const evaluation at that size.
///
/// 256 covers the bound either would have if that changed (a walk is one op per
/// tree level, so `usize::BITS`), for 0.4% of a chunk's `CHUNK_OPS` worth of
/// overlap. `ops_margin_covers_the_widest_call` pins both facts.
///
/// Being wrong here fails loudly rather than silently: the sync decoder's
/// `load_next_chunk` would find no frame in the buffer, and `read_varint_io`
/// reports that as [`truncated_stream`].
#[cfg(feature = "stream")]
const OPS_MARGIN: usize = 256;

/// Ops between polls of the transport while the decoder is working through a
/// buffered frame.
///
/// `read_ahead` runs once per op and polling a `Stream` is far from free, so the
/// pump is throttled rather than run every time. 1024 gives 64 chances to notice
/// an arrival within one [`CHUNK_OPS`] frame — ample, since acting on it any
/// time before the frame's ops run out is equally good — for 0.1% of the ops.
#[cfg(feature = "stream")]
const PUMP_INTERVAL: u32 = 1024;

/// Decodes [`Ans`]'s format from a stream of [`Bytes`](bytes::Bytes), one chunk
/// frame at a time.
///
/// Where [`AsyncRangeDecoder`](super::arith::AsyncRangeDecoder) must be able to
/// suspend on every byte — `Range` delay-interleaves its incompressible bytes,
/// so its output is decodable as it arrives — `Ans` stores each chunk's
/// incompressible bytes in a **separate region after** the entropy region. A
/// chunk's very first op can therefore need a byte from the end of the frame,
/// so nothing in a frame is decodable until all of it has arrived.
///
/// That sounds worse and is mostly better: `enter_chunk` is the only place the
/// sync decoder reads at all, so suspension happens once per `CHUNK_OPS`
/// (65536) ops rather than once per byte, and every op in between runs through
/// the ordinary sync code — which is also why the walks cannot disagree with
/// the encoder here, being literally the same code rather than a twin of it.
///
/// The real cost is the **final** chunk, whose entropy region carries no length
/// and runs to end of stream (see `AnsEncoder::flush_chunk`): its bytes cannot
/// be decoded until the transfer finishes, so a value's tail never overlaps.
/// Every earlier chunk is length-prefixed and does.
#[cfg(feature = "stream")]
pub struct AsyncAnsDecoder<S> {
    source: super::stream::ChunkSource<S>,
    inner: AnsDecoder<FrameBuffer, true>,
    /// Set once the final frame is buffered. No further frame can arrive, so
    /// the sync decoder may then run to completion without blocking — which is
    /// what [`AsyncEntropyDecoder::is_final`] reports.
    reached_final: bool,
    /// Monotone lower bound on the source bytes the next frame needs; see
    /// [`Self::next_frame_has_arrived`].
    next_frame_bytes: usize,
    /// Ops until the transport is polled again; see [`PUMP_INTERVAL`].
    pump_countdown: u32,
}

#[cfg(feature = "stream")]
impl<S> std::fmt::Debug for AsyncAnsDecoder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncAnsDecoder")
            .field("reached_final", &self.reached_final)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "stream")]
impl<S, E> AsyncAnsDecoder<S>
where
    S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    /// Build a decoder over a source, entering its first chunk.
    pub(crate) async fn from_source(source: super::stream::ChunkSource<S>) -> Self {
        let mut me = Self {
            source,
            inner: AnsDecoder {
                reader: FrameBuffer::default(),
                state: 0,
                entropy: Vec::new(),
                epos: 0,
                incompressible: Vec::new(),
                ipos: 0,
                ops_left: 0,
                error: None,
            },
            reached_final: false,
            next_frame_bytes: 1,
            // Zero, so the first `read_ahead` polls rather than waiting out an
            // interval before it has any frame in reserve.
            pump_countdown: 0,
        };
        me.buffer_next_frame().await;
        // Uniform with every later chunk: the tag is in the buffer, so the sync
        // decoder's own `load_next_chunk` reads it and enters.
        me.inner.load_next_chunk();
        me.read_ahead().await;
        me
    }

    /// Have a complete unentered frame buffered *by the time* a
    /// `load_next_chunk` from inside sync code needs one — and not before.
    ///
    /// It has to be possible from inside: a multi-step `AtMost` walk can exhaust
    /// a chunk mid-symbol, and the boundary lands wherever `CHUNK_OPS` falls,
    /// not on a value boundary. What it must *not* be is eager. Buffering the
    /// next frame the moment the current one is entered awaits the whole of it
    /// — tens of kilobytes — before decoding a single op of the frame in hand,
    /// which serializes arrival and decode instead of overlapping them. At
    /// 10 MB/s that is 5.3 ms of waiting against 1.1 ms of decoding, once per
    /// frame, and it is why `Ans::decode_stream` measured `arrival + decode`.
    ///
    /// [`ops_left`](AnsDecoder::ops_left) says when the wait is actually due:
    /// the sync decoder cannot reach the next frame until the current one's ops
    /// run out, so deferring until then lets a frame's decode run while its
    /// successor arrives.
    async fn read_ahead(&mut self) {
        if self.reached_final {
            return;
        }
        // Nothing else reads the source while the sync decoder works through a
        // buffered frame, so `ready_bytes` goes stale unless we poll — and a
        // stale zero looks exactly like a transport that has fallen behind.
        // Throttled, because polling a stream is far from free and this runs
        // once per op; see `PUMP_INTERVAL`.
        if self.pump_countdown == 0 {
            self.pump_countdown = PUMP_INTERVAL;
            self.source.drain_ready().await;
            // Everything has arrived, so taking the rest cannot suspend. Worth
            // doing in one go: it sets `reached_final`, and `is_final` then lets
            // whole values decode through the sync decoder rather than op by op.
            if self.source.is_complete() {
                while !self.reached_final {
                    self.buffer_next_frame().await;
                }
                return;
            }
            // Frames that have *wholly* arrived are free to take, and taking
            // them is what gets `is_final` true — and whole values decoding
            // synchronously — as soon as the transport is ahead of us.
            while !self.reached_final && self.next_frame_has_arrived() {
                self.buffer_next_frame().await;
            }
        } else {
            self.pump_countdown -= 1;
        }
        if !self.reached_final
            && !self.inner.reader.has_unentered()
            && self.inner.ops_left <= OPS_MARGIN
        {
            // Now it must be waited for: the sync decoder is about to reach it.
            self.buffer_next_frame().await;
        }
    }

    /// Whether the whole of the next frame is already buffered, so
    /// [`Self::buffer_next_frame`] cannot suspend.
    ///
    /// Runs once per op, so the header parse is guarded by `next_frame_bytes`, a
    /// monotone lower bound on the bytes the frame needs: below it the answer is
    /// certainly no, and the check costs a subtraction and a comparison. The
    /// bound is refined on each failed peek and reset once a frame is taken.
    ///
    /// The *final* frame's entropy region has no length — it runs to end of
    /// stream — so "wholly arrived" for it means "the stream has ended", which
    /// is [`ChunkSource::is_complete`]'s branch above rather than this one.
    fn next_frame_has_arrived(&mut self) -> bool {
        let ready = self.source.ready_bytes();
        if ready < self.next_frame_bytes {
            return false;
        }
        let buffered = self.source.peek();
        let mut rest = buffered;
        let (Some(tag), ..) = (read_varint(&mut rest),) else {
            self.next_frame_bytes = ready + 1;
            return false;
        };
        if tag & 1 == 0 {
            self.next_frame_bytes = usize::MAX;
            return false;
        }
        let (Some(entropy_len), Some(incompressible_len)) =
            (read_varint(&mut rest), read_varint(&mut rest))
        else {
            self.next_frame_bytes = ready + 1;
            return false;
        };
        let header = buffered.len() - rest.len();
        let need = header + entropy_len + incompressible_len;
        if ready >= need {
            self.next_frame_bytes = 1;
            true
        } else {
            self.next_frame_bytes = need;
            false
        }
    }

    /// Pull one whole frame off the stream and append it, tag included.
    async fn buffer_next_frame(&mut self) {
        let tag = self.read_varint().await;
        let mut frame = Vec::new();
        push_varint(&mut frame, tag);
        if tag & 1 == 0 {
            // Final: `[raw][entropy…EOF]`, no length on the entropy region.
            self.reached_final = true;
            self.append_region(&mut frame, tag >> 1).await;
            self.append_rest(&mut frame).await;
        } else {
            let entropy_len = self.read_varint().await;
            let incompressible_len = self.read_varint().await;
            push_varint(&mut frame, entropy_len);
            push_varint(&mut frame, incompressible_len);
            self.append_region(&mut frame, entropy_len).await;
            self.append_region(&mut frame, incompressible_len).await;
        }
        self.inner.reader.push_frame(&frame);
    }

    /// Read a LEB128 varint off the stream; the async twin of `read_varint_io`,
    /// including its rule that end of stream inside a frame header is
    /// truncation rather than a clean finish.
    async fn read_varint(&mut self) -> usize {
        let mut v = 0usize;
        let mut shift = 0u32;
        loop {
            let Some(b) = self.source.next_byte_or_eof().await else {
                self.inner.error.get_or_insert_with(truncated_stream);
                break;
            };
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

    /// Append `len` bytes in bounded increments, so a corrupt declared length
    /// cannot drive one giant allocation — as `read_region` does. A short read
    /// leaves the region truncated, which the sync side reports.
    async fn append_region(&mut self, out: &mut Vec<u8>, len: usize) {
        let mut remaining = len;
        while remaining > 0 {
            let piece = remaining.min(1 << 16);
            let start = out.len();
            out.resize(start + piece, 0);
            if let Err(e) = self.source.read_exact(&mut out[start..]).await {
                // Latch rather than discard: `read_exact` *takes* the error out
                // of the source, so dropping it here loses it for good, and
                // `decode_value` would then report whatever the short region
                // happened to decode to — an empty `Vec`, in the test below.
                self.inner.error.get_or_insert(e);
                out.truncate(start);
                return;
            }
            remaining -= piece;
        }
    }

    /// Append everything left in the stream, for the final chunk's unbounded
    /// entropy region. Capped one byte past `MAX_CHUNK_ENTROPY`, exactly as
    /// `read_final_region` does, so "full" stays distinguishable from
    /// "overflowed".
    async fn append_rest(&mut self, out: &mut Vec<u8>) {
        let cap = out.len() + MAX_CHUNK_ENTROPY + 1;
        while out.len() < cap {
            let ready = self.source.ready_bytes();
            if ready == 0 {
                // Nothing buffered: one awaited byte tops the source up, and the
                // next pass takes the rest in bulk. This region has no length
                // field, so a clean end of stream *is* its terminator — which is
                // why the read must be one that can report end of stream rather
                // than `read_exact`, whose whole job is to call it truncation.
                let byte = self.source.next_byte_or_eof().await;
                if let Some(e) = self.source.take_error() {
                    self.inner.error.get_or_insert(e);
                    return;
                }
                match byte {
                    Some(b) => out.push(b),
                    None => return,
                }
                continue;
            }
            let want = ready.min(cap - out.len());
            let start = out.len();
            out.resize(start + want, 0);
            if let Err(e) = self.source.read_exact(&mut out[start..]).await {
                self.inner.error.get_or_insert(e);
                out.truncate(start);
                return;
            }
        }
    }
}

#[cfg(feature = "stream")]
impl<S, E> super::AsyncEntropyDecoder for AsyncAnsDecoder<S>
where
    S: futures_core::Stream<Item = Result<bytes::Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    type Sync<'a> = AnsDecoder<&'a mut FrameBuffer, true>;

    /// Nothing to hold back: the gate here is whole frames, not bytes.
    const SETTLING_BYTES: usize = 0;

    /// Always 0, which keeps the byte-counted fast path switched off. It is the
    /// wrong instrument for `Ans` — what bounds a safe sync handoff is whether
    /// another *frame* can still arrive, not how many bytes are buffered, and
    /// [`Self::is_final`] answers that exactly rather than approximately. So no
    /// `MAX_BYTES` is consulted on this path at all.
    #[inline]
    fn ready_bytes(&self) -> usize {
        0
    }

    #[inline]
    fn is_final(&self) -> bool {
        self.reached_final
    }

    /// Hands over the sync decoder itself — no state to translate, since its
    /// whole per-chunk state already lives in owned buffers and its reader
    /// holds every remaining frame once [`Self::is_final`] holds.
    #[inline]
    fn with_sync<R>(&mut self, f: impl FnOnce(&mut Self::Sync<'_>) -> R) -> R {
        // A view rather than a reborrow only because the associated type has to
        // mention `'a` for `where Self: 'a` to imply `S: 'a`. The buffers move
        // by three words each and come straight back, so this is still a
        // handoff of the same decoder rather than a fresh one: no state is
        // re-derived and none is copied.
        let mut view = AnsDecoder::<&mut FrameBuffer, true> {
            reader: &mut self.inner.reader,
            state: self.inner.state,
            entropy: std::mem::take(&mut self.inner.entropy),
            epos: self.inner.epos,
            incompressible: std::mem::take(&mut self.inner.incompressible),
            ipos: self.inner.ipos,
            ops_left: self.inner.ops_left,
            error: self.inner.error.take(),
        };
        let result = f(&mut view);
        self.inner.state = view.state;
        self.inner.entropy = view.entropy;
        self.inner.epos = view.epos;
        self.inner.incompressible = view.incompressible;
        self.inner.ipos = view.ipos;
        self.inner.ops_left = view.ops_left;
        self.inner.error = view.error;
        result
    }

    /// A stream-level error comes first, being closer to the cause than a
    /// frame-level one the sync decoder latched from the bytes it did receive.
    fn finish(mut self) -> std::io::Result<()> {
        self.source
            .take_error()
            .or_else(|| self.inner.error.take())
            .map_or(Ok(()), Err)
    }

    #[inline]
    async fn decode_bit(&mut self, ctx: &mut super::bit_context::BitContext) -> bool {
        self.read_ahead().await;
        super::EntropyDecoder::decode_bit(&mut self.inner, ctx)
    }

    #[inline]
    async fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        self.read_ahead().await;
        super::EntropyDecoder::decode_bits(&mut self.inner, contexts)
    }

    /// Delegates to the sync walk rather than reimplementing it asynchronously.
    /// The walks are **not** interchangeable — a symbol step and N bit steps
    /// narrow the coder differently and so consume different bytes — and this
    /// sidesteps the question by running the same code the encoder was matched
    /// against. The read-ahead is what makes it safe: the walk may cross a
    /// chunk boundary mid-symbol, and finds the next frame already in memory.
    #[inline]
    async fn decode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut super::atmost::AtMostContext<MAX>,
    ) -> super::atmost::AtMost<MAX> {
        self.read_ahead().await;
        super::EntropyDecoder::decode_atmost(&mut self.inner, ctx)
    }

    #[inline]
    async fn decode_incompressible_bytes(&mut self, out: &mut [u8]) -> Result<(), std::io::Error> {
        self.read_ahead().await;
        super::EntropyDecoder::decode_incompressible_bytes(&mut self.inner, out)
    }
}

#[cfg(all(test, feature = "stream"))]
mod async_tests {
    use super::*;
    use crate::v2::stream::tests::Chunks;
    use futures_executor::block_on;

    /// The point of the whole exercise: a value spanning several `Ans` frames,
    /// re-chopped at stream boundaries that have nothing to do with where the
    /// frame boundaries fall.
    #[test]
    fn multi_frame_round_trips_from_a_stream() {
        use crate::{Encoded, Incompressible};
        type Item = (u64, Encoded<Vec<u8>, Incompressible>);
        let mut x = 0x1234_5678_9abc_def0u64;
        let mut rng = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            x
        };
        // ~5 ops per item over 60k items is several times `CHUNK_OPS`, and the
        // incompressible runs exercise the separate region that forces whole
        // frames to be buffered.
        let items: Vec<Item> = (0..60_000)
            .map(|_| {
                let len = (rng() % 5) as usize;
                let bytes: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                (rng(), Encoded::new(bytes))
            })
            .collect();
        let encoded = Ans::encode(&items);
        assert_eq!(
            Ans::decode::<Vec<Item>>(&encoded).as_ref(),
            Some(&items),
            "sync decode disagrees, so the fixture is wrong"
        );
        for chunk_size in [1, 2, 7, 1000, 65536] {
            let decoded: Vec<Item> =
                block_on(Ans::decode_stream(Chunks::new(&encoded, chunk_size))).unwrap();
            assert_eq!(decoded, items, "chunk_size = {chunk_size}");
        }
    }

    /// A value small enough to be a single (final) frame: `is_final` holds from
    /// the start, so this runs entirely through the sync decoder.
    #[test]
    fn single_frame_round_trips_from_a_stream() {
        let value: Vec<String> = vec![
            "hello".to_string(),
            "héllo — ünïcode".to_string(),
            "x".repeat(300),
            String::new(),
        ];
        let encoded = Ans::encode(&value);
        for chunk_size in [1, 3, 64, 4096] {
            let decoded: Vec<String> =
                block_on(Ans::decode_stream(Chunks::new(&encoded, chunk_size))).unwrap();
            assert_eq!(decoded, value, "chunk_size = {chunk_size}");
        }
    }

    /// A stream that yields some real chunks and then fails, so the failure
    /// lands *inside* a decode rather than at construction — the distinction
    /// R5 drew about the sync path, which applies here for the same reason: the
    /// latched source error `AsyncAnsDecoder::finish` reports must beat a
    /// downstream validation error, and only a mid-stream failure exercises it.
    struct FailAfter {
        chunks: std::collections::VecDeque<::bytes::Bytes>,
    }

    impl futures_core::Stream for FailAfter {
        type Item = Result<::bytes::Bytes, std::io::Error>;
        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            std::task::Poll::Ready(Some(match self.chunks.pop_front() {
                Some(c) => Ok(c),
                None => Err(std::io::Error::other("transient stream failure")),
            }))
        }
    }

    #[test]
    fn stream_error_wins_over_a_decode_symptom() {
        let value: Vec<u64> = (0..20_000).map(|i| i * 2_654_435_761).collect();
        let encoded = Ans::encode(&value);
        // Enough to get a decode genuinely under way, then fail.
        let head = ::bytes::Bytes::copy_from_slice(&encoded[..encoded.len() / 4]);
        let source = FailAfter {
            chunks: [head].into_iter().collect(),
        };
        let err = block_on(Ans::decode_stream::<Vec<u64>, _, _>(source)).unwrap_err();
        // Asserting the *specific* error, not merely `is_err()`: a truncated
        // stream also produces a plausible-looking decode failure, so only the
        // message distinguishes "the transport broke" from "the bytes were
        // nonsense" — which is the whole point of the latching rule.
        assert!(
            err.to_string().contains("transient stream failure"),
            "expected the latched stream error, got: {err}"
        );
    }

    /// [`OPS_MARGIN`] must exceed the ops any *single* call into the decoder can
    /// spend, since `read_ahead` runs per call and not per op.
    ///
    /// Today every call spends exactly one op, so the margin is insurance rather
    /// than load-bearing — setting it to 0 passes the whole suite. This test
    /// pins the two facts that make that true, so a change to either fails here
    /// rather than silently at a frame boundary:
    ///
    /// - The only walks that spend more than one op are the bitwise ones, and
    ///   [`Walk::production`](crate::v2::Walk::production) selects those only for
    ///   `MAX == 1` (a single bit) or `MAX >= SymbolRange::M`. The latter needs
    ///   an `AtMostContext<MAX>` of `[BitContext; MAX]`, which does not survive
    ///   const evaluation at that size — so it is unreachable in practice.
    /// - Even if it became reachable, a bitwise walk is one op per tree level
    ///   and `MAX` is a `usize`, so `usize::BITS` bounds it.
    ///
    /// `decode_bits::<N>` is the other multi-op call. Every *encode* site passes
    /// `N = 1` (batching is decode-side only), and the encoder flushes between
    /// batches, so a batch never straddles two chunks.
    #[test]
    fn ops_margin_covers_the_widest_call() {
        use crate::v2::atmost::walks::Walk;
        assert!(
            OPS_MARGIN > usize::BITS as usize,
            "a bitwise walk spends one op per tree level, bounded by usize::BITS"
        );
        for max in [1usize, 2, 7, 8, 255, 256, 1000, 65535] {
            let bitwise = matches!(
                walk_for(max),
                Some(Walk::CompleteBitwise | Walk::UnevenBitwise)
            );
            assert_eq!(
                bitwise,
                max == 1,
                "MAX={max}: only a single-bit walk should be bitwise below \
                 SymbolRange::M, or `read_ahead` needs to run per op"
            );
        }
    }

    /// `Walk::production` is const-generic over `MAX`; this reaches it for a
    /// runtime value by table, which is enough for the handful of `MAX`es the
    /// test above cares about.
    fn walk_for(max: usize) -> Option<crate::v2::atmost::walks::Walk> {
        fn at<const MAX: usize>() -> Option<crate::v2::atmost::walks::Walk> {
            use crate::v2::atmost::walks::Walk;
            Walk::production::<MAX>(<Decoder<'static> as SymbolDecoder>::SPECULATES)
        }
        match max {
            1 => at::<1>(),
            2 => at::<2>(),
            7 => at::<7>(),
            8 => at::<8>(),
            255 => at::<255>(),
            256 => at::<256>(),
            1000 => at::<1000>(),
            65535 => at::<65535>(),
            _ => unreachable!("add {max} to the table"),
        }
    }

    #[test]
    fn truncation_agrees_with_the_sync_decoder() {
        let value: Vec<u64> = (0..20_000).map(|i| i * 2_654_435_761).collect();
        let encoded = Ans::encode(&value);
        // The frame boundaries among the cuts, since `AsyncAnsDecoder` reads its
        // headers through a different varint reader than either sync decoder and
        // so needed the end-of-stream rule taught to it a third time.
        let cuts = [1, 7, encoded.len() / 3, encoded.len() - 1]
            .into_iter()
            .chain(super::frame_starts(&encoded));
        for cut in cuts {
            let sync = Ans::decode::<Vec<u64>>(&encoded[..cut]);
            let asynchronous = block_on(Ans::decode_stream::<Vec<u64>, _, _>(Chunks::new(
                &encoded[..cut],
                64,
            )));
            // Strict agreement, which the two decoders can manage because they
            // detect truncation by the same rule: a frame header declares its
            // region lengths, and the bytes are either there or they are not
            // (`Decoder::saw_whole_chunks` on one side, `read_region`'s short
            // read on the other). Neither can see a short tail of the *final*
            // chunk's entropy body, which carries no length — so they agree
            // there too, by both accepting it.
            assert_eq!(
                sync.is_some(),
                asynchronous.is_ok(),
                "cut={cut}: sync and async disagree about whether truncated input decodes"
            );
            if let (Some(s), Ok(a)) = (sync, asynchronous) {
                assert_eq!(s, a, "cut={cut}: both succeeded but disagree on the value");
            }
        }
    }
}
