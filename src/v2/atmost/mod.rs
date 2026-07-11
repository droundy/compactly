pub(crate) mod walks;

use super::bit_context::BitContext;
use super::Encode;
use walks::half;

#[cfg(test)]
use expect_test::expect;

/// An unsigned integer with a value at most `MAX`.
///
/// This type is useful if you want to compactly encode a value like the
/// variant of an enum with a range that is known at compile time to be
/// limited, with a range that may not be an integer number of bits.
///
/// Some values will take fewer bits than others, and *all* values will be
/// adapted independently so if e.g. any two variants are exclusively present
/// and equally likely, they will each take only a tiny bit more than a single
/// bit to encode (eventually).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtMost<const MAX: usize>(usize);

impl<const MAX: usize> AtMost<MAX> {
    /// Construct a new `AtMost<MAX>`.
    ///
    /// Panics if the value is greater than `MAX`.
    #[inline]
    pub const fn new(value: usize) -> Self {
        if value <= MAX {
            AtMost(value)
        } else {
            panic!("Invalid value in compactly::AtMost")
        }
    }
}

impl<const MAX: usize> From<AtMost<MAX>> for usize {
    #[inline]
    fn from(value: AtMost<MAX>) -> Self {
        value.0
    }
}

impl<const MAX: usize> TryFrom<usize> for AtMost<MAX> {
    type Error = ();
    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= MAX {
            Ok(AtMost(value))
        } else {
            Err(())
        }
    }
}

/// Adaptive context for [`AtMost<MAX>`] encoding.
///
/// Holds one bit context per internal node of the decision tree. The tree
/// over the `MAX + 1` possible values has exactly `MAX` internal nodes,
/// indexed in one of two ways depending on `MAX` (chosen consistently by
/// every walk):
///
/// - **Power-of-two value count** (`MAX + 1` a power of two): the tree is
///   complete, and contexts live in heap order
///   (`node = (node << 1) + 1 + bit`). The heap layout is what makes the hot
///   `u8` decode walk fast: a child's index depends only on the parent's, so
///   both children's contexts can be fetched before the node's bit resolves
///   (see `walks::complete::from_slot`).
/// - **Otherwise**: each node splits its interval at `split =
///   accumulated_value + value_considered`, which the encode/decode loops
///   guarantee to lie in `1..=MAX`. That `split` is the cut separating leaf
///   `split - 1` from leaf `split`, and each such cut belongs to a unique
///   node (the lowest common ancestor of those two leaves), so `split - 1`
///   is a collision-free index in `0..MAX`.
///
/// Either way a `[_; MAX]` holds every node's context exactly (with no slot
/// to spare), which keeps the context allocation-free and usable in a
/// `const`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtMostContext<const MAX: usize> {
    pub(crate) bits: [<bool as Encode>::Context; MAX],
}

impl<const MAX: usize> AtMostContext<MAX> {
    /// Initial per-node contexts, computed at compile time: each node starts
    /// at the low-count [`BitContext`] state whose probability best matches
    /// its children's leaf proportion `lo/(lo+hi)`, so a *fresh* context
    /// codes every value in the fractional `log2(MAX + 1)` bits (the coding
    /// split itself stays the plain learned probability — a static weight
    /// there would fight the adapted contexts forever; see
    /// `SymbolRange::split_reserving`). Balanced nodes seed to the ordinary
    /// default state, so for a power-of-two value count (every node
    /// balanced) this is all-default whatever the indexing scheme, and
    /// adaptation converges to the empirical distribution exactly as before.
    const SEEDED: [BitContext; MAX] = {
        let mut bits = [BitContext::True0False0; MAX];
        // Walk every internal node (start, len) of the tree; each visit pops
        // one interval and pushes its two children, so the stack grows by at
        // most one per level of the deepest path. Intervals shrink by at
        // least 1/4 per level, so 192 covers any possible `usize` MAX.
        let mut stack = [(0usize, 0usize); 192];
        stack[0] = (0, MAX + 1);
        let mut top = 1;
        while top > 0 {
            top -= 1;
            let (start, len) = stack[top];
            if len > 1 {
                let vc = half(len);
                let split = start + vc;
                bits[split - 1] = seed_context(vc as u64, (len - vc) as u64);
                stack[top] = (start, vc);
                stack[top + 1] = (split, len - vc);
                top += 2;
            }
        }
        bits
    };
}

/// The lowest-count [`BitContext`] state whose probability best matches
/// `lo/(lo+hi)`, searched by walking `adapt` up to 4 observations deep from
/// the default state — the same few-pseudo-observations seeding as the
/// generated char tables in `string/init.rs`, so a node whose real data
/// disagrees with the prior re-adapts quickly.
const fn seed_context(lo: u64, hi: u64) -> BitContext {
    let mut best = BitContext::True0False0;
    let mut best_err = seed_err(best, lo, hi);
    let mut path = 0u32;
    while path < 1 << 4 {
        let mut state = BitContext::True0False0;
        let mut k = 0;
        while k < 4 {
            state = state.adapt((path >> k) & 1 == 1);
            let err = seed_err(state, lo, hi);
            // Strict `<` keeps the first (lowest-count) state on ties.
            if err < best_err {
                best_err = err;
                best = state;
            }
            k += 1;
        }
        path += 1;
    }
    best
}

/// `|P(false) - lo/(lo+hi)|` scaled by `256*(lo+hi)` to stay in integers.
const fn seed_err(state: BitContext, lo: u64, hi: u64) -> u64 {
    let p = state.probability().prob.get() as u64;
    (p * (lo + hi)).abs_diff(256 * lo)
}

impl<const MAX: usize> Default for AtMostContext<MAX> {
    #[inline]
    fn default() -> Self {
        Self { bits: Self::SEEDED }
    }
}

impl<const MAX: usize> Encode for AtMost<MAX> {
    type Context = AtMostContext<MAX>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        writer.encode_atmost_tree(&mut ctx.bits, self.0)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self(reader.decode_atmost_tree(&mut ctx.bits)))
    }
}

#[test]
fn size() {
    use super::estimated_bits;
    fn test_urange<const MAX: usize>() {
        for i in 0..=MAX {
            let v = AtMost::<MAX>::new(i);
            println!("Testing AtMost::<{MAX}>::new({i})");
            let encoded = super::encode(&v);
            let decoded = super::decode::<AtMost<MAX>>(&encoded).unwrap();
            assert_eq!(decoded, v);
        }
    }
    test_urange::<0>();
    test_urange::<1>();
    test_urange::<2>();
    test_urange::<3>();
    test_urange::<4>();
    test_urange::<5>();
    test_urange::<6>();
    test_urange::<7>();
    test_urange::<8>();
    test_urange::<9>();
    test_urange::<254>();
    test_urange::<255>();
    test_urange::<256>();

    expect!["2"].assert_eq(&estimated_bits!(AtMost::<2>::try_from(0).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<2>::try_from(1).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<2>::try_from(2).unwrap()));

    expect!["2"].assert_eq(&estimated_bits!(AtMost::<4>::try_from(0).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<4>::try_from(1).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<4>::try_from(2).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<4>::try_from(3).unwrap()));
    expect!["2"].assert_eq(&estimated_bits!(AtMost::<4>::try_from(4).unwrap()));

    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(0).unwrap()));
    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(1).unwrap()));
    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(2).unwrap()));
    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(3).unwrap()));
    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(4).unwrap()));
    expect!["3"].assert_eq(&estimated_bits!(AtMost::<5>::try_from(5).unwrap()));

    expect!["7"].assert_eq(&estimated_bits!(AtMost::<127>::try_from(0).unwrap()));
    expect!["7"].assert_eq(&estimated_bits!(AtMost::<127>::try_from(1).unwrap()));
    expect!["7"].assert_eq(&estimated_bits!(AtMost::<127>::try_from(127).unwrap()));

    expect!["8"].assert_eq(&estimated_bits!(AtMost::<255>::try_from(0).unwrap()));
    expect!["8"].assert_eq(&estimated_bits!(AtMost::<255>::try_from(1).unwrap()));
    expect!["8"].assert_eq(&estimated_bits!(AtMost::<255>::try_from(255).unwrap()));
}

#[test]
fn context_is_const_and_allocation_free() {
    // The whole point of the array-backed context: it can live in a `const`
    // (no heap allocation, no runtime initialization).
    const _CTX: AtMostContext<2> = AtMostContext {
        bits: [super::bit_context::BitContext::True0False0; 2],
    };
    // An `AtMost<MAX>` context holds exactly `MAX` bit contexts — one per
    // internal node of the tree over `MAX + 1` values, with no unused slot.
    assert_eq!(AtMostContext::<2>::default().bits.len(), 2);
    assert_eq!(AtMostContext::<255>::default().bits.len(), 255);
    // A single-valued `AtMost<0>` needs no contexts at all.
    assert_eq!(std::mem::size_of::<AtMostContext<0>>(), 0);
}
