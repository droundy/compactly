//! Shootout benchmark for `AtMost<MAX>` tree-walk strategies.
//!
//! Times every distribution ([`Dist`]) x coder (`Ans`, `Range`) x value
//! count (`MAX`) x applicable
//! [`Walk`] for decode, and once per *distinct* encode implementation (a
//! speculating walk shares its plain twin's encode body — see
//! [`Walk::encode_with`] — so timing it a second time would just be two
//! noisy measurements of the same code), so the assumptions baked into
//! `Walk::production` (see `src/v2/atmost/walks.rs`) can be re-checked
//! against measurements taken here and now, on whatever machine is running
//! this. The walk each coder's `Walk::production` currently selects for a
//! given `MAX` is marked with `*`; an encode row a walk shares with another
//! (a speculating variant) prints `-` instead of a duplicate measurement.
//!
//! A walk beats `Walk::production`'s choice only if it is faster by at least
//! [`NOMINATE_FRACTION`] *and* the gap is [`SIGMAS`] times the error bars on
//! the two measurements — `scaling` measures those, so a margin can be tested
//! for significance where it used to have to be re-run and eyeballed. Each
//! finding reports its margin on every distribution swept, so a walk that
//! only wins on one kind of data shows up as a lopsided (or partly negative)
//! range rather than as a clean win, and a margin the error bars cannot
//! support is marked `?` rather than dropped.
//!
//! Reserve a CPU first (`quiet-bench reserve`, then `quiet-bench run cargo
//! bench …`): the `±` covers sampling noise within this process, not the
//! machine around it. The run says so on stderr if you did not.
//!
//! Uses the `#[doc(hidden)]` `encode_atmost_batch`/`decode_atmost_batch`
//! benchmark-support methods on `Range`/`Ans`, which force a specific `Walk`
//! (by indexing `WALKS` with a `const WHICH_WALK` generic) instead of going
//! through `Walk::production`.
//!
//! Run with `cargo bench --bench atmost`. Sweeps both [`Dist`]ributions by
//! default; set `ATMOST_DIST=uniform` or `ATMOST_DIST=skewed` to sweep just
//! one (halves the runtime, and lets separate quiesced processes measure the
//! two distributions independently).

use compactly::benchmarking::{config, per_unit, warn_unless_quiet};
use compactly::v2::{Ans, AtMost, Range, Walk, WALKS};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scaling::Stats;

/// Values per batch for a given `MAX`: large enough that the fixed per-call
/// overhead (context setup — `AtMostContext::<MAX>::default()` seeds `MAX`
/// tree nodes — plus `Vec` allocation) stays a small fraction of the
/// per-value cost being measured, even at the largest `MAX` this shootout
/// covers.
const fn n_values(max: usize) -> usize {
    let scaled = 4 * max;
    if scaled > 256 {
        scaled
    } else {
        256
    }
}

/// How the benchmark values are distributed over `0..=MAX`. The walks'
/// relative speed depends on the data: uniform values keep every context at
/// 50/50 (no adaptation, maximum entropy) and make the walk path
/// branch-unpredictable — the best case for the latency-hiding speculating
/// walks — while production data (string bytes, length buckets, enum
/// discriminants) concentrates on a few values, so contexts adapt hard and
/// the hardware branch predictor learns the walk path on its own. A finding
/// is only trustworthy if it holds on the distribution shaped like the
/// workload it would affect.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dist {
    Uniform,
    /// `floor((MAX + 1) * u^8)` for uniform `u`: value 0 takes ~50% of the
    /// mass at `MAX = 255` (like a dominant char or enum variant), with a
    /// heavy tail that still reaches every value.
    Skewed,
}

impl Dist {
    fn sample<const MAX: usize>(self, rng: &mut SmallRng) -> AtMost<MAX> {
        AtMost::new(match self {
            Dist::Uniform => rng.gen_range(0..=MAX),
            Dist::Skewed => {
                let u: f64 = rng.gen();
                (((MAX + 1) as f64 * u.powi(8)) as usize).min(MAX)
            }
        })
    }
}

#[derive(Clone)]
struct Timing {
    /// `None` for a walk whose encode implementation is shared with another
    /// walk in the sweep (see [`Walk::encode_with`]) — the shootout only
    /// times encode once per distinct implementation.
    encode: Option<Stats>,
    decode: Stats,
}

fn gen_values<const MAX: usize>(dist: Dist, rng: &mut SmallRng, n: usize) -> Vec<AtMost<MAX>> {
    (0..n).map(|_| dist.sample::<MAX>(rng)).collect()
}

/// Time one encode function, in ns/value with its standard error. A fresh
/// batch is generated for every timed iteration (untimed by
/// `bench_gen_env`), so branch prediction can't learn a fixed sequence.
fn bench_encode<const MAX: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    encode: fn(&[AtMost<MAX>]) -> Vec<u8>,
) -> Stats {
    let n = n_values(MAX);
    let stats = config().bench_gen_env(
        || gen_values::<MAX>(dist, rng, n),
        |values: &mut Vec<AtMost<MAX>>| encode(values.as_slice()),
    );
    per_unit(&stats, n)
}

/// Time one decode function, in ns/value with its standard error. Every
/// decode is checked to round-trip.
fn bench_decode<const MAX: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    encode: fn(&[AtMost<MAX>]) -> Vec<u8>,
    decode: fn(&[u8], usize) -> Vec<AtMost<MAX>>,
) -> Stats {
    let n = n_values(MAX);
    let stats = config().bench_gen_env(
        || {
            let values = gen_values::<MAX>(dist, rng, n);
            let bytes = encode(&values);
            (values, bytes)
        },
        |env: &mut (Vec<AtMost<MAX>>, Vec<u8>)| {
            let (values, bytes) = env;
            let decoded = decode(bytes.as_slice(), values.len());
            assert_eq!(&decoded, values, "round-trip failed for MAX={MAX}");
        },
    );
    per_unit(&stats, n)
}

/// `None` if `WALKS[WHICH_WALK]` isn't a valid implementation for this
/// `MAX` (the shootout skips those combinations); otherwise the walk timed
/// against both coders. Encode is only timed for a walk that is its own
/// [`Walk::encode_with`] — a speculating walk shares its plain twin's encode
/// body, so timing it separately would just double-count the same code.
fn bench_walk<const MAX: usize, const WHICH_WALK: usize>(
    dist: Dist,
    rng: &mut SmallRng,
) -> Option<(Walk, Timing, Timing)> {
    let walk = WALKS[WHICH_WALK];
    if !walk.applies_to::<MAX>() {
        return None;
    }
    let time_encode = walk.encode_with() == walk;
    let ans_encode = time_encode
        .then(|| bench_encode::<MAX>(dist, rng, Ans::encode_atmost_batch::<MAX, WHICH_WALK>));
    let ans_decode = bench_decode::<MAX>(
        dist,
        rng,
        Ans::encode_atmost_batch::<MAX, WHICH_WALK>,
        Ans::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    let range_encode = time_encode
        .then(|| bench_encode::<MAX>(dist, rng, Range::encode_atmost_batch::<MAX, WHICH_WALK>));
    let range_decode = bench_decode::<MAX>(
        dist,
        rng,
        Range::encode_atmost_batch::<MAX, WHICH_WALK>,
        Range::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    Some((
        walk,
        Timing {
            encode: ans_encode,
            decode: ans_decode,
        },
        Timing {
            encode: range_encode,
            decode: range_decode,
        },
    ))
}

/// One distribution's timings for a (production, challenger) pair.
#[derive(Clone)]
struct DistMargin {
    dist: Dist,
    production: Stats,
    better: Stats,
}

impl DistMargin {
    /// Fraction by which the challenger beat production (negative if it
    /// lost).
    fn margin(&self) -> f64 {
        (self.production.ns_per_iter - self.better.ns_per_iter) / self.production.ns_per_iter
    }

    /// Standard error of [`DistMargin::margin`]: the two relative errors
    /// added in quadrature and carried through the ratio.
    fn margin_error(&self) -> f64 {
        let ratio = self.better.ns_per_iter / self.production.ns_per_iter;
        ratio
            * (self.better.rel_std_error().powi(2) + self.production.rel_std_error().powi(2)).sqrt()
    }

    /// Is the gap bigger than the error bars can explain, and were those
    /// error bars worth believing in the first place?
    fn significant(&self) -> bool {
        !self.production.untrustworthy
            && !self.better.untrustworthy
            && self.margin() > SIGMAS * self.margin_error()
    }
}

/// How many standard errors a margin must clear to count. Three, so a
/// finding is not one run in twenty: with [`NOMINATE_FRACTION`] at 5% and
/// measurements good to 0.1%, a real win clears this by a wide margin and a
/// coincidence does not clear it at all.
const SIGMAS: f64 = 3.0;

/// One walk that beat production's choice by at least [`NOMINATE_FRACTION`],
/// significantly (see [`DistMargin::significant`]), on at least one swept
/// distribution. `sweep` holds the pair's timings on *every* swept
/// distribution, so the summary can report the full cross-distribution
/// range.
#[derive(Clone)]
struct Finding {
    max: usize,
    coder: &'static str,
    metric: &'static str,
    production: Walk,
    better: Walk,
    sweep: Vec<DistMargin>,
}

/// How much faster a walk must be than production's choice to be worth
/// reporting. This is a question about whether a difference *matters*, not
/// whether it is real — [`SIGMAS`] answers that — and 5% is well above the
/// ~1% that separate builds of the same code differ by anyway.
const NOMINATE_FRACTION: f64 = 0.05;

/// Time and print every applicable [`Walk`] for one `MAX` on every swept
/// distribution, appending a [`Finding`] for each (coder, metric, walk)
/// where the walk beats the one [`Walk::production`] actually picked by at
/// least [`NOMINATE_FRACTION`] on at least one distribution.
fn bench_one_max<const MAX: usize>(dists: &[Dist], findings: &mut Vec<Finding>) {
    let ans_production = Walk::production::<MAX>(Ans::SPECULATES);
    let range_production = Walk::production::<MAX>(Range::SPECULATES);

    let mut ans_by_dist: Vec<(Dist, Vec<(Walk, Timing)>)> = Vec::new();
    let mut range_by_dist: Vec<(Dist, Vec<(Walk, Timing)>)> = Vec::new();
    for &dist in dists {
        let mut rng = SmallRng::seed_from_u64(sweep_seed(MAX, dist));
        println!(
            "\nMAX = {MAX} (values 0..={MAX}, {} possible), {dist:?}",
            MAX + 1
        );
        println!(
            "{:<22} {:>20} {:>20}   {:>20} {:>20}",
            "walk", "ans enc", "ans dec", "range enc", "range dec"
        );

        let mut results: Vec<(Walk, Timing, Timing)> = Vec::new();
        collect_walk::<MAX, 0>(dist, &mut rng, &mut results);
        collect_walk::<MAX, 1>(dist, &mut rng, &mut results);
        collect_walk::<MAX, 2>(dist, &mut rng, &mut results);
        collect_walk::<MAX, 3>(dist, &mut rng, &mut results);
        collect_walk::<MAX, 4>(dist, &mut rng, &mut results);
        collect_walk::<MAX, 5>(dist, &mut rng, &mut results);

        for (walk, ans, range) in &results {
            println!(
                "{:<22} {:>20} {:>20}   {:>20} {:>20}",
                format!("{walk:?}"),
                fmt_ns(
                    ans_production.map(Walk::encode_with) == Some(*walk),
                    ans.encode.as_ref()
                ),
                fmt_ns(ans_production == Some(*walk), Some(&ans.decode)),
                fmt_ns(
                    range_production.map(Walk::encode_with) == Some(*walk),
                    range.encode.as_ref()
                ),
                fmt_ns(range_production == Some(*walk), Some(&range.decode)),
            );
        }

        ans_by_dist.push((
            dist,
            results
                .iter()
                .map(|(w, ans, _)| (*w, ans.clone()))
                .collect(),
        ));
        range_by_dist.push((
            dist,
            results
                .iter()
                .map(|(w, _, range)| (*w, range.clone()))
                .collect(),
        ));
    }

    record_findings::<MAX>("ans", ans_production, &ans_by_dist, findings);
    record_findings::<MAX>("range", range_production, &range_by_dist, findings);
}

/// Deterministic per-(`MAX`, distribution) seed for the initial sweep, so a
/// rerun reproduces the same value batches.
fn sweep_seed(max: usize, dist: Dist) -> u64 {
    0xC0FFEE ^ max as u64 ^ ((dist as u64) << 32)
}

/// Render one table cell: `-` when there's no measurement (a shared encode
/// implementation, skipped — see [`Timing::encode`]), otherwise the timing
/// and its `±`, with a leading `*` iff `marked`.
fn fmt_ns(marked: bool, stats: Option<&Stats>) -> String {
    match stats {
        Some(s) => format!("{}{s}", if marked { "*" } else { " " }),
        None => "-".to_string(),
    }
}

/// `WHICH_WALK` must be a `const` generic (not a runtime loop variable), so
/// callers unroll the six indices into [`WALKS`] explicitly.
fn collect_walk<const MAX: usize, const WHICH_WALK: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    results: &mut Vec<(Walk, Timing, Timing)>,
) {
    if let Some(result) = bench_walk::<MAX, WHICH_WALK>(dist, rng) {
        results.push(result);
    }
}

/// For one coder's per-distribution walk timings, nominate decode findings
/// (comparing every walk's decode) and encode findings (comparing only the
/// walks that actually timed an encode — see [`Timing::encode_ns`] — against
/// `production`'s canonical encode walk, [`Walk::encode_with`]).
fn record_findings<const MAX: usize>(
    coder: &'static str,
    production: Option<Walk>,
    per_dist: &[(Dist, Vec<(Walk, Timing)>)],
    findings: &mut Vec<Finding>,
) {
    let Some(production) = production else {
        return;
    };
    let decode: Vec<(Dist, Vec<(Walk, Stats)>)> = per_dist
        .iter()
        .map(|(dist, timings)| {
            (
                *dist,
                timings
                    .iter()
                    .map(|(w, t)| (*w, t.decode.clone()))
                    .collect(),
            )
        })
        .collect();
    record_metric::<MAX>(coder, "decode", production, &decode, findings);
    let encode: Vec<(Dist, Vec<(Walk, Stats)>)> = per_dist
        .iter()
        .map(|(dist, timings)| {
            (
                *dist,
                timings
                    .iter()
                    .filter_map(|(w, t)| t.encode.clone().map(|s| (*w, s)))
                    .collect(),
            )
        })
        .collect();
    record_metric::<MAX>(coder, "encode", production.encode_with(), &encode, findings);
}

/// Find each distribution's fastest walk and, for every one that isn't
/// `production`, beats it by at least [`NOMINATE_FRACTION`], and does so by
/// more than [`SIGMAS`] error bars, append one [`Finding`] carrying the
/// pair's timings on *all* distributions (so the summary can report the
/// cross-distribution range).
fn record_metric<const MAX: usize>(
    coder: &'static str,
    metric: &'static str,
    production: Walk,
    per_dist: &[(Dist, Vec<(Walk, Stats)>)],
    findings: &mut Vec<Finding>,
) {
    let stats_of = |timings: &[(Walk, Stats)], walk: Walk| {
        timings
            .iter()
            .find(|(w, _)| *w == walk)
            .map(|(_, s)| s.clone())
    };
    let mut challengers: Vec<Walk> = Vec::new();
    for (dist, timings) in per_dist {
        let Some(production_stats) = stats_of(timings, production) else {
            continue;
        };
        let Some((best_walk, best_stats)) = timings
            .iter()
            .min_by(|(_, a), (_, b)| a.ns_per_iter.total_cmp(&b.ns_per_iter))
        else {
            continue;
        };
        let candidate = DistMargin {
            dist: *dist,
            production: production_stats,
            better: best_stats.clone(),
        };
        if *best_walk != production
            && candidate.margin() >= NOMINATE_FRACTION
            && candidate.significant()
            && !challengers.contains(best_walk)
        {
            challengers.push(*best_walk);
        }
    }
    for better in challengers {
        let sweep: Vec<DistMargin> = per_dist
            .iter()
            .filter_map(|(dist, timings)| {
                Some(DistMargin {
                    dist: *dist,
                    production: stats_of(timings, production)?,
                    better: stats_of(timings, better)?,
                })
            })
            .collect();
        findings.push(Finding {
            max: MAX,
            coder,
            metric,
            production,
            better,
            sweep,
        });
    }
}

/// Every `MAX` this shootout covers, unrolled once as the single source of
/// truth: `$mac!(<max literal>, $($arg),*)` is invoked for each. Used both
/// to drive the initial sweep ([`main`]) and to dispatch a nominated
/// [`Finding`]'s runtime `max` back to its `const` generic for confirmation
/// (see [`confirm_finding`]).
///
/// Power-of-two boundaries (1, 3, 7, 15, 31, 63, 127 are MAX+1 == power of
/// two) and the `SPECULATE_MIN_MAX = 3` cutoff are covered on both sides;
/// coverage above 512 is sparse and non-power-of-two (plus one large power
/// of two, 2048). The cluster at 33/34/40/48 brackets the uneven tree's
/// worst-case-depth step from 6 to 7 (`tree_depth(35)` is the first 7), where
/// a 2026-07-12 quiesced run showed Range's `UnevenSpeculating` decode
/// flipping from winner (MAX <= 32) to 19-24% loser (MAX = 64..=512) — the
/// speculating walk's unroll grows with worst-case depth, so these points
/// test whether the flip follows the depth step or the value count. `AtMostContext::<MAX>`'s compile-time context seeding is
/// `O(MAX)` tree nodes, which trips rustc's `long_running_const_eval` lint
/// (deny by default) somewhere around `MAX ~ 4200` — well short of
/// `SymbolRange::M` (65536) — so 4095 is the practical ceiling for
/// `AtMost<MAX>` today, not just for this benchmark.
macro_rules! for_each_max {
    ($mac:ident, $($arg:expr),*) => {
        $mac!(1, $($arg),*);
        $mac!(2, $($arg),*);
        $mac!(3, $($arg),*);
        $mac!(4, $($arg),*);
        $mac!(5, $($arg),*);
        $mac!(7, $($arg),*);
        $mac!(8, $($arg),*);
        $mac!(9, $($arg),*);
        $mac!(15, $($arg),*);
        $mac!(16, $($arg),*);
        $mac!(17, $($arg),*);
        $mac!(31, $($arg),*);
        $mac!(32, $($arg),*);
        $mac!(33, $($arg),*);
        $mac!(34, $($arg),*);
        $mac!(40, $($arg),*);
        $mac!(48, $($arg),*);
        $mac!(63, $($arg),*);
        $mac!(64, $($arg),*);
        $mac!(127, $($arg),*);
        $mac!(128, $($arg),*);
        $mac!(255, $($arg),*);
        $mac!(256, $($arg),*);
        $mac!(512, $($arg),*);
        $mac!(700, $($arg),*);
        $mac!(2048, $($arg),*);
        $mac!(3000, $($arg),*);
        $mac!(4095, $($arg),*);
    };
}

/// Render per-distribution margins as an ascending range, e.g.
/// `"19±2% (Uniform) .. 41±2% (Skewed)"` (a single-distribution run
/// collapses to one entry). A `?` marks a distribution whose margin the
/// error bars cannot support — the number is printed rather than hidden,
/// since "measured, not significant" is itself worth seeing.
fn fmt_margin_range(sweep: &[DistMargin]) -> String {
    let mut entries: Vec<&DistMargin> = sweep.iter().collect();
    entries.sort_by(|a, b| a.margin().total_cmp(&b.margin()));
    entries
        .iter()
        .map(|m| {
            format!(
                "{:.0}±{:.0}%{} ({:?})",
                m.margin() * 100.0,
                m.margin_error() * 100.0,
                if m.significant() { "" } else { "?" },
                m.dist,
            )
        })
        .collect::<Vec<_>>()
        .join(" .. ")
}

fn print_findings_summary(findings: &mut [Finding]) {
    println!(
        "\n=== Summary: walks beating Walk::production by >= {:.0}% on some \
         distribution, by more than {SIGMAS:.0} standard errors (`?` marks a \
         margin the error bars cannot support) ===",
        NOMINATE_FRACTION * 100.0,
    );
    if findings.is_empty() {
        println!("(none)");
        return;
    }
    let best = |f: &Finding| {
        f.sweep
            .iter()
            .map(DistMargin::margin)
            .fold(f64::NEG_INFINITY, f64::max)
    };
    findings.sort_by(|a, b| a.coder.cmp(b.coder).then(best(b).total_cmp(&best(a))));
    for f in findings.iter() {
        println!(
            "MAX={:<6} {:<5} {:<6}: production {:?} vs {:?} — faster by {}",
            f.max,
            f.coder,
            f.metric,
            f.production,
            f.better,
            fmt_margin_range(&f.sweep),
        );
    }
}

fn main() {
    warn_unless_quiet();
    println!(
        "AtMost<MAX> walk shootout: ns/value ± standard error, batch size scales \
         with MAX (min 256). `*` marks the walk Walk::production currently picks \
         for that coder; `-` marks an encode row shared with another walk (not \
         timed separately)."
    );
    let mut findings: Vec<Finding> = Vec::new();
    macro_rules! sweep {
        ($max:literal, $dists:expr, $findings:expr) => {{
            const MAX: usize = $max;
            bench_one_max::<MAX>($dists, $findings);
        }};
    }
    let dists: &[Dist] = match std::env::var("ATMOST_DIST").as_deref() {
        Ok("uniform") => &[Dist::Uniform],
        Ok("skewed") => &[Dist::Skewed],
        Ok(other) => panic!("ATMOST_DIST must be `uniform` or `skewed`, not {other:?}"),
        Err(_) => &[Dist::Uniform, Dist::Skewed],
    };
    for_each_max!(sweep, dists, &mut findings);
    print_findings_summary(&mut findings);
}
