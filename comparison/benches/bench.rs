use bincode::Options;
use scaling::bench;

use serde::{de::DeserializeOwned, Serialize};

trait Encoding: std::fmt::Debug + Clone + Copy + Default {
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8>;
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T;
    const NAME: &str;
    fn name(self) -> &'static str {
        Self::NAME
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Compactly;
impl Encoding for Compactly {
    const NAME: &str = "compactly";
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        compactly::encode(value)
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        compactly::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BincodeVar;
impl Encoding for BincodeVar {
    const NAME: &str = "bincode varint";
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        bincode::DefaultOptions::new().serialize(value).unwrap()
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        bincode::DefaultOptions::new().deserialize(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BincodeFix;
impl Encoding for BincodeFix {
    const NAME: &str = "bincode fixint";
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        bincode::serialize(value).unwrap()
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        bincode::deserialize(bytes).unwrap()
    }
}

fn bench_one<T: compactly::Encode + Serialize + DeserializeOwned>(e: impl Encoding, value: &T) {
    let encoding_ms = bench(|| e.encode(value)).ns_per_iter * 1e-6;
    let encoded = e.encode(value);
    let decoding_ms = bench(|| e.decode::<T>(encoded.as_slice())).ns_per_iter * 1e-6;
    let size = format_sz(encoded.len());
    println!(
        "{:>20} {size:6} {encoding_ms:>5.2}ms {decoding_ms:>5.2}ms",
        e.name()
    );
}

fn bench_all<T: compactly::Encode + Serialize + DeserializeOwned>(value: T) {
    bench_one(Compactly, &value);
    bench_one(BincodeVar, &value);
    bench_one(BincodeFix, &value);
}

fn format_sz(sz: usize) -> String {
    if sz >= 1024 {
        format!("{}k", (sz + 512) / 1024)
    } else {
        format!("{}", sz)
    }
}

fn main() {
    bench_all(comparison::tenth_edition());
}
