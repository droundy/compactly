//! Do the tuned `AtMost<MAX>` tree-walk choices still hold?
//!
//! `Walk::production` (in `walks.rs`) picks, for each value count and each coder,
//! which of six tree-walk implementations to use, from measurements taken on one
//! machine at one time. This re-takes them, so that choice can be checked against
//! the machine and the code in front of you: **is any forced walk faster than what
//! the library actually does?**
//!
//! It lives inside the crate, as ignored tests, so that the forced-walk machinery
//! (`Walk`, `WALKS`, the coders' `encode_atmost_batch` and `decode_atmost_batch`)
//! can stay private. There is one test per coder and direction, which is how a
//! run is narrowed — libtest's name filter does the selecting, and nothing here
//! parses arguments. On a reserved CPU, one at a time:
//!
//! ```text
//! quiet-bench run cargo test --release --lib -- --ignored --test-threads=1 \
//!     walk_shootout_ans_decode --nocapture
//! ```
//!
//! `walk_shootout` runs all four (about 40 minutes, and hundreds of questions at
//! once); the filters `walk_shootout_ans`, `walk_shootout_range`, `_decode` and
//! `_encode` select pairs (libtest matches substrings). `--test-threads=1` matters: concurrent
//! timed tests would measure each other. Building once with
//! `cargo test --release --lib --no-run` and running the executable under
//! `quiet-bench run` keeps cargo off the reserved CPU. `cargo test` does not run
//! them: they are `#[ignore]`d, and a timed benchmark under the default test
//! profile would be meaningless anyway.
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
//! Each holds one table per value count with a column per distribution; the type
//! in the table's heading (`Batch<31>`) is the `MAX`. Two distributions, because a
//! walk's speed depends on the data:
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
//! Every call gets a batch of values nobody has measured before: candidates take
//! `&mut`, which makes `scaling` generate a new input for each call, and each
//! batch is drawn from a seed no other batch has used. So the branch predictor
//! never sees a sequence twice, which is what production decoding of new data
//! looks like. Building a batch (drawing it and encoding it with every walk) is
//! not timed but comes out of each candidate's 10 s; `scaling` marks a cell
//! `(limit)` if that leaves it short of its accuracy goal. A batch holds `max(4 * MAX, 256)` values, enough that the
//! fixed cost of seeding `MAX` contexts stays a small fraction of the per-value
//! cost. Times are per batch.

use super::walks::{Walk, WALKS};
use super::AtMost;
use crate::v2::{Ans, Range};
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[path = "../../../benches/support/mod.rs"]
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
#[derive(Clone)]
struct Prepared<const MAX: usize> {
    values: Vec<AtMost<MAX>>,
    ans: [Vec<u8>; 6],
    range: [Vec<u8>; 6],
}

/// One batch for one `MAX` and distribution.
#[derive(Clone)]
struct Batch<const MAX: usize>(Prepared<MAX>);

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
    fn new(dist: Dist, seed: u64, check: bool) -> Self {
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
                // number below meaningless, so say so before measuring. Once
                // per value count and distribution: a new batch is built for
                // every call, and checking each would double what that costs.
                if !check {
                    continue;
                }
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

/// How many batches have been built, all value counts and distributions
/// together: a different seed for each, so that no two calls see the same
/// values.
static BUILT: AtomicU64 = AtomicU64::new(0);

thread_local! {
    /// The `(MAX, distribution)`s whose batches have been checked to round-trip.
    static CHECKED: RefCell<HashSet<(usize, Dist)>> = RefCell::new(HashSet::new());
}

impl<const MAX: usize> Batch<MAX> {
    fn build(dist: Dist) -> Self {
        let n = BUILT.fetch_add(1, Ordering::Relaxed);
        let seed = (0xC0FFEE ^ MAX as u64 ^ ((dist as u64) << 32)).wrapping_add(n << 40);
        let check = CHECKED.with(|c| c.borrow_mut().insert((MAX, dist)));
        Batch(Prepared::<MAX>::new(dist, seed, check))
    }

    fn values(&self) -> &[AtMost<MAX>] {
        &self.0.values
    }
}

/// Everything a candidate needs of a batch, generic over `MAX` so that one
/// function serves every value count it is registered for.
trait Sweep: Clone + 'static {
    fn build(dist: Dist) -> Self;

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
    fn build(dist: Dist) -> Self {
        Batch::<MAX>::build(dist)
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
            T::build(Dist::Uniform)
        }

        #[scaling::input(
            group("ans encode", "ans decode", "range encode", "range decode"),
            name = "skewed",
            types($($($ty),+)?)
        )]
        fn skewed<T: Sweep>() -> T {
            T::build(Dist::Skewed)
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
        fn $f<T: Sweep>($t: &mut T) -> $ret {
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

/// Measure the one group of the shootout `group` names: `scaling` discovers every
/// function above that is registered in this crate's test binary and this keeps
/// the ones of that group.
fn run_group(group: &'static str) {
    support::config()
        .filter_groups(move |name| name == group)
        .run_and_print()
        .expect("the registered shootout benchmarks compose");
}

#[test]
#[ignore = "a timed benchmark; see the module docs for how to run it"]
fn walk_shootout_ans_decode() {
    run_group("ans decode");
}

#[test]
#[ignore = "a timed benchmark; see the module docs for how to run it"]
fn walk_shootout_ans_encode() {
    run_group("ans encode");
}

#[test]
#[ignore = "a timed benchmark; see the module docs for how to run it"]
fn walk_shootout_range_decode() {
    run_group("range decode");
}

#[test]
#[ignore = "a timed benchmark; see the module docs for how to run it"]
fn walk_shootout_range_encode() {
    run_group("range encode");
}
