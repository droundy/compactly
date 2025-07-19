use bincode::Options;
use compactly::ans::Ans;
use scaling::bench;
use std::collections::BTreeSet;
use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

trait Encodable:
    compactly::v1::Encode + compactly::ans::Encode + Serialize + DeserializeOwned + PartialEq + Debug
{
}

impl<T> Encodable for T where
    T: compactly::v1::Encode
        + compactly::ans::Encode
        + Serialize
        + DeserializeOwned
        + PartialEq
        + Debug
{
}

trait Encoding: std::fmt::Debug + Clone + Copy + Default {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8>;
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T;
    const NAME: &str;
    fn name(self) -> String {
        Self::NAME.into()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CompactlyV1;
impl Encoding for CompactlyV1 {
    const NAME: &str = "compactly-v1";
    fn encode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        compactly::v1::encode(value)
    }
    fn decode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        compactly::v1::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CompactlyRange;
impl Encoding for CompactlyRange {
    const NAME: &str = "compactly-range";
    fn encode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(
        self,
        value: &T,
    ) -> Vec<u8> {
        compactly::ans::Range::encode(value)
    }
    fn decode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        compactly::ans::Range::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CompactlyAns;
impl Encoding for CompactlyAns {
    const NAME: &str = "compactly-ans";
    fn encode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(
        self,
        value: &T,
    ) -> Vec<u8> {
        Ans::encode(value)
    }
    fn decode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        Ans::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BincodeVar;
impl Encoding for BincodeVar {
    const NAME: &str = "bincode varint";
    fn encode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        bincode::DefaultOptions::new().serialize(value).unwrap()
    }
    fn decode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        bincode::DefaultOptions::new().deserialize(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct BincodeFix;
impl Encoding for BincodeFix {
    const NAME: &str = "bincode fixint";
    fn encode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        bincode::serialize(value).unwrap()
    }
    fn decode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
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
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        zstd::bulk::compress(self.encoding.encode(value).as_slice(), self.level).unwrap()
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
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

fn bench_one<T: Encodable>(e: impl Encoding, values: &[T]) {
    let encoding_ms = bench(|| values.iter().map(|v| e.encode(v)).collect::<Vec<_>>()).ns_per_iter
        * 1e-6
        / values.len() as f64;
    let encoded = values.iter().map(|v| e.encode(v)).collect::<Vec<_>>();

    // To test that we decode correctly we can uncomment this.
    let decoded = encoded
        .iter()
        .map(|bytes| e.decode::<T>(&bytes))
        .collect::<Vec<_>>();
    assert_eq!(values, decoded);

    let decoding_ms = bench(|| {
        encoded
            .iter()
            .map(|bytes| e.decode::<T>(&bytes))
            .collect::<Vec<_>>()
    })
    .ns_per_iter
        * 1e-6
        / values.len() as f64;
    let size =
        format_sz(encoded.iter().map(|e| e.len()).sum::<usize>() as f64 / values.len() as f64);
    println!(
        "{:>25} {size:6} {} {}",
        e.name(),
        fmt_ms(encoding_ms),
        fmt_ms(decoding_ms),
    );
}

fn fmt_ms(ms: f64) -> String {
    if ms == 0.0 {
        format!("{ms:>5.2} ")
    } else if ms > 1.0 {
        format!("{ms:>5.2}ms")
    } else {
        format!("{:>5.2}us", ms * 1000.0)
    }
}

fn bench_all<T: Encodable>(name: &str, values: &[T]) {
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
    bench_one(CompactlyV1, values);
    bench_one(CompactlyRange, values);
    bench_one(CompactlyAns, values);
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 100,
        },
        values,
    );
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 9,
        },
        values,
    );
    bench_one(
        Zstd {
            encoding: BincodeVar,
            level: 3,
        },
        values,
    );
    bench_one(
        Zstd {
            encoding: BincodeFix,
            level: 3,
        },
        values,
    );
    bench_one(BincodeVar, values);
    bench_one(BincodeFix, values);
}

fn format_sz(sz: f64) -> String {
    if sz >= 1024.0 * 10.0 {
        format!("{:.0}k", sz / 1024.0)
    } else if sz < 20.0 && sz.round() != sz {
        format!("{:.1}", sz)
    } else {
        format!("{:.0}", sz)
    }
}

fn main() {
    let suicide_vec = comparison::suicides_per_million()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    bench_all("suicide data", &[suicide_vec]);
    let suicide_rates = comparison::suicides_per_million()
        .values()
        .copied()
        .collect::<Vec<_>>();
    bench_all("suicide rates", &[suicide_rates]);
    bench_all("suicide", &[comparison::suicides_per_million()]);
    let names = comparison::meteorites::meteorite_names()
        .keys()
        .cloned()
        .collect::<BTreeSet<String>>();
    bench_all("meteorite names", &[names]);
    bench_all("mtg tenth edition", &[comparison::tenth_edition()]);
    bench_all("single cards", &comparison::tenth_edition().data.cards);
    let individual_suicide = comparison::suicides_per_million()
        .iter()
        .map(|(factors, value)| (factors.clone(), *value))
        .collect::<Vec<_>>();
    bench_all("individual suicide", &individual_suicide);
    bench_all("suicide", &[comparison::suicides_per_million()]);
    bench_all("meteorites", &[comparison::meteorites::meteorites()]);
    bench_all("single meteorites", &comparison::meteorites::meteorites());
    bench_all(
        "meteorites by name",
        &[comparison::meteorites::meteorite_names()],
    );

    #[derive(
        compactly::v1::Encode,
        compactly::ans::Encode,
        serde::Serialize,
        serde::Deserialize,
        PartialEq,
        Debug,
    )]
    struct MetNames {
        #[compactly(Mapping<Compressible, Normal>)]
        mets: std::collections::BTreeMap<String, comparison::meteorites::MeteoriteData>,
    }
    bench_all(
        "meteorites by small name",
        &[MetNames {
            mets: comparison::meteorites::meteorite_names(),
        }],
    );
}
