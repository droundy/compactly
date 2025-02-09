use bincode::Options;
use scaling::bench;

use serde::{de::DeserializeOwned, Serialize};

trait Encoding: std::fmt::Debug + Clone + Copy + Default {
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8>;
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T;
    const NAME: &str;
    fn name(self) -> String {
        Self::NAME.into()
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

#[derive(Debug, Clone, Copy, Default)]
struct Zstd<E: Encoding> {
    encoding: E,
    level: i32,
}
impl<E: Encoding> Encoding for Zstd<E> {
    const NAME: &str = "zstd other";
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        zstd::bulk::compress(self.encoding.encode(value).as_slice(), self.level).unwrap()
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        self.encoding.decode(
            zstd::bulk::decompress(bytes, 10_000_000)
                .unwrap()
                .as_slice(),
        )
    }
    fn name(self) -> String {
        format!("zstd{} {}", self.level, self.encoding.name())
    }
}

fn bench_one<T: compactly::Encode + Serialize + DeserializeOwned>(e: impl Encoding, value: &T) {
    let encoding_ms = bench(|| e.encode(value)).ns_per_iter * 1e-6;
    let encoded = e.encode(value);
    let decoding_ms = bench(|| e.decode::<T>(encoded.as_slice())).ns_per_iter * 1e-6;
    let size = format_sz(encoded.len());
    println!(
        "{:>25} {size:6} {encoding_ms:>6.2}ms {decoding_ms:>5.2}ms",
        e.name()
    );
}

fn bench_all<T: compactly::Encode + Serialize + DeserializeOwned>(name: &str, value: T) {
    println!("{name}:");
    {
        let size = "size";
        let encoding_ms = "encode";
        let decoding_ms = "decode";
        println!(
            "{:>25} {size:6} {encoding_ms:>6.6}   {decoding_ms:>6.6}",
            "encoding"
        );
        let size = "----";
        let encoding_ms = "-------------";
        let decoding_ms = "-------------";
        println!(
            "{:>25} {size:6} {encoding_ms:>6.6}   {decoding_ms:>6.6}",
            "--------"
        );
    }
    bench_one(Compactly, &value);
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 100,
        },
        &value,
    );
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 9,
        },
        &value,
    );
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 3,
        },
        &value,
    );
    bench_one(
        Zstd {
            encoding: BincodeFix,
            level: 3,
        },
        &value,
    );
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
    bench_all("mtg tenth edition", comparison::tenth_edition());
    bench_all("cards", comparison::tenth_edition().data.cards[0].clone());
}
