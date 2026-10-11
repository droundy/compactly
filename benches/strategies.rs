//! Which encoding strategy suits which data.
//!
//! Each workload is a vector of 8192 values of one distribution and two
//! comparisons over it, `<workload> encode` and `<workload> decode`. The
//! candidates are compactly's strategies under both coders (`…/rng` is `Range`,
//! `…/ans` is `Ans`, so a strategy's two coders sit side by side) and, for
//! reference, bincode and bitcode. The baseline is `norm/rng`: the default
//! strategy on the default coder, which is what a caller who chose nothing gets,
//! so every other cell reads as "what choosing this instead would have done".
//!
//! compactly's strategies are the candidates the survey is about, so each is
//! counted when `scaling` corrects for how many questions it was asked. The
//! other crates are `uninteresting`: shown as a size, with no verdict, since
//! they differ from compactly by factors and nobody is asking whether a
//! difference is real.
//!
//! The workloads are `u32`, `u64` and `usize` under five distributions (log-uniform
//! magnitude, sorted, uniform bits, geometrically skewed small, and one repeated
//! constant), `i32` under two, and `Vec<u8>` under four. The other widths behave
//! as these do.
//!
//! The `size` column is the length of what each candidate wrote. A time is for
//! the whole vector: one encode or one decode.
//!
//! Every candidate is handed its own copy of the data to encode, because
//! compactly's `Encoded` wrapper takes ownership. The foreign codecs copy too,
//! so that none of them pays for something the others do not.
//!
//! ```text
//! cargo bench --bench strategies -- "u32 random"        # both directions, one workload
//! cargo bench --bench strategies -- bytes decode        # every bytes decode
//! cargo bench --bench strategies -- --quick
//! ```
//!
//! A word applies to group names; see `support/mod.rs`. A full run asks a few
//! hundred questions at once, so `scaling` judges each more strictly than a
//! filtered one does.

use std::marker::PhantomData;
use std::rc::Rc;

use bincode::config::standard;
use compactly::v2::{Ans, Range};
use compactly::{Encoded, Incompressible, LowCardinality, Normal, Small, Sorted, Values};
use rand::distributions::{Distribution, Standard};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use scaling::Metrics;

mod support;

const N: usize = 8192;

// ---------- the codecs ----------

/// A coder, as far as this survey needs to tell `Range` from `Ans`.
trait Coder {
    fn encode<T: compactly::v2::Encode>(value: &T) -> Vec<u8>;
    fn decode<T: compactly::v2::Encode>(bytes: &[u8]) -> T;
}

struct RangeCoder;
impl Coder for RangeCoder {
    fn encode<T: compactly::v2::Encode>(value: &T) -> Vec<u8> {
        Range::encode(value)
    }
    fn decode<T: compactly::v2::Encode>(bytes: &[u8]) -> T {
        Range::decode(bytes).unwrap()
    }
}

struct AnsCoder;
impl Coder for AnsCoder {
    fn encode<T: compactly::v2::Encode>(value: &T) -> Vec<u8> {
        Ans::encode(value)
    }
    fn decode<T: compactly::v2::Encode>(bytes: &[u8]) -> T {
        Ans::decode(bytes).unwrap()
    }
}

/// A way of turning a vector of `E` into bytes and back.
trait Scheme<E> {
    fn encode(data: &[E]) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> Vec<E>;
}

/// A `Vec<E>` encoded by coder `C` under strategy `S`.
struct Via<C, S>(PhantomData<(C, S)>);

impl<E: Clone, C: Coder, S> Scheme<E> for Via<C, S>
where
    Encoded<Vec<E>, S>: compactly::v2::Encode,
{
    fn encode(data: &[E]) -> Vec<u8> {
        C::encode(&Encoded::<Vec<E>, S>::from(data.to_vec()))
    }
    fn decode(bytes: &[u8]) -> Vec<E> {
        C::decode::<Encoded<Vec<E>, S>>(bytes).value()
    }
}

struct Bincode;
impl<E: Clone> Scheme<E> for Bincode
where
    Vec<E>: bincode::Encode + bincode::Decode<()>,
{
    fn encode(data: &[E]) -> Vec<u8> {
        bincode::encode_to_vec(data.to_vec(), standard()).unwrap()
    }
    fn decode(bytes: &[u8]) -> Vec<E> {
        bincode::decode_from_slice::<Vec<E>, _>(bytes, standard())
            .unwrap()
            .0
    }
}

struct Bitcode;
impl<E: Clone> Scheme<E> for Bitcode
where
    Vec<E>: bitcode::Encode + bitcode::DecodeOwned,
{
    fn encode(data: &[E]) -> Vec<u8> {
        bitcode::encode(&data.to_vec())
    }
    fn decode(bytes: &[u8]) -> Vec<E> {
        bitcode::decode::<Vec<E>>(bytes).unwrap()
    }
}

// ---------- the comparison ----------

/// One workload: a corpus, and for every candidate listed how it encodes it
/// and decodes its own bytes back. A candidate may be tagged `baseline` or
/// `uninteresting`.
///
/// The corpus and every candidate's bytes are built once per thread, checked to
/// round-trip, and handed out as `Rc` clones: `scaling` asks for an input on
/// every iteration, so anything heavier would be most of what is timed. Each
/// candidate is a module of its own so that its two functions can both be
/// called `encode` and `decode`.
macro_rules! survey {
    (
        $module:ident, $enc:literal, $dec:literal, $elem:ty, $data:expr,
        $( ($cand:ident, $label:literal, $scheme:ty $(, $flag:ident)?) ),+ $(,)?
    ) => {
        mod $module {
            use super::*;

            type E = $elem;
            type Corpus = Rc<Vec<E>>;
            type Bytes = Rc<AllBytes>;

            /// What each candidate wrote for the corpus.
            struct AllBytes {
                $( $cand: Vec<u8>, )+
            }

            thread_local! {
                static CORPUS: Corpus = Rc::new($data);
                static BYTES: Bytes = CORPUS.with(|corpus| {
                    $(
                        let $cand = <$scheme as Scheme<E>>::encode(corpus);
                        assert!(
                            &<$scheme as Scheme<E>>::decode(&$cand) == &**corpus,
                            "{} did not round-trip on {}", $label, $enc,
                        );
                    )+
                    Rc::new(AllBytes { $( $cand, )+ })
                });
            }

            #[scaling::input(group = $enc, name = "values")]
            fn corpus() -> Corpus {
                CORPUS.with(Rc::clone)
            }

            #[scaling::input(group = $dec, name = "bytes")]
            fn bytes() -> Bytes {
                BYTES.with(Rc::clone)
            }

            $(
                mod $cand {
                    use super::*;

                    #[scaling::bench(group = $enc, name = $label $(, $flag)?)]
                    pub fn encode(c: &Corpus) -> Vec<u8> {
                        <$scheme as Scheme<E>>::encode(c)
                    }

                    #[scaling::bench(group = $dec, name = $label $(, $flag)?)]
                    pub fn decode(b: &Bytes) -> Vec<E> {
                        <$scheme as Scheme<E>>::decode(&b.$cand)
                    }
                }
            )+

            #[scaling::metrics(group = $enc)]
            fn size(out: Vec<u8>) -> Metrics {
                Metrics::new().bytes("size", out.len())
            }
        }
    };
}

/// The strategies a `Vec<unsigned>` is surveyed under, for both coders.
macro_rules! unsigned {
    ($module:ident, $enc:literal, $dec:literal, $ty:ty, $data:expr) => {
        survey!(
            $module, $enc, $dec, $ty, $data,
            (rng_norm, "norm/rng", Via<RangeCoder, Values<Normal>>, baseline),
            (ans_norm, "norm/ans", Via<AnsCoder, Values<Normal>>),
            (rng_small, "sml/rng", Via<RangeCoder, Values<Small>>),
            (ans_small, "sml/ans", Via<AnsCoder, Values<Small>>),
            (rng_sorted, "srt/rng", Via<RangeCoder, Values<Sorted>>),
            (ans_sorted, "srt/ans", Via<AnsCoder, Values<Sorted>>),
            (rng_low, "low/rng", Via<RangeCoder, LowCardinality>),
            (ans_low, "low/ans", Via<AnsCoder, LowCardinality>),
            (rng_inc, "inc/rng", Via<RangeCoder, Values<Incompressible>>),
            (ans_inc, "inc/ans", Via<AnsCoder, Values<Incompressible>>),
            (bincode, "bincode", Bincode, uninteresting),
            (bitcode, "bitcode", Bitcode, uninteresting),
        );
    };
}

/// `usize` is surveyed under fewer strategies, and only against bincode and
/// bitcode.
macro_rules! sizes {
    ($module:ident, $enc:literal, $dec:literal, $data:expr) => {
        survey!(
            $module, $enc, $dec, usize, $data,
            (rng_norm, "norm/rng", Via<RangeCoder, Values<Normal>>, baseline),
            (ans_norm, "norm/ans", Via<AnsCoder, Values<Normal>>),
            (rng_small, "sml/rng", Via<RangeCoder, Values<Small>>),
            (ans_small, "sml/ans", Via<AnsCoder, Values<Small>>),
            (rng_sorted, "srt/rng", Via<RangeCoder, Values<Sorted>>),
            (ans_sorted, "srt/ans", Via<AnsCoder, Values<Sorted>>),
            (bincode, "bincode", Bincode, uninteresting),
            (bitcode, "bitcode", Bitcode, uninteresting),
        );
    };
}

macro_rules! signed {
    ($module:ident, $enc:literal, $dec:literal, $ty:ty, $data:expr) => {
        survey!(
            $module, $enc, $dec, $ty, $data,
            (rng_norm, "norm/rng", Via<RangeCoder, Values<Normal>>, baseline),
            (ans_norm, "norm/ans", Via<AnsCoder, Values<Normal>>),
            (rng_small, "sml/rng", Via<RangeCoder, Values<Small>>),
            (ans_small, "sml/ans", Via<AnsCoder, Values<Small>>),
            (rng_sorted, "srt/rng", Via<RangeCoder, Values<Sorted>>),
            (ans_sorted, "srt/ans", Via<AnsCoder, Values<Sorted>>),
            (bincode, "bincode", Bincode, uninteresting),
            (bitcode, "bitcode", Bitcode, uninteresting),
        );
    };
}

macro_rules! bytes {
    ($module:ident, $enc:literal, $dec:literal, $data:expr) => {
        survey!(
            $module, $enc, $dec, u8, $data,
            (rng_norm, "norm/rng", Via<RangeCoder, Values<Normal>>, baseline),
            (ans_norm, "norm/ans", Via<AnsCoder, Values<Normal>>),
            (rng_small, "sml/rng", Via<RangeCoder, Values<Small>>),
            (ans_small, "sml/ans", Via<AnsCoder, Values<Small>>),
            (rng_sorted, "srt/rng", Via<RangeCoder, Values<Sorted>>),
            (ans_sorted, "srt/ans", Via<AnsCoder, Values<Sorted>>),
            (rng_inc, "inc/rng", Via<RangeCoder, Values<Incompressible>>),
            (ans_inc, "inc/ans", Via<AnsCoder, Values<Incompressible>>),
            // The whole slice stored raw, with no per-byte entropy coding.
            (rng_raw, "raw/rng", Via<RangeCoder, Incompressible>),
            (ans_raw, "raw/ans", Via<AnsCoder, Incompressible>),
            (bitcode, "bitcode", Bitcode, uninteresting),
        );
    };
}

// ---------- data generation ----------
//
// Every workload is generated from one fixed seed, so it is the same data on
// every run and the `size` column means something from one run to the next.

fn rng() -> SmallRng {
    SmallRng::seed_from_u64(42)
}

fn sorted<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    values.sort();
    values
}

/// Values whose bit-length is uniform: every magnitude bucket equally likely.
trait LogUniform: Copy + Ord + 'static {
    fn gen_log_uniform<R: Rng>(rng: &mut R) -> Self;
}

macro_rules! impl_log_uniform {
    ($T:ty) => {
        impl LogUniform for $T {
            fn gen_log_uniform<R: Rng>(rng: &mut R) -> Self {
                let bits = <$T>::BITS;
                let leading: u32 = rng.gen_range(0..=bits);
                if leading >= bits {
                    0
                } else {
                    let high_bit = (1 as $T) << (bits - 1 - leading);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$T>() & mask)
                }
            }
        }
    };
}

impl_log_uniform!(u32);
impl_log_uniform!(u64);
impl_log_uniform!(usize);

fn log_uniform<T: LogUniform>() -> Vec<T> {
    let mut rng = rng();
    (0..N).map(|_| T::gen_log_uniform(&mut rng)).collect()
}

/// Plain uniform bits. Unlike `LogUniform` this weights large magnitudes far
/// more heavily: it is the distribution the default `Normal` encoding's
/// geometric seed is tuned for, and the one where `Small`'s flat seed pays its
/// extra fresh-context bits.
fn uniform<T>() -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut rng = rng();
    (0..N).map(|_| rng.gen()).collect()
}

/// A geometric distribution over bit-length, then uniform within that length:
/// most values small, with a long thin tail. `bit_length = 0` (probability
/// `1-p`) is the value 0; `k` (probability `(1-p)*p^k`) is a value in
/// `[2^(k-1), 2^k)`.
trait SkewedSmall: Copy + Ord + 'static {
    fn gen_skewed_small<R: Rng>(rng: &mut R, p: f64) -> Self;
}

macro_rules! impl_skewed_small {
    ($T:ty) => {
        impl SkewedSmall for $T {
            fn gen_skewed_small<R: Rng>(rng: &mut R, p: f64) -> Self {
                let bits = <$T>::BITS;
                let mut bit_length = 0u32;
                while bit_length < bits && rng.gen_bool(p) {
                    bit_length += 1;
                }
                if bit_length == 0 {
                    0
                } else {
                    let high_bit = (1 as $T) << (bit_length - 1);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$T>() & mask)
                }
            }
        }
    };
}

impl_skewed_small!(u32);
impl_skewed_small!(u64);
impl_skewed_small!(usize);

fn skewed_small<T: SkewedSmall>() -> Vec<T> {
    let mut rng = rng();
    (0..N).map(|_| T::gen_skewed_small(&mut rng, 0.8)).collect()
}

/// Log-uniform magnitude, negative a third of the time.
trait GenSigned: Copy + Ord + 'static {
    fn gen_signed<R: Rng>(rng: &mut R) -> Self;
}

macro_rules! impl_gen_signed {
    ($T:ty, $U:ty) => {
        impl GenSigned for $T {
            fn gen_signed<R: Rng>(rng: &mut R) -> Self {
                let bits = <$U>::BITS;
                // `leading` starts at 1 so the high bit is at most 2^(bits-2),
                // which always fits as a positive signed value.
                let leading: u32 = rng.gen_range(1..=bits);
                let mag: $U = if leading >= bits {
                    0
                } else {
                    let high_bit = (1 as $U) << (bits - 1 - leading);
                    let mask = high_bit - 1;
                    high_bit | (rng.gen::<$U>() & mask)
                };
                let signed_mag = mag as $T;
                if rng.gen_range(0u8..3) == 0 {
                    signed_mag.wrapping_neg()
                } else {
                    signed_mag
                }
            }
        }
    };
}

impl_gen_signed!(i32, u32);

fn signed_values<T: GenSigned>() -> Vec<T> {
    let mut rng = rng();
    (0..N).map(|_| T::gen_signed(&mut rng)).collect()
}

// ---------- the workloads ----------

unsigned!(
    u32_random,
    "u32 random encode",
    "u32 random decode",
    u32,
    log_uniform()
);
unsigned!(
    u32_sorted,
    "u32 sorted encode",
    "u32 sorted decode",
    u32,
    sorted(log_uniform())
);
unsigned!(
    u32_uniform,
    "u32 uniform bits encode",
    "u32 uniform bits decode",
    u32,
    uniform()
);
unsigned!(
    u32_skewed,
    "u32 skewed small encode",
    "u32 skewed small decode",
    u32,
    skewed_small()
);
unsigned!(
    u32_repeated,
    "u32 repeated small const encode",
    "u32 repeated small const decode",
    u32,
    vec![1; N]
);

unsigned!(
    u64_random,
    "u64 random encode",
    "u64 random decode",
    u64,
    log_uniform()
);
unsigned!(
    u64_sorted,
    "u64 sorted encode",
    "u64 sorted decode",
    u64,
    sorted(log_uniform())
);
unsigned!(
    u64_uniform,
    "u64 uniform bits encode",
    "u64 uniform bits decode",
    u64,
    uniform()
);
unsigned!(
    u64_skewed,
    "u64 skewed small encode",
    "u64 skewed small decode",
    u64,
    skewed_small()
);
unsigned!(
    u64_repeated,
    "u64 repeated small const encode",
    "u64 repeated small const decode",
    u64,
    vec![1; N]
);

// `usize` skewed small is the distribution its tuned default targets: real
// `usize`s are lengths, counts and indices, overwhelmingly small.
sizes!(
    usize_random,
    "usize random encode",
    "usize random decode",
    log_uniform()
);
sizes!(
    usize_sorted,
    "usize sorted encode",
    "usize sorted decode",
    sorted(log_uniform())
);
sizes!(
    usize_skewed,
    "usize skewed small encode",
    "usize skewed small decode",
    skewed_small()
);
sizes!(
    usize_repeated,
    "usize repeated small const encode",
    "usize repeated small const decode",
    vec![1; N]
);

signed!(
    i32_random,
    "i32 random encode",
    "i32 random decode",
    i32,
    signed_values()
);
signed!(
    i32_sorted,
    "i32 sorted encode",
    "i32 sorted decode",
    i32,
    sorted(signed_values())
);

fn random_bytes() -> Vec<u8> {
    let mut rng = rng();
    (0..N).map(|_| rng.gen()).collect()
}

fn ascii_bytes() -> Vec<u8> {
    let mut rng = rng();
    (0..N).map(|_| rng.gen_range(32u8..127)).collect()
}

fn sparse_bytes() -> Vec<u8> {
    let mut rng = rng();
    (0..N).map(|_| rng.gen_range(0u8..16)).collect()
}

/// A random walk with steps in `[-4, +4]`, wrapping at 256: slowly drifting,
/// which is what `Sorted` should do well on.
fn drifting_bytes() -> Vec<u8> {
    let mut rng = rng();
    let mut x = 64u8;
    (0..N)
        .map(|_| {
            let here = x;
            x = x.wrapping_add(rng.gen_range(-4i8..=4) as u8);
            here
        })
        .collect()
}

bytes!(
    bytes_random,
    "Vec<u8> random encode",
    "Vec<u8> random decode",
    random_bytes()
);
bytes!(
    bytes_ascii,
    "Vec<u8> ascii encode",
    "Vec<u8> ascii decode",
    ascii_bytes()
);
bytes!(
    bytes_sparse,
    "Vec<u8> sparse 0..16 encode",
    "Vec<u8> sparse 0..16 decode",
    sparse_bytes()
);
bytes!(
    bytes_drift,
    "Vec<u8> drift ±4 encode",
    "Vec<u8> drift ±4 decode",
    drifting_bytes()
);

fn main() -> Result<(), scaling::RegistrationError> {
    support::config().run_and_print()
}
