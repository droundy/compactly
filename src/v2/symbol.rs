//! Whole-tree symbol coding.
//!
//! The `u8` / `Bits<N>` / `UBits<N>` codes walk a heap-shaped binary tree of
//! adaptive [`BitContext`]s, historically paying one coder step (one
//! renormalization) per bit. [`SymbolRange`] lets a coder pay a *single* step
//! for the whole `log2(N)`-bit symbol: the tree walk builds one cumulative
//! sub-interval of the fixed total `M = 1 << 16`, touching and adapting exactly
//! the same contexts in the same order as the per-bit walk, and the coder codes
//! that interval in one `encode_symbol`-style operation.
//!
//! The walk itself is coder-independent, so it lives here (written once):
//! [`SymbolRange::for_value`] builds the interval for a known value (encode) and
//! [`SymbolRange::from_slot`] recovers the value from a peeked slot (decode).

use super::ans::Probability;
use super::bit_context::BitContext;

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

const fn fused_entry(state: BitContext) -> FusedContext {
    FusedContext {
        next: [state.adapt(false), state.adapt(true)],
        prob: state.probability(),
    }
}

/// Number of `BitContext` variants; must match the `Count of variants` line at
/// the end of the generated `bit_context.rs`. Too small a value fails at
/// compile time (const-eval bounds check in the BFS below); too large is
/// merely wasteful.
const N_STATES: usize = 675;

/// [`fused_entry`] for every state reachable from `BitContext::default()`,
/// indexed by discriminant. Built by compile-time BFS over `adapt`; every
/// context starts at the default state, so reachable states are exactly the
/// ones a walk can ever load. (Unreachable slots hold the default state's
/// entry and are never read.)
static FUSED: [FusedContext; N_STATES] = {
    let start = BitContext::True0False0;
    let mut table = [fused_entry(start); N_STATES];
    let mut queued = [false; N_STATES];
    let mut queue = [start; N_STATES];
    queued[start as usize] = true;
    let (mut head, mut tail) = (0, 1);
    while head < tail {
        let state = queue[head];
        head += 1;
        let entry = fused_entry(state);
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

/// A sub-interval `[start, start + width)` of the fixed total `M = 1 << BITS`,
/// playing the role for a whole tree symbol that [`Probability`] plays for a
/// single bit.
///
/// Invariant: `width >= 1` and `start + width <= M`, so every representable
/// symbol keeps at least one slot and decoding is lossless by construction for
/// any probability skew (see [`SymbolRange::split`]).
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
    /// probability and the number of tree `levels_below` this node (including
    /// the node's own bit). Each child keeps a reserve of `2^(levels_below-1)`
    /// slots so that every one of its descendant leaves is guaranteed at least
    /// one slot, whatever the probabilities say.
    ///
    /// The reserve is applied by squeezing the probability onto the reduced
    /// width (`width - 2*reserve`) rather than clamping the split, because a
    /// clamp puts two extra compare/select ops on the serial per-level
    /// dependency chain of the (latency-bound) decode walk. The squeeze skews
    /// each split toward the middle by at most a `2^(levels_below-BITS)`
    /// fraction (≤ 2^-9 for a byte tree), a sub-millibit cost per bit.
    #[inline]
    fn split(self, p: Probability, levels_below: u32) -> u32 {
        let reserve = 1u32 << (levels_below - 1);
        debug_assert!(self.width >= 2 * reserve);
        // The product fits in u32: width <= M = 2^16 and prob <= 255.
        (((self.width - 2 * reserve) * p.prob.get() as u32) >> 8) + reserve
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

    /// Walk the heap tree (`node = (node << 1) + 1 + bit`) for `value`,
    /// adapting the contexts exactly as the per-bit walk would, and return the
    /// symbol's interval. `N` must be a power of two and `value < N`.
    #[inline]
    pub(crate) fn for_value<const N: usize>(contexts: &mut [BitContext; N], value: usize) -> Self {
        let mut range = Self::full();
        let n_bits = N.ilog2();
        debug_assert_eq!(1 << n_bits, N);
        debug_assert!(value < N);
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let cur = FUSED[contexts[node] as usize];
            let split = range.split(cur.prob, i + 1);
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
    /// contexts identically to [`Self::for_value`], and return the interval
    /// together with the decoded value in `0..N`.
    #[inline]
    pub(crate) fn from_slot<const N: usize>(
        contexts: &mut [BitContext; N],
        slot: u32,
    ) -> (Self, usize) {
        let mut range = Self::full();
        let n_bits = N.ilog2();
        debug_assert_eq!(1 << n_bits, N);
        debug_assert!(slot < Self::M);
        let mut node = 0usize;
        let mut cur = FUSED[contexts[0] as usize];
        for i in (0..n_bits).rev() {
            let split = range.split(cur.prob, i + 1);
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
        (range, node - (N - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The encode-side and decode-side walks must build identical intervals
    /// and adapt the contexts identically, for every slot of the interval.
    fn check_determinism<const N: usize>(contexts: [BitContext; N]) {
        let mut total = 0u32;
        for value in 0..N {
            let mut enc_ctx = contexts;
            let range = SymbolRange::for_value(&mut enc_ctx, value);
            assert!(range.width >= 1, "leaf lost its slot for value {value}");
            assert_eq!(
                range.start, total,
                "intervals must tile [0, M) in value order"
            );
            total += range.width;
            for slot in [range.start, range.start + range.width - 1] {
                let mut dec_ctx = contexts;
                let (dec_range, decoded) = SymbolRange::from_slot(&mut dec_ctx, slot);
                assert_eq!(dec_range, range);
                assert_eq!(decoded, value);
                assert_eq!(dec_ctx, enc_ctx, "contexts must adapt identically");
            }
        }
        assert_eq!(total, SymbolRange::M, "intervals must cover all of M");
    }

    /// A context pushed to its most extreme probability, to force the reserve
    /// clamp in `split`.
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
    fn deterministic_and_lossless() {
        check_determinism::<2>([BitContext::default(); 2]);
        check_determinism::<16>([BitContext::default(); 16]);
        check_determinism::<256>([BitContext::default(); 256]);

        // Adversarially skewed contexts: every node certain-true, certain-false,
        // alternating, and random. The tiling property proves the reserve clamp
        // keeps every leaf decodable.
        check_determinism::<256>([extreme(true); 256]);
        check_determinism::<256>([extreme(false); 256]);
        check_determinism::<128>([extreme(true); 128]);
        let mut alternating = [extreme(false); 256];
        for (i, ctx) in alternating.iter_mut().enumerate() {
            if i % 2 == 0 {
                *ctx = extreme(true);
            }
        }
        check_determinism::<256>(alternating);
        for _ in 0..50 {
            let mut random = [BitContext::default(); 64];
            for ctx in random.iter_mut() {
                *ctx = rand::random();
            }
            check_determinism::<64>(random);
        }
    }

    #[test]
    fn matches_per_bit_probabilities_when_unclamped() {
        // With the default 50/50 contexts each split is exact, so a symbol's
        // width must be exactly M >> n_bits.
        let mut ctx = [BitContext::default(); 256];
        let range = SymbolRange::for_value(&mut ctx, 0x5a);
        assert_eq!(range.width, SymbolRange::M >> 8);
    }
}
