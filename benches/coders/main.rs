//! `Ans` against `Range`.
//!
//! - `routes.rs` — every route either coder supports, across workloads: the table
//!   behind "`Ans` is faster at decoding", and the places it is not.
//! - `overlap.rs` — does `decode_stream` actually overlap the wait for the next
//!   chunk? (needs the `stream` feature)
//!
//! Each route is a group and each workload an input to all of them, so a plain
//! `cargo bench --bench coders` prints the whole grid, and `--features stream`
//! adds the stream routes and `overlap`. A run takes about six minutes. To look at
//! less, add a `filter_groups` or `filter_inputs` to the [`Config`] in `main`.
//!
//! `ipv6.txt` is a local corpus, not part of the repository, read from the
//! current directory. Its workload is left out, with a note on stderr, when the
//! file is absent.
//!
//! Questions that were asked of the coders once and answered — what chunk
//! tracking costs, where ANS time goes between model and entropy coding, whether
//! `decode_bits` beats `decode_bit`, the `Arc<str>` encodings that were
//! superseded — each had a bench of its own. They are in git history (last at
//! `0c7bf74`) and their answers are in OPTIMIZING.md.

#[path = "../support/mod.rs"]
mod support;

mod routes;

#[cfg(feature = "stream")]
mod overlap;

/// `(file, input)`: the workload called `input` reads `file`.
const LOCAL_CORPORA: [(&str, &str); 1] = [("ipv6.txt", "ipv6")];

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
