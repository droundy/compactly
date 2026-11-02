//! compactly (`Range` and `Ans`) against bincode and zstd, on synthetic
//! values and on the real corpora in the `comparison` crate.
//!
//! Every workload is two comparisons, one for each direction:
//!
//! - `<workload> encode` — each codec turns the whole corpus into bytes. The
//!   `size` column is what they wrote, in total.
//! - `<workload> decode` — each codec turns *its own* bytes back into values.
//!   The `alloc` columns are what that cost in memory.
//!
//! `main range` is the baseline of both: the tip of `main`'s `Range`, so `range`
//! and `ans` say what this checkout changed and `main ans` where `Ans` stood before
//! it (see `benches/versions.rs` for which `main` that is). Those are the candidates
//! anyone asked about, so they are the only ones counted when `scaling` decides how
//! many comparisons it is correcting for; bincode and zstd are `uninteresting`,
//! which means they are shown as a size against the baseline without a verdict.
//!
//! A time is for the *whole corpus*, not for one value: the synthetic corpora
//! are a thousand-odd values each, and the real ones are a single large value
//! (a whole `FullSet`) or all of a type's records (every `Card`).
//!
//! ```text
//! cargo bench --bench bench
//! ```
//!
//! **Two encode groups are slow on purpose**: `mtg tenth edition encode` and
//! `meteorites by small name encode` take around a second per call, nearly all
//! of it `Compressible`'s Lz77 match search (a known, deprioritized cost; see
//! OPTIMIZING.md). They are most of this bench's run time. In a quiesced run they
//! still reach their accuracy goal inside the 10 s budget; on a noisy machine they
//! are the first to come out `(limit)`.
//!
//! Each encode is of values one at a time, so a small value's size includes the
//! coder's own start-up and flush: `Ans` writes about 2.7 bytes per value where
//! `Range` writes one on the `0..10` workload, and that is the per-value
//! overhead of a stream, not a poor model.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::marker::PhantomData;

use bincode1::{DefaultOptions, Options as _};
use compactly::v2::{Ans, Range};
use comparison_main::compactly as main_compactly;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scaling::Metrics;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod support;

/// Counts allocations, for the `alloc` columns of the decode groups. It slows
/// allocation-heavy code a little, the baseline included, and it counts for the
/// whole process, which is why it is installed here and not in the other
/// benches.
#[global_allocator]
static ALLOC: scaling::Allocator = scaling::Allocator::new();

/// What a workload's value must do for every codec to take it.
trait Encodable: compactly::v2::Encode + Serialize + DeserializeOwned + PartialEq + Debug {}
impl<T: compactly::v2::Encode + Serialize + DeserializeOwned + PartialEq + Debug> Encodable for T {}

trait Codec {
    fn encode<T: Encodable>(value: &T) -> Vec<u8>;
    fn decode<T: Encodable>(bytes: &[u8]) -> T;
}

struct RangeCodec;
impl Codec for RangeCodec {
    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        Range::encode(value)
    }
    fn decode<T: Encodable>(bytes: &[u8]) -> T {
        Range::decode(bytes).unwrap()
    }
}

struct AnsCodec;
impl Codec for AnsCodec {
    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        Ans::encode(value)
    }
    fn decode<T: Encodable>(bytes: &[u8]) -> T {
        Ans::decode(bytes).unwrap()
    }
}

/// bincode with variable-length integers.
struct Bincode;
impl Codec for Bincode {
    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        DefaultOptions::new().serialize(value).unwrap()
    }
    fn decode<T: Encodable>(bytes: &[u8]) -> T {
        DefaultOptions::new().deserialize(bytes).unwrap()
    }
}

/// bincode with fixed-width integers.
struct BincodeFixint;
impl Codec for BincodeFixint {
    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        bincode1::serialize(value).unwrap()
    }
    fn decode<T: Encodable>(bytes: &[u8]) -> T {
        bincode1::deserialize(bytes).unwrap()
    }
}

/// Another codec's bytes, compressed with zstd at `LEVEL`.
struct Zstd<C, const LEVEL: i32>(PhantomData<C>);
impl<C: Codec, const LEVEL: i32> Codec for Zstd<C, LEVEL> {
    fn encode<T: Encodable>(value: &T) -> Vec<u8> {
        zstd::bulk::compress(&C::encode(value), LEVEL).unwrap()
    }
    fn decode<T: Encodable>(bytes: &[u8]) -> T {
        // Not `bulk::decompress`, which allocates the whole capacity it is
        // given on every call: with room for 10 MB that is most of what a
        // small value's decode would measure.
        C::decode(&zstd::stream::decode_all(bytes).unwrap())
    }
}

type Zstd3 = Zstd<Bincode, 3>;
type Zstd9 = Zstd<Bincode, 9>;
type Zstd3Fixint = Zstd<BincodeFixint, 3>;

/// Where each codec's bytes sit in an [`Encoded`].
mod slot {
    pub const RANGE: usize = 0;
    pub const ANS: usize = 1;
    pub const BINCODE: usize = 2;
    pub const BINCODE_FIXINT: usize = 3;
    pub const ZSTD3: usize = 4;
    pub const ZSTD9: usize = 5;
    pub const ZSTD3_FIXINT: usize = 6;
}

fn encode_all<C: Codec, T: Encodable>(values: &[T]) -> Vec<Vec<u8>> {
    values.iter().map(C::encode).collect()
}

fn decode_all<C: Codec, T: Encodable>(bytes: &[Vec<u8>]) -> Vec<T> {
    bytes.iter().map(|b| C::decode(b)).collect()
}

/// Every codec's bytes for `values`, in [`slot`] order — after checking that
/// each of them decodes back to what went in, since a benchmark of a codec that
/// does not round-trip measures nothing.
fn encode_every_codec<T: Encodable>(values: &[T]) -> Vec<Vec<Vec<u8>>> {
    fn one<C: Codec, T: Encodable>(values: &[T]) -> Vec<Vec<u8>> {
        let bytes = encode_all::<C, T>(values);
        assert!(
            decode_all::<C, T>(&bytes).as_slice() == values,
            "{} did not round-trip",
            std::any::type_name::<C>()
        );
        bytes
    }
    vec![
        one::<RangeCodec, T>(values),
        one::<AnsCodec, T>(values),
        one::<Bincode, T>(values),
        one::<BincodeFixint, T>(values),
        one::<Zstd3, T>(values),
        one::<Zstd9, T>(values),
        one::<Zstd3Fixint, T>(values),
    ]
}

/// What a decode returned, on either side, so that one `memory` metric takes both.
/// The values are handed back rather than dropped, so a free is not timed.
enum Decoded<H, M> {
    Head(#[allow(dead_code)] Vec<H>),
    Main(#[allow(dead_code)] Vec<M>),
}

/// One copy of compactly: the corpora derived against it, and every value the
/// workloads are built from, which is everything here that names a corpus type.
///
/// A macro, not a generic, because each copy's corpus types are different types.
macro_rules! side {
    ($side:ident, $compactly:ident, $corpora:ident) => {
        #[allow(dead_code)]
        mod $side {
            // The renamed compactly and corpora come from here, and only their own
            // side uses them.
            #[allow(unused_imports)]
            use super::*;
            use $compactly::v2::{Ans, Encode, Range};
            pub use $corpora as comparison;
            pub use $corpora::meteorites::{self, MeteoriteData};
            pub use $corpora::shapes::MetNames;
            pub use $corpora::suicide::SuicideFactors;

            pub fn encode_range<T: Encode>(values: &[T]) -> Vec<Vec<u8>> {
                values.iter().map(|v| Range::encode(v)).collect()
            }
            pub fn encode_ans<T: Encode>(values: &[T]) -> Vec<Vec<u8>> {
                values.iter().map(|v| Ans::encode(v)).collect()
            }
            pub fn decode_range<T: Encode>(bytes: &[Vec<u8>]) -> Vec<T> {
                bytes.iter().map(|b| Range::decode(b).unwrap()).collect()
            }
            pub fn decode_ans<T: Encode>(bytes: &[Vec<u8>]) -> Vec<T> {
                bytes.iter().map(|b| Ans::decode(b).unwrap()).collect()
            }

            pub fn one<T>(value: T) -> Vec<T> {
                vec![value]
            }

            pub fn suicide_factors() -> Vec<SuicideFactors> {
                comparison::suicides_per_million().keys().cloned().collect()
            }

            pub fn suicide_rates() -> Vec<u16> {
                comparison::suicides_per_million()
                    .values()
                    .copied()
                    .collect()
            }

            pub fn individual_suicides() -> Vec<(SuicideFactors, u16)> {
                comparison::suicides_per_million().into_iter().collect()
            }

            pub fn meteorite_names() -> BTreeSet<String> {
                meteorites::meteorite_names().into_keys().collect()
            }
        }
    };
}

side!(head, compactly, comparison);
side!(on_main, main_compactly, comparison_main);

// ---- synthetic values, from one seed ----

fn seeded() -> SmallRng {
    SmallRng::seed_from_u64(3)
}

fn usizes() -> Vec<usize> {
    let mut rng = seeded();
    (0..1024).map(|_| rng.gen::<usize>()).collect()
}

fn small_numbers() -> Vec<usize> {
    let mut rng = seeded();
    (0..1024).map(|_| rng.gen_range(0..10usize)).collect()
}

fn short_vecs() -> Vec<Vec<usize>> {
    let mut rng = seeded();
    (0..256)
        .map(|_| (0..10).map(|_| rng.gen_range(0..8usize)).collect())
        .collect()
}

fn long_vecs() -> Vec<Vec<usize>> {
    let mut rng = seeded();
    (0..16)
        .map(|_| (0..1024).map(|_| rng.gen_range(0..8usize)).collect())
        .collect()
}

/// A comparison workload: a corpus, how every codec encodes it and how every
/// codec decodes its own bytes back.
///
/// `$ty` and `$make` are read twice, once in each copy of compactly's scope, so a
/// workload is written once and each copy gets its own derived types. `main range`
/// is the baseline, and the other crates run against this checkout's types.
///
/// The corpus and its encodings are `build_once` inputs: `scaling` builds each
/// once for its group and lends it to every call, so nothing is cloned or
/// rebuilt per iteration. Nothing here consumes or changes an input: `encode`
/// takes `&T`, `decode` takes `&[u8]`, and each returns something new.
macro_rules! workload {
    ($module:ident, $enc:literal, $dec:literal, $ty:ty, $make:expr) => {
        mod $module {
            use super::*;

            mod this_checkout {
                #[allow(unused_imports)]
                use crate::{head::*, *};

                pub type T = $ty;
                pub fn make() -> Vec<T> {
                    $make
                }
            }

            mod tip_of_main {
                #[allow(unused_imports)]
                use crate::{on_main::*, *};

                pub type T = $ty;
                pub fn make() -> Vec<T> {
                    $make
                }
            }

            type T = this_checkout::T;
            type M = tip_of_main::T;
            type Out = Decoded<T, M>;

            struct Corpus {
                head: Vec<T>,
                main: Vec<M>,
            }

            /// What every codec wrote for the corpus, each checked to round-trip.
            struct Bytes {
                head: Vec<Vec<Vec<u8>>>,
                main_range: Vec<Vec<u8>>,
                main_ans: Vec<Vec<u8>>,
            }

            #[scaling::input(group = $enc, name = "corpus", build_once)]
            fn corpus() -> Corpus {
                Corpus {
                    head: this_checkout::make(),
                    main: tip_of_main::make(),
                }
            }

            #[scaling::input(group = $dec, name = "bytes", build_once)]
            fn encoded() -> Bytes {
                let head = this_checkout::make();
                let main = tip_of_main::make();
                let bytes = Bytes {
                    head: encode_every_codec(&head),
                    main_range: on_main::encode_range(&main),
                    main_ans: on_main::encode_ans(&main),
                };
                assert!(
                    on_main::decode_range::<M>(&bytes.main_range) == main
                        && on_main::decode_ans::<M>(&bytes.main_ans) == main,
                    "main did not round-trip on {}",
                    $enc
                );
                bytes
            }

            #[scaling::bench(group = $enc, name = "main range", baseline)]
            fn encode_main_range(c: &Corpus) -> Vec<Vec<u8>> {
                on_main::encode_range(&c.main)
            }
            #[scaling::bench(group = $enc, name = "main ans")]
            fn encode_main_ans(c: &Corpus) -> Vec<Vec<u8>> {
                on_main::encode_ans(&c.main)
            }
            #[scaling::bench(group = $enc, name = "range")]
            fn encode_range(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<RangeCodec, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "ans")]
            fn encode_ans(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<AnsCodec, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "bincode", uninteresting)]
            fn encode_bincode(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Bincode, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "bincode fixint", uninteresting)]
            fn encode_bincode_fixint(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<BincodeFixint, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "zstd3", uninteresting)]
            fn encode_zstd3(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd3, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "zstd9", uninteresting)]
            fn encode_zstd9(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd9, T>(&c.head)
            }
            #[scaling::bench(group = $enc, name = "zstd3 fixint", uninteresting)]
            fn encode_zstd3_fixint(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd3Fixint, T>(&c.head)
            }

            #[scaling::bench(group = $dec, name = "main range", baseline)]
            fn decode_main_range(b: &Bytes) -> Out {
                Decoded::Main(on_main::decode_range(&b.main_range))
            }
            #[scaling::bench(group = $dec, name = "main ans")]
            fn decode_main_ans(b: &Bytes) -> Out {
                Decoded::Main(on_main::decode_ans(&b.main_ans))
            }
            #[scaling::bench(group = $dec, name = "range")]
            fn decode_range(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<RangeCodec, T>(&b.head[slot::RANGE]))
            }
            #[scaling::bench(group = $dec, name = "ans")]
            fn decode_ans(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<AnsCodec, T>(&b.head[slot::ANS]))
            }
            #[scaling::bench(group = $dec, name = "bincode", uninteresting)]
            fn decode_bincode(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<Bincode, T>(&b.head[slot::BINCODE]))
            }
            #[scaling::bench(group = $dec, name = "bincode fixint", uninteresting)]
            fn decode_bincode_fixint(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<BincodeFixint, T>(
                    &b.head[slot::BINCODE_FIXINT],
                ))
            }
            #[scaling::bench(group = $dec, name = "zstd3", uninteresting)]
            fn decode_zstd3(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<Zstd3, T>(&b.head[slot::ZSTD3]))
            }
            #[scaling::bench(group = $dec, name = "zstd9", uninteresting)]
            fn decode_zstd9(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<Zstd9, T>(&b.head[slot::ZSTD9]))
            }
            #[scaling::bench(group = $dec, name = "zstd3 fixint", uninteresting)]
            fn decode_zstd3_fixint(b: &Bytes) -> Out {
                Decoded::Head(decode_all::<Zstd3Fixint, T>(&b.head[slot::ZSTD3_FIXINT]))
            }

            /// Total bytes written, for the whole corpus.
            #[scaling::metrics(group = $enc)]
            fn size(out: Vec<Vec<u8>>) -> Metrics {
                Metrics::new().bytes("size", out.iter().map(Vec::len).sum::<usize>())
            }

            /// What decoding asked of the allocator: the most it held at once,
            /// and how many times it asked.
            #[scaling::metrics(group = $dec, allocation)]
            fn memory(_out: Out) -> Metrics {
                Metrics::new().peak_allocated_bytes().allocation_count()
            }
        }
    };
}

workload!(
    usize_values,
    "usize encode",
    "usize decode",
    usize,
    usizes()
);
workload!(
    small_values,
    "0..10 encode",
    "0..10 decode",
    usize,
    small_numbers()
);
workload!(
    short_vec_values,
    "vec[0..8; 10] encode",
    "vec[0..8; 10] decode",
    Vec<usize>,
    short_vecs()
);
workload!(
    long_vec_values,
    "vec[0..8; 1024] encode",
    "vec[0..8; 1024] decode",
    Vec<usize>,
    long_vecs()
);

workload!(
    books,
    "books encode",
    "books decode",
    comparison::books::Book,
    comparison::books::books()
);
workload!(
    suicide_data,
    "suicide data encode",
    "suicide data decode",
    Vec<SuicideFactors>,
    one(suicide_factors())
);
workload!(
    suicide_rates_values,
    "suicide rates encode",
    "suicide rates decode",
    Vec<u16>,
    one(suicide_rates())
);
workload!(
    suicide_map,
    "suicide encode",
    "suicide decode",
    std::collections::HashMap<SuicideFactors, u16>,
    one(comparison::suicides_per_million())
);
workload!(
    individual_suicide,
    "individual suicide encode",
    "individual suicide decode",
    (SuicideFactors, u16),
    individual_suicides()
);
workload!(
    meteorite_name_set,
    "meteorite names encode",
    "meteorite names decode",
    BTreeSet<String>,
    one(meteorite_names())
);
workload!(
    tenth_edition,
    "mtg tenth edition encode",
    "mtg tenth edition decode",
    comparison::FullSet,
    one(comparison::tenth_edition())
);
workload!(
    single_cards,
    "single cards encode",
    "single cards decode",
    comparison::Card,
    comparison::tenth_edition().data.cards
);
workload!(
    meteorite_records,
    "meteorites encode",
    "meteorites decode",
    Vec<MeteoriteData>,
    one(meteorites::meteorites())
);
workload!(
    single_meteorites,
    "single meteorites encode",
    "single meteorites decode",
    MeteoriteData,
    meteorites::meteorites()
);
workload!(
    meteorites_by_name,
    "meteorites by name encode",
    "meteorites by name decode",
    BTreeMap<String, MeteoriteData>,
    one(meteorites::meteorite_names())
);
workload!(
    meteorites_by_small_name,
    "meteorites by small name encode",
    "meteorites by small name decode",
    MetNames,
    one(MetNames {
        mets: meteorites::meteorite_names(),
    })
);

fn main() -> Result<(), scaling::RegistrationError> {
    support::run()
}
