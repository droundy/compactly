//! Async decoding from a [`Stream`] of [`Bytes`] chunks.
//!
//! The point of this module is that decoding happens *as the bytes arrive*: the
//! decoder holds the chunk it is working through and suspends only when that
//! chunk runs out, so the decode overlaps the wait for the next chunk instead of
//! following it. Collecting the whole stream and calling
//! [`v2::decode`](super::decode) would need no library support at all.
//!
//! The entry point is [`Range::decode_stream`](super::Range::decode_stream).
//! Its input bound, `Stream<Item = Result<Bytes, E>>`, is what the ecosystem
//! already speaks:
//! `aws_sdk_s3`'s `ByteStream`, `object_store`'s `GetResult::into_stream()`, and
//! `axum`'s `Body::into_data_stream()` all match it directly.
//!
//! This module holds only the coder-independent part — [`ChunkSource`], the
//! buffer that turns a chunk stream into "one byte, awaiting if necessary". Each
//! coder's async decoder, and the entry point that drives it, live beside that
//! coder's sync counterparts (`AsyncRangeDecoder` in `arith`), which is where
//! the coder state's internals are reachable.

use std::pin::Pin;

use bytes::Bytes;
use futures_core::Stream;

/// The byte source behind an async decoder: holds the chunk currently being
/// read and awaits the next one only when that chunk is used up.
///
/// Matches the sync streaming decoders' behaviour at the edges so the two agree
/// on bad input as well as good: a clean end of stream yields zero bytes (as
/// `read_one_byte` does past EOF), and a stream error is latched rather than
/// returned, to be surfaced by `finish`.
pub(crate) struct ChunkSource<S> {
    /// Boxed rather than requiring `S: Unpin`: one allocation for a whole
    /// decode is not worth constraining what callers may pass.
    stream: Pin<Box<S>>,
    /// Everything delivered and not yet consumed, as one contiguous run.
    ///
    /// Contiguity is a requirement, not a convenience: [`Self::peek`] hands
    /// this to a sync decoder that will read up to [`Self::ready_bytes`] of it,
    /// so the two must describe the same bytes. Coalescing is what
    /// [`Self::drain_ready`] pays for that, and it pays only when it actually
    /// collected something.
    current: Bytes,
    pos: usize,
    /// The stream has said it has nothing more, or has failed: it must not be
    /// polled again.
    ///
    /// [`Stream::poll_next`] documents that polling after it returns `None`
    /// "may panic, block forever, or cause other kinds of problems", and only a
    /// `FusedStream` promises otherwise. Every poll in this module is guarded by
    /// this flag, which fuses the stream for us.
    ended: bool,
    error: Option<std::io::Error>,
    /// Whether the transport ever failed — **sticky**, unlike [`Self::error`],
    /// which [`Self::take_error`] moves out to whoever will report it.
    ///
    /// [`Self::is_complete`] must not say "the stream finished cleanly" once a
    /// failure has happened, and it cannot ask `error` that: by the time anyone
    /// checks, the error has usually been taken. That matters because
    /// `is_complete` is what `AsyncRangeDecoder::sync_capacity` consults first,
    /// and a complete source hands the sync decoder `usize::MAX` capacity — i.e.
    /// licence to run to the end of the buffer — which is exactly wrong over an
    /// incomplete one.
    failed: bool,
}

impl<S> std::fmt::Debug for ChunkSource<S> {
    /// Summarize rather than dump: the derived form would print the current
    /// chunk, i.e. part of the payload.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkSource")
            .field("buffered", &(self.current.len() - self.pos))
            .field("ended", &self.ended)
            .field("error", &self.error)
            .field("failed", &self.failed)
            .finish_non_exhaustive()
    }
}

/// How much [`ChunkSource::drain_ready`] will accumulate before it stops asking
/// for more, even when the transport has data waiting.
///
/// The drain exists so that `ready_bytes` reflects what has *arrived* rather
/// than one chunk of it, but without a limit a fast transport would be emptied
/// into memory and the streaming decoder's whole point — peak memory of the
/// value plus a bounded buffer, rather than the value plus the compressed
/// input — would be lost. Leaving the surplus in the transport's own buffer is
/// also what applies backpressure to it.
///
/// 256 KiB is chosen against the largest thing a decoder asks to have in hand
/// at once: an `Ans` chunk frame, whose entropy region runs to about 128 KB for
/// a chunk flushed at `CHUNK_OPS`, with the incompressible region beside it.
/// This is a look-ahead target rather than a limit, which is what lets it stay
/// a round number now that a frame can legitimately be larger — a chunk ends at
/// the first split point at or after `CHUNK_OPS`, so one big bounded value
/// carries it past. Such a frame simply takes more than one pass to gather. The
/// `Range` decoder needs only a value's `MAX_BYTES`, which is tiny, so it is
/// not the constraint.
const READY_TARGET: usize = 1 << 18;

impl<S, E> ChunkSource<S>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    /// Build a source and **prime it**: await the first chunk, then take
    /// whatever else the transport already has, so a decode starts with
    /// everything that has arrived rather than with one chunk of it.
    pub(crate) async fn new(stream: S) -> Self {
        let mut source = ChunkSource {
            stream: Box::pin(stream),
            current: Bytes::new(),
            pos: 0,
            ended: false,
            error: None,
            failed: false,
        };
        source.fill().await;
        source
    }

    /// Take any latched stream error, for `finish`.
    pub(crate) fn take_error(&mut self) -> Option<std::io::Error> {
        self.error.take()
    }

    /// Pull one item straight from the stream, without touching the buffer or
    /// the latched error. Uses `poll_fn` over [`Stream::poll_next`] directly so
    /// this crate needs only `futures-core`, not `futures-util`.
    async fn poll_stream(&mut self) -> Option<Result<Bytes, E>> {
        let stream = &mut self.stream;
        std::future::poll_fn(move |cx| stream.as_mut().poll_next(cx)).await
    }

    /// One chunk straight from the stream, latching a failure into `error` and
    /// marking the stream not to be polled again. `None` for either end of
    /// stream or failure.
    async fn take_from_stream(&mut self) -> Option<Bytes> {
        match self.poll_stream().await {
            None => {
                self.ended = true;
                None
            }
            Some(Ok(chunk)) => Some(chunk),
            Some(Err(e)) => {
                self.error = Some(std::io::Error::other(e));
                self.failed = true;
                self.ended = true;
                None
            }
        }
    }

    /// Take everything the transport can hand over **without suspending**, and
    /// coalesce it with whatever is still unread.
    ///
    /// This is what makes [`Self::ready_bytes`] mean "what has arrived" rather
    /// than "what is in the chunk I happen to be holding". The difference is not
    /// cosmetic: the `Range` decoder decides whether to run its fast sync path
    /// by comparing `ready_bytes` against what a decode step needs, and a
    /// producer that delivers 800-byte chunks was making that comparison fail
    /// even when the whole input had in fact arrived.
    ///
    /// Coalescing costs one allocation and one copy of the ready bytes, and only
    /// when the poll actually collected something — a transport handing over one
    /// chunk at a time as it is consumed pays nothing, and neither does one
    /// handing over the whole input as a single chunk.
    ///
    /// The final poll returning `Pending` has already registered our waker with
    /// the stream, so a transport that starts a fetch when polled has been asked
    /// for the next chunk before we need it. That is the read-ahead this used to
    /// maintain explicitly, obtained here as a side effect of asking.
    pub(crate) async fn drain_ready(&mut self) {
        if self.ended || self.error.is_some() || self.ready_bytes() >= READY_TARGET {
            return;
        }
        let unread = self.ready_bytes();
        let mut extra: Vec<Bytes> = Vec::new();
        let mut collected = 0usize;
        {
            // Borrow the stream and the two flags separately from the buffer, so
            // the closure can run while `self` still owns what the collected
            // chunks will be folded into.
            let stream = &mut self.stream;
            let (ended, error, failed) = (&mut self.ended, &mut self.error, &mut self.failed);
            std::future::poll_fn(|cx| {
                while unread + collected < READY_TARGET {
                    match stream.as_mut().poll_next(cx) {
                        std::task::Poll::Pending => break,
                        std::task::Poll::Ready(None) => {
                            *ended = true;
                            break;
                        }
                        std::task::Poll::Ready(Some(Ok(chunk))) => {
                            collected += chunk.len();
                            extra.push(chunk);
                        }
                        std::task::Poll::Ready(Some(Err(e))) => {
                            *error = Some(std::io::Error::other(e));
                            *failed = true;
                            *ended = true;
                            break;
                        }
                    }
                }
                std::task::Poll::Ready(())
            })
            .await;
        }
        if collected == 0 {
            return;
        }
        let mut buf = Vec::with_capacity(unread + collected);
        buf.extend_from_slice(&self.current[self.pos..]);
        for chunk in &extra {
            buf.extend_from_slice(chunk);
        }
        self.current = Bytes::from(buf);
        self.pos = 0;
    }

    /// Make sure at least one unread byte is in hand, awaiting the transport if
    /// there is none. Returns false at a clean end of stream or once an error is
    /// latched.
    ///
    /// The loop is for empty chunks, which a producer may legitimately emit.
    async fn fill(&mut self) -> bool {
        while self.pos == self.current.len() {
            if self.ended || self.error.is_some() {
                return false;
            }
            let Some(chunk) = self.take_from_stream().await else {
                return false;
            };
            // Nothing unread, so adopting the chunk costs no copy — which is why
            // `drain_ready` is the only place that ever coalesces.
            self.current = chunk;
            self.pos = 0;
            self.drain_ready().await;
        }
        true
    }

    /// Whether every byte of the input is now in hand — the stream has finished
    /// cleanly and what remains unread is all there will ever be.
    pub(crate) fn is_complete(&self) -> bool {
        self.ended && !self.failed
    }

    /// Bytes already buffered, decodable without awaiting anything.
    pub(crate) fn ready_bytes(&self) -> usize {
        self.current.len() - self.pos
    }

    /// Everything the source has buffered, as one borrowed slice.
    ///
    /// Handing this to a sync decoder and then [advancing](Self::advance) by
    /// whatever that decoder consumed is the sync handoff; it works because the
    /// buffer is kept contiguous for exactly this reason. Borrowing rather than
    /// handing out an owned `Bytes` is what keeps a handoff free of the
    /// refcount pair, which matters once handoffs are per value.
    #[inline]
    pub(crate) fn peek(&self) -> &[u8] {
        &self.current[self.pos..]
    }

    /// Skip `n` bytes of the current chunk, after something else has read them.
    pub(crate) fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.current.len());
        self.pos += n;
    }

    /// Fold `chunk` in behind whatever is still unread.
    ///
    /// Free when nothing is unread, which is how [`Self::fill`] always calls it;
    /// otherwise it copies, and callers with several chunks in hand should
    /// coalesce them in one pass instead of calling this repeatedly.
    fn append(&mut self, chunk: Bytes) {
        if self.pos == self.current.len() {
            self.current = chunk;
        } else {
            let mut buf = Vec::with_capacity(self.ready_bytes() + chunk.len());
            buf.extend_from_slice(&self.current[self.pos..]);
            buf.extend_from_slice(&chunk);
            self.current = Bytes::from(buf);
        }
        self.pos = 0;
    }

    /// The whole input, if the stream turns out to deliver it in a single chunk.
    ///
    /// Worth an extra poll because a single-chunk input has **no overlap
    /// available** — every byte arrives at once — so running it through the
    /// async decoder is pure loss, and the caller can hand the buffer to the
    /// (measurably faster) sync slice decoder instead. An empty stream counts as
    /// a single empty chunk, so it decodes exactly as `v2::decode(&[])` does.
    /// On `None` nothing is lost: any chunk polled is kept, and the source is
    /// ready to continue asynchronously.
    ///
    /// This is the one place that *awaits* an answer the decode does not yet
    /// need — [`Self::drain_ready`] never suspends, so on a transport that goes
    /// `Pending` before signalling end it would not find out. The case that
    /// costs is a stream which delivers all its data and then delays end of
    /// stream: the decode waits for the signal rather than starting. Bodies with
    /// a known length end promptly, so this is the right default, and it is why
    /// the wait is exactly one poll deep rather than a "buffer until EOF".
    pub(crate) async fn take_if_single_chunk(&mut self) -> Option<Bytes> {
        if !self.fill().await {
            // Empty stream: a single (empty) chunk, unless the emptiness is
            // itself an error, in which case the async path surfaces it.
            return self.error.is_none().then(Bytes::new);
        }
        while !self.ended && self.error.is_none() {
            match self.take_from_stream().await {
                // An empty chunk settles nothing; ask again.
                Some(chunk) if chunk.is_empty() => continue,
                Some(chunk) => {
                    // Not a single chunk after all. Keep the bytes and take
                    // anything else already waiting, then decode asynchronously.
                    self.append(chunk);
                    self.drain_ready().await;
                    break;
                }
                None => break,
            }
        }
        // `pos == 0` is what makes this "the *whole* input" rather than merely
        // "the rest of it" — true only before any byte has been read.
        (self.is_complete() && self.pos == 0).then(|| self.current.clone())
    }

    /// One byte, awaiting the next chunk if the current one is used up; 0 at a
    /// clean end of stream (the sync decoders' zero-padding) and after an error.
    ///
    /// Once an error is latched we never touch the stream again — every later
    /// byte is a fabricated 0. This is the sync `read_one_byte` rule, for the
    /// same reason: a stream that errors once and then recovers must not splice
    /// genuine post-error bytes into the fabricated zeros, which would
    /// desynchronize the coder.
    #[inline]
    pub(crate) async fn next_byte(&mut self) -> u8 {
        self.next_byte_or_eof().await.unwrap_or(0)
    }

    /// Like [`Self::next_byte`], but reports a clean end of stream as `None`
    /// instead of fabricating a zero.
    ///
    /// Only `Ans` frame headers need the distinction, and they need it badly:
    /// a header is read only where another frame is due, so end of stream there
    /// is truncation — whereas the coder bodies read past the end legitimately,
    /// which is what `next_byte`'s zero-padding is for.
    #[inline]
    pub(crate) async fn next_byte_or_eof(&mut self) -> Option<u8> {
        if self.error.is_some() {
            return Some(0);
        }
        if !self.fill().await {
            return None;
        }
        let byte = self.current[self.pos];
        self.pos += 1;
        Some(byte)
    }

    /// Fill `out` completely, awaiting as needed; the bulk read behind
    /// `decode_incompressible_bytes`. Errors on a truncated stream rather than
    /// returning silently-short data, as the sync `read_exact` does.
    pub(crate) async fn read_exact(&mut self, out: &mut [u8]) -> std::io::Result<()> {
        // Surface an already-latched error first, as the sync decoder does:
        // reading fabricated zeros as data could, for a corrupt length,
        // otherwise drive an unbounded read.
        if let Some(e) = self.error.take() {
            return Err(e);
        }
        let mut filled = 0;
        while filled < out.len() {
            if !self.fill().await {
                return match self.error.take() {
                    Some(e) => Err(e),
                    None => Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "truncated stream: incompressible run is short",
                    )),
                };
            }
            let n = (out.len() - filled).min(self.current.len() - self.pos);
            out[filled..filled + n].copy_from_slice(&self.current[self.pos..self.pos + n]);
            self.pos += n;
            filled += n;
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::v2::decode_stream;
    use futures_executor::block_on;
    use std::task::Poll;

    /// A stream over a fixed list of chunks that yields `Pending` (waking
    /// immediately) before every chunk, so the decoder's suspension path is
    /// exercised rather than a stream that is always instantly ready.
    pub(crate) struct Chunks {
        chunks: std::collections::VecDeque<Bytes>,
        pending: bool,
    }

    impl Chunks {
        pub(crate) fn new(bytes: &[u8], chunk_size: usize) -> Self {
            let all = Bytes::copy_from_slice(bytes);
            let mut chunks = std::collections::VecDeque::new();
            let mut start = 0;
            while start < all.len() {
                let end = (start + chunk_size).min(all.len());
                chunks.push_back(all.slice(start..end));
                start = end;
            }
            Chunks {
                chunks,
                pending: true,
            }
        }
    }

    impl Stream for Chunks {
        type Item = Result<Bytes, std::io::Error>;
        fn poll_next(
            mut self: Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            // Alternate Pending/Ready so every chunk boundary really suspends.
            if self.pending {
                self.pending = false;
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            self.pending = true;
            Poll::Ready(self.chunks.pop_front().map(Ok))
        }
    }

    /// An always-ready stream over a fixed list of results, for testing the
    /// look-ahead decision directly.
    struct Ready(std::collections::VecDeque<Result<Bytes, std::io::Error>>);

    impl Stream for Ready {
        type Item = Result<Bytes, std::io::Error>;
        fn poll_next(
            mut self: Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
        ) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.0.pop_front())
        }
    }

    fn source_of(items: Vec<Result<Bytes, std::io::Error>>) -> ChunkSource<Ready> {
        block_on(ChunkSource::new(Ready(items.into_iter().collect())))
    }

    /// The single-chunk look-ahead decides which decoder runs. Both decoders
    /// produce the same answer, so a mistake here is invisible in results and
    /// shows up only as lost speed — hence testing the decision itself.
    #[test]
    fn single_chunk_lookahead_decides_correctly() {
        // One chunk then clean EOF: the whole input, for the sync decoder.
        let mut s = source_of(vec![Ok(Bytes::from_static(b"hello"))]);
        assert_eq!(
            block_on(s.take_if_single_chunk()).as_deref(),
            Some(&b"hello"[..])
        );

        // Empty stream counts as a single empty chunk, so it decodes exactly as
        // `v2::decode(&[])` does rather than taking a different path.
        let mut s = source_of(vec![]);
        assert_eq!(
            block_on(s.take_if_single_chunk()).as_deref(),
            Some(&b""[..])
        );

        // Two chunks that are *both already there*: still the whole input, and
        // still the sync decoder's job. How a ready transport chose to split its
        // buffer says nothing about whether overlap is available, and before the
        // drain this case went down the async path for nothing.
        let mut s = source_of(vec![
            Ok(Bytes::from_static(b"abc")),
            Ok(Bytes::from_static(b"de")),
        ]);
        assert_eq!(
            block_on(s.take_if_single_chunk()).as_deref(),
            Some(&b"abcde"[..]),
            "chunks already in hand should coalesce, not force the async path"
        );

        // Two chunks with a real suspension between them: overlap *is* available
        // here, so the async path it is — and nothing may be lost deciding that.
        let mut s = block_on(ChunkSource::new(Chunks::new(b"abcde", 3)));
        assert!(block_on(s.take_if_single_chunk()).is_none());
        let mut rest = [0u8; 5];
        block_on(s.read_exact(&mut rest)).unwrap();
        assert_eq!(&rest, b"abcde", "look-ahead dropped or reordered bytes");

        // Empty chunks are transparent: a stream may legitimately yield one,
        // and it must not read as end of stream (nor make a single-chunk input
        // look like several).
        let mut s = source_of(vec![
            Ok(Bytes::new()),
            Ok(Bytes::from_static(b"hello")),
            Ok(Bytes::new()),
        ]);
        assert_eq!(
            block_on(s.take_if_single_chunk()).as_deref(),
            Some(&b"hello"[..]),
            "empty chunks should not defeat the single-chunk path"
        );

        // One chunk then an error is *not* a complete input: it must not be
        // handed to the sync decoder, which would decode a truncated buffer as
        // though it were whole.
        let mut s = source_of(vec![
            Ok(Bytes::from_static(b"abc")),
            Err(std::io::Error::other("boom")),
        ]);
        assert!(block_on(s.take_if_single_chunk()).is_none());
        assert!(s.take_error().is_some(), "the error must still be latched");
    }

    /// [`READY_TARGET`] is the bounded-buffer promise, and it is the whole
    /// reason `drain_ready` stops asking. Without it a transport with the input
    /// already in hand would be emptied into memory, and `decode_stream`'s peak
    /// would be the value *plus the entire compressed input* — which is what
    /// collect-then-decode gives you for free, so the API would have nothing
    /// left to offer. Asserted here rather than through the allocator because
    /// this is the mechanism, and `stats_alloc` cannot report a peak anyway.
    #[test]
    fn the_drain_is_bounded_however_much_has_arrived() {
        // Deliberately not a divisor of `READY_TARGET`, so the drain has to
        // overshoot — it tests its budget before polling, not after, so the
        // chunk that crosses the line is taken whole.
        const CHUNK: usize = 48 * 1024;
        // 4.5 MB, seventeen times the target, every byte deliverable at once.
        const N: usize = 96;
        let mut s = source_of(
            (0..N)
                .map(|i| Ok(Bytes::from(vec![i as u8; CHUNK])))
                .collect(),
        );
        let mut buf = vec![0u8; 4096];
        let mut total = 0;
        let mut peak = 0;
        loop {
            let ready = s.ready_bytes();
            peak = peak.max(ready);
            assert!(
                ready <= READY_TARGET + CHUNK,
                "buffered {ready} bytes, past the {READY_TARGET} target (+ one \
                 {CHUNK}-byte chunk of overshoot, since the drain checks before \
                 it polls)"
            );
            if block_on(s.read_exact(&mut buf)).is_err() {
                break;
            }
            total += buf.len();
        }
        assert_eq!(total, N * CHUNK, "the cap must not lose bytes, only defer");
        assert!(
            peak >= READY_TARGET,
            "only ever buffered {peak} bytes, so the cap was never reached and \
             the bound above proves nothing"
        );
    }

    /// The two paths must agree: same bytes, one chunk versus many.
    #[test]
    fn single_chunk_and_multi_chunk_paths_agree() {
        let value: Vec<u64> = (0..500).map(|i: u64| i.wrapping_mul(2654435761)).collect();
        let encoded = crate::v2::encode(&value);
        let whole: Vec<u64> =
            block_on(decode_stream(Chunks::new(&encoded, encoded.len() + 1))).unwrap();
        let split: Vec<u64> = block_on(decode_stream(Chunks::new(&encoded, 5))).unwrap();
        assert_eq!(whole, value, "single-chunk (sync) path");
        assert_eq!(split, value, "multi-chunk (async) path");
    }

    /// The load-bearing property: the async decoder recovers exactly what the
    /// sync coder wrote, and the transport's chunking is invisible.
    #[test]
    fn round_trip_vec_u64_at_every_chunk_size() {
        for n in [0usize, 1, 2, 37, 300] {
            let value: Vec<u64> = (0..n as u64).map(|i| i.wrapping_mul(2654435761)).collect();
            let encoded = crate::v2::encode(&value);
            assert_eq!(
                crate::v2::decode::<Vec<u64>>(&encoded).as_ref(),
                Some(&value),
                "sync decode disagrees, so the fixture is wrong"
            );
            for chunk_size in [1, 2, 3, 7, 64, 4096] {
                let decoded: Vec<u64> =
                    block_on(decode_stream(Chunks::new(&encoded, chunk_size))).unwrap();
                assert_eq!(
                    decoded, value,
                    "n = {n}, chunk_size = {chunk_size}: async decode disagrees"
                );
            }
        }
    }

    /// Every `u64` bit-length regime: the small values that code as a bare
    /// symbol, and the larger ones that add incompressible full bytes plus a
    /// bit-coded partial top byte.
    #[test]
    fn round_trip_u64_across_bit_lengths() {
        for shift in 0..64 {
            for v in [
                1u64 << shift,
                (1u64 << shift) | 0x5a5a_5a5a_5a5a_5a5a >> shift,
            ] {
                let encoded = crate::v2::encode(&v);
                let decoded: u64 = block_on(decode_stream(Chunks::new(&encoded, 3))).unwrap();
                assert_eq!(decoded, v, "shift = {shift}");
            }
        }
    }

    /// The string path, including every `char` length class: ASCII, the
    /// one-continuation-byte range, and the two-continuation-byte range.
    #[test]
    fn round_trip_vec_string_at_every_chunk_size() {
        let value: Vec<String> = vec![
            String::new(),
            "a".to_string(),
            "hello world".to_string(),
            // One continuation byte (< 1 << 14), then two.
            "héllo — ünïcode".to_string(),
            "日本語のテキスト".to_string(),
            "mixed ascii 日本 and émojis 🎉🦀".to_string(),
            "x".repeat(300),
        ];
        let encoded = crate::v2::encode(&value);
        assert_eq!(
            crate::v2::decode::<Vec<String>>(&encoded).as_ref(),
            Some(&value),
            "sync decode disagrees, so the fixture is wrong"
        );
        for chunk_size in [1, 2, 3, 7, 64, 4096] {
            let decoded: Vec<String> =
                block_on(decode_stream(Chunks::new(&encoded, chunk_size))).unwrap();
            assert_eq!(decoded, value, "chunk_size = {chunk_size}");
        }
    }

    /// R4: once the transport has failed, `is_complete` must never again say the
    /// stream finished cleanly — even after the error has been taken out to be
    /// reported.
    ///
    /// It gates the unconditional sync handoff (complete ⇒ `usize::MAX`
    /// capacity), so a "clean" answer here is licence to run the sync decoder to
    /// the end of an incomplete buffer.
    #[test]
    fn a_failed_transport_never_looks_complete_again() {
        /// Yields one chunk, then fails.
        struct FailsAfterOne(bool);
        impl Stream for FailsAfterOne {
            type Item = Result<Bytes, std::io::Error>;
            fn poll_next(
                mut self: Pin<&mut Self>,
                _: &mut std::task::Context<'_>,
            ) -> Poll<Option<Self::Item>> {
                if self.0 {
                    Poll::Ready(Some(Err(std::io::Error::other("transport died"))))
                } else {
                    self.0 = true;
                    Poll::Ready(Some(Ok(Bytes::from_static(b"hello"))))
                }
            }
        }

        block_on(async {
            let mut source = ChunkSource::new(FailsAfterOne(false)).await;
            // Drive it until the failure lands: consume what arrived, then ask
            // for more.
            source.advance(source.ready_bytes());
            source.next_byte_or_eof().await;
            assert!(
                !source.is_complete(),
                "a failed transport must not report a clean finish"
            );
            let err = source.take_error().expect("the failure must be reported");
            assert!(err.to_string().contains("transport died"));
            // The whole point: taking the error must not launder the failure.
            assert!(
                !source.is_complete(),
                "is_complete went clean once the error was taken"
            );
        });
    }

    /// On truncated input the async decoder must behave exactly as the sync one
    /// does — including where the sync one legitimately succeeds.
    ///
    /// Running past the encoded data zero-pads rather than erroring (both
    /// decoders, by design), so a short prefix can still decode to *some* value;
    /// what must never differ is *which*. Agreement is the property, not
    /// failure.
    #[test]
    fn truncated_stream_agrees_with_sync() {
        let big: Vec<u64> = (0..1000).collect();
        let encoded = crate::v2::encode(&big);
        for len in [0, 1, 2, 5, 17, 64, encoded.len() / 2, encoded.len() - 1] {
            let prefix = &encoded[..len.min(encoded.len())];
            let sync = crate::v2::decode::<Vec<u64>>(prefix);
            let stream = block_on(decode_stream::<Vec<u64>, _, _>(Chunks::new(prefix, 16)));
            assert_eq!(
                sync.as_ref(),
                stream.as_ref().ok(),
                "len = {len}: sync and async disagree on truncated input"
            );
        }
    }
}
