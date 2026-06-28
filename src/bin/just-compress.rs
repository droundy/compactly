fn main() {
    let data: Vec<u64> = (0..100000).collect();
    let mut total_size = 0;
    for _ in 0..10000 {
        total_size += compactly::v2::Ans::encode(&data).len();
    }
    println!("total size is {total_size}?");
}
