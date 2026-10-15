//! Does decoding actually overlap the wait for the next chunk?
//!
//! This is the claim that justifies `decode_stream` existing at all — otherwise a
//! caller may as well collect the stream and call `v2::decode`, which is four
//! lines they can write themselves. It is a property of a *delivery schedule*,
//! which is why it is a bench of its own: `routes.rs`'s `stream` groups measure
//! what the async machinery costs when every byte is already in hand, and that is
//! a different question.
//!
//! The source models a network delivering at a constant rate: chunk `i` becomes
//! available at `start + i * interval`, regardless of how long decoding takes.
//! Both candidates of a group see exactly the same schedule.
//!
//! - `collect` — await every chunk into one buffer, then decode. The
//!   do-it-yourself baseline. Costs arrival + decode.
//! - `overlap` — `decode_stream` over the same source. Should cost roughly
//!   max(arrival, decode), because each chunk is decoded while the next is still
//!   in flight.
//!
//! There is one group per coder (`overlap range`, `overlap ans`): `Ans`'s frames
//! are decodable only once each has arrived whole, and its *final* frame runs to
//! end of stream, so the shape of the overlap is expected to differ.
//!
//! The delivery rate is chosen so that arrival takes about as long as a decode,
//! which is where overlap is most visible and also the realistic regime: a
//! network fast enough to outrun the decoder makes the whole question moot, and
//! one far slower hides the decode entirely. That decode time is measured here,
//! roughly, at startup — it only sets the schedule, not the result.
//!
//! This is wall-clock time, so it wants a reserved CPU like everything else, and
//! the source is built *inside* each candidate: it stamps the arrival clock when
//! it is created, and `scaling` generates inputs in batches ahead of use.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::time::{Duration, Instant};

use bytes::Bytes;
use compactly::v2::{decode_stream, Ans};
use futures_core::Stream;

const COUNT: usize = 100_000;
const CHUNKS: usize = 64;

/// Park the calling thread until the future completes.
///
/// Hand-rolled because a real executor would put its own scheduling in the
/// measurement. This is the standard `Wake`-by-unpark executor; it genuinely
/// sleeps rather than spinning, which matters — a spinning executor would burn
/// the very CPU time we are trying to show is being used for decoding.
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
/// and returns `Pending`, so the executor is free to do something else — which,
/// for the `overlap` arm, is decoding the chunks already delivered.
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
async fn collect(mut source: Timed) -> Vec<u8> {
    let mut buf = Vec::new();
    loop {
        let next = std::future::poll_fn(|cx| Pin::new(&mut source).poll_next(cx)).await;
        match next {
            Some(Ok(chunk)) => buf.extend_from_slice(&chunk),
            Some(Err(e)) => panic!("{e}"),
            None => break,
        }
    }
    buf
}

/// What was encoded, and how fast each coder's stream should arrive.
struct Schedule {
    range: Vec<u8>,
    ans: Vec<u8>,
    range_interval: Duration,
    ans_interval: Duration,
}

/// Roughly how long `f` takes: the median of a few runs. It sets a delivery
/// rate, so it need not be precise.
fn typical(mut f: impl FnMut()) -> Duration {
    let mut times: Vec<Duration> = (0..9)
        .map(|_| {
            let start = Instant::now();
            f();
            start.elapsed()
        })
        .collect();
    times.sort();
    times[times.len() / 2]
}

fn schedule() -> Schedule {
    let mut x = 0x123456789abcdef0u64;
    let data: Vec<u64> = (0..COUNT)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            x
        })
        .collect();
    let range = compactly::v2::encode(&data);
    let ans = Ans::encode(&data);
    let range_decode = typical(|| {
        assert_eq!(
            compactly::v2::decode::<Vec<u64>>(&range).unwrap().len(),
            COUNT
        );
    });
    let ans_decode = typical(|| {
        assert_eq!(Ans::decode::<Vec<u64>>(&ans).unwrap().len(), COUNT);
    });
    eprintln!(
        "overlap: {COUNT} u64s in {CHUNKS} chunks; a decode takes about {range_decode:?} (range) \
         and {ans_decode:?} (ans), so one chunk arrives every {:?} and {:?}",
        range_decode / CHUNKS as u32,
        ans_decode / CHUNKS as u32,
    );
    Schedule {
        range,
        ans,
        range_interval: range_decode / CHUNKS as u32,
        ans_interval: ans_decode / CHUNKS as u32,
    }
}

#[scaling::input(group("overlap range", "overlap ans"), name = "100k u64", build_once)]
fn input() -> Schedule {
    schedule()
}

#[scaling::bench(group = "overlap range", name = "collect (arrival + decode)", baseline)]
fn collect_range(s: &Schedule) -> usize {
    let source = Timed::new(&s.range, CHUNKS, s.range_interval);
    let buf = block_on(collect(source));
    compactly::v2::decode::<Vec<u64>>(&buf).unwrap().len()
}

#[scaling::bench(group = "overlap range", name = "overlap (decode hidden)")]
fn overlap_range(s: &Schedule) -> usize {
    let source = Timed::new(&s.range, CHUNKS, s.range_interval);
    block_on(decode_stream::<Vec<u64>, _, _>(source))
        .unwrap()
        .len()
}

#[scaling::bench(group = "overlap ans", name = "collect (arrival + decode)", baseline)]
fn collect_ans(s: &Schedule) -> usize {
    let source = Timed::new(&s.ans, CHUNKS, s.ans_interval);
    let buf = block_on(collect(source));
    Ans::decode::<Vec<u64>>(&buf).unwrap().len()
}

#[scaling::bench(group = "overlap ans", name = "overlap (decode hidden)")]
fn overlap_ans(s: &Schedule) -> usize {
    let source = Timed::new(&s.ans, CHUNKS, s.ans_interval);
    block_on(Ans::decode_stream::<Vec<u64>, _, _>(source))
        .unwrap()
        .len()
}
