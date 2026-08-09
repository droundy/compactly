// Investigation (task #4): can `RangeDecoder<&[u8]>` replace the bespoke slice
// decoder `arith::Decoder<'a>`, letting us delete a whole decoder type?
//
//   slice   `Range::decode`      — the hand-fused borrowing slice decoder.
//   stream  `Range::decode_from` — the streaming `RangeDecoder<R>` with R = &[u8]
//                                  (the most favorable reader: no Cursor, the
//                                  slice's own Read impl just advances a cursor).
//
// Same bytes, same build, each arm separately monomorphized so neither carries a
// runtime branch in its decode loop. Compare instruction counts (noise-free) and
// quiesced cycles:
//   bench perf stat -e instructions,cycles -- \
//     ./target/release/range-decode-collapse slice
//
// Usage: `[COUNT=n] range-decode-collapse slice|stream`
use compactly::v2::Range;

const DEFAULT_COUNT: usize = 2_000;
/// Total u64s decoded per run, spread over however many iterations `COUNT` implies.
const VALUE_BUDGET: usize = 40_000_000;

fn decode_slice(compressed: &[u8], iters: usize) -> usize {
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(Range::decode::<Vec<u64>>(compressed))
            .unwrap()
            .len();
    }
    total
}

fn decode_stream(compressed: &[u8], iters: usize) -> usize {
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(Range::decode_from::<Vec<u64>, &[u8]>(compressed))
            .unwrap()
            .len();
    }
    total
}

fn main() {
    let count: usize = std::env::var("COUNT")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(DEFAULT_COUNT);
    let iters = (VALUE_BUDGET / count).max(1);

    let mut x = 0x123456789abcdef0u64;
    let data: Vec<u64> = (0..count)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            x
        })
        .collect();
    let compressed = Range::encode(&data);

    let which = std::env::args().nth(1).unwrap_or_default();
    let total = match which.as_str() {
        "slice" => decode_slice(&compressed, iters),
        "stream" => decode_stream(&compressed, iters),
        _ => {
            eprintln!("usage: [COUNT=n] range-decode-collapse slice|stream");
            std::process::exit(2);
        }
    };
    println!(
        "{which}: count={count} iters={iters} compressed={} total={total}",
        compressed.len()
    );
}
