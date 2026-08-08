// Two A/Bs for the streaming ANS decoder, both decoding the same bytes in the
// same build. The source is always an in-memory slice, so nothing here measures
// the filesystem — the point is to compare decoder machinery, not IO.
//
//   slice | stream       `Ans::decode` (borrowing slice decoder) against
//                        `Ans::decode_from` (owning streaming decoder) over a
//                        `Cursor<&[u8]>`. Asks what the streaming decoder costs
//                        when it is handed memory, i.e. whether the slice
//                        decoder still earns its keep as a separate impl.
//
//   untracked | tracked  `AnsDecoder`'s `CHUNKED = false` single-chunk fast path
//                        against `CHUNKED = true`, both forced. Isolates the
//                        per-op `ops_left` check and decrement. Single-chunk
//                        input only — forcing `false` on a multi-chunk stream
//                        would decode into the first boundary and stop.
//
// Usage: `[COUNT=n] just-decompress-stream slice|stream|untracked|tracked`
//
// COUNT (default 2000) sets how many u64s per value; ITERS is derived so total
// values decoded stays fixed, keeping runtimes comparable across sizes. 2000
// stays under CHUNK_OPS (single chunk, cache-resident); 100000 spans several
// chunks and is memory-bound.
use compactly::v2::Ans;

const DEFAULT_COUNT: usize = 2_000;
/// Total u64s decoded per run, spread over however many iterations `COUNT` implies.
const VALUE_BUDGET: usize = 40_000_000;

fn decode_slice(compressed: &[u8], iters: usize) -> usize {
    let mut total = 0;
    for _ in 0..iters {
        total += std::hint::black_box(Ans::decode::<Vec<u64>>(compressed))
            .unwrap()
            .len();
    }
    total
}

fn decode_stream(compressed: &[u8], iters: usize) -> usize {
    let mut total = 0;
    for _ in 0..iters {
        let cursor = std::io::Cursor::new(compressed);
        total += std::hint::black_box(Ans::decode_from::<Vec<u64>, _>(cursor))
            .unwrap()
            .len();
    }
    total
}

fn decode_stream_forced<const CHUNKED: bool>(compressed: &[u8], iters: usize) -> usize {
    let mut total = 0;
    for _ in 0..iters {
        let cursor = std::io::Cursor::new(compressed);
        total += std::hint::black_box(Ans::decode_from_forced::<Vec<u64>, _, CHUNKED>(cursor))
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
    let compressed = Ans::encode(&data);
    // The tag's low bit is bit 0 of its first LEB128 byte; even means the first
    // chunk is also the final one.
    let single_chunk = compressed[0] & 1 == 0;

    let which = std::env::args().nth(1).unwrap_or_default();
    // Each arm is separately monomorphized, so none carries a runtime test for
    // the variant inside its decode loop.
    let total = match which.as_str() {
        "slice" => decode_slice(&compressed, iters),
        "stream" => decode_stream(&compressed, iters),
        "untracked" | "tracked" => {
            assert!(
                single_chunk,
                "forced variants need single-chunk input; lower COUNT (got {count})"
            );
            if which == "untracked" {
                decode_stream_forced::<false>(&compressed, iters)
            } else {
                decode_stream_forced::<true>(&compressed, iters)
            }
        }
        _ => {
            eprintln!("usage: [COUNT=n] just-decompress-stream slice|stream|untracked|tracked");
            std::process::exit(2);
        }
    };
    println!(
        "{which}: count={count} iters={iters} single_chunk={single_chunk} \
         compressed={} total={total}",
        compressed.len()
    );
}
