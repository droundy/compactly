use bincode::config::standard;
use compactly::v2::{Ans, Range};
use compactly::{Encoded, Incompressible, LowCardinality, Normal, Small, Sorted, Values};
use rand::{Rng, SeedableRng};
use scaling::bench_gen_env;

const N: usize = 8192;

// ---------- log-uniform data generation ----------

trait LogUniform: Copy + Ord + 'static {
    fn gen_log_uniform<R: Rng>(rng: &mut R) -> Self;
}

macro_rules! impl_log_uniform {
    ($T:ty) => {
        impl LogUniform for $T {
            fn gen_log_uniform<R: Rng>(rng: &mut R) -> Self {
                let bits = <$T>::BITS;
                let leading: u32 = rng.gen_range(0..=bits);
                if leading >= bits {
                    0
                } else {
                    let high_bit = (1 as $T) << (bits - 1 - leading);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$T>() & mask)
                }
            }
        }
    };
}

impl_log_uniform!(u16);
impl_log_uniform!(u32);
impl_log_uniform!(u64);
impl_log_uniform!(usize);

fn gen_log_uniform_vec<T: LogUniform, R: Rng>(rng: &mut R, n: usize) -> Vec<T> {
    (0..n).map(|_| T::gen_log_uniform(rng)).collect()
}

// ---------- benchmarking infrastructure ----------

type AlgoFn<T> = Box<dyn Fn(&Vec<T>) -> Vec<u8>>;
type DecodeFn<T> = Box<dyn Fn(&[u8]) -> Vec<T>>;
type Algo<T> = (&'static str, AlgoFn<T>, DecodeFn<T>);

fn format_time(ns: f64) -> String {
    if ns >= 1_000_000.0 {
        format!("{:.1}ms", ns / 1_000_000.0)
    } else if ns >= 1_000.0 {
        format!("{:.0}us", ns / 1_000.0)
    } else {
        format!("{:.0}ns", ns)
    }
}

fn format_sz(sz: f64) -> String {
    if sz >= 1_000_000.0 {
        format!("{:.1}M", sz / 1_000_000.0)
    } else if sz >= 1_000.0 {
        format!("{:.1}k", sz / 1_000.0)
    } else {
        format!("{:.0}", sz)
    }
}

fn run_benchmarks<T: Clone + std::fmt::Debug + Eq>(
    heading: &str,
    algos: &[Algo<T>],
    data: &Vec<T>,
) {
    const W: usize = 9;
    println!("{heading}");
    print!("{:>14}  ", "");
    for (name, _, _) in algos {
        print!(" {:>W$}", name);
    }
    println!();

    for (name, enc, dec) in algos {
        let encoded = enc(data);
        let decoded = dec(&encoded);
        assert_eq!(&decoded, data, "buggy {name} for {heading}");
    }

    print!("{:>14}:", "encode");
    for (_, enc, _) in algos {
        let t = bench_gen_env(|| data.clone(), |d| enc(d)).ns_per_iter;
        print!(" {:>W$}", format_time(t));
    }
    println!();

    print!("{:>14}:", "decode");
    for (_, enc, dec) in algos {
        let encoded = enc(data);
        let t = bench_gen_env(|| encoded.clone(), |b| dec(b)).ns_per_iter;
        print!(" {:>W$}", format_time(t));
    }
    println!();

    print!("{:>14}:", "size");
    for (_, enc, _) in algos {
        print!(" {:>W$}", format_sz(enc(data).len() as f64));
    }
    println!("\n");
}

// ---------- algorithm construction macros ----------

// Adds a rng+ans pair for one strategy type ($WrapType must impl From<Vec<$T>>).
macro_rules! add_strategy {
    ($algos:ident, $T:ty, $suffix:literal, $WrapType:ty) => {
        $algos.push((
            concat!("rng/", $suffix),
            Box::new(|d: &Vec<$T>| Range::encode(&<$WrapType>::from(d.clone()))) as AlgoFn<$T>,
            Box::new(|b: &[u8]| Range::decode::<$WrapType>(b).unwrap().value()) as DecodeFn<$T>,
        ));
        $algos.push((
            concat!("ans/", $suffix),
            Box::new(|d: &Vec<$T>| Ans::encode(&<$WrapType>::from(d.clone()))),
            Box::new(|b: &[u8]| Ans::decode::<$WrapType>(b).unwrap().value()),
        ));
    };
}

// Adds all compactly strategies as rng+ans pairs, ordered by strategy.
macro_rules! add_compactly_algos {
    ($algos:ident, $T:ty) => {
        add_strategy!($algos, $T, "norm", Encoded<Vec<$T>, Values<Normal>>);
        add_strategy!($algos, $T, "sml", Encoded<Vec<$T>, Values<Small>>);
        add_strategy!($algos, $T, "srt", Encoded<Vec<$T>, Values<Sorted>>);
        add_strategy!($algos, $T, "low", Encoded<Vec<$T>, LowCardinality>);
        add_strategy!($algos, $T, "inc", Encoded<Vec<$T>, Values<Incompressible>>);
    };
}

// Runs the full benchmark suite for one integer type.
macro_rules! benchmark_int_type {
    ($T:ty, $rng:ident) => {{
        let bits = <$T>::BITS;
        let random_data: Vec<$T> = gen_log_uniform_vec(&mut $rng, N);
        let sorted_data: Vec<$T> = {
            let mut s = random_data.clone();
            s.sort();
            s
        };

        for (variant, data) in [
            (format!("u{bits} random ({N} values)"), &random_data),
            (format!("u{bits} sorted ({N} values)"), &sorted_data),
        ] {
            let mut algos: Vec<Algo<$T>> = Vec::new();

            add_compactly_algos!(algos, $T);

            algos.push((
                "bincode",
                Box::new(|d: &Vec<$T>| bincode::encode_to_vec(d, standard()).unwrap()),
                Box::new(|b: &[u8]| {
                    bincode::decode_from_slice::<Vec<$T>, _>(b, standard())
                        .unwrap()
                        .0
                }),
            ));
            algos.push((
                "zstd+bc",
                Box::new(|d: &Vec<$T>| {
                    zstd::bulk::compress(&bincode::encode_to_vec(d, standard()).unwrap(), 3)
                        .unwrap()
                }),
                Box::new(|b: &[u8]| {
                    let v = zstd::bulk::decompress(b, 100_000_000).unwrap();
                    bincode::decode_from_slice::<Vec<$T>, _>(&v, standard())
                        .unwrap()
                        .0
                }),
            ));
            algos.push((
                "bitcode",
                Box::new(|d: &Vec<$T>| bitcode::encode(d)),
                Box::new(|b: &[u8]| bitcode::decode::<Vec<$T>>(b).unwrap()),
            ));
            algos.push((
                "zstd+bit",
                Box::new(|d: &Vec<$T>| zstd::bulk::compress(&bitcode::encode(d), 3).unwrap()),
                Box::new(|b: &[u8]| {
                    let v = zstd::bulk::decompress(b, 100_000_000).unwrap();
                    bitcode::decode::<Vec<$T>>(&v).unwrap()
                }),
            ));

            run_benchmarks(&variant, &algos, data);
        }
    }};
}

fn benchmark_usize(heading: &str, data: &Vec<usize>) {
    let mut algos: Vec<Algo<usize>> = Vec::new();
    add_strategy!(algos, usize, "norm", Encoded<Vec<usize>, Values<Normal>>);
    add_strategy!(algos, usize, "sml", Encoded<Vec<usize>, Values<Small>>);
    add_strategy!(algos, usize, "srt", Encoded<Vec<usize>, Values<Sorted>>);
    algos.push((
        "bincode",
        Box::new(|d: &Vec<usize>| bincode::encode_to_vec(d, standard()).unwrap()),
        Box::new(|b: &[u8]| {
            bincode::decode_from_slice::<Vec<usize>, _>(b, standard())
                .unwrap()
                .0
        }),
    ));
    algos.push((
        "bitcode",
        Box::new(|d: &Vec<usize>| bitcode::encode(d)),
        Box::new(|b: &[u8]| bitcode::decode::<Vec<usize>>(b).unwrap()),
    ));
    run_benchmarks(heading, &algos, data);
}

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
    benchmark_int_type!(u16, rng);
    benchmark_int_type!(u32, rng);
    benchmark_int_type!(u64, rng);

    let random_usize: Vec<usize> = gen_log_uniform_vec(&mut rng, N);
    let sorted_usize: Vec<usize> = {
        let mut s = random_usize.clone();
        s.sort();
        s
    };
    benchmark_usize(&format!("usize random ({N} values)"), &random_usize);
    benchmark_usize(&format!("usize sorted ({N} values)"), &sorted_usize);
}
