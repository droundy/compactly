use bincode::config::standard;
use compactly::v2::{Ans, Range};
use compactly::{Encoded, Small, Sorted, Values};
use rand::{Rng, SeedableRng};
use scaling::bench_gen_env;

const N: usize = 8192;

// ---------- log-uniform signed data generation ----------
// Magnitude is log-uniform; sign is negative 1/3 of the time.

trait GenSigned: Copy + Ord + 'static {
    fn gen_signed<R: Rng>(rng: &mut R) -> Self;
}

macro_rules! impl_gen_signed {
    ($T:ty, $U:ty) => {
        impl GenSigned for $T {
            fn gen_signed<R: Rng>(rng: &mut R) -> Self {
                let bits = <$U>::BITS;
                // leading starts at 1 so high_bit <= 2^(bits-2), always fits as positive signed.
                let leading: u32 = rng.gen_range(1..=bits);
                let mag: $U = if leading >= bits {
                    0
                } else {
                    let high_bit = (1 as $U) << (bits - 1 - leading);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$U>() & mask)
                };
                let signed_mag = mag as $T;
                if rng.gen_range(0u8..3) == 0 {
                    signed_mag.wrapping_neg()
                } else {
                    signed_mag
                }
            }
        }
    };
}

impl_gen_signed!(i16, u16);
impl_gen_signed!(i32, u32);
impl_gen_signed!(i64, u64);

fn gen_signed_vec<T: GenSigned, R: Rng>(rng: &mut R, n: usize) -> Vec<T> {
    (0..n).map(|_| T::gen_signed(rng)).collect()
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
        let t = bench_gen_env(|| data.clone(), |d| enc(&d)).ns_per_iter;
        print!(" {:>W$}", format_time(t));
    }
    println!();

    print!("{:>14}:", "decode");
    for (_, enc, dec) in algos {
        let encoded = enc(data);
        let t = bench_gen_env(|| encoded.clone(), |b| dec(&b)).ns_per_iter;
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

macro_rules! benchmark_signed_type {
    ($T:ty, $rng:ident) => {{
        let bits = <$T>::BITS;
        let random_data: Vec<$T> = gen_signed_vec(&mut $rng, N);
        let sorted_data: Vec<$T> = {
            let mut s = random_data.clone();
            s.sort();
            s
        };

        for (variant, data) in [
            (format!("i{bits} random ({N} values)"), &random_data),
            (format!("i{bits} sorted ({N} values)"), &sorted_data),
        ] {
            let mut algos: Vec<Algo<$T>> = Vec::new();

            add_strategy!(algos, $T, "norm", Encoded<Vec<$T>, Values<compactly::Normal>>);
            add_strategy!(algos, $T, "sml", Encoded<Vec<$T>, Values<Small>>);
            add_strategy!(algos, $T, "srt", Encoded<Vec<$T>, Values<Sorted>>);

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

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);
    benchmark_signed_type!(i16, rng);
    benchmark_signed_type!(i32, rng);
    benchmark_signed_type!(i64, rng);
}
