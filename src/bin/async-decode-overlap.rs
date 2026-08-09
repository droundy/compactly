// Does decoding actually overlap the wait for the next chunk?
//
// This is the claim that justifies `decode_stream` existing at all — otherwise
// a caller may as well collect the stream and call `v2::decode`, which is four
// lines they can write themselves. It is a **wall-clock** property, so unlike
// `async-decode-cost` it cannot be instruction-counted; the two bins measure
// different things and neither substitutes for the other.
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
// Wall-clock means this is sensitive to machine noise; run it under `bench` and
// take the min of several runs, as with everything else here.
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

use bytes::Bytes;
use compactly::v2::decode_stream;
use futures_core::Stream;

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

    /// When the whole schedule finishes, for reporting.
    fn arrival_time(&self) -> Duration {
        self.interval * self.chunks.len() as u32
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
async fn collect_then_decode(mut source: Timed) -> usize {
    let mut buf = Vec::new();
    loop {
        let next = std::future::poll_fn(|cx| Pin::new(&mut source).poll_next(cx)).await;
        match next {
            Some(Ok(chunk)) => buf.extend_from_slice(&chunk),
            Some(Err(e)) => panic!("{e}"),
            None => break,
        }
    }
    compactly::v2::decode::<Vec<u64>>(&buf).unwrap().len()
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
    let which = std::env::args()
        .nth(1)
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
    let compressed = compactly::v2::encode(&data);

    // Default the delivery rate so arrival takes about as long as a decode:
    // that is where overlap is most visible, and it is also the realistic
    // regime — a network fast enough to outrun the decoder makes the whole
    // question moot, and one far slower hides the decode entirely.
    let baseline = {
        let t = Instant::now();
        std::hint::black_box(
            compactly::v2::decode::<Vec<u64>>(&compressed)
                .unwrap()
                .len(),
        );
        t.elapsed()
    };
    let rate_mbps = env("RATE_MBPS", 0);
    let total_arrival = if rate_mbps > 0 {
        Duration::from_secs_f64(compressed.len() as f64 / (rate_mbps as f64 * 1e6))
    } else {
        baseline
    };
    let interval = total_arrival / chunks as u32;

    println!(
        "count={count} compressed={} chunks={chunks} interval={:?} \
         arrival={:?} sync_decode={:?}",
        compressed.len(),
        interval,
        total_arrival,
        baseline
    );

    let run_collect = || {
        let source = Timed::new(&compressed, chunks, interval);
        let arrival = source.arrival_time();
        let t = Instant::now();
        let n = block_on(collect_then_decode(source));
        (t.elapsed(), arrival, n)
    };
    let run_overlap = || {
        let source = Timed::new(&compressed, chunks, interval);
        let arrival = source.arrival_time();
        let t = Instant::now();
        let n = block_on(decode_stream::<Vec<u64>, _, _>(source))
            .unwrap()
            .len();
        (t.elapsed(), arrival, n)
    };

    // Min of a few: wall-clock, so the fastest run is the least contaminated.
    let best = |f: &dyn Fn() -> (Duration, Duration, usize)| {
        let mut best = Duration::MAX;
        let mut arrival = Duration::ZERO;
        let mut n = 0;
        for _ in 0..5 {
            let (d, a, got) = f();
            best = best.min(d);
            arrival = a;
            n = got;
        }
        (best, arrival, n)
    };

    if which == "collect" || which == "both" {
        let (d, a, n) = best(&run_collect);
        println!("collect: {d:?}  (arrival {a:?} + decode) elements={n}");
    }
    if which == "overlap" || which == "both" {
        let (d, a, n) = best(&run_overlap);
        println!("overlap: {d:?}  (arrival {a:?}, decode hidden) elements={n}");
    }
}
