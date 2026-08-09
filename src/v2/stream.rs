//! Async decoding from a [`Stream`] of [`Bytes`] chunks.
//!
//! The point of this module is that decoding happens *as the bytes arrive*: the
//! decoder holds the chunk it is working through and suspends only when that
//! chunk runs out, so the decode overlaps the wait for the next chunk instead of
//! following it. Collecting the whole stream and calling
//! [`v2::decode`](super::decode) would need no library support at all.
//!
//! The entry point is [`decode_stream`]. Its input bound,
//! `Stream<Item = Result<Bytes, E>>`, is what the ecosystem already speaks:
//! `aws_sdk_s3`'s `ByteStream`, `object_store`'s `GetResult::into_stream()`, and
//! `axum`'s `Body::into_data_stream()` all match it directly.
//!
//! This module holds only the coder-independent part — [`ChunkSource`], the
//! buffer that turns a chunk stream into "one byte, awaiting if necessary". Each
//! coder's async decoder lives beside its sync counterparts (`AsyncRangeDecoder`
//! in `arith`), which is where the coder state's internals are reachable.

use std::pin::Pin;

use bytes::Bytes;
use futures_core::Stream;

use super::{AsyncEntropyDecoder, DecodeAsync};

pub use super::arith::AsyncRangeDecoder;

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
    /// Set once the stream has returned `None`; from then on reads yield zeros
    /// without touching the stream again.
    exhausted: bool,
    error: Option<std::io::Error>,
}

impl<S> std::fmt::Debug for ChunkSource<S> {
    /// Summarize rather than dump: the derived form would print the current
    /// chunk, i.e. part of the payload.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkSource")
            .field("buffered", &(self.current.len() - self.pos))
            .field("exhausted", &self.exhausted)
            .field("error", &self.error)
            .finish_non_exhaustive()
    }
}

impl<S, E> ChunkSource<S>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    pub(crate) fn new(stream: S) -> Self {
        ChunkSource {
            stream: Box::pin(stream),
            current: Bytes::new(),
            pos: 0,
            exhausted: false,
            error: None,
        }
    }

    /// Take any latched stream error, for `into_result`.
    pub(crate) fn take_error(&mut self) -> Option<std::io::Error> {
        self.error.take()
    }

    /// Pull the next chunk. Uses `poll_fn` over [`Stream::poll_next`] directly
    /// so this crate needs only `futures-core`, not `futures-util`.
    async fn next_chunk(&mut self) -> Option<Result<Bytes, E>> {
        let stream = &mut self.stream;
        std::future::poll_fn(move |cx| stream.as_mut().poll_next(cx)).await
    }

    /// Make the current chunk non-empty if possible. Returns false at a clean
    /// end of stream or once an error is latched.
    async fn fill(&mut self) -> bool {
        loop {
            if self.pos < self.current.len() {
                return true;
            }
            if self.exhausted || self.error.is_some() {
                return false;
            }
            match self.next_chunk().await {
                None => {
                    self.exhausted = true;
                    return false;
                }
                Some(Ok(chunk)) => {
                    self.current = chunk;
                    self.pos = 0;
                }
                Some(Err(e)) => {
                    self.error = Some(std::io::Error::other(e));
                    return false;
                }
            }
        }
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

/// Shared async decode plumbing: run `decode_async` and fold the result with any
/// latched stream error. The async twin of `stream_decode`.
async fn stream_decode_async<T: DecodeAsync, D: AsyncEntropyDecoder>(
    mut decoder: D,
) -> std::io::Result<T> {
    let value = T::decode_async(&mut decoder, &mut T::Context::default()).await;
    decoder.into_result(value)
}

/// Decode a value from an async stream of [`Bytes`] chunks, decoding each chunk
/// as it arrives rather than waiting for the whole input.
///
/// Accepts exactly the bytes [`encode`](super::encode) and
/// [`encode_to`](super::encode_to) produce, using the same default `Range`
/// coder, so this and the sync API interoperate in both directions. Where the
/// transport happened to split the bytes makes no difference to the result.
pub async fn decode_stream<T, S, E>(stream: S) -> std::io::Result<T>
where
    T: DecodeAsync,
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    stream_decode_async::<T, _>(AsyncRangeDecoder::new(stream).await).await
}

#[cfg(test)]
mod tests {
    use super::*;
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
