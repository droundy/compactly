use compactly::ans::Ans;
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
    fn decode<T: Encodable>(self, bytes: &[u8]) -> Option<T>;
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
    fn decode<T: compactly::v1::Encode + Serialize + DeserializeOwned>(
        self,
        bytes: &[u8],
    ) -> Option<T> {
        compactly::v1::decode(bytes)
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
    fn decode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(
        self,
        bytes: &[u8],
    ) -> Option<T> {
        compactly::ans::Range::decode(bytes)
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
    fn decode<T: compactly::ans::Encode + Serialize + DeserializeOwned>(
        self,
        bytes: &[u8],
    ) -> Option<T> {
        Ans::decode(bytes)
    }
}

fn just_encode<T: Encodable>(e: impl Encoding, values: &[T]) {
    let mut total_size = 0;
    for _ in 0..100_000 {
        total_size += values.iter().map(|v| e.encode(v).len()).sum::<usize>();
    }
    println!("{total_size}");
}

fn just_decode<T: Encodable>(e: impl Encoding, values: &[T]) {
    let encoded = values.iter().map(|v| e.encode(v)).collect::<Vec<_>>();
    let mut all = 0;
    for _ in 0..100_000 {
        all += encoded
            .iter()
            .filter_map(|bytes| e.decode::<T>(&bytes))
            .count();
    }
    println!("number_decoded {all} by {}", e.name());
}

fn main() {
    println!("Run profiling with\ncargo flamegraph --bin profile --package comparison");
    // let suicide_vec = comparison::suicides_per_million()
    //     .keys()
    //     .cloned()
    //     .collect::<Vec<_>>();
    // bench_all("suicide data", &[suicide_vec]);
    // let suicide_rates = comparison::suicides_per_million()
    //     .values()
    //     .copied()
    //     .collect::<Vec<_>>();
    // bench_all("suicide rates", &[suicide_rates]);
    // bench_all("suicide", &[comparison::suicides_per_million()]);

    let algorithm = "ans";
    let values = vec![comparison::meteorites::meteorite_names()
        .keys()
        .cloned()
        .collect::<BTreeSet<String>>()];

    if false {
        match algorithm {
            "ans" => just_encode(CompactlyAns, &values),
            "v1" => just_encode(CompactlyV1, &values),
            "range" => just_encode(CompactlyRange, &values),
            _ => panic!("unrecognized algorithm"),
        }
    } else {
        match algorithm {
            "ans" => just_decode(CompactlyAns, &values),
            "v1" => just_decode(CompactlyV1, &values),
            "range" => just_decode(CompactlyRange, &values),
            _ => panic!("unrecognized algorithm"),
        }
    }
    // bench_all("mtg tenth edition", &[comparison::tenth_edition()]);
    // bench_all("single cards", &comparison::tenth_edition().data.cards);
    // let individual_suicide = comparison::suicides_per_million()
    //     .iter()
    //     .map(|(factors, value)| (factors.clone(), *value))
    //     .collect::<Vec<_>>();
    // bench_all("individual suicide", &individual_suicide);
    // bench_all("suicide", &[comparison::suicides_per_million()]);
    // bench_all("meteorites", &[comparison::meteorites::meteorites()]);
    // bench_all("single meteorites", &comparison::meteorites::meteorites());
    // bench_all(
    //     "meteorites by name",
    //     &[comparison::meteorites::meteorite_names()],
    // );

    // #[derive(
    //     compactly::v1::Encode,
    //     compactly::ans::Encode,
    //     serde::Serialize,
    //     serde::Deserialize,
    //     PartialEq,
    //     Debug,
    // )]
    // struct MetNames {
    //     #[compactly(Mapping<Compressible, Normal>)]
    //     mets: std::collections::BTreeMap<String, comparison::meteorites::MeteoriteData>,
    // }
    // bench_all(
    //     "meteorites by small name",
    //     &[MetNames {
    //         mets: comparison::meteorites::meteorite_names(),
    //     }],
    // );
}
