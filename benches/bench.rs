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
//! `Range` is the baseline of both. `Ans` is the other candidate anyone asked
//! about, so it is the only one counted when `scaling` decides how many
//! comparisons it is correcting for; bincode and zstd are `uninteresting`,
//! which means they are shown as a size against `Range` without a verdict.
//!
//! A time is for the *whole corpus*, not for one value: the synthetic corpora
//! are a thousand-odd values each, and the real ones are a single large value
//! (a whole `FullSet`) or all of a type's records (every `Card`).
//!
//! Below those, `scaling_*` benchmarks fit how encode and decode cost grow with
//! the number of elements: per codec, a constant in front of a power of `N`.
//!
//! ```text
//! cargo bench --bench bench -- usize                    # the usize workloads
//! cargo bench --bench bench -- meteorites decode        # two words narrow
//! cargo bench --bench bench -- --quick
//! ```
//!
//! A word applies to group names; see `support/mod.rs`. (`cand=` narrows the
//! candidates but leaves a group's baseline in place.)

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;

use bincode1::{DefaultOptions, Options as _};
use compactly::v2::{Ans, Range};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scaling::Metrics;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

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

/// A comparison workload: a corpus, how every codec encodes it and how every
/// codec decodes its own bytes back.
///
/// The corpus and its encodings are built once per thread and handed out as
/// `Rc` clones. `scaling` asks for an input on every iteration and clones it
/// for every candidate, so an input that was not a cheap handle would be most
/// of what was timed. Nothing here consumes or changes an input: `encode` takes
/// `&T`, `decode` takes `&[u8]`, and each returns something new.
macro_rules! workload {
    ($module:ident, $enc:literal, $dec:literal, $ty:ty, $make:expr) => {
        mod $module {
            use super::*;

            type T = $ty;
            type Corpus = Rc<Vec<T>>;
            type Encoded = Rc<Vec<Vec<Vec<u8>>>>;

            thread_local! {
                static CORPUS: Corpus = Rc::new($make);
                static ENCODED: Encoded =
                    CORPUS.with(|corpus| Rc::new(encode_every_codec(corpus)));
            }

            #[scaling::input(group = $enc, name = "corpus")]
            fn corpus() -> Corpus {
                CORPUS.with(Rc::clone)
            }

            #[scaling::input(group = $dec, name = "bytes")]
            fn encoded() -> Encoded {
                ENCODED.with(Rc::clone)
            }

            #[scaling::bench(group = $enc, name = "range", baseline)]
            fn encode_range(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<RangeCodec, T>(c)
            }
            #[scaling::bench(group = $enc, name = "ans")]
            fn encode_ans(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<AnsCodec, T>(c)
            }
            #[scaling::bench(group = $enc, name = "bincode", uninteresting)]
            fn encode_bincode(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Bincode, T>(c)
            }
            #[scaling::bench(group = $enc, name = "bincode fixint", uninteresting)]
            fn encode_bincode_fixint(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<BincodeFixint, T>(c)
            }
            #[scaling::bench(group = $enc, name = "zstd3", uninteresting)]
            fn encode_zstd3(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd3, T>(c)
            }
            #[scaling::bench(group = $enc, name = "zstd9", uninteresting)]
            fn encode_zstd9(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd9, T>(c)
            }
            #[scaling::bench(group = $enc, name = "zstd3 fixint", uninteresting)]
            fn encode_zstd3_fixint(c: &Corpus) -> Vec<Vec<u8>> {
                encode_all::<Zstd3Fixint, T>(c)
            }

            #[scaling::bench(group = $dec, name = "range", baseline)]
            fn decode_range(e: &Encoded) -> Vec<T> {
                decode_all::<RangeCodec, T>(&e[slot::RANGE])
            }
            #[scaling::bench(group = $dec, name = "ans")]
            fn decode_ans(e: &Encoded) -> Vec<T> {
                decode_all::<AnsCodec, T>(&e[slot::ANS])
            }
            #[scaling::bench(group = $dec, name = "bincode", uninteresting)]
            fn decode_bincode(e: &Encoded) -> Vec<T> {
                decode_all::<Bincode, T>(&e[slot::BINCODE])
            }
            #[scaling::bench(group = $dec, name = "bincode fixint", uninteresting)]
            fn decode_bincode_fixint(e: &Encoded) -> Vec<T> {
                decode_all::<BincodeFixint, T>(&e[slot::BINCODE_FIXINT])
            }
            #[scaling::bench(group = $dec, name = "zstd3", uninteresting)]
            fn decode_zstd3(e: &Encoded) -> Vec<T> {
                decode_all::<Zstd3, T>(&e[slot::ZSTD3])
            }
            #[scaling::bench(group = $dec, name = "zstd9", uninteresting)]
            fn decode_zstd9(e: &Encoded) -> Vec<T> {
                decode_all::<Zstd9, T>(&e[slot::ZSTD9])
            }
            #[scaling::bench(group = $dec, name = "zstd3 fixint", uninteresting)]
            fn decode_zstd3_fixint(e: &Encoded) -> Vec<T> {
                decode_all::<Zstd3Fixint, T>(&e[slot::ZSTD3_FIXINT])
            }

            /// Total bytes written, for the whole corpus.
            #[scaling::metrics(group = $enc)]
            fn size(out: Vec<Vec<u8>>) -> Metrics {
                Metrics::new().bytes("size", out.iter().map(Vec::len).sum::<usize>())
            }

            /// What decoding asked of the allocator: the most it held at once,
            /// and how many times it asked.
            #[scaling::metrics(group = $dec, allocation)]
            fn memory(_out: Vec<T>) -> Metrics {
                Metrics::new().peak_allocated_bytes().allocation_count()
            }
        }
    };
}

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

// ---- the `comparison` corpora ----

use comparison::meteorites::{self, MeteoriteData};
use comparison::suicide::SuicideFactors;

/// A map whose values are mapped with a strategy of their own, which is not
/// something `BTreeMap<String, _>` alone says.
#[derive(Debug, PartialEq, Serialize, Deserialize, compactly::v2::Encode)]
struct MetNames {
    #[compactly(Mapping<Compressible, Normal>)]
    mets: BTreeMap<String, MeteoriteData>,
}

fn one<T>(value: T) -> Vec<T> {
    vec![value]
}

fn suicide_factors() -> Vec<SuicideFactors> {
    comparison::suicides_per_million().keys().cloned().collect()
}

fn suicide_rates() -> Vec<u16> {
    comparison::suicides_per_million()
        .values()
        .copied()
        .collect()
}

fn individual_suicides() -> Vec<(SuicideFactors, u16)> {
    comparison::suicides_per_million().into_iter().collect()
}

fn meteorite_names() -> BTreeSet<String> {
    meteorites::meteorite_names().into_keys().collect()
}

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

// ---- how cost grows with the number of elements ----

#[derive(Debug, Serialize, Deserialize, PartialEq, compactly::v2::Encode)]
struct CompactSet {
    #[compactly(Small)]
    set: BTreeSet<u64>,
}

#[derive(
    Debug,
    compactly::v2::Encode,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
)]
enum ThreeOptions {
    A,
    B,
    C,
}

/// A fixed seed for each size, so that a value of size `n` is always the same
/// one.
fn rng_for(n: usize) -> SmallRng {
    SmallRng::seed_from_u64(n as u64)
}

fn hashset_of_usize(n: usize) -> HashSet<usize> {
    let mut rng = rng_for(n);
    (0..n).map(|_| rng.gen::<usize>()).collect()
}

fn btreeset_of_usize(n: usize) -> BTreeSet<usize> {
    let mut rng = rng_for(n);
    (0..n).map(|_| rng.gen::<usize>()).collect()
}

fn compact_btreeset(n: usize) -> CompactSet {
    let mut rng = rng_for(n);
    let mx = 2 * n as u64;
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert(rng.gen::<u64>() % mx);
    }
    CompactSet { set }
}

fn btreeset_of_vecs(n: usize) -> BTreeSet<Vec<usize>> {
    let mut rng = rng_for(n);
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert((0..7).map(|_| rng.gen_range(0..8usize)).collect::<Vec<_>>());
    }
    set
}

fn btreeset_of_three_option_vecs(n: usize) -> BTreeSet<Vec<ThreeOptions>> {
    let mut rng = rng_for(n);
    let mut option = || match rng.gen_range(0..10) {
        0 | 1 => ThreeOptions::A,
        2 | 3 => ThreeOptions::B,
        _ => ThreeOptions::C,
    };
    let mut set = BTreeSet::new();
    while set.len() < n {
        set.insert((0..15).map(|_| option()).collect::<Vec<_>>());
    }
    set
}

/// Four codecs, encode and decode, for one generator. The module name is what
/// names them in the report (`bench::scaling_hashset::encode_range`).
macro_rules! scaling_workload {
    ($module:ident, $ty:ty, $gen:path) => {
        mod $module {
            use super::*;

            #[scaling::bench_scaling(nmin = 5, make_input = |n| $gen(n))]
            fn encode_range(v: &mut $ty) -> Vec<u8> {
                RangeCodec::encode(v)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| $gen(n))]
            fn encode_ans(v: &mut $ty) -> Vec<u8> {
                AnsCodec::encode(v)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| $gen(n))]
            fn encode_bincode(v: &mut $ty) -> Vec<u8> {
                Bincode::encode(v)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| $gen(n))]
            fn encode_zstd3(v: &mut $ty) -> Vec<u8> {
                Zstd3::encode(v)
            }

            #[scaling::bench_scaling(nmin = 5, make_input = |n| RangeCodec::encode(&$gen(n)))]
            fn decode_range(b: &mut Vec<u8>) -> $ty {
                RangeCodec::decode(b)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| AnsCodec::encode(&$gen(n)))]
            fn decode_ans(b: &mut Vec<u8>) -> $ty {
                AnsCodec::decode(b)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| Bincode::encode(&$gen(n)))]
            fn decode_bincode(b: &mut Vec<u8>) -> $ty {
                Bincode::decode(b)
            }
            #[scaling::bench_scaling(nmin = 5, make_input = |n| Zstd3::encode(&$gen(n)))]
            fn decode_zstd3(b: &mut Vec<u8>) -> $ty {
                Zstd3::decode(b)
            }
        }
    };
}

scaling_workload!(scaling_hashset, HashSet<usize>, hashset_of_usize);
scaling_workload!(scaling_btreeset, BTreeSet<usize>, btreeset_of_usize);
scaling_workload!(scaling_compact_btreeset, CompactSet, compact_btreeset);
scaling_workload!(
    scaling_btreeset_of_vecs,
    BTreeSet<Vec<usize>>,
    btreeset_of_vecs
);
scaling_workload!(
    scaling_btreeset_of_enum_vecs,
    BTreeSet<Vec<ThreeOptions>>,
    btreeset_of_three_option_vecs
);

fn main() -> Result<(), scaling::RegistrationError> {
    support::config().run_and_print()
}
