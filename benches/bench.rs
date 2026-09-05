use bincode1::{DefaultOptions, Options};
use compactly::v2::Ans;
use rand::{Rng, SeedableRng};
use scaling::{bench_gen_env, bench_scaling_gen};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

trait Encodable: compactly::v1::Encode + compactly::v2::Encode + Serialize + DeserializeOwned {}
impl<T: compactly::v1::Encode + compactly::v2::Encode + Serialize + DeserializeOwned> Encodable
    for T
{
}

trait Encoding: Debug + Clone + Copy + Default {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8>;
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T;
}

#[derive(Debug, Clone, Copy, Default)]
struct Compactly;
impl Encoding for Compactly {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        compactly::v2::Range::encode(value)
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
        compactly::v2::Range::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CompactlyAns;
impl Encoding for CompactlyAns {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        Ans::encode(value)
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
        Ans::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SerdeVar;
impl Encoding for SerdeVar {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        DefaultOptions::new().serialize(value).unwrap()
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
        DefaultOptions::new().deserialize(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ZstdSerdeVar;
impl Encoding for ZstdSerdeVar {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        let v = DefaultOptions::new().serialize(value).unwrap();
        zstd::bulk::compress(v.as_slice(), 3).unwrap()
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
        let v = zstd::bulk::decompress(bytes, 10_000_000).unwrap();
        DefaultOptions::new().deserialize(&v).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ZstdSerde;
impl Encoding for ZstdSerde {
    fn encode<T: Encodable>(self, value: &T) -> Vec<u8> {
        let v = bincode1::serialize(value).unwrap();
        zstd::bulk::compress(v.as_slice(), 3).unwrap()
    }
    fn decode<T: Encodable>(self, bytes: &[u8]) -> T {
        let v = zstd::bulk::decompress(bytes, 10_000_000).unwrap();
        bincode1::deserialize(&v).unwrap()
    }
}

fn mem_allocated<T>(f: impl Fn() -> T) -> (T, usize) {
    let reg = Region::new(GLOBAL);
    let v = f();
    let stats = reg.change();
    let total = stats.bytes_allocated as isize + stats.bytes_reallocated;
    if total >= 0 {
        (v, total as usize)
    } else {
        (v, 0)
    }
}

/// One table per workload: a row per encoding, so each cell has room for the
/// `±` `scaling` reports beside the time. A number here without its error bar
/// would be unreadable as a comparison — the codecs differ by factors, but
/// successive runs of the same one differ by percents, and only the `±` says
/// which of those you are looking at.
fn bench_encoding<T: Encodable>(name: &str, mut gen: impl FnMut() -> T) {
    header(name, "encode", "decode", FLAT_CELL);
    macro_rules! row {
        ($encoding:ident, $label:expr) => {{
            let encode = bench_gen_env(&mut gen, |value| $encoding.encode(value));
            let decode = bench_gen_env(
                || $encoding.encode(&gen()),
                |bytes| $encoding.decode::<T>(bytes),
            );
            let size = (0..3)
                .map(|_| $encoding.encode(&gen()).len())
                .sum::<usize>() as f64
                / 3.0;
            let encoded = $encoding.encode(&gen());
            let mem = mem_allocated(|| $encoding.decode::<T>(&encoded)).1 as f64;
            print_row(
                $label,
                &encode.to_string(),
                &decode.to_string(),
                size,
                mem,
                FLAT_CELL,
            );
        }};
    }
    row!(Compactly, "compactly");
    row!(CompactlyAns, "ans");
    row!(SerdeVar, "bincode");
    row!(ZstdSerdeVar, "zstd");
    row!(ZstdSerde, "zstdfix");
}

/// As [`bench_encoding`], but each cell is a fitted scaling law rather than a
/// single time. `R²` rides along with the `±` because the two answer different
/// questions — which law, and how big its constant — and a tight `±` beside
/// `R²=0.000` means the shape was never pinned down.
fn bench_scaling<T: Encodable>(name: &str, mut gen: impl FnMut(usize) -> T) {
    header(name, "encode /N", "decode /N", SCALING_CELL);
    macro_rules! row {
        ($encoding:ident, $label:expr) => {{
            let encode = bench_scaling_gen(&mut gen, |value| $encoding.encode(value), 5);
            let decode = bench_scaling_gen(
                |n| $encoding.encode(&gen(n)),
                |bytes| $encoding.decode::<T>(bytes),
                5,
            );
            let size = (0..10)
                .map(|_| $encoding.encode(&gen(1024)).len())
                .sum::<usize>() as f64
                / 10.0;
            let encoded = $encoding.encode(&gen(1024));
            let mem = mem_allocated(|| $encoding.decode::<T>(&encoded)).1 as f64;
            print_row(
                $label,
                &encode.to_string(),
                &decode.to_string(),
                size,
                mem,
                SCALING_CELL,
            );
        }};
    }
    row!(Compactly, "compactly");
    row!(CompactlyAns, "ans");
    row!(SerdeVar, "bincode");
    row!(ZstdSerdeVar, "zstd");
    row!(ZstdSerde, "zstdfix");
}

/// Cell width for the flat tables, and for the scaling ones — a fitted law
/// carries its `R²` and possibly a `(limit)` mark, so it needs more room.
const FLAT_CELL: usize = 34;
const SCALING_CELL: usize = 48;

fn header(name: &str, encode: &str, decode: &str, cell: usize) {
    println!("\n{name}");
    println!(
        "{:<11} {encode:>cell$} {decode:>cell$} {:>8} {:>10}",
        "", "size", "decode mem"
    );
}

fn print_row(name: &str, encode: &str, decode: &str, size: f64, mem: f64, cell: usize) {
    println!(
        "{name:<11} {encode:>cell$} {decode:>cell$} {:>8} {:>10}",
        format_sz(size),
        format_sz(mem)
    );
}

fn format_sz(sz: f64) -> String {
    if sz >= 1e4 {
        format!("{:.0}k", sz / 1000.0)
    } else {
        format!("{:.0}", sz)
    }
}

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(3);
    bench_encoding("usize", || rng.gen::<usize>());
    bench_encoding("0..8", || rng.gen_range(0..10usize));
    bench_encoding("vec[0..8; 10]", || {
        (0..10)
            .map(|_| rng.gen_range(0..8usize))
            .collect::<Vec<_>>()
    });
    bench_encoding("vec[0..8; 1024]", || {
        (0..1024)
            .map(|_| rng.gen_range(0..8usize))
            .collect::<Vec<_>>()
    });

    bench_scaling("hashset<usize>", |sz| {
        (0..sz).map(|_| rng.gen::<usize>()).collect::<HashSet<_>>()
    });
    bench_scaling("btreeset<usize>", |sz| {
        (0..sz).map(|_| rng.gen::<usize>()).collect::<BTreeSet<_>>()
    });
    #[derive(Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode)]
    struct CompactSet {
        #[compactly(Small)]
        set: BTreeSet<u64>,
    }
    bench_scaling("compact btreeset<u64>", |sz| {
        let mx = 2 * sz;
        let mut set = BTreeSet::new();
        while set.len() < sz {
            set.insert(rng.gen::<u64>() % mx as u64);
        }
        CompactSet { set }
    });
    bench_scaling("btreeset<vec[0..128; 7]>", |sz| {
        let mut set = BTreeSet::new();
        while set.len() < sz {
            set.insert((0..7).map(|_| rng.gen_range(0..8usize)).collect::<Vec<_>>());
        }
        set
    });

    #[derive(
        compactly::v1::Encode,
        compactly::v2::Encode,
        Serialize,
        Deserialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Clone,
    )]
    enum ThreeOptions {
        A,
        B,
        C,
    }
    bench_scaling("btreeset<vec![ThreeOptions; 15]>", |sz| {
        let mut option = || match rng.gen_range(0..10) {
            0 | 1 => ThreeOptions::A,
            2 | 3 => ThreeOptions::B,
            _ => ThreeOptions::C,
        };
        let mut set = BTreeSet::new();
        while set.len() < sz {
            set.insert(vec![
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
                option(),
            ]);
        }
        set
    });
}
