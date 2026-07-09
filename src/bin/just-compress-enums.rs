//! Focused benchmark for the enum-discriminant encode hot path (the
//! `ULessThan<3>` walk); the encode-side twin of `just-decompress-enums`,
//! same data and distribution.
//!
//! Usage: `just-compress-enums [ans|range] [iterations]` (defaults:
//! `ans`, 2000).

#[derive(Debug, Clone, Copy, PartialEq, Eq, compactly::v2::Encode)]
enum ThreeOptions {
    A,
    B,
    C,
}

fn data() -> Vec<ThreeOptions> {
    // Simple LCG for reproducible pseudo-random skewed data, as in
    // `just-decompress`.
    let mut x = 0x123456789abcdef0u64;
    (0..100_000)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            match (x >> 33) % 10 {
                0 | 1 => ThreeOptions::A,
                2 | 3 => ThreeOptions::B,
                _ => ThreeOptions::C,
            }
        })
        .collect()
}

fn main() {
    let coder = std::env::args()
        .find(|a| a == "ans" || a == "range")
        .unwrap_or("ans".to_string());
    let iterations: usize = std::env::args()
        .filter_map(|a| a.parse().ok())
        .next()
        .unwrap_or(2000);
    let data = data();
    println!("encoding {} enums with {coder}", data.len());

    let mut total = 0usize;
    match coder.as_str() {
        "ans" => {
            for _ in 0..iterations {
                total += std::hint::black_box(compactly::v2::Ans::encode(&data)).len();
            }
        }
        "range" => {
            for _ in 0..iterations {
                total += std::hint::black_box(compactly::v2::encode(&data)).len();
            }
        }
        _ => unreachable!(),
    }
    println!("total encoded {total}");
}
