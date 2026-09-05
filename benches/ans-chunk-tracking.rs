// What does `AnsDecoder`'s per-op chunk-boundary tracking cost?
//
// `untracked` forces the `CHUNKED = false` single-chunk fast path, `tracked`
// the `CHUNKED = true` general one, on the same bytes in the same build — so
// the only difference is the per-op `ops_left` check and decrement. Both are
// separately monomorphized, and neither carries a runtime test for the variant
// inside its decode loop.
//
// Single-chunk input only: forcing `false` on a multi-chunk stream would decode
// into the first boundary and stop, so that is asserted rather than trusted.
//
// The `slice` vs `decode_from` comparison this bin used to also carry is now
// `coder-routes u64 ans slice|from`, which measures the same thing for either
// coder on any workload.
//
// Usage: `[COUNT=n] ans-chunk-tracking untracked|tracked`
//
// COUNT (default 2000) sets how many u64s per value, and must stay under
// CHUNK_OPS for the forced arms to be valid.
use common::{args, per_unit, print, report};
use compactly::v2::Ans;

mod common;

const DEFAULT_COUNT: usize = 2_000;

fn main() {
    let count: usize = std::env::var("COUNT")
        .ok()
        .and_then(|c| c.parse().ok())
        .unwrap_or(DEFAULT_COUNT);

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

    let which = args()
        .first()
        .cloned()
        .unwrap_or_else(|| "tracked".to_string());
    eprintln!(
        "{which}: count={count} single_chunk={single_chunk} compressed={}",
        compressed.len()
    );
    assert!(
        single_chunk,
        "the forced variants need single-chunk input; lower COUNT (got {count})"
    );
    let stats = match which.as_str() {
        "untracked" => report("decode Vec<u64> (CHUNKED=false)", || {
            Ans::decode_from_forced::<Vec<u64>, _, false>(std::io::Cursor::new(&compressed))
                .unwrap()
                .len()
        }),
        "tracked" => report("decode Vec<u64> (CHUNKED=true)", || {
            Ans::decode_from_forced::<Vec<u64>, _, true>(std::io::Cursor::new(&compressed))
                .unwrap()
                .len()
        }),
        _ => {
            eprintln!("usage: [COUNT=n] ans-chunk-tracking untracked|tracked");
            std::process::exit(2);
        }
    };
    print("  per value", &per_unit(&stats, count));
}
