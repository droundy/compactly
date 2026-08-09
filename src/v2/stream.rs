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
/// returned, to be surfaced by `into_result`.
pub(crate) struct ChunkSource<S> {
    /// Boxed rather than requiring `S: Unpin`: one allocation for a whole
    /// decode is not worth constraining what callers may pass.
    stream: Pin<Box<S>>,
    current: Bytes,
    pos: usize,
    /// The next chunk, fetched while `current` was still being read.
    ///
    /// **The invariant**, established by [`ChunkSource::new`] and restored by
    /// [`ChunkSource::fill`] at every chunk promotion: this is `Some` unless the
    /// stream has ended or failed. Two things follow, and they are why the
    /// read-ahead is maintained rather than fetched on demand:
    ///
    /// - `queued.is_none() && error.is_none()` *is* end of stream, so
    ///   [`ChunkSource::is_final_chunk`] answers without polling and there is no
    ///   separate `exhausted` flag to keep in step.
    /// - Exhausting `current` never has to wait, because the next chunk was
    ///   asked for a whole chunk earlier.
    ///
    /// It also fuses the stream for free. [`Stream::poll_next`] documents that
    /// polling after it returns `None` "may panic, block forever, or cause other
    /// kinds of problems", and only a `FusedStream` promises otherwise — but the
    /// sole caller of the stream is [`ChunkSource::read_ahead`], which is only
    /// ever entered when the previous poll yielded a chunk, and which returns
    /// the moment it sees `None`. So a second poll past the end cannot happen.
    queued: Option<Bytes>,
    error: Option<std::io::Error>,
}

impl<S> std::fmt::Debug for ChunkSource<S> {
    /// Summarize rather than dump: the derived form would print the current
    /// chunk, i.e. part of the payload.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkSource")
            .field("buffered", &(self.current.len() - self.pos))
            .field("read_ahead", &self.queued.as_ref().map(Bytes::len))
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<S, E> ChunkSource<S>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    /// Build a source and **prime it**: fetch the first chunk and the
    /// read-ahead behind it, so the invariant on `queued` holds from the start.
    ///
    /// Priming here rather than lazily on first use is what removes the
    /// "not yet started" state — without it, `queued.is_none()` would be
    /// ambiguous between "nothing fetched yet" and "nothing left", and telling
    /// them apart would need the extra flag this design does without.
    pub(crate) async fn new(stream: S) -> Self {
        let mut source = ChunkSource {
            stream: Box::pin(stream),
            current: Bytes::new(),
            pos: 0,
            queued: None,
            error: None,
        };
        source.current = source.take_from_stream().await.unwrap_or_default();
        source.read_ahead().await;
        source
    }

    /// Take any latched stream error, for `into_result`.
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

    /// One chunk straight from the stream, latching a failure into `error`.
    /// `None` for either end of stream or failure.
    async fn take_from_stream(&mut self) -> Option<Bytes> {
        match self.poll_stream().await {
            None => None,
            Some(Ok(chunk)) => Some(chunk),
            Some(Err(e)) => {
                self.error = Some(std::io::Error::other(e));
                None
            }
        }
    }

    /// Restore the one-chunk read-ahead, so that [`Self::is_final_chunk`] can
    /// answer and so exhausting `current` need not wait.
    ///
    /// Awaiting here is the point, not an oversight: it is what makes the
    /// invariant hold. The cost is that decoding starts one chunk later than it
    /// strictly could; the benefit is that it then never stalls at a chunk
    /// boundary, because the next chunk was requested a whole chunk earlier.
    /// Over a stream of `n` chunks that trades one interval of latency for
    /// `n - 1` avoided stalls.
    ///
    /// Empty chunks are skipped rather than queued, so that `queued.is_some()`
    /// means "there is real data ahead". Queueing an empty chunk would leave
    /// [`Self::is_final_chunk`] answering `false` on a stream that in fact has
    /// nothing left — costing the single-chunk fast path to any producer that
    /// pads with empty chunks.
    ///
    /// Returning immediately on `None` is what fuses the stream: this is the
    /// only place the stream is polled after construction, and it is only
    /// reached when the previous poll produced a chunk.
    async fn read_ahead(&mut self) {
        debug_assert!(self.queued.is_none(), "read-ahead already in hand");
        while self.error.is_none() {
            match self.take_from_stream().await {
                Some(chunk) if chunk.is_empty() => continue,
                Some(chunk) => {
                    self.queued = Some(chunk);
                    return;
                }
                None => return,
            }
        }
    }

    /// Make the current chunk non-empty if possible, and restore the read-ahead.
    /// Returns false at a clean end of stream or once an error is latched.
    ///
    /// Maintains the invariant that whenever this returns true, `queued` holds
    /// the next chunk *or* the stream is known to be finished or failed.
    ///
    /// A loop rather than an `if` only because the *first* chunk may be empty:
    /// [`Self::new`] takes it straight from the stream, while every later chunk
    /// comes from `queued`, which [`Self::read_ahead`] never fills with an empty
    /// one.
    async fn fill(&mut self) -> bool {
        while self.pos == self.current.len() {
            // No read-ahead means no more data — the invariant says so, with no
            // need to poll and find out.
            let Some(next) = self.queued.take() else {
                return false;
            };
            self.current = next;
            self.pos = 0;
            // Restoring the read-ahead belongs here, with the promotion that
            // consumed it, and not after the loop: the invariant is established
            // when `current` changes and holds until it changes again, so doing
            // it after the loop would re-check it on every single byte, which
            // measured 1.2% of the async path's instructions.
            self.read_ahead().await;
        }
        true
    }

    /// Whether the chunk in hand is the last one. Free, by the invariant: no
    /// read-ahead and no error means the stream is spent.
    pub(crate) fn is_final_chunk(&self) -> bool {
        self.queued.is_none() && self.error.is_none()
    }

    /// Bytes already buffered, decodable without awaiting anything.
    pub(crate) fn ready_bytes(&self) -> usize {
        self.current.len() - self.pos
    }

    /// Everything the source has buffered, as one slice.
    ///
    /// Returned owned (a `Bytes` slice is a refcount bump, not a copy) rather
    /// than borrowed, so the caller can go on mutating the source while holding
    /// it — which is exactly what handing it to a sync decoder and then
    /// [advancing](Self::advance) by what that decoder consumed requires.
    pub(crate) fn buffered(&self) -> Bytes {
        self.current.slice(self.pos..)
    }

    /// Skip `n` bytes of the current chunk, after something else has read them.
    pub(crate) fn advance(&mut self, n: usize) {
        debug_assert!(self.pos + n <= self.current.len());
        self.pos += n;
    }

    /// The whole input, if the stream turns out to deliver it in a single chunk.
    ///
    /// Worth a look-ahead poll because a single-chunk input has **no overlap
    /// available** — every byte arrives at once — so running it through the
    /// async decoder is pure loss, and the caller can hand the buffer to the
    /// (measurably faster) sync slice decoder instead. An empty stream counts as
    /// a single empty chunk, so it decodes exactly as `v2::decode(&[])` does.
    ///
    /// This needs no look-ahead of its own: [`Self::fill`] already maintains
    /// one, so the question is just whether the first chunk is also the last.
    /// On `None` the source is untouched and ready to continue asynchronously.
    ///
    /// The one case the read-ahead pessimizes is a stream that delivers all its
    /// data and then delays signalling end of stream: the decode waits for that
    /// signal rather than starting. Bodies with a known length end promptly, so
    /// this is the right default, but it is why the read-ahead is exactly one
    /// chunk deep and not a general "buffer until EOF".
    pub(crate) async fn take_if_single_chunk(&mut self) -> Option<Bytes> {
        if !self.fill().await {
            // Empty stream: a single (empty) chunk, unless the emptiness is
            // itself an error, in which case the async path surfaces it.
            return self.error.is_none().then(Bytes::new);
        }
        // `pos == 0` is what makes this "the *whole* input" rather than merely
        // "the rest of it" — true only before any byte has been read.
        (self.is_final_chunk() && self.pos == 0).then(|| self.current.clone())
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
        if self.error.is_some() || !self.fill().await {
            return 0;
        }
        let byte = self.current[self.pos];
        self.pos += 1;
        byte
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
mod tests {
    use super::*;
    use crate::v2::decode_stream;
    use futures_executor::block_on;
    use std::task::Poll;

    /// A stream over a fixed list of chunks that yields `Pending` (waking
    /// immediately) before every chunk, so the decoder's suspension path is
    /// exercised rather than a stream that is always instantly ready.
    struct Chunks {
        chunks: std::collections::VecDeque<Bytes>,
        pending: bool,
    }

    impl Chunks {
        fn new(bytes: &[u8], chunk_size: usize) -> Self {
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

        // Two chunks: not single, and nothing may be lost by the look-ahead.
        let mut s = source_of(vec![
            Ok(Bytes::from_static(b"abc")),
            Ok(Bytes::from_static(b"de")),
        ]);
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
