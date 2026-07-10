//! Focused benchmark for the `AtMost<MAX>` decode walk across tree depths:
//! decode a `Vec<AtMost<MAX>>` of 50k uniform values repeatedly. Uniform
//! data keeps every tree level exercised (contexts adapt to ~50/50), which is
//! the stress case for comparing the plain walk against a speculative
//! prefetching one — see `symbol::decode_walk_speculating`.
//!
//! Usage: `just-decompress-uless <N> [ans|range] [iterations]` (defaults:
//! `ans`, 2000). `N` is the *value count* (`MAX + 1`) and must be one of the
//! monomorphized ladder below.

use compactly::v2::AtMost;

fn data<const MAX: usize>() -> Vec<AtMost<MAX>> {
    // Simple LCG for reproducible pseudo-random data, as in `just-decompress`.
    let mut x = 0x123456789abcdef0u64;
    (0..50_000)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            AtMost::<MAX>::new(((x >> 33) as usize) % (MAX + 1))
        })
        .collect()
}

fn run<const MAX: usize>(coder: &str, iterations: usize) -> usize {
    let data = data::<MAX>();
    let mut total = 0usize;
    match coder {
        "ans" => {
            let encoded = compactly::v2::Ans::encode(&data);
            println!("encoded size {}", encoded.len());
            for _ in 0..iterations {
                total += std::hint::black_box(
                    compactly::v2::Ans::decode::<Vec<AtMost<MAX>>>(&encoded).unwrap(),
                )
                .len();
            }
        }
        "range" => {
            let encoded = compactly::v2::encode(&data);
            println!("encoded size {}", encoded.len());
            for _ in 0..iterations {
                total += std::hint::black_box(
                    compactly::v2::decode::<Vec<AtMost<MAX>>>(&encoded).unwrap(),
                )
                .len();
            }
        }
        _ => unreachable!(),
    }
    total
}

fn main() {
    let mut numbers = std::env::args().filter_map(|a| a.parse::<usize>().ok());
    let n = numbers
        .next()
        .expect("usage: just-decompress-uless <N> [ans|range] [iterations]");
    let iterations = numbers.next().unwrap_or(2000);
    let coder = std::env::args()
        .find(|a| a == "ans" || a == "range")
        .unwrap_or("ans".to_string());
    println!(
        "decoding 50000 uniform AtMost<{}> values with {coder}",
        n - 1
    );
    let total = match n {
        3 => run::<2>(&coder, iterations),
        4 => run::<3>(&coder, iterations),
        6 => run::<5>(&coder, iterations),
        8 => run::<7>(&coder, iterations),
        12 => run::<11>(&coder, iterations),
        16 => run::<15>(&coder, iterations),
        24 => run::<23>(&coder, iterations),
        32 => run::<31>(&coder, iterations),
        64 => run::<63>(&coder, iterations),
        128 => run::<127>(&coder, iterations),
        _ => panic!("N={n} is not in the monomorphized ladder (3 4 6 8 12 16 24 32 64 128)"),
    };
    println!("total decoded {total}");
}
