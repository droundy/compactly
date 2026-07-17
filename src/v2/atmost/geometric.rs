//! Seeding for the hierarchical integer encoding in `src/v2/ints.rs`, which
//! codes a value's *bit length* (`bl`, in `0..=bits`) Elias-delta style: a
//! `blbl = bit_length_of(bl)` symbol through an [`super::AtMost`] tree, then
//! `bl`'s offset within its `blbl` bucket as one more `AtMost` symbol, then
//! the value's own mantissa. The functions here compute compile-time seeds
//! for the `blbl` tree ([`blbl_tree_seeded`]) and the per-bucket offset
//! trees ([`bl_offset_seeded`]) under one of two priors:
//!
//! - `mirror = false`: a uniformly-random `bits`-bit value, where large
//!   magnitudes dominate exponentially (bit length `b` covers `2^(b-1)`
//!   values). A fresh context then costs close to `bits` bits for an
//!   arbitrary/incompressible value, as a plain literal encoding would.
//!   Used by the default `u16`/`u32`/`u64`/`u128` `Encode`.
//! - `mirror = true`: the same weights reversed end-for-end over bit
//!   length, so *tiny* magnitudes dominate exactly as strongly. Used by
//!   `usize`'s default `Encode` (`src/v2/usizes.rs`): real `usize`s are
//!   lengths, counts, and indices, overwhelmingly `0`/`1`/small.
//!
//! `Small` uses the same encoding with no seeding at all (flat
//! [`super::AtMostContext::SEEDED`] tree, 50/50 mantissa bits), which is
//! what makes these priors pure `Default`-overrides on a shared context
//! type rather than separate encodings.

use super::walks::half;
use super::{node_index, seed_context, BitContext};

/// Weight of bit length `b` (in `0..=bits`) under the chosen prior, out of
/// `2^bits` total: `bl = 0` covers the single value `0`, and `bl = b >= 1`
/// covers the `2^(b-1)` values with their top bit at position `b - 1`.
/// `mirror` reverses the weights end-for-end over `0..=bits`, making tiny
/// bit lengths dominant instead of large ones.
const fn bl_weight(bits: usize, mirror: bool, b: usize) -> u128 {
    let b = if mirror { bits - b } else { b };
    if b == 0 {
        1
    } else {
        1u128 << (b - 1)
    }
}

/// Weight of `blbl` leaf `c`: the summed [`bl_weight`] of every bit length
/// whose own bit length is `c`, capped at `bits`. Leaf `0` is `bl = 0`
/// (the value `0`), leaf `c` in `1..blbl_max` covers `bl` in
/// `2^(c-1)..2^c`, and the top leaf `blbl_max` is exactly `bl = bits`
/// (`bits` is a power of two, so it is the only valid bit length with that
/// many bits — the encoding skips the `bl` mantissa there entirely).
const fn blbl_weight(bits: usize, blbl_max: usize, mirror: bool, c: usize) -> u128 {
    if c == 0 {
        bl_weight(bits, mirror, 0)
    } else if c == blbl_max {
        bl_weight(bits, mirror, bits)
    } else {
        let mut sum = 0u128;
        let mut b = 1usize << (c - 1);
        while b < 1usize << c {
            sum += bl_weight(bits, mirror, b);
            b += 1;
        }
        sum
    }
}

/// Sum of [`blbl_weight`] over leaves `start..start + count`. Every range
/// the tree walk ever sums is a *strict* subset of the `blbl_max + 1`
/// leaves (a node's child interval), and each leaf weighs at least 1, so
/// the sum stays at most `2^bits - 1` and cannot overflow the `u128` even
/// at `bits = 128` (where the full-tree total, exactly `2^128`, would).
const fn blbl_weight_range(
    bits: usize,
    blbl_max: usize,
    mirror: bool,
    start: usize,
    count: usize,
) -> u128 {
    let mut sum = 0u128;
    let mut c = start;
    while c < start + count {
        sum += blbl_weight(bits, blbl_max, mirror, c);
        c += 1;
    }
    sum
}

/// Right-shift needed to bring `v` down to around `2^40`, so it and its
/// sibling both fit `seed_context`'s `u64` arithmetic (`seed_err` computes
/// `p*(lo+hi)` and `256*lo`, `p <= 255`) with ample headroom under
/// `u64::MAX` (~`2^64`).
///
/// Computed **per split**, from that split's own `lo`/`hi` magnitude,
/// rather than once from `bits`: weights shrink exponentially toward the
/// non-dominant end of the tree, so a single global shift would round every
/// node at that end to `0/0` and silently collapse the intended bias to
/// `seed_context`'s flat default. At the widest magnitude gaps the smaller
/// side can still round to exactly `0`; that's harmless — the true ratio
/// already exceeds what `seed_context`'s ~8-bit-resolution `BitContext`
/// states can represent, so `0` picks the same maximally-skewed state a
/// tiny nonzero value would.
///
/// `v` is always `>= 1` here — every weight summed by the callers is
/// `>= 1` — so `v.leading_zeros()` is always well-defined.
const fn shift_for_value(v: u128) -> u32 {
    let bit_length = 128 - v.leading_zeros();
    bit_length.saturating_sub(40)
}

/// Seeds for the `blbl` [`super::AtMost`]`<BLBL_MAX>` tree (over the
/// `BLBL_MAX + 1` codes `0..=BLBL_MAX`): the identical stack-based walk as
/// [`super::AtMostContext::SEEDED`] (same `half()` splits, so the exact
/// same tree topology), but each node's seed comes from the prior's
/// relative weight of values on either side of the split instead of the
/// raw leaf count. Each seed must land in the same array slot the walk
/// itself reads for that node — `node_index` (shared with `SEEDED`) picks
/// that slot, covering both the heap-order and split-order layouts.
///
/// `bits` (the integer width this tree describes) is `1 << (BLBL_MAX - 1)`,
/// computed internally: the width is a power of two for every integer type,
/// and its bit length `bits + 1`... rather, the largest possible bit length
/// is exactly `bits`, whose own bit length is `BLBL_MAX`.
pub(crate) const fn blbl_tree_seeded<const BLBL_MAX: usize>(
    mirror: bool,
) -> [BitContext; BLBL_MAX] {
    let bits = 1usize << (BLBL_MAX - 1);
    let mut ctxs = [BitContext::True0False0; BLBL_MAX];
    // The tree over BLBL_MAX + 1 <= 9 leaves is at most 4 deep; 64 slots is
    // far more stack than any walk can use.
    let mut stack = [(0usize, 0usize); 64];
    stack[0] = (0, BLBL_MAX + 1);
    let mut top = 1;
    while top > 0 {
        top -= 1;
        let (start, len) = stack[top];
        if len > 1 {
            let vc = half(len);
            let split = start + vc;
            let lo_full = blbl_weight_range(bits, BLBL_MAX, mirror, start, vc);
            let hi_full = blbl_weight_range(bits, BLBL_MAX, mirror, split, len - vc);
            let shift = shift_for_value(if lo_full > hi_full { lo_full } else { hi_full });
            let lo = (lo_full >> shift) as u64;
            let hi = (hi_full >> shift) as u64;
            ctxs[node_index::<BLBL_MAX>(start, len)] = seed_context(lo, hi);
            stack[top] = (start, vc);
            stack[top + 1] = (split, len - vc);
            top += 2;
        }
    }
    ctxs
}

/// Seeds for one bucket's `bl`-offset [`super::AtMost`]`<MAX>` tree: the
/// bucket holding bit lengths `MAX + 1 .. 2 * (MAX + 1)` (i.e. `blbl == c`
/// where `MAX + 1 == 2^(c-1)`), whose offset `bl - (MAX + 1)` is coded as
/// one complete-tree symbol. Each node is seeded from the prior's relative
/// weight of the bit lengths on either side of its split — the exact
/// conditional at every node, since the tree's per-prefix contexts are
/// indexed the same way.
///
/// A bucket that cannot occur for this width (its smallest bit length
/// `MAX + 1` already reaches `bits`, which the top `blbl` code pins
/// exactly) returns the flat default — which for a power-of-two value
/// count is `AtMostContext`'s seeded default too, so nothing is lost.
pub(crate) const fn bl_offset_seeded<const MAX: usize>(
    bits: usize,
    mirror: bool,
) -> [BitContext; MAX] {
    if MAX + 1 >= bits {
        return [BitContext::True0False0; MAX];
    }
    let mut ctxs = [BitContext::True0False0; MAX];
    // Complete tree over MAX + 1 <= 64 leaves: at most 7 levels; 64 slots
    // of stack is far more than any walk can use.
    let mut stack = [(0usize, 0usize); 64];
    stack[0] = (0, MAX + 1);
    let mut top = 1;
    while top > 0 {
        top -= 1;
        let (start, len) = stack[top];
        if len > 1 {
            let vc = half(len);
            let split = start + vc;
            // Leaf `o` of this tree is bit length `MAX + 1 + o`.
            let mut lo_full = 0u128;
            let mut o = start;
            while o < split {
                lo_full += bl_weight(bits, mirror, MAX + 1 + o);
                o += 1;
            }
            let mut hi_full = 0u128;
            while o < start + len {
                hi_full += bl_weight(bits, mirror, MAX + 1 + o);
                o += 1;
            }
            let shift = shift_for_value(if lo_full > hi_full { lo_full } else { hi_full });
            let lo = (lo_full >> shift) as u64;
            let hi = (hi_full >> shift) as u64;
            ctxs[node_index::<MAX>(start, len)] = seed_context(lo, hi);
            stack[top] = (start, vc);
            stack[top + 1] = (split, len - vc);
            top += 2;
        }
    }
    ctxs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::atmost::AtMostContext;

    #[test]
    fn bl_weights_sum_to_total_range() {
        // `2^128` (the true total for bits=128) doesn't fit in a u128, so
        // the direct check covers the narrower widths; 128 is checked as
        // two halves, mirroring how the walk itself only ever sums strict
        // subsets of the leaves.
        for bits in [16usize, 32, 64] {
            for mirror in [false, true] {
                let mut total = 0u128;
                for b in 0..=bits {
                    total += bl_weight(bits, mirror, b);
                }
                assert_eq!(
                    total,
                    1u128 << bits,
                    "bit-length weights for bits={bits} mirror={mirror} should sum to the total value count"
                );
            }
        }
        for mirror in [false, true] {
            let mut total_low = 0u128;
            for b in 0..=64 {
                total_low += bl_weight(128, mirror, b);
            }
            let mut total_high = 0u128;
            for b in 65..=128 {
                total_high += bl_weight(128, mirror, b);
            }
            // The two halves together are exactly 2^128; check via the
            // complement to avoid forming 2^128 itself.
            assert_eq!(total_low, u128::MAX - total_high + 1);
        }
    }

    #[test]
    fn blbl_weights_partition_bl_weights() {
        // Every bit length lands in exactly one blbl bucket, so the bucket
        // weights must re-sum to the same total as the raw weights.
        for (bits, blbl_max) in [(16usize, 5usize), (32, 6), (64, 7)] {
            for mirror in [false, true] {
                let bucketed = blbl_weight_range(bits, blbl_max, mirror, 0, blbl_max + 1);
                assert_eq!(
                    bucketed,
                    1u128 << bits,
                    "blbl bucket weights for bits={bits} mirror={mirror} should partition the total"
                );
            }
        }
    }

    #[test]
    fn seeds_lean_with_the_prior() {
        // The u64 blbl tree has 8 leaves (a complete heap tree; root at
        // index 0) splitting tiny bit lengths (leaves 0..4, magnitudes
        // < 128) from large ones (leaves 4..8). A uniform-value prior must
        // lean toward the upper half (likely bit true); the mirrored prior
        // toward the lower (likely bit false); and both must differ from
        // the flat leaf-count seed.
        let uniform = blbl_tree_seeded::<7>(false);
        let tiny = blbl_tree_seeded::<7>(true);
        assert!(uniform[0].probability().likely_bit());
        assert!(!tiny[0].probability().likely_bit());
        assert_ne!(uniform[0], AtMostContext::<7>::SEEDED[0]);
        assert_ne!(tiny[0], AtMostContext::<7>::SEEDED[0]);

        // Offset trees: within any bucket, the uniform prior weights the
        // larger bit lengths (upper half of the tree) exponentially more,
        // so every node leans toward true; mirrored, toward false. Check
        // u64's bucket 6 (bit lengths 32..64) at the root and one deep
        // node.
        let uniform_off = bl_offset_seeded::<31>(64, false);
        let tiny_off = bl_offset_seeded::<31>(64, true);
        for node in [0usize, 1, 2, 30] {
            assert!(
                uniform_off[node].probability().likely_bit(),
                "uniform prior must lean toward large bit lengths at node {node}"
            );
            assert!(
                !tiny_off[node].probability().likely_bit(),
                "mirrored prior must lean toward small bit lengths at node {node}"
            );
        }
    }

    #[test]
    fn impossible_bucket_stays_flat() {
        // u64's would-be bucket 7 (bit lengths 64..128) can't occur — bit
        // length 64 is pinned by the top `blbl` code — so its tree must be
        // the flat default (which, for a power-of-two value count, is also
        // `AtMostContext`'s seeded default).
        assert_eq!(
            bl_offset_seeded::<63>(64, false),
            [BitContext::True0False0; 63]
        );
        assert_eq!(
            bl_offset_seeded::<63>(64, true),
            [BitContext::True0False0; 63]
        );
        // For u128 the same tree is a real bucket and must be seeded.
        assert_ne!(
            bl_offset_seeded::<63>(128, false),
            [BitContext::True0False0; 63]
        );
    }
}
