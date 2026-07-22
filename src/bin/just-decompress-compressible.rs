//! Focused decode-only benchmark for the `Compressible` (Lz77) byte path.
//! Reads the meteorite CSV (real, redundant natural-language/tabular text),
//! encodes it once as one `Encoded<Vec<u8>, Compressible>`, then decodes it
//! `iterations` times so `perf stat -e cpu_core/cycles/` measures Lz77 decode
//! cycles. Run from the workspace root so the CSV is found.
//!
//! Usage: `just-decompress-compressible [ans|range] [iterations]` (defaults
//! `ans`, 2000).
use compactly::v2::{Ans, Range};
use compactly::{Compressible, Encoded};

fn main() {
    let coder = std::env::args()
        .find(|a| a == "ans" || a == "range")
        .unwrap_or_else(|| "ans".to_string());
    let iterations: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(2000);
    let csv = std::fs::read_to_string("comparison/src/meteorites.csv")
        .or_else(|_| std::fs::read_to_string("../comparison/src/meteorites.csv"))
        .expect("run from the workspace root so comparison/src/meteorites.csv is found");
    let corpus: Encoded<Vec<u8>, Compressible> = Encoded::new(csv.into_bytes());
    let raw_len = corpus.len();

    let mut checksum = 0u64;
    match coder.as_str() {
        "ans" => {
            let encoded = Ans::encode(&corpus);
            eprintln!(
                "{raw_len} bytes -> {} encoded ({:.2}x) with ans",
                encoded.len(),
                raw_len as f64 / encoded.len() as f64
            );
            for _ in 0..iterations {
                let decoded: Encoded<Vec<u8>, Compressible> =
                    Ans::decode(&encoded).expect("decode failed");
                checksum = checksum.wrapping_add(decoded[raw_len / 2] as u64);
            }
        }
        _ => {
            let encoded = Range::encode(&corpus);
            eprintln!(
                "{raw_len} bytes -> {} encoded ({:.2}x) with range",
                encoded.len(),
                raw_len as f64 / encoded.len() as f64
            );
            for _ in 0..iterations {
                let decoded: Encoded<Vec<u8>, Compressible> =
                    Range::decode(&encoded).expect("decode failed");
                checksum = checksum.wrapping_add(decoded[raw_len / 2] as u64);
            }
        }
    }
    eprintln!("checksum {checksum}");
}
