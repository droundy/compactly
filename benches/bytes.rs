use compactly::v2::{Ans, Range};
use compactly::{Encoded, Incompressible, Normal, Small, Values};
use rand::{Rng, SeedableRng};
use scaling::bench_gen_env;

const N: usize = 8192;

type AlgoFn = Box<dyn Fn(&Vec<u8>) -> Vec<u8>>;
type DecodeFn = Box<dyn Fn(&[u8]) -> Vec<u8>>;
type Algo = (&'static str, AlgoFn, DecodeFn);

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

macro_rules! add_strategy {
    ($algos:ident, $suffix:literal, $WrapType:ty) => {
        $algos.push((
            concat!("rng/", $suffix),
            Box::new(|d: &Vec<u8>| Range::encode(&<$WrapType>::from(d.clone()))) as AlgoFn,
            Box::new(|b: &[u8]| Range::decode::<$WrapType>(b).unwrap().value()) as DecodeFn,
        ));
        $algos.push((
            concat!("ans/", $suffix),
            Box::new(|d: &Vec<u8>| Ans::encode(&<$WrapType>::from(d.clone()))),
            Box::new(|b: &[u8]| Ans::decode::<$WrapType>(b).unwrap().value()),
        ));
    };
}

fn run_benchmarks(heading: &str, algos: &[Algo], data: &Vec<u8>) {
    const W: usize = 9;
    println!("{heading}");
    print!("{:>14}  ", "");
    for (name, _, _) in algos {
        print!(" {:>W$}", name);
    }
    println!();

    // Verify correctness first
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

fn build_algos() -> Vec<Algo> {
    let mut algos: Vec<Algo> = Vec::new();
    add_strategy!(algos, "norm", Encoded<Vec<u8>, Values<Normal>>);
    add_strategy!(algos, "sml", Encoded<Vec<u8>, Values<Small>>);
    add_strategy!(algos, "inc", Encoded<Vec<u8>, Values<Incompressible>>);
    // Vec<u8>-level incompressible: stores the whole slice raw (no per-byte entropy coding)
    add_strategy!(algos, "raw", Encoded<Vec<u8>, Incompressible>);
    algos.push((
        "bitcode",
        Box::new(|d: &Vec<u8>| bitcode::encode(d)),
        Box::new(|b: &[u8]| bitcode::decode::<Vec<u8>>(b).unwrap()),
    ));
    algos
}

fn main() {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

    let random: Vec<u8> = (0..N).map(|_| rng.gen()).collect();
    let ascii: Vec<u8> = (0..N).map(|_| rng.gen_range(32u8..127)).collect();
    let sparse: Vec<u8> = (0..N).map(|_| rng.gen_range(0u8..16)).collect();
    let repeated: Vec<u8> = (0..N).map(|i| (i % 4) as u8).collect();

    let algos = build_algos();
    run_benchmarks(&format!("Vec<u8> random ({N} values)"), &algos, &random);
    run_benchmarks(&format!("Vec<u8> ascii ({N} values)"), &algos, &ascii);
    run_benchmarks(
        &format!("Vec<u8> sparse 0..16 ({N} values)"),
        &algos,
        &sparse,
    );
    run_benchmarks(
        &format!("Vec<u8> repeated ({N} values)"),
        &algos,
        &repeated,
    );
}
