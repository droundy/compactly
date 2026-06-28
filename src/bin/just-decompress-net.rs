// Focused decode-only benchmark for the library `Ipv6Addr` impl, using the ANS
// coder (the one whose batched primitive keeps state register-resident).
// Loads ipv6.txt, encodes once, then decodes the whole Vec `ITERS` times so
// `perf stat -e cpu_core/cycles/` measures decode cycles (see OPTIMIZING.md).
use compactly::v2::Ans;
use std::net::Ipv6Addr;

const ITERS: usize = 2000;

fn main() {
    let ipv6s: Vec<Ipv6Addr> = std::fs::read_to_string("ipv6.txt")
        .expect("ipv6.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse().expect("valid IPv6"))
        .collect();

    let encoded = Ans::encode(&ipv6s);
    eprintln!(
        "{} addresses, encoded {} bytes ({:.2} B/addr)",
        ipv6s.len(),
        encoded.len(),
        encoded.len() as f64 / ipv6s.len() as f64
    );

    let mut checksum = 0u64;
    for _ in 0..ITERS {
        let decoded = Ans::decode::<Vec<Ipv6Addr>>(&encoded).expect("decode failed");
        checksum = checksum.wrapping_add(decoded[decoded.len() / 2].octets()[0] as u64);
    }
    eprintln!("checksum {checksum}");
}
