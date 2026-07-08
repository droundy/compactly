//! Benchmark separating the two phases of ANS *encoding*:
//!
//! 1. **build** — running `Encode::encode` to fill the `Ans` op buffer
//!    (`Vec<Op>`), i.e. the model/context work.
//! 2. **into_vec** — running the rANS coder backwards over that buffer to
//!    produce the bitstream.
//!
//! `into_vec` consumes the `Ans`, so phase 2 is timed as `clone + into_vec`
//! with the clone cost measured separately and subtracted.
//!
//! Usage: `ans-encode-phases [iterations]` (default 200). Reads meteorite
//! names from `comparison/src/meteorites.csv` (falling back to
//! `../comparison/src/meteorites.csv`), so run it from the workspace root.

use std::collections::BTreeSet;
use std::time::Instant;

use compactly::v2::{Ans, EntropyCoder};

/// Extract the first (quote-aware) CSV field of each record, skipping the
/// header row. Good enough for the meteorite names; avoids a csv dependency.
fn first_fields(csv: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
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
    out
}

fn main() {
    let iterations: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(200);
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let names = first_fields(&csv);
    println!(
        "encoding {} meteorite names, {iterations} iterations per phase",
        names.len()
    );

    // Phase 1: build the op buffer.
    let start = Instant::now();
    let mut ops = Ans::default();
    for _ in 0..iterations {
        ops = std::hint::black_box(<Ans as EntropyCoder>::encode(&names));
    }
    let build = start.elapsed();

    // Clone cost (needed below because into_vec consumes the Ans).
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(ops.clone());
    }
    let clone = start.elapsed();

    // Phase 2: entropy-code the op buffer into the bitstream.
    let start = Instant::now();
    let mut encoded = Vec::new();
    for _ in 0..iterations {
        encoded = std::hint::black_box(ops.clone().into_vec());
    }
    let clone_plus_into_vec = start.elapsed();
    let into_vec = clone_plus_into_vec.saturating_sub(clone);

    println!("encoded size {}", encoded.len());
    let per_iter = |d: std::time::Duration| d.as_secs_f64() * 1e3 / iterations as f64;
    let total = build + into_vec;
    println!(
        "build Vec<Op>:  {:8.3} ms/iter  ({:5.1}% of encode)",
        per_iter(build),
        100.0 * build.as_secs_f64() / total.as_secs_f64()
    );
    println!(
        "into_vec:       {:8.3} ms/iter  ({:5.1}% of encode)",
        per_iter(into_vec),
        100.0 * into_vec.as_secs_f64() / total.as_secs_f64()
    );
    println!(
        "(clone, subtracted from into_vec: {:.3} ms/iter)",
        per_iter(clone)
    );
}
