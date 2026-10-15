//! What the benches share: the `scaling` configuration, a warning when the
//! machine is not quiesced, and a pinned allocator.
//!
//! This is a module, not a crate, because every `benches/*.rs` is its own
//! crate root; and it is a directory rather than `benches/support.rs` so cargo
//! does not take it for another benchmark to run.
//!
//! **It registers nothing.** A `#[scaling::…]` attribute written here would
//! exist in every binary that includes this module — and a metrics function
//! marked `allocation` would make every binary without the counting allocator
//! refuse to run.
//!
//! There is no argument handling. A bench runs everything registered in it;
//! `scaling`'s own defaults are the precision policy (a 1% difference is
//! significant, 10 s per candidate), and what is to be measured is chosen by
//! which bench is run. To look at less, narrow the [`Config`] in that bench's
//! `main` with `filter_groups`, `filter_candidates` or `filter_inputs`.

#![allow(dead_code)]

use scaling::Config;

/// Make the allocator behave the same wherever a benchmark runs.
///
/// glibc decides, as the process goes, whether a big allocation comes from fresh
/// pages (`mmap`, and a page fault for each page touched) or from recycled heap,
/// by a threshold it moves in response to what has been freed. So the cost of
/// allocating a few hundred kilobytes — an 800 KB decoded `Vec<f64>` — depends on
/// what the process did first: the *same* `Ans` decode of the *same* bytes read
/// +160% to +320% against `Range` depending only on heap layout, and a cell run
/// alone differed from the same cell run inside the whole suite. Pinning the
/// thresholds makes it the same every time: allocations up to 32 MB (glibc's
/// maximum for the `mmap` threshold) come from the heap, which is never trimmed
/// and starts with room to spare.
///
/// With this, the cell above gave +137% six times running — alone, in the suite,
/// and repeated. It is a choice of what to measure: the coder, not the page
/// faults a particular allocator history happened to charge it.
#[cfg(all(target_os = "linux", target_env = "gnu"))]
fn steady_allocator() {
    // SAFETY: `mallopt` only sets allocator tunables, and this runs before any
    // benchmark has allocated anything large.
    unsafe {
        libc::mallopt(libc::M_MMAP_THRESHOLD, 32 << 20);
        libc::mallopt(libc::M_TRIM_THRESHOLD, i32::MAX);
        libc::mallopt(libc::M_TOP_PAD, 64 << 20);
    }
}

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
fn steady_allocator() {}

/// Say on stderr when this process is not on a reserved CPU. `scaling`'s `±`
/// covers the noise it can see while sampling, not the machine around it: the
/// frequency state, a neighbour sharing the core.
pub fn warn_unless_quiet() {
    use scaling::quiet::{status, Status};
    match status() {
        Status::Pinned { .. } => {}
        other => eprintln!(
            "warning: {other}\n         run through `quiet-bench run` after `sudo quiet-bench \
             reserve`; see OPTIMIZING.md"
        ),
    }
}

/// The [`Config`] every bench starts from: `scaling`'s defaults, after fixing the
/// allocator and saying so if the machine is not quiesced.
pub fn config() -> Config {
    steady_allocator();
    warn_unless_quiet();
    Config::default()
}

/// Measure everything registered in this binary and print it: a bench's whole
/// `main`.
pub fn run() -> Result<(), scaling::RegistrationError> {
    config().run_and_print()
}
