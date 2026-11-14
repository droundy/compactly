//! The corpora of `comparison`, derived against the tip of `main`.
//!
//! There is no code here. `comparison`'s source is included by path, as it is in
//! `comparison-published`, so what `comparison` does must compile against `main`
//! too: a corpus type that wants something a branch has just added stays out of
//! `comparison`'s `lib.rs` until that branch lands.

#[path = "../../comparison/src/lib.rs"]
mod corpora;

pub use corpora::*;

/// `main`'s compactly itself, so a bench names its `Range` and `Ans` without a
/// second dependency that would have to be kept in step with this one.
pub use compactly;
