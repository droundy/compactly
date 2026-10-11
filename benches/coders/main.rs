//! The A/B workbench: `Ans` against `Range`, and the questions about either
//! coder's internals that a workload table cannot ask.
//!
//! Everything in the first list is available with no feature flags; the rest
//! appear when the feature that exposes what they measure is on, and are
//! simply not registered otherwise — so a plain `cargo bench` measures the route
//! table instead of cargo silently skipping the target.
//!
//! | file | question | needs |
//! |---|---|---|
//! | `routes.rs` | `Ans` vs `Range` on every route either supports, across workloads | — (`stream` adds the stream routes) |
//! | `phases.rs` | where does ANS time go: model work or entropy coding? | `benchmarking` |
//! | `micro_batch.rs` | `decode_bits` against `decode_bit`, nothing else in the loop | `benchmarking` |
//! | `chunk_tracking.rs` | what does per-op chunk-boundary tracking cost? | `benchmarking` |
//! | `overlap.rs` | does decoding overlap the wait for the next chunk? | `stream` |
//! | `arc_str.rs` | the current `Arc<str>` encoding against the ones it replaced | `benchmarking` |
//!
//! Arguments choose what runs — see `support/mod.rs`. A bare word selects
//! groups, so:
//!
//! ```text
//! cargo bench --bench coders --features benchmarking,stream
//! cargo bench --bench coders -- slice                    # the slice route, every workload
//! cargo bench --bench coders -- slice input=strings      # one cell of that grid
//! cargo bench --bench coders -- encode                   # `encode` and `encode-to`
//! cargo bench --bench coders -- phases
//! ```
//!
//! Two corpora are local files, not part of the repository: `ipv6.txt` and
//! `ipv4.txt`, read from the current directory. A workload whose file is absent
//! is left out, with a note on stderr, rather than failing the whole run.

#[path = "../support/mod.rs"]
mod support;

mod routes;

#[cfg(feature = "benchmarking")]
mod arc_str;
#[cfg(feature = "benchmarking")]
mod chunk_tracking;
#[cfg(feature = "benchmarking")]
mod micro_batch;
#[cfg(feature = "stream")]
mod overlap;
#[cfg(feature = "benchmarking")]
mod phases;

/// `(file, input)`: the workload called `input` reads `file`.
const LOCAL_CORPORA: [(&str, &str); 2] = [("ipv6.txt", "ipv6"), ("ipv4.txt", "ipv4")];

fn main() -> Result<(), scaling::RegistrationError> {
    let mut config = support::config();
    for (file, input) in LOCAL_CORPORA {
        if !std::path::Path::new(file).exists() {
            eprintln!(
                "note: leaving out the `{input}` workload: {file} is not in the current directory"
            );
            config = config.filter_inputs(move |name| name != input);
        }
    }
    config.run_and_print()
}
