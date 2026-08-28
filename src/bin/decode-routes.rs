//! The same bytes decoded six ways: three decode routes × both coders.
//!
//! The `just-decompress-*` bins each compare `Ans` against `Range` on **one**
//! route, the borrowing slice decoder, which is the fastest path and not the one
//! a streaming caller takes. That leaves the interesting question unanswered: a
//! coder that wins on slices can lose once the bytes arrive through a `Read` or
//! a `Stream`, because the routes differ in what they can keep in registers and
//! in how much they must buffer before decoding anything.
//!
//! So this runs the whole matrix over one workload set:
//!
//!   slice   `Ans::decode` / `v2::decode` — borrows the buffer.
//!   from    `Ans::decode_from` / `Range::decode_from` over a `&[u8]` used as a
//!           `Read`. Owned-ish, one byte per `read`, no filesystem in the loop.
//!   stream  `Ans::decode_stream` / `v2::decode_stream` over a chunked source.
//!
//! The workloads are the ones the `just-decompress-*` bins already use, brought
//! together so every route runs identical harness code — a comparison across
//! routes is only worth anything if nothing but the route differs.
//!
//! Usage: `decode-routes <workload> [ans|range] [slice|from|stream] [iters]`
//!
//! Workloads: `strings`, `enums`, `enums17`, `floats`, `compressible`,
//! `records`, `records-wide`, and `atmost<N>` for N in the monomorphized ladder
//! (3 4 6 8 12 16 24 32 64 128). All but `enums`, `enums17`, `floats` and the
//! ladder read `comparison/src/meteorites.csv`, so run from the workspace root.
//!
//! `records`/`records-wide` are the same types and corpus `async-decode-cost`
//! uses, so their sizes and slice numbers line up with that bin's.
//!
//! `CHUNKS` (default 64) sets how many pieces the `stream` route delivers the
//! buffer in; it is the only knob that changes what that route has in hand.
use std::collections::BTreeSet;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use compactly::v2::{decode_stream, Ans, AtMost, Encode, Range};
use compactly::{Compressible, Encoded};
use futures_core::Stream;

/// Deliver the buffer in `CHUNKS` equal pieces, never `Pending` — this measures
/// the decode route, not suspension, and `block_on` asserts that by panicking if
/// anything suspends.
struct Chunks(std::vec::IntoIter<Bytes>);

impl Chunks {
    fn new(bytes: &[u8], chunks: usize) -> Self {
        let all = Bytes::copy_from_slice(bytes);
        let size = all.len().div_ceil(chunks.max(1)).max(1);
        let mut parts = Vec::new();
        let mut start = 0;
        while start < all.len() {
            let end = (start + size).min(all.len());
            parts.push(all.slice(start..end));
            start = end;
        }
        if parts.is_empty() {
            parts.push(all);
        }
        Chunks(parts.into_iter())
    }
}

impl Stream for Chunks {
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(self.0.next().map(Ok))
    }
}

/// Drive a future known never to suspend. Hand-rolled because
/// `futures-executor` is a dev-dependency and so is not available to `src/bin`,
/// and a real executor would put its own scheduling in the measurement.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("future suspended: the source was supposed to be fully buffered"),
    }
}

fn chunks() -> usize {
    std::env::var("CHUNKS")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(64)
}

/// Encode once with `coder`, then decode `iters` times by `route`, folding
/// `measure` over each decoded value so the decode cannot be optimized away.
///
/// The coder and route are branched on **once**, outside the loop, so the
/// measurement contains the decode and nothing else.
fn run<T: Encode>(
    coder: &str,
    route: &str,
    iters: usize,
    value: &T,
    measure: impl Fn(&T) -> usize,
) -> usize {
    let encoded = match coder {
        "ans" => Ans::encode(value),
        "range" => compactly::v2::encode(value),
        other => panic!("unknown coder {other:?}; use ans|range"),
    };
    println!("encoded size {}", encoded.len());
    let n = chunks();
    let mut total = 0usize;
    macro_rules! loop_over {
        ($decode:expr) => {
            for _ in 0..iters {
                total += measure(&std::hint::black_box($decode).unwrap());
            }
        };
    }
    match (coder, route) {
        ("ans", "slice") => loop_over!(Ans::decode::<T>(&encoded)),
        ("range", "slice") => loop_over!(compactly::v2::decode::<T>(&encoded)),
        ("ans", "from") => loop_over!(Ans::decode_from::<T, _>(&encoded[..])),
        ("range", "from") => loop_over!(Range::decode_from::<T, _>(&encoded[..])),
        ("ans", "stream") => {
            loop_over!(block_on(Ans::decode_stream::<T, _, _>(Chunks::new(
                &encoded, n
            ))))
        }
        ("range", "stream") => {
            loop_over!(block_on(decode_stream::<T, _, _>(Chunks::new(&encoded, n))))
        }
        (_, other) => panic!("unknown route {other:?}; use slice|from|stream"),
    }
    total
}

/// Extract the first (quote-aware) CSV field of each record, skipping the header
/// row — the same reader `just-decompress-strings` uses, so the corpus matches.
fn meteorite_names() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in csv().lines().skip(1) {
        let name = if let Some(quoted) = line.strip_prefix('"') {
            let Some(end) = quoted.find('"') else {
                continue;
            };
            quoted[..end].to_string()
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
    out
}

fn csv() -> String {
    std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found")
}

fn lcg() -> impl FnMut() -> u64 {
    seeded_lcg(0x123456789abcdef0)
}

fn seeded_lcg(seed: u64) -> impl FnMut() -> u64 {
    let mut x = seed;
    move || {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        x
    }
}

/// A record shaped the way streamed data usually is: a short string beside a
/// couple of integers. Same definition and same corpus as `async-decode-cost`'s
/// workload of this name, so the sizes and the slice numbers line up.
#[derive(Debug, Clone, PartialEq, compactly::v2::Encode)]
struct Record {
    id: u64,
    count: u32,
    name: String,
    active: bool,
}

/// The same record with the integer side widened, which holds the number of
/// string characters fixed while adding non-string work around them.
#[derive(Debug, Clone, PartialEq, compactly::v2::Encode)]
struct WideRecord {
    id: u64,
    count: u32,
    a: u64,
    b: u64,
    c: u32,
    d: u16,
    name: String,
    active: bool,
}

fn records() -> (Vec<Record>, Vec<WideRecord>) {
    let mut rng = seeded_lcg(0x243f_6a88_85a3_08d3);
    let names: Vec<String> = meteorite_names().into_iter().collect();
    let mut narrow = Vec::with_capacity(names.len());
    let mut wide = Vec::with_capacity(names.len());
    for name in names {
        let (id, r) = (rng(), rng());
        narrow.push(Record {
            id,
            count: r as u32,
            name: name.clone(),
            active: r & 1 == 0,
        });
        wide.push(WideRecord {
            id,
            count: r as u32,
            a: rng(),
            b: rng(),
            c: rng() as u32,
            d: rng() as u16,
            name,
            active: r & 1 == 0,
        });
    }
    (narrow, wide)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, compactly::v2::Encode)]
enum ThreeOptions {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, compactly::v2::Encode)]
#[rustfmt::skip]
enum SeventeenOptions {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,
}

/// 20% A, 20% B, 60% C — the same skew `just-decompress-enums` uses, so the
/// slice numbers here are comparable with that bin's.
fn three() -> Vec<ThreeOptions> {
    let mut rng = lcg();
    (0..100_000)
        .map(|_| match (rng() >> 33) % 10 {
            0 | 1 => ThreeOptions::A,
            2 | 3 => ThreeOptions::B,
            _ => ThreeOptions::C,
        })
        .collect()
}

fn seventeen() -> Vec<SeventeenOptions> {
    use SeventeenOptions::*;
    const VARIANTS: [SeventeenOptions; 17] = [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q];
    let mut rng = lcg();
    (0..100_000)
        .map(|_| VARIANTS[((rng() >> 33) % 17) as usize])
        .collect()
}

/// Non-integer `f64`s, so decode takes the raw-bits path rather than the
/// integer fast path — the same corpus `just-decompress-floats` builds.
fn floats() -> Vec<f64> {
    let mut rng = lcg();
    (0..100_000)
        .map(|_| {
            let x = rng();
            let bits = if (x >> 52) & 0x7FF == 0x7FF {
                x & !(1u64 << 52)
            } else {
                x
            };
            f64::from_bits(bits)
        })
        .collect()
}

fn atmost<const MAX: usize>(coder: &str, route: &str, iters: usize) -> usize {
    let mut rng = lcg();
    let data: Vec<AtMost<MAX>> = (0..50_000)
        .map(|_| AtMost::<MAX>::new(((rng() >> 33) as usize) % (MAX + 1)))
        .collect();
    run(coder, route, iters, &data, |v| v.len())
}

fn main() {
    let workload = std::env::args().nth(1).unwrap_or_default();
    let coder = std::env::args()
        .find(|a| a == "ans" || a == "range")
        .unwrap_or_else(|| "ans".to_string());
    let route = std::env::args()
        .find(|a| a == "slice" || a == "from" || a == "stream")
        .unwrap_or_else(|| "slice".to_string());
    let iters: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(2000);
    println!("{workload} / {coder} / {route}, {iters} iterations");

    let total = match workload.as_str() {
        "strings" => run(&coder, &route, iters, &meteorite_names(), |v| v.len()),
        "enums" => run(&coder, &route, iters, &three(), |v| v.len()),
        "enums17" => run(&coder, &route, iters, &seventeen(), |v| v.len()),
        "floats" => run(&coder, &route, iters, &floats(), |v| v.len()),
        "compressible" => {
            let corpus: Encoded<Vec<u8>, Compressible> = Encoded::new(csv().into_bytes());
            run(&coder, &route, iters, &corpus, |v| v.len())
        }
        "records" => run(&coder, &route, iters, &records().0, |v| v.len()),
        "records-wide" => run(&coder, &route, iters, &records().1, |v| v.len()),
        // The ladder is monomorphized, so `MAX` has to be a literal here; these
        // are the values `just-decompress-uless` compiles.
        "atmost3" => atmost::<2>(&coder, &route, iters),
        "atmost4" => atmost::<3>(&coder, &route, iters),
        "atmost6" => atmost::<5>(&coder, &route, iters),
        "atmost8" => atmost::<7>(&coder, &route, iters),
        "atmost12" => atmost::<11>(&coder, &route, iters),
        "atmost16" => atmost::<15>(&coder, &route, iters),
        "atmost24" => atmost::<23>(&coder, &route, iters),
        "atmost32" => atmost::<31>(&coder, &route, iters),
        "atmost64" => atmost::<63>(&coder, &route, iters),
        "atmost128" => atmost::<127>(&coder, &route, iters),
        other => {
            eprintln!(
                "unknown workload {other:?}\n\
                 usage: decode-routes <workload> [ans|range] [slice|from|stream] [iters]\n\
                 workloads: strings enums enums17 floats compressible \
                 records records-wide atmost{{3,4,6,8,12,16,24,32,64,128}}"
            );
            std::process::exit(2);
        }
    };
    println!("total decoded {total}");
}
