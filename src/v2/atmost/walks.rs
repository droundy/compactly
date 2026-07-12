//! The tree walks behind [`AtMost<MAX>`](super::AtMost): an implementation
//! detail of `AtMost`, hidden behind the coder traits' `encode_atmost`/
//! `decode_atmost` methods.
//!
//! The `AtMost<MAX>` code (and through it `u8`, whose power-of-two-count
//! tree is the balanced special case) walks a binary search tree of adaptive
//! [`BitContext`]s, historically paying one coder step (one renormalization)
//! per bit. [`SymbolRange`] lets a coder pay a
//! *single* step for the whole symbol: the tree walk builds one cumulative
//! sub-interval of the fixed total `M = 1 << 16`, touching and adapting
//! exactly the same contexts in the same order as the per-bit walk, and the
//! coder codes that interval in one `encode_symbol`-style operation.
//!
//! Every walk is generic over `MAX`, the largest encodable value: the tree
//! covers the `MAX + 1` values `0..=MAX` and has exactly `MAX` internal
//! nodes, so the contexts arrive as a snug `[BitContext; MAX]`.
//!
//! The walks are coder-independent, so they live here (written once). Each
//! tree layout is a complete, cutoff-free implementation in its own module —
//! [`complete`] for a power-of-two value count (heap-ordered contexts, plain
//! and speculating decode variants) and [`uneven`] for arbitrary `MAX`
//! (split-ordered contexts, plain and speculating decode variants) — so each
//! can be tested and benchmarked on its own. [`Walk`] names every concrete
//! (encode, decode) pair; [`Walk::production`] picks the right one from `MAX`
//! at compile time (the only place the cutoffs live), and
//! [`encode_atmost_walk`]/[`decode_atmost_walk`] dispatch to it. The
//! `benches/atmost.rs` shootout drives the same dispatchers with an
//! explicitly forced [`Walk`] to measure the off-diagonal combinations
//! production never takes.
//!
//! The *definition* of the code is the recursive bit-at-a-time
//! `reference_for_value` walk in this module's tests; every production walk
//! below is an unrolled, fused, or speculating restatement of it, and the
//! tests hold them all to bit-identical behavior (same intervals, same
//! context adaptation).
//!
//! # Walk inventory
//!
//! | [`Walk`] variant | Used by | Schedule | Why (measured) |
//! |------|---------|----------|----------------|
//! | [`Walk::Complete`] ([`complete::for_value`] / [`complete::from_slot`]) | shootout only | plain | measures the heap-speculation assumption below against a real baseline |
//! | [`Walk::CompleteSpeculating`] ([`complete::for_value`] / [`complete::from_slot_speculating`]) | `Ans` *and* `Range` decode, power-of-two value count | speculating | heap indexing makes child indexes independent of the bit, so speculation is nearly free; ~8–11% on `u8`-heavy string decode |
//! | [`Walk::Uneven`] ([`uneven::for_value`] / [`uneven::from_slot`]) | `Ans` decode; `Range` below [`SPECULATE_MIN_MAX`] | plain | `Ans`'s lean symbol step leaves speculative work exposed: +4…+22% slower at *every* value count |
//! | [`Walk::UnevenSpeculating`] ([`uneven::for_value`] / [`uneven::from_slot_speculating`]) | `Range` decode at `MAX >= SPECULATE_MIN_MAX` | speculating | `Range`'s u64-division latency shadow absorbs the ~2x instructions: −4…−17% for every value count ≥ 4 |
//! | [`Walk::CompleteBitwise`] / [`Walk::UnevenBitwise`] ([`complete`]/[`uneven`] `encode_bitwise`/`decode_bitwise`) | `Raw` always; symbol coders when the value count exceeds `M` | one coder step per bit | the historical per-bit code — and `Raw`'s bit-packed format |

use super::super::bit_context::BitContext;
use super::super::model::{SymbolCoder, SymbolDecoder, SymbolRange};
use super::super::{EntropyCoder, EntropyDecoder};
use super::{AtMost, AtMostContext};

/// The whole-symbol `encode_atmost` shared by every [`SymbolCoder`]: the
/// symbol/bitwise guards live here, exactly once. A single-valued
/// `AtMost<0>` costs nothing; a value count beyond [`SymbolRange::M`] cannot
/// give every leaf a slot, so it falls back to the per-bit walk; everything
/// else is one tree walk plus one [`SymbolCoder::encode_symbol`] step.
///
/// `inline(always)`: this is a thin dispatch layer, and the walk must fuse
/// into the coder's symbol step (measured +13% instructions on `AtMost<7>`
/// decode when the compiler outlined the equivalent decode layer).
#[inline(always)]
pub(crate) fn encode_symbol_or_bitwise<C: SymbolCoder, const MAX: usize>(
    coder: &mut C,
    ctx: &mut AtMostContext<MAX>,
    value: AtMost<MAX>,
) {
    let value = usize::from(value);
    if let Some(walk) = Walk::production::<MAX>(false) {
        encode_atmost_walk(walk, coder, ctx, value);
    }
    // MAX == 0: nothing to code, the one possible value carries no information.
}

/// The decode side of [`encode_symbol_or_bitwise`], with the same guards;
/// the walk comes from [`Walk::production`] using the coder's measured
/// [`SymbolDecoder::SPECULATES`] policy (const, so the branch folds away per
/// coder). `inline(always)` as on the encode side.
#[inline(always)]
pub(crate) fn decode_symbol_or_bitwise<D: SymbolDecoder, const MAX: usize>(
    reader: &mut D,
    ctx: &mut AtMostContext<MAX>,
) -> AtMost<MAX> {
    let value = match Walk::production::<MAX>(D::SPECULATES) {
        Some(walk) => decode_atmost_walk(walk, reader, ctx),
        None => 0,
    };
    // Every walk returns a value in 0..=MAX by construction.
    debug_assert!(value <= MAX);
    AtMost(value)
}

/// Where the [`AtMost`](super::AtMost) binary-search tree cuts an
/// interval of `i` possible values: the largest power of two below `i` (so
/// every walk in [`uneven`] agrees on the shape).
#[inline]
pub(super) const fn half(i: usize) -> usize {
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
const fn tree_depth(len: usize) -> u32 {
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

/// From this `MAX` up, [`Walk::production`] resolves to
/// [`Walk::UnevenSpeculating`] instead of the plain [`Walk::Uneven`] (a
/// power-of-two value count always takes [`Walk::CompleteSpeculating`],
/// whose heap layout makes speculation nearly free). Only `Range` asks for
/// the speculating walk: its symbol step
/// carries a u64 division whose latency shadow absorbs the speculation's ~2x
/// instruction count (measured −4…−17% for every value count ≥ 4, +11% at
/// 3); `Ans`'s lean symbol step leaves that work exposed and measures
/// *slower at every value count* (+4…+22%), so it always takes the plain
/// walk. Swept with `just-decompress-uless` over value counts 3…128; table
/// in OPTIMIZING.md.
const SPECULATE_MIN_MAX: usize = 3;

/// Every concrete tree-walk implementation, named for the shootout benchmark
/// (`benches/atmost.rs`) and for [`Walk::production`]'s dispatch. Each
/// variant pairs one encode walk with one bit-identical decode walk (proved
/// by the tests below), so using a single `Walk` for both sides of a round
/// trip is always correct — independent of which coder or `MAX` would
/// normally choose it.
#[doc(hidden)] // benchmark-support surface; not part of the stable API
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Walk {
    /// [`complete::for_value`] / [`complete::from_slot`]: heap tree, decode
    /// loads only the child the bit selects. Not used by production (which
    /// always speculates on the heap tree — see [`Walk::CompleteSpeculating`]);
    /// included so the shootout can measure that choice against a baseline.
    Complete,
    /// [`complete::for_value`] / [`complete::from_slot_speculating`]: heap
    /// tree, decode fetches both children before the bit resolves. What
    /// [`Walk::production`] picks for every power-of-two value count, on
    /// both coders.
    CompleteSpeculating,
    /// [`uneven::for_value`] / [`uneven::from_slot`]: split-ordered tree,
    /// plain decode.
    Uneven,
    /// [`uneven::for_value`] / [`uneven::from_slot_speculating`]: split
    /// tree, decode fetches both children before the bit resolves.
    UnevenSpeculating,
    /// [`complete::encode_bitwise`] / [`complete::decode_bitwise`]: one
    /// coder step per bit, heap indexing. `Raw`'s format, and the symbol
    /// coders' fallback once a power-of-two value count exceeds
    /// [`SymbolRange::M`].
    CompleteBitwise,
    /// [`uneven::encode_bitwise`] / [`uneven::decode_bitwise`]: one coder
    /// step per bit, split indexing. Same role as [`Walk::CompleteBitwise`]
    /// for a non-power-of-two value count.
    UnevenBitwise,
}

impl Walk {
    /// The walk [`encode_symbol_or_bitwise`]/[`decode_symbol_or_bitwise`]
    /// use for a value count of `MAX + 1`, given whether the coder wants to
    /// speculate on a non-power-of-two count (see
    /// [`SymbolDecoder::SPECULATES`]; encode callers pass `false`, since
    /// every encode walk is shared by its plain/speculating decode twins).
    /// `None` means `MAX == 0`: the single possible value carries no
    /// information, so nothing is coded.
    ///
    /// `inline(always)`: called with const-evaluable arguments from the
    /// `inline(always)` dispatch layer, so it must fold to a single constant
    /// at every call site — see the module doc's cutoff note.
    #[inline(always)]
    #[doc(hidden)]
    pub const fn production<const MAX: usize>(speculate: bool) -> Option<Walk> {
        if MAX == 0 {
            None
        } else if MAX >= SymbolRange::M as usize {
            Some(if (MAX + 1).is_power_of_two() {
                Walk::CompleteBitwise
            } else {
                Walk::UnevenBitwise
            })
        } else if (MAX + 1).is_power_of_two() {
            // Heap-layout speculation is nearly free, so both coders always
            // take it (see `CompleteSpeculating`'s docs).
            Some(Walk::CompleteSpeculating)
        } else if speculate && MAX >= SPECULATE_MIN_MAX {
            Some(Walk::UnevenSpeculating)
        } else {
            Some(Walk::Uneven)
        }
    }

    /// Whether this walk is a valid implementation for value count `MAX + 1`
    /// — used by the shootout benchmark to skip nonsensical combinations
    /// (e.g. [`Walk::Complete`] when `MAX + 1` isn't a power of two).
    #[doc(hidden)]
    pub const fn applies_to<const MAX: usize>(self) -> bool {
        match self {
            Walk::Complete | Walk::CompleteSpeculating => {
                (MAX + 1).is_power_of_two() && MAX < SymbolRange::M as usize
            }
            Walk::Uneven | Walk::UnevenSpeculating => MAX < SymbolRange::M as usize,
            Walk::CompleteBitwise => (MAX + 1).is_power_of_two(),
            Walk::UnevenBitwise => true,
        }
    }

    /// The walk whose *encode* implementation this walk shares: a
    /// speculating variant differs from its plain twin only on decode ([`Walk::production`]
    /// never speculates on encode — see its `speculate` doc), so
    /// [`encode_atmost_walk`] only has four distinct bodies, not six.
    /// [`Walk::Complete`]/[`Walk::Uneven`]/the bitwise walks map to
    /// themselves. Used by the shootout benchmark to avoid timing the same
    /// encode code twice under two different walk names.
    #[doc(hidden)]
    pub const fn encode_with(self) -> Walk {
        match self {
            Walk::CompleteSpeculating => Walk::Complete,
            Walk::UnevenSpeculating => Walk::Uneven,
            other => other,
        }
    }
}

/// Every [`Walk`] variant, for the shootout benchmark to iterate. The
/// benchmark indexes this by a `const WHICH_WALK: usize` generic (see
/// `encode_atmost_batch`/`decode_atmost_batch`) so each walk monomorphizes to
/// its own branch-free function, exactly like production's
/// [`Walk::production`]-selected constant.
#[doc(hidden)]
pub const WALKS: [Walk; 6] = [
    Walk::Complete,
    Walk::CompleteSpeculating,
    Walk::Uneven,
    Walk::UnevenSpeculating,
    Walk::CompleteBitwise,
    Walk::UnevenBitwise,
];

/// Code one symbol using an explicitly chosen [`Walk`]. Production calls
/// this with the constant [`Walk::production`] resolves for `MAX` (folding
/// away every branch below but one — see the module doc's `inline(always)`
/// note); the shootout benchmark calls it with a `const WHICH_WALK`-derived
/// constant from [`WALKS`] for the same effect. Dispatches on
/// [`Walk::encode_with`], since a speculating walk shares its plain twin's
/// encode body; the `unreachable!` arms are just `encode_with`'s codomain
/// made explicit, so they always fold away at the (const) call sites here.
#[inline(always)]
pub(crate) fn encode_atmost_walk<C: SymbolCoder, const MAX: usize>(
    walk: Walk,
    coder: &mut C,
    ctx: &mut AtMostContext<MAX>,
    value: usize,
) {
    match walk.encode_with() {
        Walk::Complete => coder.encode_symbol(complete::for_value(&mut ctx.bits, value)),
        Walk::Uneven => coder.encode_symbol(uneven::for_value(&mut ctx.bits, value)),
        Walk::CompleteBitwise => complete::encode_bitwise(coder, &mut ctx.bits, value),
        Walk::UnevenBitwise => uneven::encode_bitwise(coder, &mut ctx.bits, value),
        Walk::CompleteSpeculating | Walk::UnevenSpeculating => {
            unreachable!("Walk::encode_with never returns a speculating variant")
        }
    }
}

/// The decode side of [`encode_atmost_walk`].
#[inline(always)]
pub(crate) fn decode_atmost_walk<D: SymbolDecoder, const MAX: usize>(
    walk: Walk,
    reader: &mut D,
    ctx: &mut AtMostContext<MAX>,
) -> usize {
    let contexts = &mut ctx.bits;
    match walk {
        Walk::Complete => reader.decode_symbol_step(|slot| complete::from_slot(contexts, slot)),
        Walk::CompleteSpeculating => {
            reader.decode_symbol_step(|slot| complete::from_slot_speculating(contexts, slot))
        }
        Walk::Uneven => reader.decode_symbol_step(|slot| uneven::from_slot(contexts, slot)),
        Walk::UnevenSpeculating => {
            reader.decode_symbol_step(|slot| uneven::from_slot_speculating(contexts, slot))
        }
        Walk::CompleteBitwise => complete::decode_bitwise(reader, contexts),
        Walk::UnevenBitwise => uneven::decode_bitwise(reader, contexts),
    }
}

/// Encode every value in `values` with an explicitly forced walk, sharing
/// one adaptive [`AtMostContext`] across the whole batch exactly as a real
/// `Vec<AtMost<MAX>>` encode would. `WHICH_WALK` indexes [`WALKS`] as a
/// `const` generic (rather than taking a runtime [`Walk`]) so the walk is a
/// compile-time constant at every call site, folding
/// [`encode_atmost_walk`]'s dispatch to a single branch-free path — the
/// benchmark-side analog of [`Walk::production`]'s constant folding. Backs
/// the `#[doc(hidden)]` `encode_atmost_batch` methods on `Range` and `Ans`
/// that `benches/atmost.rs` calls.
pub(crate) fn encode_atmost_batch<C: SymbolCoder, const MAX: usize, const WHICH_WALK: usize>(
    mut coder: C,
    values: &[AtMost<MAX>],
) -> C {
    let walk = const { WALKS[WHICH_WALK] };
    let mut ctx = AtMostContext::<MAX>::default();
    for &value in values {
        encode_atmost_walk(walk, &mut coder, &mut ctx, usize::from(value));
    }
    coder
}

/// The decode side of [`encode_atmost_batch`]: decode `n` values with an
/// explicitly forced walk, sharing one adaptive context across the batch.
pub(crate) fn decode_atmost_batch<D: SymbolDecoder, const MAX: usize, const WHICH_WALK: usize>(
    mut reader: D,
    n: usize,
) -> Vec<AtMost<MAX>> {
    let walk = const { WALKS[WHICH_WALK] };
    let mut ctx = AtMostContext::<MAX>::default();
    (0..n)
        .map(|_| AtMost::new(decode_atmost_walk(walk, &mut reader, &mut ctx)))
        .collect()
}

/// Code one symbol bit-by-bit, picking the layout from `MAX` at compile
/// time: `Raw`'s `EntropyCoder::encode_atmost`/`decode_atmost` default
/// implementations use this directly, since `Raw` does not implement
/// [`SymbolCoder`] and so cannot go through [`encode_atmost_walk`]'s
/// [`Walk`]-based dispatch.
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

/// The balanced-tree implementation for a **power-of-two value count**
/// (`MAX + 1` a power of two): a complete binary tree with contexts stored
/// in heap order (`node = (node << 1) + 1 + bit`), the layout the `u8` hot
/// path relies on. A child's index depends only on the parent's — not on the
/// current interval — so the decode walk fetches both children's
/// [`BitModel`](super::super::model::BitModel) entries before the node's bit
/// resolves at no extra index arithmetic, hiding the load latency that
/// otherwise dominates the serial per-level chain (measured worth ~8-11% on
/// the `u8`-heavy string decode path for both coders versus the
/// split-ordered walks in [`uneven`]).
mod complete {
    use super::*;

    /// Walk the heap tree for `value`, adapting the contexts exactly as the
    /// per-bit walk would, and return the symbol's interval. `MAX + 1` must
    /// be a power of two and `value <= MAX`.
    #[inline]
    pub(super) fn for_value<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) -> SymbolRange {
        let mut range = SymbolRange::full();
        let n_bits = (MAX + 1).ilog2();
        debug_assert_eq!(1 << n_bits, MAX + 1);
        debug_assert!(value <= MAX);
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let cur = contexts[node].model();
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
    ///
    /// This is the plain walk: each level's model load waits for the
    /// previous level's bit, unlike [`from_slot_speculating`]. Production
    /// never picks this ([`Walk::production`] always speculates on a
    /// power-of-two value count); it exists for the shootout benchmark to
    /// measure that choice against a baseline.
    #[inline]
    pub(super) fn from_slot<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        slot: u32,
    ) -> (SymbolRange, usize) {
        let mut range = SymbolRange::full();
        let n_bits = (MAX + 1).ilog2();
        debug_assert_eq!(1 << n_bits, MAX + 1);
        debug_assert!(slot < SymbolRange::M);
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let cur = contexts[node].model();
            let reserve = 1u32 << i;
            let split = range.split_reserving(cur.prob, reserve, reserve);
            let lower = range.lower(split);
            let bit = !lower.contains(slot);
            contexts[node] = cur.next[bit as usize];
            range = if bit { range.upper(split) } else { lower };
            node = (node << 1) + 1 + bit as usize;
        }
        (range, node - MAX)
    }

    /// The speculative variant of [`from_slot`]: both children's model
    /// entries are fetched a level ahead of the serial split/compare chain
    /// (their heap indices depend only on `node`, not on the bit), leaving
    /// just a select on the critical path. What [`Walk::production`] uses
    /// for every power-of-two value count, on both coders — see the module
    /// doc's walk inventory.
    #[inline]
    pub(super) fn from_slot_speculating<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        slot: u32,
    ) -> (SymbolRange, usize) {
        let mut range = SymbolRange::full();
        if MAX == 0 {
            return (range, 0);
        }
        let n_bits = (MAX + 1).ilog2();
        debug_assert_eq!(1 << n_bits, MAX + 1);
        debug_assert!(slot < SymbolRange::M);
        let mut node = 0usize;
        let mut cur = contexts[0].model();
        for i in (0..n_bits).rev() {
            let reserve = 1u32 << i;
            let split = range.split_reserving(cur.prob, reserve, reserve);
            let lower = range.lower(split);
            let bit = !lower.contains(slot);
            let adapted = cur.next[bit as usize];
            if i > 0 {
                // Speculatively fetch both children's model entries: these
                // loads depend only on `node`, not on `bit`, so they issue a
                // full level ahead of the serial split/compare chain, leaving
                // just a select on the critical path.
                let left = contexts[2 * node + 1].model();
                let right = contexts[2 * node + 2].model();
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
    pub(super) fn encode_bitwise<E: EntropyCoder, const MAX: usize>(
        writer: &mut E,
        contexts: &mut [BitContext; MAX],
        value: usize,
    ) {
        debug_assert!(value <= MAX);
        let n_bits = (MAX + 1).ilog2();
        let mut node = 0usize;
        for i in (0..n_bits).rev() {
            let bit = (value >> i) & 1 == 1;
            writer.encode_bit(&mut contexts[node], bit);
            node = (node << 1) + 1 + bit as usize;
        }
    }

    /// The per-bit inverse of [`encode_bitwise`].
    #[inline]
    pub(super) fn decode_bitwise<D: EntropyDecoder, const MAX: usize>(
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
mod uneven {
    use super::*;

    /// Walk the search tree for `value`, adapting the contexts exactly as
    /// the per-bit walk would, and return the symbol's interval. Requires
    /// `MAX + 1 <= M` so every leaf can reserve a slot.
    #[inline]
    pub(super) fn for_value<const MAX: usize>(
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
            let cur = contexts[split - 1].model();
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
    /// This is the plain walk: each level's model load waits for the
    /// previous level's bit. The [`from_slot_speculating`] variant hides
    /// that latency but roughly doubles the instruction count (both
    /// children's [`half`] index arithmetic, double model loads,
    /// register-pressure spills), which only `Range`'s division-heavy
    /// symbol step can absorb — see [`SPECULATE_MIN_MAX`].
    #[inline]
    pub(super) fn from_slot<const MAX: usize>(
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
            let cur = contexts[split - 1].model();
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
    /// depend only on the current interval, not the bit), so the model
    /// loads issue a level ahead of the serial split/compare chain, leaving
    /// just a select on the critical path. A leaf child gets a harmless
    /// dummy index 0 — if the walk descends into it the loop ends and the
    /// fetched entry is unused.
    #[inline]
    pub(super) fn from_slot_speculating<const MAX: usize>(
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
        let mut cur = contexts[split - 1].model();
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
            let lo_cur = contexts[if lo_len > 1 { lo_split - 1 } else { 0 }].model();
            let hi_cur = contexts[if hi_len > 1 { hi_split - 1 } else { 0 }].model();
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
    pub(super) fn encode_bitwise<E: EntropyCoder, const MAX: usize>(
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
            writer.encode_bit(&mut contexts[split - 1], bit);
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
    pub(super) fn decode_bitwise<D: EntropyDecoder, const MAX: usize>(
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The *definition* of `AtMost` coding, in its original bit-at-a-time
    /// form: recursively split the interval of possible values at [`half`],
    /// code one adaptive bit per split using the plain generated
    /// `probability()`/`adapt()` tables (independent of the fused
    /// `BitContext::model` path the production walks take), and narrow the
    /// slot interval with `split_reserving` exactly as a per-bit coder
    /// would. Every production walk must match it exactly — in interval and
    /// in post-walk contexts. `index_of(start, len)` names the context of
    /// the node covering values `start..start + len`, the only thing the
    /// two layouts disagree on.
    fn reference_for_value<const MAX: usize>(
        contexts: &mut [BitContext; MAX],
        index_of: impl Fn(usize, usize) -> usize + Copy,
        start: usize,
        len: usize,
        range: SymbolRange,
        value: usize,
    ) -> SymbolRange {
        if len <= 1 {
            return range;
        }
        let vc = half(len);
        let node = index_of(start, len);
        let split =
            range.split_reserving(contexts[node].probability(), vc as u32, (len - vc) as u32);
        let bit = value >= start + vc;
        contexts[node] = contexts[node].adapt(bit);
        if bit {
            reference_for_value(
                contexts,
                index_of,
                start + vc,
                len - vc,
                range.upper(split),
                value,
            )
        } else {
            reference_for_value(contexts, index_of, start, vc, range.lower(split), value)
        }
    }

    /// [`complete`]'s heap index for the node covering `start..start + len`
    /// of a complete tree over `MAX + 1` leaves.
    fn heap_index<const MAX: usize>(start: usize, len: usize) -> usize {
        (MAX + 1) / len - 1 + start / len
    }

    /// [`uneven`]'s split-order index for the node covering
    /// `start..start + len`: the cut at `split` lives at `split - 1`.
    fn split_index(start: usize, len: usize) -> usize {
        start + half(len) - 1
    }

    /// The encode-side and decode-side [`complete`] walks (power-of-two
    /// value count) must match the bit-at-a-time reference definition and
    /// each other: identical intervals, identical context adaptation, for
    /// every slot of the interval.
    fn check_complete_determinism<const MAX: usize>(contexts: [BitContext; MAX]) {
        let mut total = 0u32;
        for value in 0..=MAX {
            let mut ref_ctx = contexts;
            let range = reference_for_value(
                &mut ref_ctx,
                heap_index::<MAX>,
                0,
                MAX + 1,
                SymbolRange::full(),
                value,
            );
            let mut enc_ctx = contexts;
            assert_eq!(
                complete::for_value(&mut enc_ctx, value),
                range,
                "interval must match the bit-at-a-time reference for value {value}"
            );
            assert_eq!(enc_ctx, ref_ctx, "contexts must adapt like the reference");
            assert!(range.width() >= 1, "leaf lost its slot for value {value}");
            assert_eq!(
                range.start(),
                total,
                "intervals must tile [0, M) in value order"
            );
            total += range.width();
            for slot in [range.start(), range.start() + range.width() - 1] {
                let mut dec_ctx = contexts;
                let (dec_range, decoded) = complete::from_slot(&mut dec_ctx, slot);
                assert_eq!(dec_range, range);
                assert_eq!(decoded, value);
                assert_eq!(dec_ctx, enc_ctx, "contexts must adapt identically");
                // The speculating walk must be indistinguishable from the
                // plain one.
                let mut spec_ctx = contexts;
                let (spec_range, spec_decoded) =
                    complete::from_slot_speculating(&mut spec_ctx, slot);
                assert_eq!(spec_range, range);
                assert_eq!(spec_decoded, value);
                assert_eq!(spec_ctx, enc_ctx, "speculating walk must adapt identically");
            }
        }
        assert_eq!(total, SymbolRange::M, "intervals must cover all of M");
    }

    /// Same as [`check_complete_determinism`] but for the [`uneven`] walks
    /// (any `MAX`): every walk must match the bit-at-a-time reference,
    /// intervals must tile `[0, M)` in value order (the search tree keeps
    /// values ordered), every leaf must keep a slot, and both decode walks
    /// must agree with the encode walk for every slot boundary.
    fn check_uneven_determinism<const MAX: usize>(contexts: [BitContext; MAX]) {
        let mut total = 0u32;
        for value in 0..=MAX {
            let mut ref_ctx = contexts;
            let range = reference_for_value(
                &mut ref_ctx,
                split_index,
                0,
                MAX + 1,
                SymbolRange::full(),
                value,
            );
            let mut enc_ctx = contexts;
            assert_eq!(
                uneven::for_value(&mut enc_ctx, value),
                range,
                "interval must match the bit-at-a-time reference for value {value}"
            );
            assert_eq!(enc_ctx, ref_ctx, "contexts must adapt like the reference");
            assert!(range.width() >= 1, "leaf lost its slot for value {value}");
            assert_eq!(
                range.start(),
                total,
                "intervals must tile [0, M) in value order"
            );
            total += range.width();
            for slot in [range.start(), range.start() + range.width() - 1] {
                let mut dec_ctx = contexts;
                let (dec_range, decoded) = uneven::from_slot(&mut dec_ctx, slot);
                assert_eq!(dec_range, range);
                assert_eq!(decoded, value);
                assert_eq!(dec_ctx, enc_ctx, "contexts must adapt identically");
                // The speculating walk must be indistinguishable from the
                // plain one.
                let mut spec_ctx = contexts;
                let (spec_range, spec_decoded) = uneven::from_slot_speculating(&mut spec_ctx, slot);
                assert_eq!(spec_range, range);
                assert_eq!(spec_decoded, value);
                assert_eq!(spec_ctx, enc_ctx, "speculating walk must adapt identically");
            }
        }
        assert_eq!(total, SymbolRange::M, "intervals must cover all of M");
    }

    /// The per-bit walks must adapt the contexts exactly like the reference
    /// (this is the invariant that lets the symbol coders and the bitwise
    /// fallback share one context array), and must round-trip through a real
    /// coder.
    fn check_bitwise_matches_reference<const MAX: usize>(contexts: [BitContext; MAX]) {
        let index_of = |start: usize, len: usize| {
            if (MAX + 1).is_power_of_two() {
                heap_index::<MAX>(start, len)
            } else {
                split_index(start, len)
            }
        };
        for value in 0..=MAX {
            let mut ref_ctx = contexts;
            reference_for_value(
                &mut ref_ctx,
                index_of,
                0,
                MAX + 1,
                SymbolRange::full(),
                value,
            );
            let mut enc_ctx = contexts;
            let mut coder = crate::v2::Range::default();
            encode_bitwise(&mut coder, &mut enc_ctx, value);
            assert_eq!(
                enc_ctx, ref_ctx,
                "bitwise encode must adapt like the reference for value {value}"
            );
            let bytes = coder.into_vec();
            let mut decoder = crate::v2::arith::Decoder::new(&bytes);
            let mut dec_ctx = contexts;
            let decoded = decode_bitwise(&mut decoder, &mut dec_ctx);
            assert_eq!(decoded, value);
            assert_eq!(
                dec_ctx, ref_ctx,
                "bitwise decode must adapt like the reference for value {value}"
            );
        }
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

    #[test]
    fn complete_deterministic_and_lossless() {
        check_complete_determinism::<0>([]);
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

    #[test]
    fn bitwise_matches_reference() {
        check_bitwise_matches_reference::<0>([]);
        check_bitwise_matches_reference::<1>([BitContext::default(); 1]);
        check_bitwise_matches_reference::<5>([BitContext::default(); 5]);
        check_bitwise_matches_reference::<255>([BitContext::default(); 255]);
        check_bitwise_matches_reference::<256>([BitContext::default(); 256]);
        check_bitwise_matches_reference::<255>([extreme(true); 255]);
        check_bitwise_matches_reference::<99>([extreme(false); 99]);
        for _ in 0..20 {
            let mut random = [BitContext::default(); 76];
            for ctx in random.iter_mut() {
                *ctx = rand::random();
            }
            check_bitwise_matches_reference::<76>(random);
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

    /// Every [`Walk`] applicable to a given `MAX` must round-trip every
    /// value through a real coder — the invariant the shootout benchmark
    /// leans on to force a walk that production would never pick for that
    /// `MAX`.
    fn check_walk_round_trips<const MAX: usize>() {
        for &walk in WALKS.iter() {
            if !walk.applies_to::<MAX>() {
                continue;
            }
            for value in 0..=MAX {
                let mut enc_ctx = AtMostContext::<MAX>::default();
                let mut coder = crate::v2::Range::default();
                encode_atmost_walk(walk, &mut coder, &mut enc_ctx, value);
                let bytes = coder.into_vec();
                let mut decoder = crate::v2::arith::Decoder::new(&bytes);
                let mut dec_ctx = AtMostContext::<MAX>::default();
                let decoded = decode_atmost_walk(walk, &mut decoder, &mut dec_ctx);
                assert_eq!(
                    decoded, value,
                    "{walk:?} failed to round-trip {value} of 0..={MAX}"
                );
            }
        }
    }

    #[test]
    fn every_applicable_walk_round_trips() {
        check_walk_round_trips::<0>();
        check_walk_round_trips::<1>();
        check_walk_round_trips::<2>();
        check_walk_round_trips::<3>();
        check_walk_round_trips::<7>();
        check_walk_round_trips::<8>();
        check_walk_round_trips::<9>();
        check_walk_round_trips::<255>();
        check_walk_round_trips::<256>();
    }

    /// [`check_walk_round_trips`] drives everything through `Range`, so it
    /// never exercises `Ans`'s decode path or the public
    /// `encode_atmost_batch`/`decode_atmost_batch` methods the shootout
    /// benchmark uses (previously only covered by the benchmark itself,
    /// which doesn't run in CI). This round-trips a whole batch — forward
    /// then reversed, so the adaptive contexts see real movement — through
    /// both coders' batch methods for every applicable walk.
    fn check_batch_round_trips<const MAX: usize, const WHICH_WALK: usize>() {
        if !WALKS[WHICH_WALK].applies_to::<MAX>() {
            return;
        }
        let values: Vec<AtMost<MAX>> = (0..=MAX).chain((0..=MAX).rev()).map(AtMost::new).collect();

        let ans_bytes = crate::v2::Ans::encode_atmost_batch::<MAX, WHICH_WALK>(&values);
        let ans_decoded =
            crate::v2::Ans::decode_atmost_batch::<MAX, WHICH_WALK>(&ans_bytes, values.len());
        assert_eq!(
            ans_decoded, values,
            "Ans batch round-trip failed for MAX={MAX}, {:?}",
            WALKS[WHICH_WALK]
        );

        let range_bytes = crate::v2::Range::encode_atmost_batch::<MAX, WHICH_WALK>(&values);
        let range_decoded =
            crate::v2::Range::decode_atmost_batch::<MAX, WHICH_WALK>(&range_bytes, values.len());
        assert_eq!(
            range_decoded, values,
            "Range batch round-trip failed for MAX={MAX}, {:?}",
            WALKS[WHICH_WALK]
        );
    }

    #[test]
    fn every_applicable_walk_batch_round_trips() {
        macro_rules! check_all_walks {
            ($max:expr) => {
                check_batch_round_trips::<$max, 0>();
                check_batch_round_trips::<$max, 1>();
                check_batch_round_trips::<$max, 2>();
                check_batch_round_trips::<$max, 3>();
                check_batch_round_trips::<$max, 4>();
                check_batch_round_trips::<$max, 5>();
            };
        }
        check_all_walks!(0);
        check_all_walks!(1);
        check_all_walks!(2);
        check_all_walks!(3);
        check_all_walks!(7);
        check_all_walks!(8);
        check_all_walks!(9);
        check_all_walks!(255);
        check_all_walks!(256);
    }

    #[test]
    fn production_matches_walks_array() {
        // `Walk::production` must always name a walk that `applies_to` the
        // `MAX` it was resolved for, on both the plain and speculating
        // policies -- otherwise `WALKS` and `Walk::production` could silently
        // diverge.
        fn check<const MAX: usize>() {
            for speculate in [false, true] {
                if let Some(walk) = Walk::production::<MAX>(speculate) {
                    assert!(
                        WALKS.contains(&walk),
                        "{walk:?} missing from WALKS for MAX={MAX}"
                    );
                    assert!(
                        walk.applies_to::<MAX>(),
                        "{walk:?} does not apply_to MAX={MAX}"
                    );
                }
            }
        }
        check::<0>();
        check::<1>();
        check::<2>();
        check::<3>();
        check::<7>();
        check::<8>();
        check::<9>();
        check::<255>();
        check::<256>();
    }
}
