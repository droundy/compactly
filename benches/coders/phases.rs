//! Where does ANS time go? Both directions split into their two phases, over
//! the meteorite names.
//!
//! In each group the *whole* is the baseline and the *part* is a candidate, so
//! the part's row reads as a fraction of the whole (`-60%` means the part is 40%
//! of it) and the remainder is the other phase. That comparison is paired —
//! both are measured in the same round on the same input — so its error bar is
//! tighter than subtracting two independent timings, which could not see how
//! much of their noise they share.
//!
//! **decode** (`ans decode phases`):
//!
//! 1. `entropy only` — the rANS state-advance and byte-refill work alone,
//!    measured by replaying the encoder's recorded op buffer (which supplies the
//!    probabilities and symbol intervals the adaptive contexts would produce)
//!    via `Ans::replay_entropy_decode`.
//! 2. `full decode` — the real `Ans::decode`, which additionally does the model
//!    work (context lookup and adaptation, tree walks) and constructs the value.
//!
//! The entropy replay needs the op buffer, which only survives while the value
//! fits in one chunk (flushing clears it), so the input is trimmed to the
//! largest single-chunk prefix. Both phases measure the same data, so the
//! comparison is unaffected.
//!
//! **encode** (`ans encode phases`):
//!
//! 1. `build only` — `Encode::encode` filling the `Ans` op buffer: the model
//!    work.
//! 2. `build + into_vec` — that plus running the rANS coder backwards over the
//!    buffer to produce the bitstream. With chunking most of the entropy coding
//!    already happened during phase 1 (each full chunk is flushed as it fills),
//!    so what `into_vec` adds is the trailing chunk only.
//!
//! The names are shuffled with a fixed seed. They used to be put in unsorted
//! order by a `HashSet`, which was deliberate (a sorted list would flatter a
//! tree) but also varied from one process to the next.

use std::rc::Rc;

use compactly::v2::{Ans, EntropyCoder};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

fn shuffled_names() -> Vec<String> {
    let mut names: Vec<String> = comparison::meteorites::meteorite_names()
        .into_keys()
        .collect();
    names.shuffle(&mut SmallRng::seed_from_u64(7));
    names
}

// ---------- encode ----------

#[derive(Clone)]
struct Names(Rc<Vec<String>>);

thread_local! {
    static NAMES: Names = Names(Rc::new(shuffled_names()));
}

#[scaling::input(group = "ans encode phases", name = "meteorite names")]
fn encode_input() -> Names {
    NAMES.with(Names::clone)
}

#[scaling::bench(group = "ans encode phases", name = "build + into_vec", baseline)]
fn build_and_flush(names: &Names) -> usize {
    <Ans as EntropyCoder>::encode(&*names.0).into_vec().len()
}

/// Returns the coder itself, so that the work of filling the op buffer cannot be
/// optimized away and nothing but dropping it is added.
#[scaling::bench(group = "ans encode phases", name = "build only")]
fn build_only(names: &Names) -> Ans {
    <Ans as EntropyCoder>::encode(&*names.0)
}

// ---------- decode ----------

#[derive(Clone)]
struct DecodePhases {
    /// The encoder, kept as the op-buffer oracle for the entropy replay.
    ops: Rc<Ans>,
    /// What a second, finished encode wrote (`into_vec` consumes the first).
    encoded: Rc<Vec<u8>>,
    names: usize,
}

fn decode_phases() -> DecodePhases {
    let all = shuffled_names();
    // Binary searched rather than hardcoded, so it tracks `CHUNK_OPS` and keeps
    // as much data as will fit (ops per name vary, so this cannot be computed
    // directly).
    let fits = |n: usize| <Ans as EntropyCoder>::encode(&all[..n].to_vec()).is_single_chunk();
    let (mut lo, mut hi) = (1, all.len());
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if fits(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let names = all[..lo].to_vec();
    assert!(!names.is_empty(), "even one name should fit in a chunk");
    if names.len() < all.len() {
        eprintln!(
            "ans decode phases: trimmed {} -> {} names to stay within one chunk",
            all.len(),
            names.len()
        );
    }
    DecodePhases {
        ops: Rc::new(<Ans as EntropyCoder>::encode(&names)),
        encoded: Rc::new(<Ans as EntropyCoder>::encode(&names).into_vec()),
        names: names.len(),
    }
}

thread_local! {
    static DECODE: DecodePhases = decode_phases();
}

#[scaling::input(group = "ans decode phases", name = "meteorite names")]
fn decode_input() -> DecodePhases {
    DECODE.with(DecodePhases::clone)
}

#[scaling::bench(group = "ans decode phases", name = "full decode", baseline)]
fn full_decode(p: &DecodePhases) -> usize {
    let decoded: Vec<String> = Ans::decode(&p.encoded).expect("decode failed");
    assert_eq!(decoded.len(), p.names);
    decoded.len()
}

#[scaling::bench(group = "ans decode phases", name = "entropy only")]
fn entropy_only(p: &DecodePhases) -> u32 {
    p.ops.replay_entropy_decode(&p.encoded)
}
