//! This checkout's compactly against the last published one, on the same data.
//!
//! Every workload is two comparisons:
//!
//! - `<workload> encode` — each candidate turns the corpus into bytes.
//! - `<workload> decode` — each candidate turns *its own* bytes back into values.
//!
//! `published range` is the baseline of both: the released compactly, encoding
//! the released compactly's copy of the corpus. `head range` and `head ans` are
//! this checkout's, `published ans` the released `Ans`. `scaling` pairs every
//! call, so "how much faster than the release" has a real error bar.
//!
//! Two copies of one corpus type, each derived against its own compactly, come
//! from the `comparison` crate and its twin `comparison-published`; see the
//! comment in the latter's `Cargo.toml`. Both read the same source files, so the
//! data is the same, and what differs is the library that encodes it.
//!
//! **`head range again` is a control, not a result.** It is the same function as
//! `head range`, a second time; compiled twice it lands at different addresses,
//! and the gap between the two is how far *code placement alone* moves a number
//! here. A published-versus-head gap no larger than that is not evidence of
//! anything. (`.cargo/config.toml` aligns functions so it is small, but about
//! 8% of the effect survives.)
//!
//! ```text
//! cargo bench --bench versions
//! ```

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
            // `published_compactly` is the only thing this brings in, and only the
            // published side uses it.
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

/// Both sides' corpora for one workload.
struct Corpora<H, P> {
    head: Vec<H>,
    published: Vec<P>,
}

/// Every side's bytes for one workload, each side's from its own encoders.
struct Bytes {
    head_range: Vec<Vec<u8>>,
    head_ans: Vec<Vec<u8>>,
    published_range: Vec<Vec<u8>>,
    published_ans: Vec<Vec<u8>>,
}

/// A published-versus-head workload, as an encode group and a decode group.
///
/// The corpora and their bytes are `build_once` inputs: nothing in a candidate
/// consumes or changes them. The bytes are checked to round-trip on both sides
/// before anything is timed, since a codec that does not decode what it wrote has
/// nothing to measure.
macro_rules! versions {
    ($module:ident, $enc:literal, $dec:literal, $head:ty, $published:ty, $make:ident) => {
        mod $module {
            use super::*;

            type Pair = Corpora<$head, $published>;

            fn corpora() -> Pair {
                Corpora {
                    head: head::$make(),
                    published: published::$make(),
                }
            }

            #[scaling::input(group = $enc, name = "corpus", build_once)]
            fn corpus() -> Pair {
                corpora()
            }

            #[scaling::input(group = $dec, name = "bytes", build_once)]
            fn encoded() -> Bytes {
                let Corpora { head, published } = corpora();
                let bytes = Bytes {
                    head_range: head::encode_range(&head),
                    head_ans: head::encode_ans(&head),
                    published_range: published::encode_range(&published),
                    published_ans: published::encode_ans(&published),
                };
                assert!(
                    head::decode_range::<$head>(&bytes.head_range) == head
                        && head::decode_ans::<$head>(&bytes.head_ans) == head
                        && published::decode_range::<$published>(&bytes.published_range)
                            == published
                        && published::decode_ans::<$published>(&bytes.published_ans) == published,
                    "{} did not round-trip",
                    $enc
                );
                bytes
            }

            #[scaling::bench(group = $enc, name = "published range", baseline)]
            fn encode_published_range(c: &Pair) -> Vec<Vec<u8>> {
                published::encode_range(&c.published)
            }
            #[scaling::bench(group = $enc, name = "head range")]
            fn encode_head_range(c: &Pair) -> Vec<Vec<u8>> {
                head::encode_range(&c.head)
            }
            #[scaling::bench(group = $enc, name = "head range again")]
            fn encode_head_range_again(c: &Pair) -> Vec<Vec<u8>> {
                head::encode_range(&c.head)
            }
            #[scaling::bench(group = $enc, name = "published ans")]
            fn encode_published_ans(c: &Pair) -> Vec<Vec<u8>> {
                published::encode_ans(&c.published)
            }
            #[scaling::bench(group = $enc, name = "head ans")]
            fn encode_head_ans(c: &Pair) -> Vec<Vec<u8>> {
                head::encode_ans(&c.head)
            }

            #[scaling::bench(group = $dec, name = "published range", baseline)]
            fn decode_published_range(b: &Bytes) -> Vec<$published> {
                published::decode_range(&b.published_range)
            }
            #[scaling::bench(group = $dec, name = "head range")]
            fn decode_head_range(b: &Bytes) -> Vec<$head> {
                head::decode_range(&b.head_range)
            }
            #[scaling::bench(group = $dec, name = "head range again")]
            fn decode_head_range_again(b: &Bytes) -> Vec<$head> {
                head::decode_range(&b.head_range)
            }
            #[scaling::bench(group = $dec, name = "published ans")]
            fn decode_published_ans(b: &Bytes) -> Vec<$published> {
                published::decode_ans(&b.published_ans)
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
    single_cards
);
versions!(
    single_meteorites,
    "single meteorites encode",
    "single meteorites decode",
    comparison::meteorites::MeteoriteData,
    comparison_published::meteorites::MeteoriteData,
    single_meteorites
);
versions!(
    meteorites,
    "meteorites encode",
    "meteorites decode",
    Vec<comparison::meteorites::MeteoriteData>,
    Vec<comparison_published::meteorites::MeteoriteData>,
    meteorites
);
versions!(
    suicide_map,
    "suicide encode",
    "suicide decode",
    std::collections::HashMap<comparison::suicide::SuicideFactors, u16>,
    std::collections::HashMap<comparison_published::suicide::SuicideFactors, u16>,
    suicide_map
);
versions!(
    books,
    "books encode",
    "books decode",
    comparison::books::Book,
    comparison_published::books::Book,
    books
);

fn main() -> Result<(), scaling::RegistrationError> {
    support::run()
}
