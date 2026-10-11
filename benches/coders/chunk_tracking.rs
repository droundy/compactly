//! What does `AnsDecoder`'s per-op chunk-boundary tracking cost?
//!
//! `untracked` forces the `CHUNKED = false` single-chunk fast path, `tracked` the
//! `CHUNKED = true` general one, on the same bytes in the same build — so the
//! only difference is the per-op `ops_left` check and decrement. Both are
//! separately monomorphized, and neither carries a runtime test for the variant
//! inside its decode loop.
//!
//! Single-chunk input only: forcing `false` on a multi-chunk stream would decode
//! into the first boundary and stop, so the input asserts that rather than
//! trusting it. The count (2000 `u64`s) stays under `CHUNK_OPS` for that reason.
//!
//! The `slice` against `decode_from` comparison this bench used to carry is now
//! `routes.rs`, which measures the same thing for either coder on any workload.

use std::rc::Rc;

use compactly::v2::Ans;

const COUNT: usize = 2_000;

fn encoded() -> Rc<Vec<u8>> {
    let mut x = 0x123456789abcdef0u64;
    let data: Vec<u64> = (0..COUNT)
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
    assert!(
        compressed[0] & 1 == 0,
        "the forced variants need single-chunk input; lower COUNT (got {COUNT})"
    );
    Rc::new(compressed)
}

thread_local! {
    static BYTES: Rc<Vec<u8>> = encoded();
}

#[scaling::input(group = "chunk tracking", name = "2000 u64")]
fn input() -> Rc<Vec<u8>> {
    BYTES.with(Rc::clone)
}

#[scaling::bench(group = "chunk tracking", name = "untracked (CHUNKED=false)", baseline)]
fn untracked(bytes: &Rc<Vec<u8>>) -> usize {
    Ans::decode_from_forced::<Vec<u64>, _, false>(std::io::Cursor::new(&bytes[..]))
        .unwrap()
        .len()
}

#[scaling::bench(group = "chunk tracking", name = "tracked (CHUNKED=true)")]
fn tracked(bytes: &Rc<Vec<u8>>) -> usize {
    Ans::decode_from_forced::<Vec<u64>, _, true>(std::io::Cursor::new(&bytes[..]))
        .unwrap()
        .len()
}
