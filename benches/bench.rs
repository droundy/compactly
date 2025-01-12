use bincode::Options;
use rand::{Rng, SeedableRng};
use scaling::{bench_gen_env, bench_scaling_gen};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{
    alloc::System,
    collections::{BTreeSet, HashSet},
    fmt::Debug,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

trait Encoding: Debug + Clone + Copy + Default {
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8>;
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T;
}

#[derive(Debug, Clone, Copy, Default)]
struct Compactly;
impl Encoding for Compactly {
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        compactly::encode(value)
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        compactly::decode(bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct SerdeVar;
impl Encoding for SerdeVar {
    fn encode<T: compactly::Encode + Serialize + DeserializeOwned>(self, value: &T) -> Vec<u8> {
        bincode::DefaultOptions::new().serialize(value).unwrap()
    }
    fn decode<T: compactly::Encode + Serialize + DeserializeOwned>(self, bytes: &[u8]) -> T {
        bincode::DefaultOptions::new().deserialize(bytes).unwrap()
    }
}

fn mem_allocated<T>(f: impl Fn() -> T) -> (T, usize) {
    let reg = Region::new(&GLOBAL);
    let v = f();
    let stats = reg.change();
    let total = stats.bytes_allocated as isize + stats.bytes_reallocated;
    if total >= 0 {
        (v, total as usize)
    } else {
        (v, 0)
    }
}

fn bench_encoding<T: compactly::Encode + Serialize + DeserializeOwned>(
    name: &str,
    mut gen: impl FnMut() -> T,
) {
    if name.len() <= 11 {
        println!("{:11}:{:>13} {:>13}", name, "compactly", "bincode",);
    } else {
        println!("\n{}", name);
        println!("{:11}:{:>13} {:>13}", "", "compactly", "bincode",);
    }

    print_times(
        "encode",
        &[
            bench_gen_env(&mut gen, |value| Compactly.encode(value)).ns_per_iter,
            bench_gen_env(&mut gen, |value| SerdeVar.encode(value)).ns_per_iter,
        ],
    );
    print_times(
        "decode",
        &[
            bench_gen_env(
                || Compactly.encode(&gen()),
                |bytes| Compactly.decode::<T>(bytes),
            )
            .ns_per_iter,
            bench_gen_env(
                || SerdeVar.encode(&gen()),
                |bytes| SerdeVar.decode::<T>(bytes),
            )
            .ns_per_iter,
        ],
    );
    macro_rules! get_size {
        ($encoding:ident) => {
            (0..3)
                .map(|_| $encoding.encode(&gen()).len())
                .sum::<usize>() as f64
                / 3.0
        };
    }
    print_sizes("", &[get_size!(Compactly), get_size!(SerdeVar)]);
    macro_rules! decoding_mem {
        ($encoding:ident) => {{
            let encoded = $encoding.encode(&gen());
            mem_allocated(|| $encoding.decode::<T>(&encoded)).1 as f64
        }};
    }
    print_sizes(
        "decoding",
        &[decoding_mem!(Compactly), decoding_mem!(SerdeVar)],
    );
}

fn bench_scaling<T: compactly::Encode + Serialize + DeserializeOwned>(
    name: &str,
    mut gen: impl FnMut(usize) -> T,
) {
    if name.len() <= 11 {
        println!("{:11}:{:>13} {:>13}", name, "compactly", "bincode",);
    } else {
        println!("\n{}", name);
        println!("{:11}:{:>13} {:>13}", "", "compactly", "bincode",);
    }

    print_scalings(
        "encode",
        &[
            bench_scaling_gen(&mut gen, |value| Compactly.encode(value), 5).scaling,
            bench_scaling_gen(&mut gen, |value| SerdeVar.encode(value), 5).scaling,
        ],
    );
    print_scalings(
        "decode",
        &[
            bench_scaling_gen(
                |n| Compactly.encode(&gen(n)),
                |bytes| Compactly.decode::<T>(bytes),
                5,
            )
            .scaling,
            bench_scaling_gen(
                |n| SerdeVar.encode(&gen(n)),
                |bytes| SerdeVar.decode::<T>(bytes),
                5,
            )
            .scaling,
        ],
    );
    macro_rules! get_size {
        ($encoding:ident) => {
            (0..10)
                .map(|_| $encoding.encode(&gen(1024)).len())
                .sum::<usize>() as f64
                / 10.0
        };
    }
    print_sizes("", &[get_size!(Compactly), get_size!(SerdeVar)]);
    macro_rules! decoding_mem {
        ($encoding:ident) => {{
            let encoded = $encoding.encode(&gen(1024));
            mem_allocated(|| $encoding.decode::<T>(&encoded)).1 as f64
        }};
    }
    print_sizes(
        "decoding",
        &[decoding_mem!(Compactly), decoding_mem!(SerdeVar)],
    );
}

fn print_sizes(name: &str, sizes: &[f64]) {
    print!("{:>11} ", name);
    for s in sizes.iter().cloned() {
        print!(" {:6}  {:<6}", "", format_sz(s));
    }
    println!("");
}
fn print_times(name: &str, times: &[f64]) {
    print!("{:>11}:    ", name);
    for t in times.iter().cloned() {
        print!(" {:^13}", format!("{:.0}ns", t));
    }
    println!("");
}
fn print_scalings(name: &str, times: &[scaling::Scaling]) {
    print!("{:>11}:    ", name);
    for t in times.iter().cloned() {
        print!(" {t:^13}");
    }
    println!("");
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
    bench_scaling("btreeset<vec[0..128; 7]>", |sz| {
        let mut set = BTreeSet::new();
        while set.len() < sz {
            set.insert((0..7).map(|_| rng.gen_range(0..8usize)).collect::<Vec<_>>());
        }
        set
    });

    #[derive(compactly::Encode, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
