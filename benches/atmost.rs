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

/// One walk beating production's choice by at least [`SIGNIFICANT_FRACTION`]
/// on one (coder, encode/decode) metric.
struct Finding {
    max: usize,
    coder: &'static str,
    metric: &'static str,
    production: Walk,
    production_ns: f64,
    better: Walk,
    better_ns: f64,
}

impl Finding {
    fn pct_faster(&self) -> f64 {
        100.0 * (self.production_ns - self.better_ns) / self.production_ns
    }
}

/// A walk counts as "significantly" faster than production's choice if it
/// beats it by at least this fraction — small enough to catch a real effect,
/// large enough to stay above the run-to-run noise floor on an unquiesced
/// machine (see `bench-quiet.sh` in the repo root for a quieter setup).
const SIGNIFICANT_FRACTION: f64 = 0.10;

/// Time and print every applicable [`Walk`] for one `MAX`, appending a
/// [`Finding`] for each (coder, metric) where some walk beats the one
/// [`Walk::production`] actually picked by at least [`SIGNIFICANT_FRACTION`].
/// `Ans`'s `SPECULATES` is `false`, `Range`'s is `true` (see
/// `SymbolDecoder::SPECULATES` in `src/v2/model.rs`) — hardcoded here since
/// that trait const isn't part of the exposed benchmark surface.
fn bench_one_max<const MAX: usize>(rng: &mut SmallRng, findings: &mut Vec<Finding>) {
    println!("\nMAX = {MAX} (values 0..={MAX}, {} possible)", MAX + 1);
    println!(
        "{:<22} {:>11} {:>11}   {:>11} {:>11}",
        "walk", "ans enc", "ans dec", "range enc", "range dec"
    );

    let mut results: Vec<(Walk, Timing, Timing)> = Vec::new();
    collect_walk::<MAX, 0>(rng, &mut results);
    collect_walk::<MAX, 1>(rng, &mut results);
    collect_walk::<MAX, 2>(rng, &mut results);
    collect_walk::<MAX, 3>(rng, &mut results);
    collect_walk::<MAX, 4>(rng, &mut results);
    collect_walk::<MAX, 5>(rng, &mut results);

    for (walk, ans, range) in &results {
        let ans_mark = if Walk::production::<MAX>(false) == Some(*walk) {
            "*"
        } else {
            " "
        };
        let range_mark = if Walk::production::<MAX>(true) == Some(*walk) {
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

    let ans_metrics: Vec<(Walk, f64, f64)> = results
        .iter()
        .map(|(w, ans, _)| (*w, ans.encode_ns, ans.decode_ns))
        .collect();
    record_findings::<MAX>(
        "ans",
        Walk::production::<MAX>(false),
        &ans_metrics,
        findings,
    );
    let range_metrics: Vec<(Walk, f64, f64)> = results
        .iter()
        .map(|(w, _, range)| (*w, range.encode_ns, range.decode_ns))
        .collect();
    record_findings::<MAX>(
        "range",
        Walk::production::<MAX>(true),
        &range_metrics,
        findings,
    );
}

/// `WHICH_WALK` must be a `const` generic (not a runtime loop variable), so
/// callers unroll the six indices into [`WALKS`] explicitly.
fn collect_walk<const MAX: usize, const WHICH_WALK: usize>(
    rng: &mut SmallRng,
    results: &mut Vec<(Walk, Timing, Timing)>,
) {
    if let Some(result) = bench_walk::<MAX, WHICH_WALK>(rng) {
        results.push(result);
    }
}

/// For one coder's `(walk, encode_ns, decode_ns)` measurements, find the
/// fastest walk on each metric and, if it isn't `production` and beats it by
/// at least [`SIGNIFICANT_FRACTION`], append a [`Finding`].
fn record_findings<const MAX: usize>(
    coder: &'static str,
    production: Option<Walk>,
    metrics: &[(Walk, f64, f64)],
    findings: &mut Vec<Finding>,
) {
    let Some(production) = production else {
        return;
    };
    for (metric, encode) in [("encode", true), ("decode", false)] {
        let mut production_ns = None;
        let mut best: Option<(Walk, f64)> = None;
        for &(walk, encode_ns, decode_ns) in metrics {
            let value = if encode { encode_ns } else { decode_ns };
            if walk == production {
                production_ns = Some(value);
            }
            match &mut best {
                Some((_, best_ns)) if *best_ns <= value => {}
                _ => best = Some((walk, value)),
            }
        }
        let (Some(production_ns), Some((best_walk, best_ns))) = (production_ns, best) else {
            continue;
        };
        if best_walk != production && best_ns < production_ns * (1.0 - SIGNIFICANT_FRACTION) {
            findings.push(Finding {
                max: MAX,
                coder,
                metric,
                production,
                production_ns,
                better: best_walk,
                better_ns: best_ns,
            });
        }
    }
}

fn print_findings_summary(findings: &mut [Finding]) {
    println!(
        "\n=== Summary: walks that beat Walk::production by >= {:.0}% ===",
        SIGNIFICANT_FRACTION * 100.0
    );
    if findings.is_empty() {
        println!(
            "(none — production's choice was within {:.0}% of the fastest walk everywhere measured)",
            SIGNIFICANT_FRACTION * 100.0
        );
        return;
    }
    findings.sort_by(|a, b| {
        a.coder
            .cmp(b.coder)
            .then(b.pct_faster().partial_cmp(&a.pct_faster()).unwrap())
    });
    for f in findings {
        println!(
            "MAX={:<6} {:<5} {:<6}: production {:?} ({:.1}ns) vs {:?} ({:.1}ns) — {:.0}% faster",
            f.max,
            f.coder,
            f.metric,
            f.production,
            f.production_ns,
            f.better,
            f.better_ns,
            f.pct_faster(),
        );
    }
}

/// `MAX` must be a `const` generic (not a runtime loop variable), so
/// `main` unrolls each value as its own macro invocation.
macro_rules! bench_max {
    ($findings:expr, $max:expr) => {{
        const MAX: usize = $max;
        let mut rng = SmallRng::seed_from_u64(0xC0FFEE ^ MAX as u64);
        bench_one_max::<MAX>(&mut rng, $findings);
    }};
}

fn main() {
    println!(
        "AtMost<MAX> walk shootout: ns/value, {N_VALUES} values/batch. \
         `*` marks the walk Walk::production currently picks for that coder."
    );
    let mut findings: Vec<Finding> = Vec::new();
    // Power-of-two boundaries (1, 3, 7, 15, 31, 63, 127 are MAX+1 == power
    // of two) and the SPECULATE_MIN_MAX = 3 cutoff, both sides.
    bench_max!(&mut findings, 1);
    bench_max!(&mut findings, 2);
    bench_max!(&mut findings, 3);
    bench_max!(&mut findings, 4);
    bench_max!(&mut findings, 5);
    bench_max!(&mut findings, 7);
    bench_max!(&mut findings, 8);
    bench_max!(&mut findings, 9);
    bench_max!(&mut findings, 15);
    bench_max!(&mut findings, 16);
    bench_max!(&mut findings, 17);
    bench_max!(&mut findings, 31);
    bench_max!(&mut findings, 32);
    bench_max!(&mut findings, 63);
    bench_max!(&mut findings, 64);
    bench_max!(&mut findings, 127);
    bench_max!(&mut findings, 128);
    bench_max!(&mut findings, 255);
    bench_max!(&mut findings, 256);
    bench_max!(&mut findings, 512);
    // Sparse, non-power-of-two coverage above 512 (plus one large power of
    // two, 2048). AtMostContext::<MAX>'s compile-time context seeding is
    // O(MAX) tree nodes, which trips rustc's long_running_const_eval lint
    // (deny by default) somewhere around MAX ~ 4200 — well short of
    // SymbolRange::M (65536) — so this is the practical ceiling for
    // AtMost<MAX> today, not just for this benchmark.
    bench_max!(&mut findings, 700);
    bench_max!(&mut findings, 2048);
    bench_max!(&mut findings, 3000);
    bench_max!(&mut findings, 4095);

    print_findings_summary(&mut findings);
}
