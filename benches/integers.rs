use bincode::config::standard;
use compactly::v2::{Ans, Range};
use compactly::{Encoded, Incompressible, LowCardinality, Normal, Small, Sorted, Values};
use rand::distributions::{Distribution, Standard};
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

// ---------- uniform (genuinely incompressible) data generation ----------
//
// Unlike `LogUniform` (which weights every bit-length bucket equally),
// plain uniform bits weight large-magnitude buckets far more heavily —
// this is the distribution the default `Normal` encoding's geometric seed is
// tuned for, and the one where `Small`'s flat seed pays its ~log2(bits) extra
// fresh-context bits.

fn gen_uniform_vec<T, R: Rng>(rng: &mut R, n: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    (0..n).map(|_| rng.gen()).collect()
}

// ---------- skewed-small data generation ----------
//
// A true geometric distribution over bit-length (repeated Bernoulli trials
// with "continue growing" probability `p`, capped at `bits`), then a
// uniformly random value within that bit-length — i.e. most values are
// small, with a long thin tail, unlike the single-repeated-constant worst
// case below. `bit_length = 0` (probability `1-p`) means the value 0;
// `bit_length = k` (probability `(1-p)*p^k`) means a value with its
// highest set bit at position `k-1`, i.e. in `[2^(k-1), 2^k)`.

trait SkewedSmall: Copy + Ord + 'static {
    fn gen_skewed_small<R: Rng>(rng: &mut R, p: f64) -> Self;
}

macro_rules! impl_skewed_small {
    ($T:ty) => {
        impl SkewedSmall for $T {
            fn gen_skewed_small<R: Rng>(rng: &mut R, p: f64) -> Self {
                let bits = <$T>::BITS;
                let mut bit_length = 0u32;
                while bit_length < bits && rng.gen_bool(p) {
                    bit_length += 1;
                }
                if bit_length == 0 {
                    0
                } else {
                    let high_bit = (1 as $T) << (bit_length - 1);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$T>() & mask)
                }
            }
        }
    };
}

impl_skewed_small!(u16);
impl_skewed_small!(u32);
impl_skewed_small!(u64);
impl_skewed_small!(usize);

fn gen_skewed_small_vec<T: SkewedSmall, R: Rng>(rng: &mut R, n: usize, p: f64) -> Vec<T> {
    (0..n).map(|_| T::gen_skewed_small(rng, p)).collect()
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
        // Genuinely incompressible: the case the default `Normal` geometric
        // seed targets directly (goal: ~bits per value, beating `Small`).
        let uniform_data: Vec<$T> = gen_uniform_vec(&mut $rng, N);
        // Most values small with a long thin tail: the case that tests
        // real-world adaptation speed, distinct from the constant-repeat
        // worst case below.
        let skewed_small_data: Vec<$T> = gen_skewed_small_vec(&mut $rng, N, 0.8);
        // Worst case for the default's geometric seed: many repeats of one
        // small constant, making the known fresh-cost trade-off (the prior is
        // biased *away* from small values, unlike `Small`'s) visible at bench
        // scale.
        let repeated_small_data: Vec<$T> = vec![1 as $T; N];

        for (variant, data) in [
            (format!("u{bits} random ({N} values)"), &random_data),
            (format!("u{bits} sorted ({N} values)"), &sorted_data),
            (format!("u{bits} uniform bits ({N} values)"), &uniform_data),
            (
                format!("u{bits} skewed small ({N} values)"),
                &skewed_small_data,
            ),
            (
                format!("u{bits} repeated small const ({N} values)"),
                &repeated_small_data,
            ),
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
    // The distribution `usize`'s tuned default `Encode` actually targets:
    // real `usize`s are lengths/counts/indices, overwhelmingly small, not
    // log-uniform across the whole magnitude range.
    let skewed_small_usize: Vec<usize> = gen_skewed_small_vec(&mut rng, N, 0.8);
    let repeated_small_usize: Vec<usize> = vec![1_usize; N];
    benchmark_usize(&format!("usize random ({N} values)"), &random_usize);
    benchmark_usize(&format!("usize sorted ({N} values)"), &sorted_usize);
    benchmark_usize(
        &format!("usize skewed small ({N} values)"),
        &skewed_small_usize,
    );
    benchmark_usize(
        &format!("usize repeated small const ({N} values)"),
        &repeated_small_usize,
    );
}
