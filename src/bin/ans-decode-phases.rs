//! Benchmark separating the two costs of ANS *decoding*:
//!
//! 1. **entropy** — the rANS state-advance and byte-refill work alone,
//!    measured by replaying the encoder's recorded op buffer (which supplies
//!    the probabilities and symbol intervals the adaptive contexts would
//!    produce) via `Ans::replay_entropy_decode`.
//! 2. **full decode** — the real `Ans::decode`, which additionally does the
//!    model work (context lookup/adaptation, tree walks) and constructs the
//!    decoded value.
//!
//! The difference is the model + value-construction cost. My goal is to use
//! this to decide where to focus decode optimization.
//!
//! Usage: `ans-decode-phases [iterations]` (default 500). Reads meteorite
//! names from `comparison/src/meteorites.csv` (falling back to
//! `../comparison/src/meteorites.csv`), so run it from the workspace root.

use std::collections::HashSet;
use std::time::Instant;

use compactly::v2::{Ans, EntropyCoder};

/// Extract the first (quote-aware) CSV field of each record, skipping the
/// header row. Good enough for the meteorite names; avoids a csv dependency.
/// Deduplicated through a `HashSet` so the resulting `Vec` is *not* sorted.
fn first_fields(csv: &str) -> Vec<String> {
    let mut out = HashSet::new();
    for line in csv.lines().skip(1) {
        let name = if let Some(quoted) = line.strip_prefix('"') {
            let Some(end) = quoted.find('"') else { continue };
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

fn main() {
    let iterations: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(500);
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let names = first_fields(&csv);
    println!(
        "decoding {} meteorite names, {iterations} iterations per phase",
        names.len()
    );

    let ops = <Ans as EntropyCoder>::encode(&names);
    let encoded = ops.clone().into_vec();
    println!("encoded size {}", encoded.len());

    // Entropy-only: replay the recorded ops against the bitstream, doing just
    // the rANS state advances and byte refills.
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ops.replay_entropy_decode(&encoded));
    }
    let entropy = start.elapsed();

    // Full decode: entropy + context adaptation + value construction.
    let start = Instant::now();
    for _ in 0..iterations {
        let decoded: Vec<String> =
            std::hint::black_box(Ans::decode(&encoded).expect("decode failed"));
        assert_eq!(decoded.len(), names.len());
    }
    let full = start.elapsed();

    let per_iter = |d: std::time::Duration| d.as_secs_f64() * 1e3 / iterations as f64;
    let model = full.saturating_sub(entropy);
    println!(
        "entropy only:        {:8.3} ms/iter  ({:5.1}% of full decode)",
        per_iter(entropy),
        100.0 * entropy.as_secs_f64() / full.as_secs_f64()
    );
    println!(
        "model+construction:  {:8.3} ms/iter  ({:5.1}% of full decode, by subtraction)",
        per_iter(model),
        100.0 * model.as_secs_f64() / full.as_secs_f64()
    );
    println!("full decode:         {:8.3} ms/iter", per_iter(full));
}
