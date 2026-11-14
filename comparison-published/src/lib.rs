//! The corpora of `comparison`, derived against the published compactly.
//!
//! There is no code here. `comparison`'s source is included by path, so every
//! type exists twice, once derived against this checkout's compactly and once
//! against the published one. What `comparison` does must therefore compile
//! against the published release too: a corpus type that wants something newer
//! stays out of `comparison`'s `lib.rs`, or this crate stops building. Its
//! `include_str!` data is found relative to the files themselves, so those
//! read the same bytes as `comparison`'s do.

#[path = "../../comparison/src/lib.rs"]
mod corpora;

pub use corpora::*;

/// The published compactly itself, so a bench names its `Range` and `Ans` without
/// a second dependency that would have to be kept in step with this one.
pub use compactly;
