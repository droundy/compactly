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
//! A walk that beats `Walk::production`'s choice on the initial sweep of
//! *either* distribution is only *nominated* (see [`NOMINATE_FRACTION`]),
//! then re-timed against production [`CONFIRM_ROUNDS`] times per swept
//! distribution, alternating which walk is measured first each round — this
//! filters out what could otherwise have been a one-off noisy sample on this
//! (often unquiesced) machine. Each confirmed finding reports its range of
//! median margins across the distributions, so a walk that only wins on one
//! kind of data shows up as a lopsided (or partly negative) range rather
//! than as a clean win. See `bench-quiet.sh` in the repo root for a quieter
//! setup.
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

use compactly::v2::{Ans, AtMost, Range, Walk, WALKS};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use scaling::bench_gen_env;

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

#[derive(Clone, Copy)]
struct Timing {
    /// `None` for a walk whose encode implementation is shared with another
    /// walk in the sweep (see [`Walk::encode_with`]) — the shootout only
    /// times encode once per distinct implementation.
    encode_ns: Option<f64>,
    decode_ns: f64,
}

fn gen_values<const MAX: usize>(dist: Dist, rng: &mut SmallRng, n: usize) -> Vec<AtMost<MAX>> {
    (0..n).map(|_| dist.sample::<MAX>(rng)).collect()
}

/// Time one encode function, in ns/value. A fresh batch is generated for
/// every timed iteration (untimed by `bench_gen_env`), so branch prediction
/// can't learn a fixed sequence.
fn bench_encode<const MAX: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    encode: fn(&[AtMost<MAX>]) -> Vec<u8>,
) -> f64 {
    let n = n_values(MAX);
    bench_gen_env(
        || gen_values::<MAX>(dist, rng, n),
        |values: &mut Vec<AtMost<MAX>>| encode(values.as_slice()),
    )
    .ns_per_iter
        / n as f64
}

/// Time one decode function, in ns/value. Every decode is checked to
/// round-trip.
fn bench_decode<const MAX: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    encode: fn(&[AtMost<MAX>]) -> Vec<u8>,
    decode: fn(&[u8], usize) -> Vec<AtMost<MAX>>,
) -> f64 {
    let n = n_values(MAX);
    bench_gen_env(
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
    )
    .ns_per_iter
        / n as f64
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
    let ans_encode_ns = time_encode
        .then(|| bench_encode::<MAX>(dist, rng, Ans::encode_atmost_batch::<MAX, WHICH_WALK>));
    let ans_decode_ns = bench_decode::<MAX>(
        dist,
        rng,
        Ans::encode_atmost_batch::<MAX, WHICH_WALK>,
        Ans::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    let range_encode_ns = time_encode
        .then(|| bench_encode::<MAX>(dist, rng, Range::encode_atmost_batch::<MAX, WHICH_WALK>));
    let range_decode_ns = bench_decode::<MAX>(
        dist,
        rng,
        Range::encode_atmost_batch::<MAX, WHICH_WALK>,
        Range::decode_atmost_batch::<MAX, WHICH_WALK>,
    );
    Some((
        walk,
        Timing {
            encode_ns: ans_encode_ns,
            decode_ns: ans_decode_ns,
        },
        Timing {
            encode_ns: range_encode_ns,
            decode_ns: range_decode_ns,
        },
    ))
}

/// One distribution's initial-sweep timings for a (production, challenger)
/// pair.
#[derive(Clone, Copy)]
struct DistMargin {
    dist: Dist,
    production_ns: f64,
    better_ns: f64,
}

impl DistMargin {
    /// Fraction by which the challenger beat production (negative if it
    /// lost).
    fn margin(&self) -> f64 {
        (self.production_ns - self.better_ns) / self.production_ns
    }
}

/// One walk that beat production's choice by at least [`NOMINATE_FRACTION`]
/// on the initial (single-sample) sweep of at least one distribution — a
/// *candidate*, not yet a confirmed finding; see [`confirm_finding`].
/// `sweep` holds the pair's timings on *every* swept distribution, so the
/// confirmation pass can report the full cross-distribution range.
#[derive(Clone)]
struct Finding {
    max: usize,
    coder: &'static str,
    metric: &'static str,
    production: Walk,
    better: Walk,
    sweep: Vec<DistMargin>,
}

/// A walk is *nominated* if it beats production's choice by at least this
/// fraction on the initial sweep. Deliberately low (lower than the old
/// single-sample threshold this replaced) because a nomination isn't trusted
/// on its own — [`confirm_finding`] re-times every nominated pair
/// [`CONFIRM_ROUNDS`] times before calling it a finding, so nominating too
/// eagerly costs a few extra reruns rather than a false result.
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
            "{:<22} {:>11} {:>11}   {:>11} {:>11}",
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
                "{:<22} {:>11} {:>11}   {:>11} {:>11}",
                format!("{walk:?}"),
                fmt_ns(
                    ans_production.map(Walk::encode_with) == Some(*walk),
                    ans.encode_ns
                ),
                fmt_ns(ans_production == Some(*walk), Some(ans.decode_ns)),
                fmt_ns(
                    range_production.map(Walk::encode_with) == Some(*walk),
                    range.encode_ns
                ),
                fmt_ns(range_production == Some(*walk), Some(range.decode_ns)),
            );
        }

        ans_by_dist.push((dist, results.iter().map(|(w, ans, _)| (*w, *ans)).collect()));
        range_by_dist.push((
            dist,
            results.iter().map(|(w, _, range)| (*w, *range)).collect(),
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
/// implementation, skipped — see [`Timing::encode_ns`]), otherwise the
/// timing with a leading `*` iff `marked`.
fn fmt_ns(marked: bool, ns: Option<f64>) -> String {
    match ns {
        Some(ns) => format!("{}{:.1}ns", if marked { "*" } else { " " }, ns),
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
    let decode: Vec<(Dist, Vec<(Walk, f64)>)> = per_dist
        .iter()
        .map(|(dist, timings)| {
            (
                *dist,
                timings.iter().map(|(w, t)| (*w, t.decode_ns)).collect(),
            )
        })
        .collect();
    record_metric::<MAX>(coder, "decode", production, &decode, findings);
    let encode: Vec<(Dist, Vec<(Walk, f64)>)> = per_dist
        .iter()
        .map(|(dist, timings)| {
            (
                *dist,
                timings
                    .iter()
                    .filter_map(|(w, t)| t.encode_ns.map(|ns| (*w, ns)))
                    .collect(),
            )
        })
        .collect();
    record_metric::<MAX>(coder, "encode", production.encode_with(), &encode, findings);
}

/// Find each distribution's fastest walk and, for every one that isn't
/// `production` and beats it by at least [`NOMINATE_FRACTION`] on its
/// nominating distribution, append one [`Finding`] carrying the pair's
/// sweep timings on *all* distributions (so the summary can report the
/// cross-distribution range).
fn record_metric<const MAX: usize>(
    coder: &'static str,
    metric: &'static str,
    production: Walk,
    per_dist: &[(Dist, Vec<(Walk, f64)>)],
    findings: &mut Vec<Finding>,
) {
    let ns_of = |timings: &[(Walk, f64)], walk: Walk| {
        timings.iter().find(|(w, _)| *w == walk).map(|&(_, ns)| ns)
    };
    let mut challengers: Vec<Walk> = Vec::new();
    for (_, timings) in per_dist {
        let Some(production_ns) = ns_of(timings, production) else {
            continue;
        };
        let Some(&(best_walk, best_ns)) = timings.iter().min_by(|(_, a), (_, b)| a.total_cmp(b))
        else {
            continue;
        };
        if best_walk != production
            && best_ns < production_ns * (1.0 - NOMINATE_FRACTION)
            && !challengers.contains(&best_walk)
        {
            challengers.push(best_walk);
        }
    }
    for better in challengers {
        let sweep: Vec<DistMargin> = per_dist
            .iter()
            .filter_map(|(dist, timings)| {
                Some(DistMargin {
                    dist: *dist,
                    production_ns: ns_of(timings, production)?,
                    better_ns: ns_of(timings, better)?,
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

/// Rerun rounds for [`confirm_finding`]: each nominated (production,
/// challenger) pair is re-timed this many times, alternating which walk is
/// measured first each round (cancels a monotonic drift/thermal bias across
/// the pair — see the alternate-A/B lesson in OPTIMIZING.md), before being
/// reported as a confirmed finding rather than possible noise.
const CONFIRM_ROUNDS: usize = 3;

/// The rerun verdict for one distribution of a nominated pair.
struct DistConfirmation {
    dist: Dist,
    /// Median signed margin across the rerun rounds (negative: the
    /// challenger lost).
    median_margin: f64,
    /// The challenger won every round with a median of at least
    /// [`NOMINATE_FRACTION`] — the same bar the old single-distribution
    /// confirmation used.
    reproduced: bool,
}

/// Time a single (coder, metric, walk) combination once, `which_walk` being
/// a runtime index into [`WALKS`] dispatched to the `const` generic via a
/// `0..6` match (the const generic can't take a runtime value directly).
fn time_one<const MAX: usize>(
    dist: Dist,
    rng: &mut SmallRng,
    coder: &'static str,
    metric: &'static str,
    which_walk: usize,
) -> f64 {
    macro_rules! dispatch {
        ($which:expr) => {
            match (coder, metric) {
                ("ans", "encode") => {
                    bench_encode::<MAX>(dist, rng, Ans::encode_atmost_batch::<MAX, $which>)
                }
                ("ans", "decode") => bench_decode::<MAX>(
                    dist,
                    rng,
                    Ans::encode_atmost_batch::<MAX, $which>,
                    Ans::decode_atmost_batch::<MAX, $which>,
                ),
                ("range", "encode") => {
                    bench_encode::<MAX>(dist, rng, Range::encode_atmost_batch::<MAX, $which>)
                }
                ("range", "decode") => bench_decode::<MAX>(
                    dist,
                    rng,
                    Range::encode_atmost_batch::<MAX, $which>,
                    Range::decode_atmost_batch::<MAX, $which>,
                ),
                _ => unreachable!("unknown coder/metric {coder}/{metric}"),
            }
        };
    }
    match which_walk {
        0 => dispatch!(0),
        1 => dispatch!(1),
        2 => dispatch!(2),
        3 => dispatch!(3),
        4 => dispatch!(4),
        5 => dispatch!(5),
        _ => unreachable!("WALKS has 6 entries"),
    }
}

/// Re-time `finding`'s (production, challenger) pair [`CONFIRM_ROUNDS`]
/// times on every distribution the sweep covered, alternating measurement
/// order each round. Every distribution's median (signed) margin is
/// reported, so the summary can show the full cross-distribution range even
/// where the challenger lost.
fn confirm_finding_at<const MAX: usize>(
    rng: &mut SmallRng,
    finding: &Finding,
) -> Vec<DistConfirmation> {
    let production_idx = WALKS
        .iter()
        .position(|w| *w == finding.production)
        .expect("a Finding's production walk must be in WALKS");
    let better_idx = WALKS
        .iter()
        .position(|w| *w == finding.better)
        .expect("a Finding's challenger walk must be in WALKS");
    finding
        .sweep
        .iter()
        .map(|sweep| {
            let dist = sweep.dist;
            let mut won_every_round = true;
            let mut margins = Vec::with_capacity(CONFIRM_ROUNDS);
            for round in 0..CONFIRM_ROUNDS {
                let (production_ns, better_ns) = if round % 2 == 0 {
                    let p =
                        time_one::<MAX>(dist, rng, finding.coder, finding.metric, production_idx);
                    let b = time_one::<MAX>(dist, rng, finding.coder, finding.metric, better_idx);
                    (p, b)
                } else {
                    let b = time_one::<MAX>(dist, rng, finding.coder, finding.metric, better_idx);
                    let p =
                        time_one::<MAX>(dist, rng, finding.coder, finding.metric, production_idx);
                    (p, b)
                };
                won_every_round &= better_ns < production_ns;
                margins.push((production_ns - better_ns) / production_ns);
            }
            margins.sort_by(f64::total_cmp);
            let median_margin = margins[margins.len() / 2];
            DistConfirmation {
                dist,
                median_margin,
                reproduced: won_every_round && median_margin >= NOMINATE_FRACTION,
            }
        })
        .collect()
}

/// Dispatch `finding.max` (a runtime value) back to the `const` generic
/// [`confirm_finding_at`] expects, via [`for_each_max`]'s single literal
/// list.
fn confirm_finding(rng: &mut SmallRng, finding: &Finding) -> Vec<DistConfirmation> {
    macro_rules! arm {
        ($max:literal, $rng:expr, $finding:expr) => {
            if $finding.max == $max {
                return confirm_finding_at::<$max>($rng, $finding);
            }
        };
    }
    for_each_max!(arm, rng, finding);
    unreachable!("MAX {} not in the benched set", finding.max)
}

/// Render per-distribution margins as an ascending range, e.g.
/// `"19% (Uniform) .. 41% (Skewed)"` (a single-distribution run collapses to
/// one entry). A `?` after the percentage marks a distribution where the
/// rerun did not reproduce the win.
fn fmt_margin_range(mut entries: Vec<(Dist, f64, bool)>) -> String {
    entries.sort_by(|(_, a, _), (_, b, _)| a.total_cmp(b));
    entries
        .iter()
        .map(|(dist, margin, reproduced)| {
            format!(
                "{:.0}%{} ({dist:?})",
                margin * 100.0,
                if *reproduced { "" } else { "?" }
            )
        })
        .collect::<Vec<_>>()
        .join(" .. ")
}

fn print_findings_summary(
    confirmed: &mut [(Finding, Vec<DistConfirmation>)],
    not_reproduced: &[(Finding, Vec<DistConfirmation>)],
) {
    println!(
        "\n=== Summary: confirmed findings (nominated at >= {:.0}% on some distribution, \
         reproduced across {CONFIRM_ROUNDS} alternated rerun rounds; margins are rerun \
         medians, `?` marks a distribution that did not reproduce the win) ===",
        NOMINATE_FRACTION * 100.0,
    );
    let rerun_range = |confirmations: &[DistConfirmation]| {
        fmt_margin_range(
            confirmations
                .iter()
                .map(|c| (c.dist, c.median_margin, c.reproduced))
                .collect(),
        )
    };
    if confirmed.is_empty() {
        println!("(none confirmed)");
    } else {
        let best = |confirmations: &[DistConfirmation]| {
            confirmations
                .iter()
                .map(|c| c.median_margin)
                .fold(f64::NEG_INFINITY, f64::max)
        };
        confirmed.sort_by(|(a, a_conf), (b, b_conf)| {
            a.coder
                .cmp(b.coder)
                .then(best(b_conf).total_cmp(&best(a_conf)))
        });
        for (f, confirmations) in confirmed.iter() {
            println!(
                "MAX={:<6} {:<5} {:<6}: production {:?} vs {:?} — faster by {}",
                f.max,
                f.coder,
                f.metric,
                f.production,
                f.better,
                rerun_range(confirmations),
            );
        }
    }
    if !not_reproduced.is_empty() {
        println!(
            "\n--- nominated but not reproduced under rerun (likely noise): {} ---",
            not_reproduced.len()
        );
        for (f, confirmations) in not_reproduced {
            let sweep_range =
                fmt_margin_range(f.sweep.iter().map(|s| (s.dist, s.margin(), true)).collect());
            println!(
                "MAX={:<6} {:<5} {:<6}: production {:?} vs {:?} — sweep said {}, rerun said {}",
                f.max,
                f.coder,
                f.metric,
                f.production,
                f.better,
                sweep_range,
                rerun_range(confirmations),
            );
        }
    }
}

fn main() {
    println!(
        "AtMost<MAX> walk shootout: ns/value, batch size scales with MAX (min 256). \
         `*` marks the walk Walk::production currently picks for that coder; \
         `-` marks an encode row shared with another walk (not timed separately)."
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

    let mut confirmed: Vec<(Finding, Vec<DistConfirmation>)> = Vec::new();
    let mut not_reproduced: Vec<(Finding, Vec<DistConfirmation>)> = Vec::new();
    for finding in &findings {
        let mut rng = SmallRng::seed_from_u64(0xC0FFEE ^ finding.max as u64 ^ 0x5EED);
        let confirmations = confirm_finding(&mut rng, finding);
        if confirmations.iter().any(|c| c.reproduced) {
            confirmed.push((finding.clone(), confirmations));
        } else {
            not_reproduced.push((finding.clone(), confirmations));
        }
    }
    print_findings_summary(&mut confirmed, &not_reproduced);
}
