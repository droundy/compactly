//! Do the tuned `AtMost<MAX>` tree-walk choices still hold?
//!
//! `Walk::production` (in `src/v2/atmost/walks.rs`) picks, for each value count
//! and each coder, which of six tree-walk implementations to use, from measurements
//! taken on one machine at one time. This bench re-takes them, so that choice can
//! be checked against the machine and the code in front of you: **is any forced
//! walk faster than what the library actually does?**
//!
//! Every comparison is one coder, one direction and one value count `MAX`, on one
//! distribution:
//!
//! - the baseline, `production`, runs whichever walk `Walk::production` picks for
//!   that `MAX` and that coder, looked up as the library looks it up — so it stays
//!   right if the tuning table changes, and a walk that beats it is a walk that
//!   should have been chosen;
//! - each other candidate forces one walk (`Complete`, `CompleteSpeculating`,
//!   `Uneven`, `UnevenSpeculating`, `CompleteBitwise`, `UnevenBitwise`) wherever
//!   that walk is valid for the `MAX`. The forced walk equal to production's is
//!   in there too, and its row should read as no difference: it is the noise
//!   floor.
//!
//! A candidate faster than `production` shows as a negative difference with an
//! error bar clear of zero; most rows read `(< x%)` or positive. Because the walks
//! share one bitstream per encode implementation, a speculating walk's *encode* is
//! its plain twin's, so encode has four forced candidates and decode six.
//!
//! The groups are `ans encode`, `ans decode`, `range encode` and `range decode`.
//! Each holds one table per value count and distribution; the type in the
//! table's heading (`Batch<31>`) is the `MAX`. Two distributions, because a walk's
//! speed depends on the data:
//!
//! - `uniform` keeps every context at 50/50 and the walk path
//!   branch-unpredictable — the best case for the latency-hiding speculating
//!   walks;
//! - `skewed` is `floor((MAX + 1) * u^8)`: value 0 takes about half the mass,
//!   like a dominant character or enum variant, so contexts adapt hard and the
//!   branch predictor learns the path on its own. Production data (string bytes,
//!   length buckets, enum discriminants) looks like this, so a finding is only
//!   trustworthy if it holds here.
//!
//! Each input is a pool of 16 batches of fresh random values, handed out in turn,
//! so the branch predictor cannot learn one fixed sequence; a batch holds
//! `max(4 * MAX, 256)` values, enough that the fixed cost of seeding `MAX`
//! contexts stays a small fraction of the per-value cost. Times are per batch.
//!
//! ```text
//! cargo bench --bench atmost --features benchmarking
//! cargo bench --bench atmost --features benchmarking -- "ans decode"
//! cargo bench --bench atmost --features benchmarking -- input=skewed
//! ```
//!
//! This is the family-wise worst case — hundreds of comparisons in one run, so
//! `scaling` judges each strictly — and a full run takes a while; narrow it by
//! group and by distribution. It uses the `#[doc(hidden)]`
//! `encode_atmost_batch`/`decode_atmost_batch` methods on `Range` and `Ans`,
//! which force a walk by indexing `WALKS` with a const generic, so it needs the
//! `benchmarking` feature.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use compactly::v2::{Ans, AtMost, Range, Walk, WALKS};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

mod support;

/// How the benchmark values are distributed over `0..=MAX`; see the module docs.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dist {
    Uniform,
    Skewed,
}

impl Dist {
    fn sample<const MAX: usize>(self, rng: &mut SmallRng) -> AtMost<MAX> {
        AtMost::new(match self {
            Dist::Uniform => rng.gen_range(0..=MAX),
            Dist::Skewed => {
                let u: f64 = rng.gen();
                (((MAX + 1) as f64 * u.powi(8)) as usize).min(MAX)
            }
        })
    }
}

/// Values per batch for a given `MAX`.
const fn n_values(max: usize) -> usize {
    let scaled = 4 * max;
    if scaled > 256 {
        scaled
    } else {
        256
    }
}

/// Batches in a pool: enough that a run does not see the same sequence twice in
/// a row, few enough to build instantly.
const POOL: usize = 16;

/// Which index of [`WALKS`] encodes the same bitstream as `WALKS[walk]`: a
/// speculating walk shares its plain twin's encode, so those bytes are the ones
/// it decodes.
const fn class(walk: usize) -> usize {
    match walk {
        1 => 0,
        3 => 2,
        other => other,
    }
}

/// One batch of values, and what each coder wrote for it with each distinct
/// encode implementation, indexed by [`WALKS`] (empty where that walk does not
/// apply or is another's twin).
struct Prepared<const MAX: usize> {
    values: Vec<AtMost<MAX>>,
    ans: [Vec<u8>; 6],
    range: [Vec<u8>; 6],
}

/// A pool of batches for one `MAX`, as a cheap-to-clone handle.
#[derive(Clone)]
struct Batch<const MAX: usize>(Rc<Prepared<MAX>>);

macro_rules! by_walk {
    ($idx:expr, $call:ident::<$max:ident>($($arg:expr),*)) => {
        match $idx {
            0 => $call::<$max, 0>($($arg),*),
            1 => $call::<$max, 1>($($arg),*),
            2 => $call::<$max, 2>($($arg),*),
            3 => $call::<$max, 3>($($arg),*),
            4 => $call::<$max, 4>($($arg),*),
            5 => $call::<$max, 5>($($arg),*),
            other => unreachable!("no walk {other}"),
        }
    };
}

fn ans_encode<const MAX: usize, const W: usize>(values: &[AtMost<MAX>]) -> Vec<u8> {
    Ans::encode_atmost_batch::<MAX, W>(values)
}
fn range_encode<const MAX: usize, const W: usize>(values: &[AtMost<MAX>]) -> Vec<u8> {
    Range::encode_atmost_batch::<MAX, W>(values)
}
fn ans_decode<const MAX: usize, const W: usize>(bytes: &[u8], n: usize) -> usize {
    Ans::decode_atmost_batch::<MAX, W>(bytes, n).len()
}
fn range_decode<const MAX: usize, const W: usize>(bytes: &[u8], n: usize) -> usize {
    Range::decode_atmost_batch::<MAX, W>(bytes, n).len()
}

impl<const MAX: usize> Prepared<MAX> {
    fn new(dist: Dist, seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);
        let values: Vec<AtMost<MAX>> = (0..n_values(MAX))
            .map(|_| dist.sample::<MAX>(&mut rng))
            .collect();
        let mut ans: [Vec<u8>; 6] = Default::default();
        let mut range: [Vec<u8>; 6] = Default::default();
        for (idx, walk) in WALKS.iter().enumerate() {
            if class(idx) == idx && walk.applies_to::<MAX>() {
                ans[idx] = by_walk!(idx, ans_encode::<MAX>(&values));
                range[idx] = by_walk!(idx, range_encode::<MAX>(&values));
                // A forced walk that does not round-trip would make every
                // number below meaningless, so say so before measuring.
                for (name, bytes, decoded) in [
                    (
                        "ans",
                        &ans[idx],
                        by_walk!(idx, ans_decode::<MAX>(&ans[idx], values.len())),
                    ),
                    (
                        "range",
                        &range[idx],
                        by_walk!(idx, range_decode::<MAX>(&range[idx], values.len())),
                    ),
                ] {
                    assert!(
                        !bytes.is_empty() && decoded == values.len(),
                        "{name} {walk:?} did not round-trip at MAX={MAX}"
                    );
                }
            }
        }
        Prepared { values, ans, range }
    }
}

/// A pool of batches, type-erased because a `thread_local` cannot be generic,
/// and how many of them have been handed out.
type Pool = (Vec<Rc<dyn Any>>, usize);

thread_local! {
    /// `(MAX, distribution)` → its pool.
    static POOLS: RefCell<HashMap<(usize, Dist), Pool>> = RefCell::new(HashMap::new());
}

impl<const MAX: usize> Batch<MAX> {
    fn pooled(dist: Dist) -> Self {
        POOLS.with(|pools| {
            let mut pools = pools.borrow_mut();
            let (batches, next) = pools.entry((MAX, dist)).or_insert_with(|| {
                let seed = 0xC0FFEE ^ MAX as u64 ^ ((dist as u64) << 32);
                let batches = (0..POOL as u64)
                    .map(|i| Rc::new(Prepared::<MAX>::new(dist, seed ^ (i << 40))) as Rc<dyn Any>)
                    .collect();
                (batches, 0)
            });
            let batch = batches[*next % POOL].clone();
            *next += 1;
            Batch(
                batch
                    .downcast::<Prepared<MAX>>()
                    .expect("a pool holds one MAX"),
            )
        })
    }

    fn values(&self) -> &[AtMost<MAX>] {
        &self.0.values
    }
}

/// Everything a candidate needs of a batch, generic over `MAX` so that one
/// function serves every value count it is registered for.
trait Sweep: Clone + 'static {
    fn pooled(dist: Dist) -> Self;

    /// Encode, forcing `WALKS[W]`.
    fn ans_encode<const W: usize>(&self) -> Vec<u8>;
    fn range_encode<const W: usize>(&self) -> Vec<u8>;
    /// Decode the bytes `WALKS[W]` reads, forcing `WALKS[W]`; returns how many
    /// values came out.
    fn ans_decode<const W: usize>(&self) -> usize;
    fn range_decode<const W: usize>(&self) -> usize;

    /// The same, with whichever walk `Walk::production` picks.
    fn ans_encode_production(&self) -> Vec<u8>;
    fn range_encode_production(&self) -> Vec<u8>;
    fn ans_decode_production(&self) -> usize;
    fn range_decode_production(&self) -> usize;
}

/// Where `walk` is in [`WALKS`].
fn index_of(walk: Walk) -> usize {
    WALKS
        .iter()
        .position(|w| *w == walk)
        .expect("every walk is in WALKS")
}

impl<const MAX: usize> Sweep for Batch<MAX> {
    fn pooled(dist: Dist) -> Self {
        Batch::<MAX>::pooled(dist)
    }

    fn ans_encode<const W: usize>(&self) -> Vec<u8> {
        assert!(
            WALKS[W].applies_to::<MAX>(),
            "{:?} is not valid at MAX={MAX}",
            WALKS[W]
        );
        Ans::encode_atmost_batch::<MAX, W>(self.values())
    }
    fn range_encode<const W: usize>(&self) -> Vec<u8> {
        assert!(
            WALKS[W].applies_to::<MAX>(),
            "{:?} is not valid at MAX={MAX}",
            WALKS[W]
        );
        Range::encode_atmost_batch::<MAX, W>(self.values())
    }
    fn ans_decode<const W: usize>(&self) -> usize {
        assert!(
            WALKS[W].applies_to::<MAX>(),
            "{:?} is not valid at MAX={MAX}",
            WALKS[W]
        );
        Ans::decode_atmost_batch::<MAX, W>(&self.0.ans[class(W)], self.values().len()).len()
    }
    fn range_decode<const W: usize>(&self) -> usize {
        assert!(
            WALKS[W].applies_to::<MAX>(),
            "{:?} is not valid at MAX={MAX}",
            WALKS[W]
        );
        Range::decode_atmost_batch::<MAX, W>(&self.0.range[class(W)], self.values().len()).len()
    }

    fn ans_encode_production(&self) -> Vec<u8> {
        let walk = Walk::production::<MAX>(Ans::SPECULATES).expect("MAX > 0");
        by_walk!(
            index_of(walk.encode_with()),
            ans_encode::<MAX>(self.values())
        )
    }
    fn range_encode_production(&self) -> Vec<u8> {
        let walk = Walk::production::<MAX>(Range::SPECULATES).expect("MAX > 0");
        by_walk!(
            index_of(walk.encode_with()),
            range_encode::<MAX>(self.values())
        )
    }
    fn ans_decode_production(&self) -> usize {
        let idx = index_of(Walk::production::<MAX>(Ans::SPECULATES).expect("MAX > 0"));
        by_walk!(
            idx,
            ans_decode::<MAX>(&self.0.ans[class(idx)], self.values().len())
        )
    }
    fn range_decode_production(&self) -> usize {
        let idx = index_of(Walk::production::<MAX>(Range::SPECULATES).expect("MAX > 0"));
        by_walk!(
            idx,
            range_decode::<MAX>(&self.0.range[class(idx)], self.values().len())
        )
    }
}

// ---------- which value counts, and which walks are valid for them ----------
//
// `MAX + 1` values. These are the counts the production walk is tuned for: the
// power-of-two boundaries and the places beside them where the tree shape and
// so the winner changes.

/// Every value count swept, handed to `$m` after its own arguments and a `;`.
macro_rules! every_max {
    ($m:ident!($($arg:tt)*)) => {
        $m!($($arg)* ; Batch<1>, Batch<2>, Batch<3>, Batch<4>, Batch<5>, Batch<7>, Batch<8>,
            Batch<9>, Batch<15>, Batch<16>, Batch<17>, Batch<31>, Batch<32>, Batch<33>,
            Batch<34>, Batch<40>, Batch<48>, Batch<63>, Batch<64>, Batch<127>, Batch<128>,
            Batch<255>, Batch<256>, Batch<512>, Batch<700>, Batch<2048>, Batch<3000>,
            Batch<4095>);
    };
}

/// The value counts with `MAX + 1` a power of two, where a complete (heap) tree
/// applies. `Walk::applies_to` is the authority; each candidate asserts it, so a
/// list that drifts from it fails on its first call rather than measuring a walk
/// that is not valid there.
macro_rules! power_of_two_max {
    ($m:ident!($($arg:tt)*)) => {
        $m!($($arg)* ; Batch<1>, Batch<3>, Batch<7>, Batch<15>, Batch<31>, Batch<63>,
            Batch<127>, Batch<255>, Batch<4095>);
    };
}

// ---------- the inputs ----------

macro_rules! inputs {
    ($(; $($ty:ty),+)?) => {
        #[scaling::input(
            group("ans encode", "ans decode", "range encode", "range decode"),
            name = "uniform",
            types($($($ty),+)?)
        )]
        fn uniform<T: Sweep>() -> T {
            T::pooled(Dist::Uniform)
        }

        #[scaling::input(
            group("ans encode", "ans decode", "range encode", "range decode"),
            name = "skewed",
            types($($($ty),+)?)
        )]
        fn skewed<T: Sweep>() -> T {
            T::pooled(Dist::Skewed)
        }
    };
}
every_max!(inputs!());

// ---------- the candidates ----------

/// One candidate: `$body`, run on the batch `$t`, in `$group`, called `$label`,
/// registered for each of the types listed after the `;`.
macro_rules! candidate {
    (
        $f:ident, $group:literal, $label:literal, $ret:ty, [$($flag:ident)?],
        $t:ident => $body:expr ; $($ty:ty),+
    ) => {
        #[scaling::bench(group = $group, name = $label $(, $flag)?, types($($ty),+))]
        fn $f<T: Sweep>($t: &T) -> $ret {
            $body
        }
    };
}

macro_rules! ans_encode_candidates {
    () => {
        every_max!(candidate!(ans_encode_production, "ans encode", "production", Vec<u8>, [baseline],
            t => t.ans_encode_production()));
        power_of_two_max!(candidate!(ans_encode_complete, "ans encode", "Complete", Vec<u8>, [],
            t => t.ans_encode::<0>()));
        every_max!(candidate!(ans_encode_uneven, "ans encode", "Uneven", Vec<u8>, [],
            t => t.ans_encode::<2>()));
        power_of_two_max!(candidate!(ans_encode_complete_bitwise, "ans encode", "CompleteBitwise", Vec<u8>, [],
            t => t.ans_encode::<4>()));
        every_max!(candidate!(ans_encode_uneven_bitwise, "ans encode", "UnevenBitwise", Vec<u8>, [],
            t => t.ans_encode::<5>()));
    };
}

macro_rules! range_encode_candidates {
    () => {
        every_max!(candidate!(range_encode_production, "range encode", "production", Vec<u8>, [baseline],
            t => t.range_encode_production()));
        power_of_two_max!(candidate!(range_encode_complete, "range encode", "Complete", Vec<u8>, [],
            t => t.range_encode::<0>()));
        every_max!(candidate!(range_encode_uneven, "range encode", "Uneven", Vec<u8>, [],
            t => t.range_encode::<2>()));
        power_of_two_max!(candidate!(range_encode_complete_bitwise, "range encode", "CompleteBitwise", Vec<u8>, [],
            t => t.range_encode::<4>()));
        every_max!(candidate!(range_encode_uneven_bitwise, "range encode", "UnevenBitwise", Vec<u8>, [],
            t => t.range_encode::<5>()));
    };
}

macro_rules! ans_decode_candidates {
    () => {
        every_max!(candidate!(ans_decode_production, "ans decode", "production", usize, [baseline],
            t => t.ans_decode_production()));
        power_of_two_max!(candidate!(ans_decode_complete, "ans decode", "Complete", usize, [],
            t => t.ans_decode::<0>()));
        power_of_two_max!(candidate!(ans_decode_complete_speculating, "ans decode", "CompleteSpeculating", usize, [],
            t => t.ans_decode::<1>()));
        every_max!(candidate!(ans_decode_uneven, "ans decode", "Uneven", usize, [],
            t => t.ans_decode::<2>()));
        every_max!(candidate!(ans_decode_uneven_speculating, "ans decode", "UnevenSpeculating", usize, [],
            t => t.ans_decode::<3>()));
        power_of_two_max!(candidate!(ans_decode_complete_bitwise, "ans decode", "CompleteBitwise", usize, [],
            t => t.ans_decode::<4>()));
        every_max!(candidate!(ans_decode_uneven_bitwise, "ans decode", "UnevenBitwise", usize, [],
            t => t.ans_decode::<5>()));
    };
}

macro_rules! range_decode_candidates {
    () => {
        every_max!(candidate!(range_decode_production, "range decode", "production", usize, [baseline],
            t => t.range_decode_production()));
        power_of_two_max!(candidate!(range_decode_complete, "range decode", "Complete", usize, [],
            t => t.range_decode::<0>()));
        power_of_two_max!(candidate!(range_decode_complete_speculating, "range decode", "CompleteSpeculating", usize, [],
            t => t.range_decode::<1>()));
        every_max!(candidate!(range_decode_uneven, "range decode", "Uneven", usize, [],
            t => t.range_decode::<2>()));
        every_max!(candidate!(range_decode_uneven_speculating, "range decode", "UnevenSpeculating", usize, [],
            t => t.range_decode::<3>()));
        power_of_two_max!(candidate!(range_decode_complete_bitwise, "range decode", "CompleteBitwise", usize, [],
            t => t.range_decode::<4>()));
        every_max!(candidate!(range_decode_uneven_bitwise, "range decode", "UnevenBitwise", usize, [],
            t => t.range_decode::<5>()));
    };
}

ans_encode_candidates!();
range_encode_candidates!();
ans_decode_candidates!();
range_decode_candidates!();

fn main() -> Result<(), scaling::RegistrationError> {
    support::config().run_and_print()
}
