//! This checkout's compactly against the last published one and against the tip
//! of `main`, on the same data.
//!
//! Every workload is two comparisons:
//!
//! - `<workload> encode` — each candidate turns the corpus into bytes.
//! - `<workload> decode` — each candidate turns *its own* bytes back into values.
//!
//! `main range` is the baseline of both: the tip of `main`, encoding `main`'s copy
//! of the corpus. `head range` and `head ans` are this checkout's, `main ans` is
//! `main`'s `Ans`, and `published range` and `published ans` are the last release's.
//! `scaling` pairs every call with the baseline, so "what did this branch change"
//! and "how far has `main` come from the release" each have a real error bar. How a
//! branch stands against the release is the difference of those two rows, and on
//! `main` itself, where `head` is the baseline, the `published` rows say it directly.
//!
//! Copies of one corpus type, each derived against its own compactly, come from the
//! `comparison` crate and its twins `comparison-published` and `comparison-main`;
//! see the comment in `comparison-published`'s `Cargo.toml`. All three read the same
//! source files, so the data is the same, and what differs is the library that
//! encodes it.
//!
//! **`head range again` is a control, not a result.** It is the same function as
//! `head range`, a second time; compiled twice it lands at different addresses,
//! and the gap between the two is how far *code placement alone* moves a number
//! here. A gap from `head` no larger than that is not evidence of anything.
//! (`.cargo/config.toml` aligns functions so it is small, but about 8% of the
//! effect survives.)
//!
//! **Check which `main` you are measuring.** `Cargo.lock` is not committed, so CI
//! always builds the current `main`, but a local lockfile stays on the commit it
//! first resolved, however far `main` has moved. Refresh it first.
//!
//! ```text
//! cargo update -p 'https://github.com/droundy/compactly#compactly'
//! cargo bench --bench versions
//! ```

use comparison_main::compactly as main_compactly;
use comparison_published::compactly as published_compactly;

mod support;

/// Counts allocations, for the decode groups' memory columns.
#[global_allocator]
static ALLOC: scaling::Allocator = scaling::Allocator::new();

/// One side of the comparison: a compactly, and the corpora derived against it.
///
/// A macro, not a generic, because each side's `Encode` is a different trait and
/// each side's corpus types are different types.
macro_rules! side {
    ($side:ident, $corpora:ident, $compactly:ident) => {
        mod $side {
            // The renamed compactlys are the only thing this brings in, and only
            // their own sides use them.
            #[allow(unused_imports)]
            use super::*;
            use $compactly::v2::{Ans, Encode, Range};

            pub fn encode_range<T: Encode>(values: &[T]) -> Vec<Vec<u8>> {
                values.iter().map(|v| Range::encode(v)).collect()
            }
            pub fn encode_ans<T: Encode>(values: &[T]) -> Vec<Vec<u8>> {
                values.iter().map(|v| Ans::encode(v)).collect()
            }
            // `unwrap` because the release returns an `Option` and this checkout a
            // `Result`.
            pub fn decode_range<T: Encode>(bytes: &[Vec<u8>]) -> Vec<T> {
                bytes.iter().map(|b| Range::decode(b).unwrap()).collect()
            }
            pub fn decode_ans<T: Encode>(bytes: &[Vec<u8>]) -> Vec<T> {
                bytes.iter().map(|b| Ans::decode(b).unwrap()).collect()
            }

            pub fn single_cards() -> Vec<$corpora::Card> {
                $corpora::tenth_edition().data.cards
            }
            pub fn single_meteorites() -> Vec<$corpora::meteorites::MeteoriteData> {
                $corpora::meteorites::meteorites()
            }
            pub fn meteorites() -> Vec<Vec<$corpora::meteorites::MeteoriteData>> {
                vec![$corpora::meteorites::meteorites()]
            }
            pub fn suicide_map(
            ) -> Vec<std::collections::HashMap<$corpora::suicide::SuicideFactors, u16>> {
                vec![$corpora::suicides_per_million()]
            }
            pub fn books() -> Vec<$corpora::books::Book> {
                $corpora::books::books()
            }
        }
    };
}

side!(head, comparison, compactly);
side!(published, comparison_published, published_compactly);
side!(on_main, comparison_main, main_compactly);

/// Every side's corpora for one workload.
struct Corpora<H, P, M> {
    head: Vec<H>,
    published: Vec<P>,
    main: Vec<M>,
}

/// Every side's bytes for one workload, each side's from its own encoders.
struct Bytes {
    head_range: Vec<Vec<u8>>,
    head_ans: Vec<Vec<u8>>,
    published_range: Vec<Vec<u8>>,
    published_ans: Vec<Vec<u8>>,
    main_range: Vec<Vec<u8>>,
    main_ans: Vec<Vec<u8>>,
}

/// One workload, as an encode group and a decode group.
///
/// The corpora and their bytes are `build_once` inputs: they are real corpora, of
/// which there is only one, and nothing in a candidate consumes or changes them.
/// The bytes are checked to round-trip on every side before anything is timed,
/// since a codec that does not decode what it wrote has nothing to measure.
macro_rules! versions {
    (
        $module:ident, $enc:literal, $dec:literal,
        $head:ty, $published:ty, $main:ty, $make:ident
    ) => {
        mod $module {
            use super::*;

            type Sides = Corpora<$head, $published, $main>;

            fn corpora() -> Sides {
                Corpora {
                    head: head::$make(),
                    published: published::$make(),
                    main: on_main::$make(),
                }
            }

            #[scaling::input(group = $enc, name = "corpus", build_once)]
            fn corpus() -> Sides {
                corpora()
            }

            #[scaling::input(group = $dec, name = "bytes", build_once)]
            fn encoded() -> Bytes {
                let Corpora {
                    head,
                    published,
                    main,
                } = corpora();
                let bytes = Bytes {
                    head_range: head::encode_range(&head),
                    head_ans: head::encode_ans(&head),
                    published_range: published::encode_range(&published),
                    published_ans: published::encode_ans(&published),
                    main_range: on_main::encode_range(&main),
                    main_ans: on_main::encode_ans(&main),
                };
                assert!(
                    head::decode_range::<$head>(&bytes.head_range) == head
                        && head::decode_ans::<$head>(&bytes.head_ans) == head
                        && published::decode_range::<$published>(&bytes.published_range)
                            == published
                        && published::decode_ans::<$published>(&bytes.published_ans) == published
                        && on_main::decode_range::<$main>(&bytes.main_range) == main
                        && on_main::decode_ans::<$main>(&bytes.main_ans) == main,
                    "{} did not round-trip",
                    $enc
                );
                bytes
            }

            #[scaling::bench(group = $enc, name = "published range")]
            fn encode_published_range(c: &Sides) -> Vec<Vec<u8>> {
                published::encode_range(&c.published)
            }
            #[scaling::bench(group = $enc, name = "published ans")]
            fn encode_published_ans(c: &Sides) -> Vec<Vec<u8>> {
                published::encode_ans(&c.published)
            }
            #[scaling::bench(group = $enc, name = "main range", baseline)]
            fn encode_main_range(c: &Sides) -> Vec<Vec<u8>> {
                on_main::encode_range(&c.main)
            }
            #[scaling::bench(group = $enc, name = "main ans")]
            fn encode_main_ans(c: &Sides) -> Vec<Vec<u8>> {
                on_main::encode_ans(&c.main)
            }
            #[scaling::bench(group = $enc, name = "head range")]
            fn encode_head_range(c: &Sides) -> Vec<Vec<u8>> {
                head::encode_range(&c.head)
            }
            #[scaling::bench(group = $enc, name = "head range again")]
            fn encode_head_range_again(c: &Sides) -> Vec<Vec<u8>> {
                head::encode_range(&c.head)
            }
            #[scaling::bench(group = $enc, name = "head ans")]
            fn encode_head_ans(c: &Sides) -> Vec<Vec<u8>> {
                head::encode_ans(&c.head)
            }

            #[scaling::bench(group = $dec, name = "published range")]
            fn decode_published_range(b: &Bytes) -> Vec<$published> {
                published::decode_range(&b.published_range)
            }
            #[scaling::bench(group = $dec, name = "published ans")]
            fn decode_published_ans(b: &Bytes) -> Vec<$published> {
                published::decode_ans(&b.published_ans)
            }
            #[scaling::bench(group = $dec, name = "main range", baseline)]
            fn decode_main_range(b: &Bytes) -> Vec<$main> {
                on_main::decode_range(&b.main_range)
            }
            #[scaling::bench(group = $dec, name = "main ans")]
            fn decode_main_ans(b: &Bytes) -> Vec<$main> {
                on_main::decode_ans(&b.main_ans)
            }
            #[scaling::bench(group = $dec, name = "head range")]
            fn decode_head_range(b: &Bytes) -> Vec<$head> {
                head::decode_range(&b.head_range)
            }
            #[scaling::bench(group = $dec, name = "head range again")]
            fn decode_head_range_again(b: &Bytes) -> Vec<$head> {
                head::decode_range(&b.head_range)
            }
            #[scaling::bench(group = $dec, name = "head ans")]
            fn decode_head_ans(b: &Bytes) -> Vec<$head> {
                head::decode_ans(&b.head_ans)
            }

            /// Total bytes written, for the whole corpus.
            #[scaling::metrics(group = $enc)]
            fn size(out: Vec<Vec<u8>>) -> scaling::Metrics {
                scaling::Metrics::new().bytes("size", out.iter().map(Vec::len).sum::<usize>())
            }
        }
    };
}

versions!(
    single_cards,
    "single cards encode",
    "single cards decode",
    comparison::Card,
    comparison_published::Card,
    comparison_main::Card,
    single_cards
);
versions!(
    single_meteorites,
    "single meteorites encode",
    "single meteorites decode",
    comparison::meteorites::MeteoriteData,
    comparison_published::meteorites::MeteoriteData,
    comparison_main::meteorites::MeteoriteData,
    single_meteorites
);
versions!(
    meteorites,
    "meteorites encode",
    "meteorites decode",
    Vec<comparison::meteorites::MeteoriteData>,
    Vec<comparison_published::meteorites::MeteoriteData>,
    Vec<comparison_main::meteorites::MeteoriteData>,
    meteorites
);
versions!(
    suicide_map,
    "suicide encode",
    "suicide decode",
    std::collections::HashMap<comparison::suicide::SuicideFactors, u16>,
    std::collections::HashMap<comparison_published::suicide::SuicideFactors, u16>,
    std::collections::HashMap<comparison_main::suicide::SuicideFactors, u16>,
    suicide_map
);
versions!(
    books,
    "books encode",
    "books decode",
    comparison::books::Book,
    comparison_published::books::Book,
    comparison_main::books::Book,
    books
);

fn main() -> Result<(), scaling::RegistrationError> {
    support::run()
}
