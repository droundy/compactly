//! Where does ANS coding time actually go? Both directions split into their
//! two phases, on the same corpus.
//!
//! **decode** (`ans-phases decode`):
//!
//! 1. **entropy** — the rANS state-advance and byte-refill work alone,
//!    measured by replaying the encoder's recorded op buffer (which supplies
//!    the probabilities and symbol intervals the adaptive contexts would
//!    produce) via `Ans::replay_entropy_decode`.
//! 2. **full decode** — the real `Ans::decode`, which additionally does the
//!    model work (context lookup/adaptation, tree walks) and constructs the
//!    decoded value.
//!
//! **encode** (`ans-phases encode`):
//!
//! 1. **build** — running `Encode::encode` to fill the `Ans` op buffer
//!    (`Vec<Op>`), i.e. the model/context work.
//! 2. **into_vec** — running the rANS coder backwards over that buffer to
//!    produce the bitstream. With chunking most of the entropy coding already
//!    happened during phase 1 (each full chunk is flushed as it fills), so
//!    this is the trailing chunk only.
//!
//! Either way the second phase is isolated by subtraction, and the error bars
//! add in quadrature — so the derived line is known less precisely than the
//! two it came from, and says so.
//!
//! Usage: `ans-phases encode|decode` (default `decode`). Reads meteorite names
//! from `comparison/src/meteorites.csv` (falling back to
//! `../comparison/src/meteorites.csv`), so run it from the workspace root.

use std::collections::HashSet;

use common::{args, difference, print, report};
use compactly::v2::{Ans, EntropyCoder};

/// Extract the first (quote-aware) CSV field of each record, skipping the
/// header row. Good enough for the meteorite names; avoids a csv dependency.
/// Deduplicated through a `HashSet` so the resulting `Vec` is *not* sorted.
fn first_fields(csv: &str) -> Vec<String> {
    let mut out = HashSet::new();
    for line in csv.lines().skip(1) {
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
    out.into_iter().collect()
}

/// Split encode into "fill the op buffer" and "run the coder over it".
fn encode_phases(names: &[String]) {
    let names = names.to_vec();
    eprintln!(
        "encoded size {}",
        <Ans as EntropyCoder>::encode(&names).into_vec().len()
    );
    let build = report("build Vec<Op>", || <Ans as EntropyCoder>::encode(&names));
    // No clone needed to isolate the flush: `Ans` owns its writer, so timing
    // `encode + into_vec` and subtracting the build above leaves the
    // final-chunk flush.
    let both = report("build + into_vec", || {
        <Ans as EntropyCoder>::encode(&names).into_vec().len()
    });
    let into_vec = difference(&both, &build);
    print("into_vec (by difference)", &into_vec);
    println!(
        "build {:.1}% / into_vec {:.1}% of encode",
        100.0 * build.ns_per_iter / both.ns_per_iter,
        100.0 * into_vec.ns_per_iter / both.ns_per_iter,
    );
}

/// Split decode into the entropy coder alone and everything the model and the
/// value construction add on top.
///
/// The entropy phase replays the encoder's op buffer, which only survives
/// while the value fits in one chunk (flushing clears it), so the input is
/// trimmed to the largest single-chunk prefix. Both phases then measure the
/// same data, so the comparison — the point of this tool — is unaffected.
fn decode_phases(all_names: &[String]) {
    // Binary searched rather than hardcoded, so it tracks CHUNK_OPS and keeps
    // as much data as will fit (ops per name vary, so this cannot be computed
    // directly).
    let fits = |n: usize| <Ans as EntropyCoder>::encode(&all_names[..n].to_vec()).is_single_chunk();
    let mut lo = 1;
    let mut hi = all_names.len();
    while lo < hi {
        let mid = (lo + hi).div_ceil(2);
        if fits(mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let names: Vec<String> = all_names[..lo].to_vec();
    assert!(!names.is_empty(), "even one name should fit in a chunk");
    if names.len() < all_names.len() {
        eprintln!(
            "trimmed {} -> {} names to stay within one chunk",
            all_names.len(),
            names.len()
        );
    }

    // Two independent encodes: one kept as the op-buffer oracle for the
    // entropy replay, one finished into the bitstream (into_vec consumes).
    let ops = <Ans as EntropyCoder>::encode(&names);
    let encoded = <Ans as EntropyCoder>::encode(&names).into_vec();
    eprintln!("encoded size {}", encoded.len());

    let entropy = report("entropy only", || ops.replay_entropy_decode(&encoded));
    let full = report("full decode", || {
        let decoded: Vec<String> = Ans::decode(&encoded).expect("decode failed");
        assert_eq!(decoded.len(), names.len());
        decoded.len()
    });
    let model = difference(&full, &entropy);
    print("model+construction (by diff)", &model);
    println!(
        "entropy {:.1}% / model {:.1}% of full decode",
        100.0 * entropy.ns_per_iter / full.ns_per_iter,
        100.0 * model.ns_per_iter / full.ns_per_iter,
    );
}

mod common;

fn main() {
    let which = args()
        .first()
        .cloned()
        .unwrap_or_else(|| "decode".to_string());
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let names = first_fields(&csv);
    eprintln!("{which} phases over {} meteorite names", names.len());
    match which.as_str() {
        "encode" => encode_phases(&names),
        "decode" => decode_phases(&names),
        other => {
            eprintln!("unknown phase {other:?}; usage: ans-phases encode|decode");
            std::process::exit(2);
        }
    }
}
