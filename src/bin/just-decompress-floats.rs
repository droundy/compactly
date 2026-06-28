fn main() {
    // Reproducible pseudo-random *non-integer* f64s, so decode goes through the
    // raw-bits path (the batched decode_bits) rather than the integer fast path.
    let mut x = 0x123456789abcdef0u64;
    let data: Vec<f64> = (0..100000)
        .map(|_| {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            // Random *finite* f64 with a varied (random) exponent: clear one
            // exponent bit when all are set, so we never produce inf/NaN.
            let bits = if (x >> 52) & 0x7FF == 0x7FF { x & !(1u64 << 52) } else { x };
            f64::from_bits(bits)
        })
        .collect();
    let compressed = compactly::v2::Ans::encode(&data);
    eprintln!("compressed {} floats to {} bytes ({:.3} bytes/float)",
        data.len(), compressed.len(), compressed.len() as f64 / data.len() as f64);
    let mut total = 0.0f64;
    for _ in 0..1000 {
        let decoded = std::hint::black_box(compactly::v2::Ans::decode::<Vec<f64>>(&compressed)).unwrap();
        total += decoded[0];
    }
    println!("total is {total}?");
}
