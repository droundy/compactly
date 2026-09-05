//! Reporting helpers shared by the benchmark binaries in `src/bin/`.
//!
//! Every timing in this crate comes from [`scaling`], which samples a closure
//! until the standard error of its mean is small enough and marks the line
//! `(limit)` when it ran out of time first, or `(untrusted)` when too few
//! samples were taken for that error bar to mean anything. So a benchmark
//! here has no iteration count to tune and no external `perf` wrapper to
//! remember: it says how long one operation took, and how well it knows that.
//!
//! What the `±` does *not* cover is anything that differs between whole
//! processes — CPU frequency state, code layout, ASLR — so comparing two
//! *builds* still means alternating them, and comparing anything at all means
//! reserving a CPU first with `quiet-bench reserve`. [`warn_unless_quiet`],
//! which every function here calls, says so on stderr when that has not been
//! done.
//!
//! Available only under the `benchmarking` feature, and not covered by
//! semver. See "How to benchmark on this machine" in OPTIMIZING.md.

use scaling::Config;
pub use scaling::Stats;
use std::time::Duration;

/// The standard error every benchmark here aims for, as a fraction of the
/// measurement.
///
/// 0.1%, set by the smallest difference this project acts on. Changes under
/// 1% are not trusted here — separate builds of the same code land about that
/// far apart on binary layout alone — so 1% is the threshold that has to come
/// out clearly significant, and an error bar a tenth of it makes a 1%
/// difference roughly seven sigma once both sides are counted. Asking for
/// much less would leave 1% inside the noise; asking for much more would only
/// buy precision about a single process that binary layout then swamps.
///
/// On a quiesced CPU the millisecond-scale workloads here reach it in a
/// second or two. Override with `BENCH_REL_ERROR` (e.g.
/// `BENCH_REL_ERROR=0.01` for a quick pass over many cells).
pub const TARGET_REL_ERROR: f64 = 0.001;

/// How long one measurement may spend trying to reach [`TARGET_REL_ERROR`]
/// before giving up and reporting `(limit)`.
///
/// A backstop, not a target — nothing here sizes its work against it.
/// Override with `BENCH_MAX_SECONDS`.
pub const MAX_SECONDS: f64 = 10.0;

/// The configuration every measurement in this crate runs under.
pub fn config() -> Config {
    let env = |name: &str, default: f64| -> f64 {
        std::env::var(name)
            .ok()
            .and_then(|v| v.parse().ok())
            .filter(|v: &f64| *v > 0.0)
            .unwrap_or(default)
    };
    Config {
        target_rel_error: env("BENCH_REL_ERROR", TARGET_REL_ERROR),
        max_time: Duration::from_secs_f64(env("BENCH_MAX_SECONDS", MAX_SECONDS)),
        ..Config::default()
    }
}

/// Complain on stderr unless this process is pinned to a reserved CPU.
///
/// Printed once per process, however many measurements it takes. Not fatal:
/// a rough number is still worth having while iterating, and a script that
/// wants to insist can gate on `quiet-bench status` instead.
pub fn warn_unless_quiet() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static WARNED: AtomicBool = AtomicBool::new(false);
    if WARNED.swap(true, Ordering::Relaxed) {
        return;
    }
    let status = scaling::quiet::status();
    if !matches!(status, scaling::quiet::Status::Pinned { .. }) {
        eprintln!("warning: {status}");
        eprintln!("         run under `quiet-bench run …`; see OPTIMIZING.md");
    }
}

/// Time `f` and print one labelled line, returning the statistics too.
///
/// The closure's return value is passed through `black_box` by `scaling`, so
/// return something derived from the work — a length, a checksum — to keep
/// the optimiser from deleting it.
pub fn report<O>(label: &str, f: impl FnMut() -> O) -> Stats {
    warn_unless_quiet();
    let stats = config().bench(f);
    print(label, &stats);
    stats
}

/// [`report`], but with a fresh environment built (untimed) for each
/// iteration — for benchmarks that consume or mutate what they are given.
pub fn report_env<I, O>(
    label: &str,
    gen_env: impl FnMut() -> I,
    f: impl FnMut(&mut I) -> O,
) -> Stats {
    warn_unless_quiet();
    let stats = config().bench_gen_env(gen_env, f);
    print(label, &stats);
    stats
}

/// One phase of a measurement isolated as the difference between two others,
/// e.g. "encode plus flush" minus "encode".
///
/// The errors add in quadrature, which is what makes this worth a helper: a
/// difference between two nearly equal measurements is known far *less*
/// precisely than either of them, and printing it without saying so is how a
/// phase split ends up reporting a number that is mostly noise. The flags are
/// merged too, so an untrustworthy input taints the difference.
pub fn difference(whole: &Stats, part: &Stats) -> Stats {
    Stats {
        ns_per_iter: whole.ns_per_iter - part.ns_per_iter,
        std_error: (whole.std_error.powi(2) + part.std_error.powi(2)).sqrt(),
        iterations: whole.iterations + part.iterations,
        samples: whole.samples + part.samples,
        hit_limit: whole.hit_limit || part.hit_limit,
        untrustworthy: whole.untrustworthy || part.untrustworthy,
    }
}

/// Rescale a measurement from "per iteration" to "per element", so
/// benchmarks whose value size is a knob stay comparable across settings.
///
/// The error bar is divided by the same factor, so the line still says how
/// well the per-element figure is known.
pub fn per_unit(stats: &Stats, units: usize) -> Stats {
    let units = units.max(1) as f64;
    Stats {
        ns_per_iter: stats.ns_per_iter / units,
        std_error: stats.std_error / units,
        ..stats.clone()
    }
}

/// Print a [`Stats`] — one computed by [`difference`], typically — on the
/// same line shape [`report`] uses.
pub fn print(label: &str, stats: &Stats) {
    println!("{label:<32} {stats}");
}
