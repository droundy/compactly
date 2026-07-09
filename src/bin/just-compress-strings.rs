//! Focused benchmark for the per-character string-*encode* hot path (the
//! `char`/`u8` tree walk): encode a `BTreeSet<String>` of meteorite names
//! repeatedly. The decode-side counterpart is `just-decompress-strings`.
//!
//! Usage: `just-compress-strings [ans|range] [iterations]` (defaults: `ans`,
//! 2000). Reads the names from `comparison/src/meteorites.csv` (falling back
//! to `../comparison/src/meteorites.csv`), so run it from the workspace root.

use std::collections::BTreeSet;

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
    let coder = std::env::args()
        .find(|a| a == "ans" || a == "range")
        .unwrap_or("ans".to_string());
    let iterations: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(2000);
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let names = first_fields(&csv);
    println!("encoding {} meteorite names with {coder}", names.len());

    let mut total = 0usize;
    match coder.as_str() {
        "ans" => {
            for _ in 0..iterations {
                total += std::hint::black_box(compactly::v2::Ans::encode(&names)).len();
            }
        }
        "range" => {
            for _ in 0..iterations {
                total += std::hint::black_box(compactly::v2::encode(&names)).len();
            }
        }
        _ => unreachable!(),
    }
    println!("total encoded bytes {total}");
}
