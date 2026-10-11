//! What the `scaling` 0.9 benches share: how arguments become filters, the
//! precision policy, and a warning when the machine is not quiesced.
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
//! ## Arguments
//!
//! A bench is `harness = false`, so it is a plain `main` that sees its own
//! arguments (`cargo bench --bench bench -- <args>`). They choose which
//! registrations run — the one thing `scaling` still decides at run time:
//!
//! ```text
//! WORD            keep groups whose name contains WORD, ignoring case
//! group=WORD      the same, spelled out
//! cand=WORD       keep candidates whose name contains WORD
//! input=WORD      keep inputs whose name contains WORD
//! --quick         stop at a 5% sensitivity instead of 1%
//! --accuracy=F    stop at a sensitivity of F (0.01 is 1%)
//! --time=SECONDS  give each candidate at most this long (default 10)
//! ```
//!
//! Every filter narrows the others, and so does each repeated word. A bare
//! word applies to *group* names only because the three kinds are ANDed and
//! never see one another's names, so a word matched against all three would
//! run nothing unless group, candidate and input each contained it. A
//! benchmark that stands alone has no group of its own and is asked with its
//! module-qualified name, so a bare word still finds it — but `input=` drops
//! it, since a standalone benchmark's input is named `""`.

#![allow(dead_code)]

use scaling::Config;
use std::time::Duration;

/// Sensitivity to stop at: the smallest difference worth calling a change, as
/// a fraction of the baseline. This project does not trust changes under 1%
/// (separate builds differ by about that from code placement alone), so
/// stopping once a 1% change would be significant is exactly the policy; a
/// tighter target costs far more time than it buys.
pub const ACCURACY: f64 = 0.01;
/// What `--quick` asks for.
pub const QUICK_ACCURACY: f64 = 0.05;
/// How long one candidate may take before giving up and saying `(limit)`.
pub const MAX_SECONDS: f64 = 10.0;

/// The command line, as filters and limits.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub groups: Vec<String>,
    pub candidates: Vec<String>,
    pub inputs: Vec<String>,
    pub accuracy: f64,
    pub seconds: f64,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            groups: Vec::new(),
            candidates: Vec::new(),
            inputs: Vec::new(),
            accuracy: ACCURACY,
            seconds: MAX_SECONDS,
        }
    }
}

const USAGE: &str = "\
usage: cargo bench --bench NAME -- [WORD | group=WORD | cand=WORD | input=WORD]...
                                   [--quick] [--accuracy=F] [--time=SECONDS]

A WORD keeps the groups whose name contains it (case does not matter); the
prefixed forms select candidates or inputs instead. Each one narrows the rest.";

/// Read the arguments after the program name. `--bench` is what cargo adds to a
/// `harness = false` target and is ignored.
pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Options, String> {
    let mut options = Options::default();
    for arg in args {
        let number = |text: &str, what: &str| {
            text.parse::<f64>()
                .ok()
                .filter(|x| x.is_finite() && *x > 0.0)
                .ok_or_else(|| format!("{what} wants a positive number, not `{text}`"))
        };
        if arg == "--bench" {
        } else if arg == "--quick" {
            options.accuracy = QUICK_ACCURACY;
        } else if let Some(value) = arg.strip_prefix("--accuracy=") {
            options.accuracy = number(value, "--accuracy")?;
        } else if let Some(value) = arg.strip_prefix("--time=") {
            options.seconds = number(value, "--time")?;
        } else if arg.starts_with('-') {
            return Err(format!("unknown option `{arg}`"));
        } else if let Some(word) = arg.strip_prefix("group=") {
            options.groups.push(word.to_lowercase());
        } else if let Some(word) = arg.strip_prefix("cand=") {
            options.candidates.push(word.to_lowercase());
        } else if let Some(word) = arg.strip_prefix("input=") {
            options.inputs.push(word.to_lowercase());
        } else if arg.contains('=') {
            return Err(format!(
                "`{arg}` is not group=, cand= or input=; a bare word has no `=`"
            ));
        } else {
            options.groups.push(arg.to_lowercase());
        }
    }
    Ok(options)
}

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

/// The [`Config`] this run asks for: the policy above, narrowed by whatever
/// the command line named. Exits, with the usage, when the arguments make no
/// sense, and says on stderr — once — if the machine is not quiesced.
pub fn config() -> Config {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        println!("{USAGE}");
        std::process::exit(0);
    }
    let options = match parse(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("error: {message}\n\n{USAGE}");
            std::process::exit(2);
        }
    };
    steady_allocator();
    warn_unless_quiet();
    let mut config = Config::default()
        .with_relative_error(options.accuracy)
        .with_max_time(Duration::from_secs_f64(options.seconds));
    for word in options.groups {
        config = config.filter_groups(move |name| name.to_lowercase().contains(&word));
    }
    for word in options.candidates {
        config = config.filter_candidates(move |name| name.to_lowercase().contains(&word));
    }
    for word in options.inputs {
        config = config.filter_inputs(move |name| name.to_lowercase().contains(&word));
    }
    config
}

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
