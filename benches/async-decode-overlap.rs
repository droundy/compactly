// Does decoding actually overlap the wait for the next chunk?
//
// This is the claim that justifies `decode_stream` existing at all — otherwise
// a caller may as well collect the stream and call `v2::decode`, which is four
// lines they can write themselves. It is a property of a *delivery schedule*,
// which is why it needs its own bin: `coder-routes … stream` measures what the
// async machinery costs when every byte is already in hand, and that is a
// different question.
//
// The source models a network delivering at a constant rate: chunk `i` becomes
// available at `start + i * interval`, regardless of how long decoding takes.
// Both arms see exactly the same schedule.
//
//   collect  await every chunk into one buffer, then `v2::decode` — the
//            do-it-yourself baseline. Costs arrival + decode.
//   overlap  `decode_stream` over the same source. Should cost roughly
//            max(arrival, decode), because each chunk is decoded while the
//            next is still in flight.
//
// Usage: `[COUNT=n] [CHUNKS=n] [RATE_MBPS=n] async-decode-overlap collect|overlap|both`
//
// Wall-clock means this is sensitive to machine noise, so it wants a reserved
// CPU (`quiet-bench run`) like everything else here; the `±` on each line says
// how much of the gap between the two arms is real.
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

use bytes::Bytes;
use common::{args, report_env};
use compactly::v2::{decode_stream, Ans};
use futures_core::Stream;

mod common;

const DEFAULT_COUNT: usize = 100_000;
const DEFAULT_CHUNKS: usize = 64;

/// Park the calling thread until the future completes.
///
/// Hand-rolled because `futures-executor` is a dev-dependency and so is not
/// available to `src/bin`. This is the standard `Wake`-by-unpark executor; it
/// genuinely sleeps rather than spinning, which matters — a spinning executor
/// would burn the very CPU time we are trying to show is being used for
/// decoding.
struct ThreadWaker(std::thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(std::thread::current())));
    let mut cx = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => std::thread::park(),
        }
    }
}

/// Chunks on a fixed arrival schedule: chunk `i` is due at `start + i *
/// interval`. Polling before a chunk is due registers a wake for its due time
/// and returns `Pending`, so the executor is free to do something else —
/// which, for the `overlap` arm, is decoding the chunks already delivered.
struct Timed {
    chunks: std::vec::IntoIter<Bytes>,
    start: Instant,
    interval: Duration,
    delivered: u32,
}

impl Timed {
    fn new(bytes: &[u8], chunks: usize, interval: Duration) -> Self {
        let all = Bytes::copy_from_slice(bytes);
        let chunk_size = all.len().div_ceil(chunks.max(1));
        let mut parts = Vec::new();
        let mut start = 0;
        while start < all.len() {
            let end = (start + chunk_size).min(all.len());
            parts.push(all.slice(start..end));
            start = end;
        }
        Timed {
            chunks: parts.into_iter(),
            start: Instant::now(),
            interval,
            delivered: 0,
        }
    }
}

impl Stream for Timed {
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let due = self.start + self.interval * self.delivered;
        let now = Instant::now();
        if now < due {
            // Wake at the due time. One short-lived thread per wait: this is a
            // benchmark harness, not the library, and there are only `CHUNKS`
            // of them.
            let waker = cx.waker().clone();
            let sleep = due - now;
            std::thread::spawn(move || {
                std::thread::sleep(sleep);
                waker.wake();
            });
            return Poll::Pending;
        }
        self.delivered += 1;
        Poll::Ready(self.chunks.next().map(Ok))
    }
}

/// Await every chunk, concatenate, then decode — the do-it-yourself baseline.
async fn collect_then_decode(mut source: Timed, ans: bool) -> usize {
    let mut buf = Vec::new();
    loop {
        let next = std::future::poll_fn(|cx| Pin::new(&mut source).poll_next(cx)).await;
        match next {
            Some(Ok(chunk)) => buf.extend_from_slice(&chunk),
            Some(Err(e)) => panic!("{e}"),
            None => break,
        }
    }
    if ans {
        Ans::decode::<Vec<u64>>(&buf).unwrap().len()
    } else {
        compactly::v2::decode::<Vec<u64>>(&buf).unwrap().len()
    }
}

fn main() {
    let env = |k: &str, d: usize| {
        std::env::var(k)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(d)
    };
    let count = env("COUNT", DEFAULT_COUNT);
    let chunks = env("CHUNKS", DEFAULT_CHUNKS);
    let which = args()
        .first()
        .cloned()
        .unwrap_or_else(|| "both".to_string());

    let mut x = 0x123456789abcdef0u64;
    let data: Vec<u64> = (0..count)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            x
        })
        .collect();
    // `CODER=ans` measures the rANS decoder instead. Its frames are decodable
    // only once each has arrived whole, and its *final* frame runs to end of
    // stream, so the shape of the overlap is expected to differ.
    let ans = std::env::var("CODER").is_ok_and(|c| c == "ans");
    let compressed = if ans {
        Ans::encode(&data)
    } else {
        compactly::v2::encode(&data)
    };

    // Default the delivery rate so arrival takes about as long as a decode:
    // that is where overlap is most visible, and it is also the realistic
    // regime — a network fast enough to outrun the decoder makes the whole
    // question moot, and one far slower hides the decode entirely.
    let baseline = {
        let stats = common::report("sync decode (no stream)", || {
            if ans {
                Ans::decode::<Vec<u64>>(&compressed).unwrap().len()
            } else {
                compactly::v2::decode::<Vec<u64>>(&compressed)
                    .unwrap()
                    .len()
            }
        });
        Duration::from_secs_f64(stats.ns_per_iter / 1e9)
    };
    let rate_mbps = env("RATE_MBPS", 0);
    let total_arrival = if rate_mbps > 0 {
        Duration::from_secs_f64(compressed.len() as f64 / (rate_mbps as f64 * 1e6))
    } else {
        baseline
    };
    let interval = total_arrival / chunks as u32;

    eprintln!(
        "count={count} compressed={} chunks={chunks} interval={:?} \
         arrival={:?} sync_decode={:?}",
        compressed.len(),
        interval,
        total_arrival,
        baseline
    );

    // The source is built fresh for each iteration and *not* timed: its
    // constructor stamps the arrival clock, and one is consumed per decode.
    // `Option` because the decode takes it by value while `bench_gen_env`
    // hands out `&mut`.
    let source = || Some(Timed::new(&compressed, chunks, interval));

    if which == "collect" || which == "both" {
        report_env("collect (arrival + decode)", source, |s| {
            block_on(collect_then_decode(s.take().unwrap(), ans))
        });
    }
    if which == "overlap" || which == "both" {
        report_env("overlap (decode hidden)", source, |s| {
            let s = s.take().unwrap();
            if ans {
                block_on(Ans::decode_stream::<Vec<u64>, _, _>(s))
                    .unwrap()
                    .len()
            } else {
                block_on(decode_stream::<Vec<u64>, _, _>(s)).unwrap().len()
            }
        });
    }
}
