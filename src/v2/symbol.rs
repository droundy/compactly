//! Whole-tree symbol coding.
//!
//! The [`AtMost<MAX>`](super::AtMost) code (and through it `u8` /
//! `UBits<N>`, whose power-of-two-count trees are the balanced special case)
//! walks a binary search tree of adaptive [`BitContext`]s, historically
//! paying one coder step (one renormalization) per bit. [`SymbolRange`] lets
//! a coder pay a *single* step for the whole symbol: the tree walk builds one
//! cumulative sub-interval of the fixed total `M = 1 << 16`, touching and
//! adapting exactly the same contexts in the same order as the per-bit walk,
//! and the coder codes that interval in one `encode_symbol`-style operation.
//!
//! Every walk is generic over `MAX`, the largest encodable value: the tree
//! covers the `MAX + 1` values `0..=MAX` and has exactly `MAX` internal
//! nodes, so the contexts arrive as a snug `[BitContext; MAX]`.
//!
//! The walks are coder-independent, so they live here (written once). Each
//! tree layout is a complete, cutoff-free implementation in its own module —
//! [`complete`] for a power-of-two value count (heap-ordered contexts,
//! speculative decode) and [`uneven`] for arbitrary `MAX` (split-ordered
//! contexts, plain and prefetching decode variants) — so each can be tested
//! and benchmarked on its own. The dispatchers at the bottom
//! ([`encode_walk`], [`decode_walk`], [`decode_walk_speculating`],
//! [`encode_bitwise`], [`decode_bitwise`]) pick the right implementation
//! from `MAX` at compile time; they are the only place the cutoffs live.

use super::ans::Probability;
use super::bit_context::BitContext;
use super::{EntropyCoder, EntropyDecoder};

/// One [`BitContext`]'s hot-path data gathered into a single table entry: its
/// bit probability plus both `adapt` successors. The tree walks below pay one
/// load per node from [`FUSED`] instead of separate `probability()` and
/// `adapt()` table lookups, and the successor for either bit outcome is
/// already in hand when the bit resolves.
#[derive(Clone, Copy)]
struct FusedContext {
    /// `[adapt(false), adapt(true)]`.
    next: [BitContext; 2],
    /// The probability that the bit is false.
    prob: Probability,
}

impl FusedContext {
    const fn new(state: BitContext) -> Self {
        Self {
            next: [state.adapt(false), state.adapt(true)],
            prob: state.probability(),
        }
    }
}

/// [`FusedContext::new`] for every state reachable from
/// `BitContext::default()`, indexed by discriminant. Built by compile-time BFS
/// over `adapt`; every context starts at the default state, so reachable
/// states are exactly the ones a walk can ever load. (Unreachable slots hold
/// the default state's entry and are never read.)
static FUSED: [FusedContext; BitContext::COUNT] = {
    let start = BitContext::True0False0;
    let mut table = [FusedContext::new(start); BitContext::COUNT];
    let mut queued = [false; BitContext::COUNT];
    let mut queue = [start; BitContext::COUNT];
    queued[start as usize] = true;
    let (mut head, mut tail) = (0, 1);
    while head < tail {
        let state = queue[head];
        head += 1;
        let entry = FusedContext::new(state);
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

/// Where the [`AtMost`](super::AtMost) binary-search tree cuts an
/// interval of `i` possible values: the largest power of two below `i` (so
/// every walk in [`uneven`] agrees on the shape).
#[inline]
pub(crate) const fn half(i: usize) -> usize {
    let half = i / 2;
    if half > 1 {
        1 << half.ilog(2)
    } else {
        half
    }
}

/// The number of levels on the longest path of the [`half`]-split search
/// tree over `len` values. The [`uneven`] walks run a
/// `0..tree_depth(MAX + 1)` loop (with an early break at each leaf) instead
/// of `while len > 1`: the compile-time trip count lets the loop fully
/// unroll.
pub(crate) const fn tree_depth(len: usize) -> u32 {
    if len <= 1 {
        0
    } else {
        let vc = half(len);
        // The lower child covers `vc` values — a power of two, so its
        // subtree is complete with exactly `log2(vc)` further levels.
        let lo = vc.ilog2();
        let hi = tree_depth(len - vc);
        1 + if lo > hi { lo } else { hi }
    }
}

/// From this `MAX` up, [`decode_walk_speculating`] uses
/// [`uneven::from_slot_prefetching`] instead of the plain
/// [`uneven::from_slot`] (a power-of-two value count always takes
/// [`complete::from_slot`], whose heap layout makes speculation nearly
/// free). Only `Range` asks for the speculating walk: its symbol step
/// carries a u64 division whose latency shadow absorbs the speculation's ~2x
/// instruction count (measured −4…−17% for every value count ≥ 4, +11% at
/// 3); `Ans`'s lean symbol step leaves that work exposed and measures
/// *slower at every value count* (+4…+22%), so it always takes the plain
/// walk. Swept with `just-decompress-uless` over value counts 3…128; table
/// in OPTIMIZING.md.
pub(crate) const PREFETCH_MIN_MAX: usize = 3;

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
    /// from the tree walks below).
    #[cfg(test)]
    pub(crate) fn test_new(start: u32, width: u32) -> Self {
        assert!(width >= 1 && start + width <= Self::M);
        Self { start, width }
    }

    #[inline]
    fn full() -> Self {
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
    fn split_reserving(self, p: Probability, lo: u32, hi: u32) -> u32 {
        debug_assert!(self.width >= lo + hi);
        // The product below must fit in u32: width <= M = 2^BITS and prob < 2^8.
        // If a deeper-fusion experiment bumps BITS past 24, revisit this method.
        const { assert!(Self::BITS + 8 <= u32::BITS) };
        (((self.width - lo - hi) * p.prob.get() as u32) >> 8) + lo
    }

    #[inline]
    fn lower(self, split: u32) -> Self {
        Self {
            start: self.start,
            width: split,
        }
    }
    #[inline]
    fn upper(self, split: u32) -> Self {
        Self {
            start: self.start + split,
            width: self.width - split,
        }
    }
    /// Whether `slot` falls in this interval. Callers maintain `slot >= start`.
    #[inline]
    fn contains(self, slot: u32) -> bool {
        debug_assert!(slot >= self.start);
        slot - self.start < self.width
    }
}

/// The balanced-tree implementation for a **power-of-two value count**
/// (`MAX + 1` a power of two): a complete binary tree with contexts stored
/// in heap order (`node = (node << 1) + 1 + bit`), the layout the `u8` hot
/// path relies on. A child's index depends only on the parent's — not on the
/// current interval — so the decode walk fetches both children's [`FUSED`]
/// entries before the node's bit resolves at no extra index arithmetic,
/// hiding the load latency that otherwise dominates the serial per-level
/// chain (measured worth ~8-11% on the `u8`-heavy string decode path for
/// both coders versus the split-ordered walks in [`uneven`]).
pub(crate) mod complete {
    use super::*;

    /// Walk the heap tree for `value`, adapting the contexts exactly as the
    /// per-bit walk would, and return the symbol's interval. `MAX + 1` must
    /// be a power of two and `value <= MAX`.
    #[inline]
    pub(crate) fn for_value<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) -> SymbolRange {
        let mut range = SymbolRange::full();
        let n_bits = (MAX + 1).ilog2();
        debug_assert_eq!(1 << n_bits, MAX + 1);
        debug_assert!(value <= MAX);
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let cur = FUSED[contexts[node] as usize];
            let reserve = 1u32 << i;
            let split = range.split_reserving(cur.prob, reserve, reserve);
            let bit = (value >> i) & 1 == 1;
            range = if bit {
                range.upper(split)
            } else {
                range.lower(split)
            };
            contexts[node] = cur.next[bit as usize];
            node = (node << 1) + 1 + bit as usize;
        }
        range
    }

    /// Walk the heap tree driven by a peeked `slot` in `[0, M)`, adapting the
    /// contexts identically to [`for_value`], and return the interval
    /// together with the decoded value in `0..=MAX`.
    #[inline]
    pub(crate) fn from_slot<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        slot: u32,
    ) -> (SymbolRange, usize) {
        let mut range = SymbolRange::full();
        let n_bits = (MAX + 1).ilog2();
        debug_assert_eq!(1 << n_bits, MAX + 1);
        debug_assert!(slot < SymbolRange::M);
        let mut node = 0usize;
        let mut cur = FUSED[contexts[0] as usize];
        for i in (0..n_bits).rev() {
            let reserve = 1u32 << i;
            let split = range.split_reserving(cur.prob, reserve, reserve);
            let lower = range.lower(split);
            let bit = !lower.contains(slot);
            let adapted = cur.next[bit as usize];
            if i > 0 {
                // Speculatively fetch both children's fused entries: these
                // loads depend only on `node`, not on `bit`, so they issue a
                // full level ahead of the serial split/compare chain, leaving
                // just a select on the critical path.
                let left = FUSED[contexts[2 * node + 1] as usize];
                let right = FUSED[contexts[2 * node + 2] as usize];
                cur = if bit { right } else { left };
            }
            contexts[node] = adapted;
            range = if bit { range.upper(split) } else { lower };
            node = (node << 1) + 1 + bit as usize;
        }
        (range, node - MAX)
    }

    /// The per-bit walk over the same tree and contexts: the `Raw` coder's
    /// format, and the fallback when the value count exceeds
    /// [`SymbolRange::M`].
    #[inline]
    pub(crate) fn encode_bitwise<E: EntropyCoder, const MAX: usize>(
        writer: &mut E,
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) {
        debug_assert!(value <= MAX);
        let n_bits = (MAX + 1).ilog2();
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let bit = (value >> i) & 1 == 1;
            let context = &mut contexts[node];
            writer.encode_bit(context.probability(), bit);
            *context = context.adapt(bit);
            node = (node << 1) + 1 + bit as usize;
        }
    }

    /// The per-bit inverse of [`encode_bitwise`].
    #[inline]
    pub(crate) fn decode_bitwise<D: EntropyDecoder, const MAX: usize>(
        reader: &mut D,
        contexts: &mut [BitContext; MAX],
    ) -> usize {
        let n_bits = (MAX + 1).ilog2();
        let mut node = 0usize;
        for _ in 0..n_bits {
            let bit = reader.decode_bit(&mut contexts[node]);
            node = (node << 1) + 1 + bit as usize;
        }
        node - MAX
    }
}

/// The binary-search implementation for **arbitrary `MAX`**: the tree splits
/// each interval of values at [`half`], and the context for the cut at
/// `split` lives at index `split - 1` (each cut belongs to a unique node —
/// the lowest common ancestor of leaves `split - 1` and `split` — so the
/// index is collision-free in `0..MAX`). Fresh contexts are seeded so every
/// value costs the fractional `log2(MAX + 1)` bits; see `AtMostContext`.
pub(crate) mod uneven {
    use super::*;

    /// Walk the search tree for `value`, adapting the contexts exactly as
    /// the per-bit walk would, and return the symbol's interval. Requires
    /// `MAX + 1 <= M` so every leaf can reserve a slot.
    #[inline]
    pub(crate) fn for_value<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) -> SymbolRange {
        debug_assert!(MAX < SymbolRange::M as usize);
        debug_assert!(value <= MAX);
        let mut range = SymbolRange::full();
        let mut accumulated_value = 0;
        let mut possible_values_left = MAX + 1;
        for _ in 0..const { tree_depth(MAX + 1) } {
            if possible_values_left <= 1 {
                break;
            }
            let value_considered = half(possible_values_left);
            let split = accumulated_value + value_considered;
            let cur = FUSED[contexts[split - 1] as usize];
            let slot_split = range.split_reserving(
                cur.prob,
                value_considered as u32,
                (possible_values_left - value_considered) as u32,
            );
            let bit = value >= split;
            contexts[split - 1] = cur.next[bit as usize];
            if bit {
                range = range.upper(slot_split);
                accumulated_value = split;
                possible_values_left -= value_considered;
            } else {
                range = range.lower(slot_split);
                possible_values_left = value_considered;
            }
        }
        range
    }

    /// Walk the search tree driven by a peeked `slot` in `[0, M)`, adapting
    /// the contexts identically to [`for_value`], and return the interval
    /// together with the decoded value in `0..=MAX`.
    ///
    /// This is the plain walk: each level's [`FUSED`] load waits for the
    /// previous level's bit. The [`from_slot_prefetching`] variant hides
    /// that latency but roughly doubles the instruction count (both
    /// children's [`half`] index arithmetic, double [`FUSED`] loads,
    /// register-pressure spills), which only `Range`'s division-heavy
    /// symbol step can absorb — see [`PREFETCH_MIN_MAX`].
    #[inline]
    pub(crate) fn from_slot<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        slot: u32,
    ) -> (SymbolRange, usize) {
        debug_assert!(MAX < SymbolRange::M as usize);
        debug_assert!(slot < SymbolRange::M);
        let mut range = SymbolRange::full();
        let mut accumulated_value = 0;
        let mut possible_values_left = MAX + 1;
        for _ in 0..const { tree_depth(MAX + 1) } {
            if possible_values_left <= 1 {
                break;
            }
            let value_considered = half(possible_values_left);
            let split = accumulated_value + value_considered;
            let cur = FUSED[contexts[split - 1] as usize];
            let slot_split = range.split_reserving(
                cur.prob,
                value_considered as u32,
                (possible_values_left - value_considered) as u32,
            );
            let lower = range.lower(slot_split);
            let bit = !lower.contains(slot);
            contexts[split - 1] = cur.next[bit as usize];
            if bit {
                range = range.upper(slot_split);
                accumulated_value = split;
                possible_values_left -= value_considered;
            } else {
                range = lower;
                possible_values_left = value_considered;
            }
        }
        (range, accumulated_value)
    }

    /// The speculative variant of [`from_slot`]: both candidate child
    /// contexts are fetched before the node's bit resolves (their positions
    /// depend only on the current interval, not the bit), so the [`FUSED`]
    /// loads issue a level ahead of the serial split/compare chain, leaving
    /// just a select on the critical path. A leaf child gets a harmless
    /// dummy index 0 — if the walk descends into it the loop ends and the
    /// fetched entry is unused.
    #[inline]
    pub(crate) fn from_slot_prefetching<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        slot: u32,
    ) -> (SymbolRange, usize) {
        let mut range = SymbolRange::full();
        if MAX == 0 {
            return (range, 0);
        }
        let mut accumulated_value = 0;
        let mut possible_values_left = MAX + 1;
        let mut value_considered = half(MAX + 1);
        let mut split = value_considered;
        let mut cur = FUSED[contexts[split - 1] as usize];
        // Every path breaks at a leaf within `tree_depth(MAX + 1)`
        // iterations; the compile-time bound is what lets the loop fully
        // unroll.
        for _ in 0..const { tree_depth(MAX + 1) } {
            let lo_len = value_considered;
            let hi_len = possible_values_left - value_considered;
            let slot_split = range.split_reserving(cur.prob, lo_len as u32, hi_len as u32);
            let lo_vc = half(lo_len);
            let hi_vc = half(hi_len);
            let lo_split = accumulated_value + lo_vc;
            let hi_split = split + hi_vc;
            let lo_cur = FUSED[contexts[if lo_len > 1 { lo_split - 1 } else { 0 }] as usize];
            let hi_cur = FUSED[contexts[if hi_len > 1 { hi_split - 1 } else { 0 }] as usize];
            let lower = range.lower(slot_split);
            let bit = !lower.contains(slot);
            contexts[split - 1] = cur.next[bit as usize];
            range = if bit { range.upper(slot_split) } else { lower };
            if bit {
                accumulated_value = split;
                possible_values_left = hi_len;
                if hi_len <= 1 {
                    break;
                }
                value_considered = hi_vc;
                split = hi_split;
                cur = hi_cur;
            } else {
                possible_values_left = lo_len;
                if lo_len <= 1 {
                    break;
                }
                value_considered = lo_vc;
                split = lo_split;
                cur = lo_cur;
            }
        }
        (range, accumulated_value)
    }

    /// The per-bit walk over the same tree and contexts: the `Raw` coder's
    /// format, and the fallback when the value count exceeds
    /// [`SymbolRange::M`].
    #[inline]
    pub(crate) fn encode_bitwise<E: EntropyCoder, const MAX: usize>(
        writer: &mut E,
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) {
        debug_assert!(value <= MAX);
        let mut accumulated_value = 0;
        let mut possible_values_left = MAX + 1;
        while possible_values_left > 1 {
            let value_considered = half(possible_values_left);
            let split = accumulated_value + value_considered;
            let bit = value >= split;
            let context = &mut contexts[split - 1];
            writer.encode_bit(context.probability(), bit);
            *context = context.adapt(bit);
            if bit {
                accumulated_value = split;
                possible_values_left -= value_considered;
            } else {
                possible_values_left = value_considered;
            }
        }
    }

    /// The per-bit inverse of [`encode_bitwise`].
    #[inline]
    pub(crate) fn decode_bitwise<D: EntropyDecoder, const MAX: usize>(
        reader: &mut D,
        contexts: &mut [BitContext; MAX],
    ) -> usize {
        let mut accumulated_value = 0;
        let mut possible_values_left = MAX + 1;
        while possible_values_left > 1 {
            let value_considered = half(possible_values_left);
            let split = accumulated_value + value_considered;
            let bit = reader.decode_bit(&mut contexts[split - 1]);
            if bit {
                accumulated_value = split;
                possible_values_left -= value_considered;
            } else {
                possible_values_left = value_considered;
            }
        }
        accumulated_value
    }
}

/// Build the interval for a known `value` (the encode-side walk), picking
/// the implementation from `MAX` at compile time: [`complete::for_value`]
/// for a power-of-two value count, [`uneven::for_value`] otherwise.
#[inline]
pub(crate) fn encode_walk<const MAX: usize>(
    contexts: &mut [BitContext; MAX],
    value: usize,
) -> SymbolRange {
    if (MAX + 1).is_power_of_two() {
        complete::for_value(contexts, value)
    } else {
        uneven::for_value(contexts, value)
    }
}

/// Recover the value from a peeked `slot` (the decode-side walk), keeping
/// the serial dependency chain as lean as possible: [`complete::from_slot`]
/// for a power-of-two value count (its heap-layout speculation is nearly
/// free), [`uneven::from_slot`] otherwise. This is the right choice for
/// `Ans`, whose lean symbol step leaves any speculative work exposed.
#[inline]
pub(crate) fn decode_walk<const MAX: usize>(
    contexts: &mut [BitContext; MAX],
    slot: u32,
) -> (SymbolRange, usize) {
    if (MAX + 1).is_power_of_two() {
        complete::from_slot(contexts, slot)
    } else {
        uneven::from_slot(contexts, slot)
    }
}

/// [`decode_walk`], but spending extra instructions to hide the per-level
/// [`FUSED`] load latency where a heavier symbol step can absorb them: the
/// right choice for `Range`, whose u64 division provides the latency shadow.
/// A power-of-two value count takes [`complete::from_slot`] as always;
/// other `MAX` at or above [`PREFETCH_MIN_MAX`] take
/// [`uneven::from_slot_prefetching`].
#[inline]
pub(crate) fn decode_walk_speculating<const MAX: usize>(
    contexts: &mut [BitContext; MAX],
    slot: u32,
) -> (SymbolRange, usize) {
    if (MAX + 1).is_power_of_two() {
        complete::from_slot(contexts, slot)
    } else if MAX >= PREFETCH_MIN_MAX {
        uneven::from_slot_prefetching(contexts, slot)
    } else {
        uneven::from_slot(contexts, slot)
    }
}

/// Code one symbol bit-by-bit: the default `encode_atmost_tree` (preserving
/// `Raw`'s bit-packed format), and the fallback the symbol coders use when
/// the value count exceeds [`SymbolRange::M`] (a whole-symbol interval
/// cannot give every leaf a slot then). Same trees and context indexing as
/// [`encode_walk`].
#[inline]
pub(crate) fn encode_bitwise<E: EntropyCoder, const MAX: usize>(
    writer: &mut E,
    contexts: &mut [BitContext; MAX],
    value: usize,
) {
    if (MAX + 1).is_power_of_two() {
        complete::encode_bitwise(writer, contexts, value)
    } else {
        uneven::encode_bitwise(writer, contexts, value)
    }
}

/// The per-bit inverse of [`encode_bitwise`].
#[inline]
pub(crate) fn decode_bitwise<D: EntropyDecoder, const MAX: usize>(
    reader: &mut D,
    contexts: &mut [BitContext; MAX],
) -> usize {
    if (MAX + 1).is_power_of_two() {
        complete::decode_bitwise(reader, contexts)
    } else {
        uneven::decode_bitwise(reader, contexts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The encode-side and decode-side [`complete`] walks (power-of-two
    /// value count) must build identical intervals and adapt the contexts
    /// identically, for every slot of the interval.
    fn check_complete_determinism<const MAX: usize>(contexts: [BitContext; MAX]) {
        let mut total = 0u32;
        for value in 0..=MAX {
            let mut enc_ctx = contexts;
            let range = complete::for_value(&mut enc_ctx, value);
            assert!(range.width >= 1, "leaf lost its slot for value {value}");
            assert_eq!(
                range.start, total,
                "intervals must tile [0, M) in value order"
            );
            total += range.width;
            for slot in [range.start, range.start + range.width - 1] {
                let mut dec_ctx = contexts;
                let (dec_range, decoded) = complete::from_slot(&mut dec_ctx, slot);
                assert_eq!(dec_range, range);
                assert_eq!(decoded, value);
                assert_eq!(dec_ctx, enc_ctx, "contexts must adapt identically");
            }
        }
        assert_eq!(total, SymbolRange::M, "intervals must cover all of M");
    }

    /// Same as [`check_complete_determinism`] but for the [`uneven`] walks
    /// (any `MAX`): intervals must tile `[0, M)` in value order (the search
    /// tree keeps values ordered), every leaf must keep a slot, and both
    /// decode walks must agree with the encode walk for every slot boundary.
    fn check_uneven_determinism<const MAX: usize>(contexts: [BitContext; MAX]) {
        let mut total = 0u32;
        for value in 0..=MAX {
            let mut enc_ctx = contexts;
            let range = uneven::for_value(&mut enc_ctx, value);
            assert!(range.width >= 1, "leaf lost its slot for value {value}");
            assert_eq!(
                range.start, total,
                "intervals must tile [0, M) in value order"
            );
            total += range.width;
            for slot in [range.start, range.start + range.width - 1] {
                let mut dec_ctx = contexts;
                let (dec_range, decoded) = uneven::from_slot(&mut dec_ctx, slot);
                assert_eq!(dec_range, range);
                assert_eq!(decoded, value);
                assert_eq!(dec_ctx, enc_ctx, "contexts must adapt identically");
                // The speculative walk must be indistinguishable from the
                // plain one.
                let mut spec_ctx = contexts;
                let (spec_range, spec_decoded) = uneven::from_slot_prefetching(&mut spec_ctx, slot);
                assert_eq!(spec_range, range);
                assert_eq!(spec_decoded, value);
                assert_eq!(spec_ctx, enc_ctx, "speculative walk must adapt identically");
            }
        }
        assert_eq!(total, SymbolRange::M, "intervals must cover all of M");
    }

    /// A context pushed to its most extreme probability, to force the reserve
    /// clamp in `split_reserving`.
    fn extreme(bit: bool) -> BitContext {
        let mut ctx = BitContext::default();
        for _ in 0..2000 {
            ctx = ctx.adapt(bit);
        }
        ctx
    }

    /// Every state a context can hold must have a `FUSED` entry that agrees
    /// with the `probability()`/`adapt()` tables (random sampling covers all
    /// variants statistically), and the BFS seed must be the default state.
    #[test]
    fn fused_matches_tables() {
        assert_eq!(BitContext::default(), BitContext::True0False0);
        for _ in 0..20_000 {
            let state: BitContext = rand::random();
            let entry = FUSED[state as usize];
            assert_eq!(entry.prob, state.probability());
            assert_eq!(entry.next, [state.adapt(false), state.adapt(true)]);
        }
    }

    #[test]
    fn complete_deterministic_and_lossless() {
        check_complete_determinism::<1>([BitContext::default(); 1]);
        check_complete_determinism::<15>([BitContext::default(); 15]);
        check_complete_determinism::<255>([BitContext::default(); 255]);

        // Adversarially skewed contexts: every node certain-true, certain-false,
        // alternating, and random. The tiling property proves the reserve clamp
        // keeps every leaf decodable.
        check_complete_determinism::<255>([extreme(true); 255]);
        check_complete_determinism::<255>([extreme(false); 255]);
        check_complete_determinism::<127>([extreme(true); 127]);
        let mut alternating = [extreme(false); 255];
        for (i, ctx) in alternating.iter_mut().enumerate() {
            if i % 2 == 0 {
                *ctx = extreme(true);
            }
        }
        check_complete_determinism::<255>(alternating);
        for _ in 0..50 {
            let mut random = [BitContext::default(); 63];
            for ctx in random.iter_mut() {
                *ctx = rand::random();
            }
            check_complete_determinism::<63>(random);
        }
    }

    #[test]
    fn uneven_deterministic_and_lossless() {
        check_uneven_determinism::<0>([]);
        check_uneven_determinism::<1>([BitContext::default(); 1]);
        check_uneven_determinism::<2>([BitContext::default(); 2]);
        check_uneven_determinism::<4>([BitContext::default(); 4]);
        check_uneven_determinism::<5>([BitContext::default(); 5]);
        check_uneven_determinism::<6>([BitContext::default(); 6]);
        check_uneven_determinism::<9>([BitContext::default(); 9]);
        check_uneven_determinism::<254>([BitContext::default(); 254]);
        check_uneven_determinism::<255>([BitContext::default(); 255]);
        check_uneven_determinism::<256>([BitContext::default(); 256]);

        // Adversarially skewed contexts, as in `complete_deterministic_and_lossless`.
        check_uneven_determinism::<256>([extreme(true); 256]);
        check_uneven_determinism::<256>([extreme(false); 256]);
        check_uneven_determinism::<99>([extreme(true); 99]);
        let mut alternating = [extreme(false); 256];
        for (i, ctx) in alternating.iter_mut().enumerate() {
            if i % 2 == 0 {
                *ctx = extreme(true);
            }
        }
        check_uneven_determinism::<256>(alternating);
        for _ in 0..50 {
            let mut random = [BitContext::default(); 76];
            for ctx in random.iter_mut() {
                *ctx = rand::random();
            }
            check_uneven_determinism::<76>(random);
        }
    }

    /// On a power-of-two-count tree with fresh (default, all-balanced)
    /// contexts the two implementations must agree exactly: same interval
    /// for every value. (Their context *indexing* differs, so this only
    /// holds from equal uniform starting states — which is exactly the
    /// fresh-context case.)
    #[test]
    fn complete_and_uneven_agree_on_balanced_trees() {
        fn check<const MAX: usize>() {
            for value in 0..=MAX {
                let mut heap_ctx = [BitContext::default(); MAX];
                let mut split_ctx = [BitContext::default(); MAX];
                assert_eq!(
                    complete::for_value(&mut heap_ctx, value),
                    uneven::for_value(&mut split_ctx, value),
                    "implementations disagree for value {value} of 0..={MAX}"
                );
            }
        }
        check::<1>();
        check::<7>();
        check::<255>();
    }

    #[test]
    fn matches_per_bit_probabilities_when_unclamped() {
        // With the default 50/50 contexts each split is exact, so a symbol's
        // width must be exactly M >> n_bits.
        let mut ctx = [BitContext::default(); 255];
        let range = complete::for_value(&mut ctx, 0x5a);
        assert_eq!(range.width(), SymbolRange::M >> 8);
    }
}
