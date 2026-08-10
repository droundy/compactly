// What does the async decode path cost, with no IO in the measurement?
//
// Three arms decode the same bytes in the same build. The async arm is fed the
// whole compressed buffer as a *single* `Bytes`, so no await ever suspends —
// what is left is purely the machinery (the generated state machines, the owned
// buffer, the index cursor), not concurrency. That makes this deterministic and
// therefore instruction-countable with the usual quiesced `perf stat` method,
// unlike the overlap claim, which needs wall-clock.
//
//   slice   `v2::decode` — the bespoke borrowing slice decoder, the fastest
//           path and the one most callers use today.
//
//   stream  `Range::decode_from` over a `&[u8]` — the sync *streaming* decoder.
//           Owned-ish, pulls one byte per `Read::read`. Included because it is
//           the honest baseline for the async arm: comparing async against
//           `slice` alone would bill it for the borrowed-vs-owned difference
//           that OPTIMIZING.md already measured separately.
//
//   async   `v2::stream::decode_stream` over a one-chunk stream — which its
//           look-ahead routes to the sync slice decoder, so this measures the
//           single-chunk fast path and should land on top of `slice`.
//
//   async-split  the same bytes as two chunks, which defeats the look-ahead and
//           forces the async decoder. This is what measures the machinery.
//
// So `async-split | stream` isolates the async machinery, `async | slice` shows
// what the fast path costs, and `stream | slice` is the already-known cost of
// not borrowing.
//
// Usage: `[COUNT=n] async-decode-cost slice|stream|async|async-split|ans-slice|ans-async [u64|strings]`
//
// COUNT (default 2000) sets how many u64s per value; ITERS is derived so the
// total number of values decoded stays fixed, keeping runtimes comparable
// across sizes. 2000 is cache-resident, 100000 is memory-bound. The `strings`
// workload uses the meteorite names, so run it from the workspace root.
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use compactly::v2::{decode_stream, Ans, DecodeAsync, Encode, Range};
use compactly::Normal;
use futures_core::Stream;

/// How many chunks the `async-split` arm delivers. 2 (a single byte, then the
/// rest) is the smallest split that defeats the single-chunk look-ahead, and so
/// the *best* case for the final-chunk handoff — everything after byte one is
/// decoded synchronously. Larger values show how the benefit scales, since the
/// handoff can only happen once the last chunk is in hand: with `n` equal
/// chunks, only the final `1/n` of the input is reached with nothing left to
/// wait for.
fn split_chunks() -> usize {
    std::env::var("CHUNKS")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(2)
}

const DEFAULT_COUNT: usize = 2_000;
/// Total values decoded per run, spread over however many iterations `COUNT`
/// implies.
const VALUE_BUDGET: usize = 40_000_000;
/// The strings workload is far more work per element, so it gets its own budget.
const STRING_VALUE_BUDGET: usize = 4_000_000;

/// The compressed buffer as either one chunk or two. Deliberately never
/// `Pending`: this benchmark measures the async machinery, not suspension, and
/// `block_on` below asserts that by panicking if anything ever does suspend.
///
/// One chunk is what `decode_stream`'s look-ahead routes to the sync slice
/// decoder, so `async` measures the fast path. Splitting is what forces the
/// async decoder, and the split is a single byte off the front — the smallest
/// thing that defeats the look-ahead, so the two arms differ in *which decoder
/// runs* and in essentially nothing else. (A trailing empty chunk would not
/// work: empty chunks are transparent to the source, by design.)
struct Chunks(std::vec::IntoIter<Bytes>);

impl Chunks {
    fn new(bytes: &[u8], chunks: usize) -> Self {
        let all = Bytes::copy_from_slice(bytes);
        let parts = if chunks <= 1 || all.len() < 2 {
            vec![all]
        } else if chunks == 2 {
            vec![all.slice(..1), all.slice(1..)]
        } else {
            let size = all.len().div_ceil(chunks);
            let mut parts = Vec::new();
            let mut start = 0;
            while start < all.len() {
                let end = (start + size).min(all.len());
                parts.push(all.slice(start..end));
                start = end;
            }
            parts
        };
        Chunks(parts.into_iter())
    }
}

impl Stream for Chunks {
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.next().map(Ok))
    }
}

/// Drive a future that is known never to suspend.
///
/// Hand-rolled rather than pulling in an executor: `futures-executor` is a
/// dev-dependency and so is not available to `src/bin`, and a real executor
/// would put its own scheduling in the measurement. Panicking on `Pending`
/// makes the "never suspends" property an assertion rather than a comment.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("future suspended: the source was supposed to be a single chunk"),
    }
}

fn decode_slice<T: Encode + Len>(compressed: &[u8], iters: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(compactly::v2::decode::<T>(compressed))
            .unwrap()
            .len();
    }
    total
}

fn decode_sync_stream<T: Encode + Len>(compressed: &[u8], iters: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(Range::decode_from::<T, _>(compressed))
            .unwrap()
            .len();
    }
    total
}

fn decode_ans_slice<T: Encode + Len>(compressed: &[u8], iters: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(Ans::decode::<T>(compressed))
            .unwrap()
            .len();
    }
    total
}

/// `Ans` has no single-chunk look-ahead: a frame is decodable only once it has
/// arrived whole, so the async decoder runs whatever the stream shape. What
/// varies with `chunks` is only how the frames are cut up on the way in.
fn decode_ans_async<T: Encode + Len>(compressed: &[u8], iters: usize, chunks: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    let mut total = 0;
    for _ in 0..iters {
        let source = Chunks::new(compressed, chunks);
        total += std::hint::black_box(block_on(Ans::decode_stream::<T, _, _>(source)))
            .unwrap()
            .len();
    }
    total
}

fn decode_async<T: Encode + Len>(compressed: &[u8], iters: usize, chunks: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    let mut total = 0;
    for _ in 0..iters {
        let source = Chunks::new(compressed, chunks);
        total += std::hint::black_box(block_on(decode_stream::<T, _, _>(source)))
            .unwrap()
            .len();
    }
    total
}

/// Just enough to keep the three arms generic over the workload without pulling
/// the element count out of band.
trait Len {
    fn len(&self) -> usize;
}
impl Len for Vec<u64> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}
impl Len for Vec<String> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

/// Extract the first (quote-aware) CSV field of each record, skipping the header
/// row — the same reader `ans-encode-phases` uses, to keep the workloads
/// comparable.
fn meteorite_names() -> Vec<String> {
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let mut out = std::collections::BTreeSet::new();
    for line in csv.lines().skip(1) {
        let name = if let Some(quoted) = line.strip_prefix('"') {
            match quoted.find('"') {
                Some(end) => quoted[..end].to_string(),
                None => continue,
            }
        } else {
            match line.split_once(',') {
                Some((first, _)) => first.to_string(),
                None => line.to_string(),
            }
        };
        if !name.is_empty() {
            out.insert(name);
        }
    }
    out.into_iter().collect()
}

fn run<T: Encode + Len>(which: &str, compressed: &[u8], iters: usize) -> usize
where
    Normal: DecodeAsync<T>,
{
    match which {
        "slice" => decode_slice::<T>(compressed, iters),
        "stream" => decode_sync_stream::<T>(compressed, iters),
        "async" => decode_async::<T>(compressed, iters, 1),
        "async-split" => decode_async::<T>(compressed, iters, split_chunks()),
        "ans-slice" => decode_ans_slice::<T>(compressed, iters),
        "ans-async" => decode_ans_async::<T>(compressed, iters, split_chunks()),
        _ => {
            eprintln!(
                "usage: [COUNT=n] async-decode-cost slice|stream|async|async-split|ans-slice|ans-async [u64|strings]"
            );
            std::process::exit(2);
        }
    }
}

fn main() {
    let count: usize = std::env::var("COUNT")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(DEFAULT_COUNT);
    let which = std::env::args().nth(1).unwrap_or_default();
    let workload = std::env::args().nth(2).unwrap_or_else(|| "u64".to_string());

    let (compressed, iters, elements) = match workload.as_str() {
        "u64" => {
            let mut x = 0x123456789abcdef0u64;
            let data: Vec<u64> = (0..count)
                .map(|_| {
                    x = x
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    x
                })
                .collect();
            let bytes = if which.starts_with("ans") {
                Ans::encode(&data)
            } else {
                compactly::v2::encode(&data)
            };
            (bytes, (VALUE_BUDGET / count).max(1), count)
        }
        "strings" => {
            let names = meteorite_names();
            let n = names.len();
            let bytes = if which.starts_with("ans") {
                Ans::encode(&names)
            } else {
                compactly::v2::encode(&names)
            };
            (bytes, (STRING_VALUE_BUDGET / n).max(1), n)
        }
        _ => {
            eprintln!("usage: [COUNT=n] async-decode-cost slice|stream|async [u64|strings]");
            std::process::exit(2);
        }
    };

    let total = match workload.as_str() {
        "u64" => run::<Vec<u64>>(&which, &compressed, iters),
        _ => run::<Vec<String>>(&which, &compressed, iters),
    };
    println!(
        "{which}/{workload}: elements={elements} iters={iters} compressed={} total={total}",
        compressed.len()
    );
}
