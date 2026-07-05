fn main() {
    // Reproducible pseudo-random *non-integer* f64s, so decode goes through the
    // raw-bits path (the batched decode_bits) rather than the integer fast path.
    let mut x = 0x123456789abcdef0u64;
    let data: Vec<f64> = (0..100000)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            // Random *finite* f64 with a varied (random) exponent: clear one
            // exponent bit when all are set, so we never produce inf/NaN.
            let bits = if (x >> 52) & 0x7FF == 0x7FF {
                x & !(1u64 << 52)
            } else {
                x
            };
            f64::from_bits(bits)
        })
        .collect();
    // Coder selectable via argv: "ans" (default) or "range" (the public-API
    // default coder). Both exercise the batched `decode_bits` raw-bits path.
    let coder = std::env::args().nth(1).unwrap_or_else(|| "ans".to_string());
    use compactly::v2::{Ans, Range};
    let compressed = match coder.as_str() {
        "ans" => Ans::encode(&data),
        "range" => Range::encode(&data),
        other => panic!("unknown coder {other:?}; use ans|range"),
    };
    eprintln!(
        "coder={coder} compressed {} floats to {} bytes ({:.3} bytes/float)",
        data.len(),
        compressed.len(),
        compressed.len() as f64 / data.len() as f64
    );
    // Branch on the coder *once*, outside the hot loop, so the measurement only
    // reflects the decoder under test.
    let mut total = 0.0f64;
    if coder == "ans" {
        for _ in 0..1000 {
            let decoded = std::hint::black_box(Ans::decode::<Vec<f64>>(&compressed)).unwrap();
            total += decoded[0];
        }
    } else {
        for _ in 0..1000 {
            let decoded = std::hint::black_box(Range::decode::<Vec<f64>>(&compressed)).unwrap();
            total += decoded[0];
        }
    }
    println!("total is {total}?");
}
