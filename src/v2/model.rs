//! The interval vocabulary shared by the entropy coders and the probability
//! model: a "symbol" is a sub-interval of a fixed power-of-two total, and
//! coding means narrowing the coder's state to that sub-interval.
//!
//! Two sizes of the same idea live here:
//!
//! - [`Probability`] — the 2-slot case with total `2^SHIFT = 256`: a single
//!   bit's interval split, as learned by a [`BitContext`].
//! - [`SymbolRange`] — the general case with total `M = 2^16`: one whole
//!   tree-coded symbol's interval, built by the walks in `atmost::walks`.
//!
//! [`BitModel`] packages a [`BitContext`]'s hot-path data (its probability
//! plus both `adapt` successors) into one table entry so the tree walks pay a
//! single load per node.

use super::bit_context::BitContext;
use super::{EntropyCoder, EntropyDecoder};
use std::num::NonZeroU8;

/// A coder with a whole-symbol primitive: it can code one [`SymbolRange`] in
/// a single coding step (one renormalization) instead of one per tree level.
///
/// This is the coder's entire contribution to symbol coding; the tree shape,
/// the walk schedule, and the `MAX` guards all live in `atmost::walks`.
pub(crate) trait SymbolCoder: EntropyCoder {
    /// Code one whole symbol occupying `range` of the total `M`.
    fn encode_symbol(&mut self, range: SymbolRange);
}

/// Which decode walk a [`SymbolDecoder`] wants (see the walk inventory in
/// `atmost::walks`): whether spending ~2x the instructions to fetch both
/// candidate children before each bit resolves pays off depends on how much
/// latency the coder's own symbol step can hide, so the choice is a measured
/// per-coder policy, not a per-type one.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WalkStyle {
    /// Keep the serial dependency chain as lean as possible (`Ans`: its lean
    /// symbol step leaves speculative work exposed — measured +4…+22% slower
    /// at every value count).
    Plain,
    /// Speculate on both children (`Range`: its u64-division latency shadow
    /// absorbs the extra instructions — measured −4…−17% at value counts ≥ 4).
    Speculating,
}

/// The decode side of [`SymbolCoder`].
pub(crate) trait SymbolDecoder: EntropyDecoder {
    /// The measured walk policy for this coder.
    const WALK: WalkStyle;

    /// One whole-symbol decode step: peek the coder state's current slot in
    /// `[0, M)`, let `walk` recover the value and interval (adapting its
    /// contexts), then advance and renormalize once for the whole symbol.
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize;
}

/// log2 of the number of slots a single bit is coded out of: probabilities
/// are `prob / 256`.
pub(crate) const SHIFT: u8 = 8;

/// The probability that the bit will be false.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Probability {
    /// The probability is `prob / 256`
    pub(crate) prob: NonZeroU8,
}

impl Probability {
    /// Create a new probability based on a given number of true and false observations
    pub const fn new(trues: u64, falses: u64) -> Self {
        let prob = if falses == 0 {
            256 / (2 + trues)
        } else if trues == 0 {
            (1 + falses) * 256 / (2 + falses)
        } else {
            falses * 256 / (trues + falses)
        };
        let prob = prob as u8;
        Probability {
            prob: NonZeroU8::new(prob).unwrap(),
        }
    }

    /// The more likely value for the bit
    #[inline]
    pub fn likely_bit(&self) -> bool {
        self.prob.get() < (1 << (SHIFT - 1))
    }
    /// The probability of zero as an `f64` value.
    #[inline]
    pub fn as_f64(self) -> f64 {
        self.prob.get() as f64 / (1_u64 << SHIFT) as f64
    }
}

impl std::fmt::Debug for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trues = 256 - self.prob.get() as u64;
        let falses = self.prob;
        write!(f, "Probability::new({trues},{falses})")
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.prob.get() as f64 / (1_u64 << SHIFT) as f64;
        write!(f, "{v}")
    }
}

/// A sub-interval `[start, start + width)` of the fixed total `M = 1 << BITS`,
/// playing the role for a whole tree symbol that [`Probability`] plays for a
/// single bit.
///
/// Invariant: `width >= 1` and `start + width <= M`, so every representable
/// symbol keeps at least one slot and decoding is lossless by construction for
/// any probability skew (see [`SymbolRange::split_reserving`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolRange {
    start: u32,
    width: u32,
}

impl SymbolRange {
    /// log2 of the total slot count. 16 bits gives every leaf of a tree of up
    /// to 8 levels at least one slot even under the reserve clamp, while
    /// keeping the `Ans` symbol step inside the existing `u32` state. Kept as
    /// a single named const so a future deeper-fusion experiment can bump it.
    pub const BITS: u32 = 16;
    /// The total number of slots that a whole symbol is coded out of.
    pub const M: u32 = 1 << Self::BITS;

    /// First slot of the interval.
    #[inline]
    pub(crate) fn start(self) -> u32 {
        self.start
    }
    /// Number of slots in the interval (the symbol's frequency out of `M`).
    #[inline]
    pub(crate) fn width(self) -> u32 {
        self.width
    }

    /// Arbitrary interval for coder-level tests (real intervals only come
    /// from the tree walks in `atmost::walks`).
    #[cfg(test)]
    pub(crate) fn test_new(start: u32, width: u32) -> Self {
        assert!(width >= 1 && start + width <= Self::M);
        Self { start, width }
    }

    #[inline]
    pub(crate) fn full() -> Self {
        Self {
            start: 0,
            width: Self::M,
        }
    }

    /// Split point for the false/lower branch, given the node's bit
    /// probability and children carrying `lo` and `hi` leaves: the lower
    /// child reserves `lo` slots and the upper `hi`, so by induction every
    /// descendant leaf keeps at least one slot at any probability skew.
    ///
    /// The reserve is applied by squeezing the probability onto the reduced
    /// width (`width - lo - hi`) rather than clamping the split, because a
    /// clamp puts two extra compare/select ops on the serial per-level
    /// dependency chain of the (latency-bound) decode walk. The squeeze skews
    /// each split toward the middle by at most an `N/M` fraction (≤ 2^-8 for
    /// a byte tree), a sub-millibit cost per bit.
    ///
    /// The cut itself is the plain learned probability, NOT weighted by the
    /// `lo : hi` leaf counts: the adaptive context converges to the empirical
    /// bit frequency, and any static weighting multiplied on top would skew
    /// the coded probability away from that optimum forever after (measured
    /// +3% on adapted skewed 3-variant enums). The fractional-bit cost for
    /// *unadapted* values comes from seeding each node's initial context at
    /// `lo/(lo+hi)` instead — see `AtMostContext::default`.
    #[inline]
    pub(crate) fn split_reserving(self, p: Probability, lo: u32, hi: u32) -> u32 {
        debug_assert!(self.width >= lo + hi);
        // The product below must fit in u32: width <= M = 2^BITS and prob < 2^8.
        // If a deeper-fusion experiment bumps BITS past 24, revisit this method.
        const { assert!(Self::BITS + 8 <= u32::BITS) };
        (((self.width - lo - hi) * p.prob.get() as u32) >> 8) + lo
    }

    #[inline]
    pub(crate) fn lower(self, split: u32) -> Self {
        Self {
            start: self.start,
            width: split,
        }
    }
    #[inline]
    pub(crate) fn upper(self, split: u32) -> Self {
        Self {
            start: self.start + split,
            width: self.width - split,
        }
    }
    /// Whether `slot` falls in this interval. Callers maintain `slot >= start`.
    #[inline]
    pub(crate) fn contains(self, slot: u32) -> bool {
        debug_assert!(slot >= self.start);
        slot - self.start < self.width
    }
}

/// One [`BitContext`]'s hot-path data gathered into a single table entry: its
/// bit probability plus both `adapt` successors. The tree walks pay one load
/// per node via [`BitContext::model`] instead of separate `probability()` and
/// `adapt()` table lookups, and the successor for either bit outcome is
/// already in hand when the bit resolves.
#[derive(Clone, Copy)]
pub(crate) struct BitModel {
    /// `[adapt(false), adapt(true)]`.
    pub(crate) next: [BitContext; 2],
    /// The probability that the bit is false.
    pub(crate) prob: Probability,
}

impl BitModel {
    const fn new(state: BitContext) -> Self {
        Self {
            next: [state.adapt(false), state.adapt(true)],
            prob: state.probability(),
        }
    }
}

/// [`BitModel::new`] for every state reachable from `BitContext::default()`,
/// indexed by discriminant. Built by compile-time BFS over `adapt`; every
/// context starts at the default state, so reachable states are exactly the
/// ones a walk can ever load. (Unreachable slots hold the default state's
/// entry and are never read.)
///
/// A `static`, not a `const`: this table is indexed at runtime from hot
/// monomorphized code, and a promoted `const` may be duplicated per codegen
/// unit (linker merging is an optimization, not a guarantee) — the
/// single-shared-copy cache behavior is the table's whole point.
static FUSED: [BitModel; BitContext::COUNT] = {
    let start = BitContext::True0False0;
    let mut table = [BitModel::new(start); BitContext::COUNT];
    let mut queued = [false; BitContext::COUNT];
    let mut queue = [start; BitContext::COUNT];
    queued[start as usize] = true;
    let (mut head, mut tail) = (0, 1);
    while head < tail {
        let state = queue[head];
        head += 1;
        let entry = BitModel::new(state);
        table[state as usize] = entry;
        let mut j = 0;
        while j < 2 {
            let next = entry.next[j];
            if !queued[next as usize] {
                queued[next as usize] = true;
                queue[tail] = next;
                tail += 1;
            }
            j += 1;
        }
    }
    table
};

impl BitContext {
    /// This state's probability plus both `adapt` successors, in one load —
    /// the fused analogue of the generated `probability()`/`adapt()` tables.
    #[inline(always)]
    pub(crate) fn model(self) -> BitModel {
        FUSED[self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Standard, prelude::*};

    impl Distribution<Probability> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Probability {
            let prob = rng.gen_range(1u8..255);
            let prob = NonZeroU8::new(prob).unwrap();
            Probability { prob }
        }
    }

    /// Every state a context can hold must have a [`BitContext::model`] entry
    /// that agrees with the `probability()`/`adapt()` tables (random sampling
    /// covers all variants statistically), and the BFS seed must be the
    /// default state.
    #[test]
    fn model_matches_tables() {
        assert_eq!(BitContext::default(), BitContext::True0False0);
        for _ in 0..20_000 {
            let state: BitContext = rand::random();
            let entry = state.model();
            assert_eq!(entry.prob, state.probability());
            assert_eq!(entry.next, [state.adapt(false), state.adapt(true)]);
        }
    }
}
