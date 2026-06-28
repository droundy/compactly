fn main() {
    // Use a simple LCG for reproducible pseudo-random data that stresses the branch predictor.
    let mut x = 0x123456789abcdef0u64;
    let data: Vec<u64> = (0..100000)
        .map(|_| {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            x
        })
        .collect();
    let compressed = compactly::v2::Ans::encode(&data);
    let mut total_size = 0;
    for _ in 0..5000 {
        total_size += std::hint::black_box(compactly::v2::Ans::decode::<Vec<u64>>(&compressed))
            .unwrap()
            .len();
    }
    println!("total size is {total_size}?");
}
