//! Shootout benchmark for `AtMost<MAX>` tree-walk strategies.
//!
//! Times every coder (`Ans`, `Range`) x value count (`MAX`) x applicable
//! [`Walk`] for encode and decode, so the assumptions baked into
//! `Walk::production` (see `src/v2/atmost/walks.rs`) can be re-checked
//! against measurements taken here and now, on whatever machine is running
//! this. The walk each coder's `Walk::production` currently selects for a
//! given `MAX` is marked with `*`.
//!
//! Uses the `#[doc(hidden)]` `encode_atmost_batch`/`decode_atmost_batch`
//! benchmark-support methods on `Range`/`Ans`, which force a specific `Walk`
//! (by indexing `WALKS` with a `const WHICH_WALK` generic) instead of going
//! through `Walk::production`.
//!
//! Run with `cargo bench --bench atmost`.
#![cfg(feature = "v2")]

use compactly::v2::{Ans, AtMost, Range, Walk, WALKS};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use scaling::bench_gen_env;

/// Values per batch: large enough to amortize the fixed per-call overhead
/// (context setup, `Vec` allocation) down to a small fraction of the
/// per-value cost being measured.
const N_VALUES: usize = 256;

struct Timing {
    encode_ns: f64,
    decode_ns: f64,
}

fn gen_values<const MAX: usize>(rng: &mut SmallRng) -> Vec<AtMost<MAX>> {
    (0..N_VALUES)
        .map(|_| AtMost::new(rng.gen_range(0..=MAX)))
        .collect()
}

/// Time one (coder, walk) pair's encode and decode, in ns/value. A fresh
/// batch of `N_VALUES` random values is generated for every timed iteration
/// (untimed by `bench_gen_env`), so branch prediction can't learn a fixed
/// sequence. Every decode is checked to round-trip.
fn bench_one<const MAX: usize>(
    rng: &mut SmallRng,
    encode: fn(&[AtMost<MAX>]) -> Vec<u8>,
    decode: fn(&[u8], usize) -> Vec<AtMost<MAX>>,
) -> Timing {
    let encode_ns = bench_gen_env(
        || gen_values::<MAX>(rng),
        |values: &mut Vec<AtMost<MAX>>| encode(values.as_slice()),
    )
    .ns_per_iter
        / N_VALUES as f64;

    let decode_ns = bench_gen_env(
        || {
            let values = gen_values::<MAX>(rng);
            let bytes = encode(&values);
            (values, bytes)
        },
        |env: &mut (Vec<AtMost<MAX>>, Vec<u8>)| {
            let (values, bytes) = env;
            let decoded = decode(bytes.as_slice(), values.len());
            assert_eq!(&decoded, values, "round-trip failed for MAX={MAX}");
        },
    )
    .ns_per_iter
        / N_VALUES as f64;

    Timing {
        encode_ns,
        decode_ns,
    }
}

/// `None` if `WALKS[WHICH_WALK]` isn't a valid implementation for this
/// `MAX` (the shootout skips those combinations); otherwise the walk timed
/// against both coders.
fn bench_walk<const MAX: usize, const WHICH_WALK: usize>(
    rng: &mut SmallRng,
) -> Option<(Walk, Timing, Timing)> {
    let walk = WALKS[WHICH_WALK];
    if !walk.applies_to::<MAX>() {
        return None;
    }
    let ans = bench_one::<MAX>(
        rng,
        Ans::encode_atmost_batch::<MAX, WHICH_WALK>,
        Ans::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    let range = bench_one::<MAX>(
        rng,
        Range::encode_atmost_batch::<MAX, WHICH_WALK>,
        Range::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    Some((walk, ans, range))
}

/// `Ans`'s `SPECULATES` is `false`, `Range`'s is `true` (see
/// `SymbolDecoder::SPECULATES` in `src/v2/model.rs`) — hardcoded here since
/// that trait const isn't part of the exposed benchmark surface.
fn print_walk<const MAX: usize, const WHICH_WALK: usize>(rng: &mut SmallRng) {
    let Some((walk, ans, range)) = bench_walk::<MAX, WHICH_WALK>(rng) else {
        return;
    };
    let ans_mark = if Walk::production::<MAX>(false) == Some(walk) {
        "*"
    } else {
        " "
    };
    let range_mark = if Walk::production::<MAX>(true) == Some(walk) {
        "*"
    } else {
        " "
    };
    println!(
        "{:<22} {:>11} {:>11}   {:>11} {:>11}",
        format!("{walk:?}"),
        format!("{ans_mark}{:.1}ns", ans.encode_ns),
        format!("{ans_mark}{:.1}ns", ans.decode_ns),
        format!("{range_mark}{:.1}ns", range.encode_ns),
        format!("{range_mark}{:.1}ns", range.decode_ns),
    );
}

/// Bench every `Walk` for one `MAX`. `WHICH_WALK` must be a `const` generic
/// (not a runtime loop variable), so the six indices into [`WALKS`] are
/// unrolled explicitly.
macro_rules! bench_max {
    ($max:expr) => {{
        const MAX: usize = $max;
        let mut rng = SmallRng::seed_from_u64(0xC0FFEE ^ MAX as u64);
        println!("\nMAX = {MAX} (values 0..={MAX}, {} possible)", MAX + 1);
        println!(
            "{:<22} {:>11} {:>11}   {:>11} {:>11}",
            "walk", "ans enc", "ans dec", "range enc", "range dec"
        );
        print_walk::<MAX, 0>(&mut rng);
        print_walk::<MAX, 1>(&mut rng);
        print_walk::<MAX, 2>(&mut rng);
        print_walk::<MAX, 3>(&mut rng);
        print_walk::<MAX, 4>(&mut rng);
        print_walk::<MAX, 5>(&mut rng);
    }};
}

fn main() {
    println!(
        "AtMost<MAX> walk shootout: ns/value, {N_VALUES} values/batch. \
         `*` marks the walk Walk::production currently picks for that coder."
    );
    // Power-of-two boundaries (1, 3, 7, 15, 31, 63, 127 are MAX+1 == power
    // of two) and the SPECULATE_MIN_MAX = 3 cutoff, both sides.
    bench_max!(1);
    bench_max!(2);
    bench_max!(3);
    bench_max!(4);
    bench_max!(5);
    bench_max!(7);
    bench_max!(8);
    bench_max!(9);
    bench_max!(15);
    bench_max!(16);
    bench_max!(17);
    bench_max!(31);
    bench_max!(32);
    bench_max!(63);
    bench_max!(64);
    bench_max!(127);
    bench_max!(128);
    bench_max!(255);
    bench_max!(256);
    bench_max!(512);
}
