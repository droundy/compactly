//! Seeding for [`super::AtMostContext`] under a *geometric* prior, matching
//! the true distribution of `leading_zeros()` over a uniformly-random
//! `bits`-bit value, rather than [`super::AtMostContext::SEEDED`]'s flat
//! per-leaf-count prior.
//!
//! Used by the default integer `Encode` (`src/v2/ints.rs`): it reuses
//! `Small`'s exact leading-zero-count encoding, but wants a *fresh* context
//! to cost close to `bits` bits for an arbitrary/incompressible value (as a
//! plain literal encoding would) rather than `Small`'s flat
//! `log2(bits)`-bits-extra cost. That requires seeding each node so its split
//! matches the relative population of `bits`-bit values on either side, not
//! the leaf count.

use super::walks::half;
use super::{seed_context, BitContext};

/// Weight of `AtMost<bits-1>` leaf `i` (i.e. `afewbits_val == i` in
/// `Small`'s encoding) under a uniformly-distributed `bits`-bit magnitude:
///
/// - leaf `0` covers `lz ∈ {0, 1}` (disambiguated separately by
///   `lz_is_one`): weight `2^(bits-1) + 2^(bits-2) = 3·2^(bits-2)`.
/// - leaf `i` for `1 <= i < bits-1` covers `lz == i+1`: weight
///   `2^(bits-2-i)`.
/// - leaf `bits-1` covers `lz == bits` (the single value `0`): weight `1`.
///
/// These sum to exactly `1u128 << bits`, the total count of `bits`-bit
/// values (checked by `weights_sum_to_total_range` below).
const fn leaf_weight(bits: usize, i: usize) -> u128 {
    if i == 0 {
        3u128 << (bits - 2)
    } else if i < bits - 1 {
        1u128 << (bits - 2 - i)
    } else {
        1
    }
}

/// Sum of `leaf_weight(bits, i)` for `i` in `start..start+count`. At most
/// `bits` terms even at `bits = 128`, so a plain loop is cheap at
/// const-eval time and needs no closed form.
const fn weight_range(bits: usize, start: usize, count: usize) -> u128 {
    let mut sum = 0u128;
    let mut i = start;
    while i < start + count {
        sum += leaf_weight(bits, i);
        i += 1;
    }
    sum
}

/// Right-shift needed to bring `v` down to around `2^40`, so it and its
/// sibling both fit `seed_context`'s `u64` arithmetic (`seed_err` computes
/// `p*(lo+hi)` and `256*lo`, `p <= 255`) with ample headroom under
/// `u64::MAX` (~`2^64`).
///
/// This is computed **per split**, from that split's own `lo`/`hi`
/// magnitude, rather than once from `bits`. `leaf_weight` shrinks by
/// roughly half per leaf index, so a shift sized only for the root's
/// dominant leaf-0 weight rounds every deep node (the "many leading
/// zeros"/small-value end of the tree) down to `0`, silently collapsing
/// the intended geometric bias to `seed_context`'s flat default there. A
/// per-node shift preserves the ratio `lo/(lo+hi)` to full `u64`
/// precision at every node, not just the root.
///
/// `v` is always `>= 1` here — every `leaf_weight` is `>= 1`, and
/// `weight_range` sums at least one term whenever `geometric_seeded`'s
/// loop guard (`len > 1`) holds — so `v.leading_zeros()` is always
/// well-defined.
const fn shift_for_value(v: u128) -> u32 {
    let bit_length = 128 - v.leading_zeros();
    bit_length.saturating_sub(40)
}

/// The geometric-prior analogue of [`super::AtMostContext::SEEDED`]: the
/// identical stack-based tree walk (same `half()` splits, so the exact
/// same tree topology and thus fully compatible with `AtMost`'s existing
/// encode/decode/walk machinery), but each node's seed comes from the
/// relative population of `bits`-bit values on either side of the split,
/// instead of the raw leaf count.
///
/// `MAX` here is `AtMost<MAX>`'s own `MAX` (i.e. `$bits - 1`, matching
/// `AtMostContext<MAX>`'s array size) rather than `$bits` itself — `bits`
/// (the actual integer width) is `MAX + 1`, computed internally, to avoid
/// needing `$bits - 1` in a const-generic array-size position, which isn't
/// allowed on stable Rust.
pub(crate) const fn geometric_seeded<const MAX: usize>() -> [BitContext; MAX] {
    let bits = MAX + 1;
    let mut bits_ctx = [BitContext::True0False0; MAX];
    // Same bound as AtMostContext::SEEDED: intervals shrink by at least
    // 1/4 per level, so 192 covers any possible `usize` MAX.
    let mut stack = [(0usize, 0usize); 192];
    stack[0] = (0, bits);
    let mut top = 1;
    while top > 0 {
        top -= 1;
        let (start, len) = stack[top];
        if len > 1 {
            let vc = half(len);
            let split = start + vc;
            let lo_full = weight_range(bits, start, vc);
            let hi_full = weight_range(bits, split, len - vc);
            let shift = shift_for_value(if lo_full > hi_full { lo_full } else { hi_full });
            let lo = (lo_full >> shift) as u64;
            let hi = (hi_full >> shift) as u64;
            bits_ctx[split - 1] = seed_context(lo, hi);
            stack[top] = (start, vc);
            stack[top + 1] = (split, len - vc);
            top += 2;
        }
    }
    bits_ctx
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::atmost::AtMostContext;

    #[test]
    fn weights_sum_to_total_range() {
        // `2^128` (the true total for bits=128) doesn't fit in a u128
        // (max representable is `2^128 - 1`), so this direct check is
        // only meaningful for bits <= 64; bits=128 is covered by
        // `weight_range_bits128_halves_no_overflow` below, since
        // `geometric_seeded` itself never sums a full `0..bits` range in
        // one call (it always recurses into halves first).
        for bits in [16usize, 32, 64] {
            assert_eq!(
                weight_range(bits, 0, bits),
                1u128 << bits,
                "leaf weights for bits={bits} should sum to the total value count"
            );
        }
    }

    #[test]
    fn weight_range_bits128_halves_no_overflow() {
        // The true mathematical total for bits=128 is exactly `2^128`,
        // which doesn't fit in a u128 (max representable is `2^128 - 1`) —
        // so `left + right` would itself overflow if computed directly;
        // that's expected, not a bug (and it's why `geometric_seeded`
        // always shifts before combining halves, never adding raw
        // unshifted weights). Check each half against its closed-form
        // value instead: left = weight(0)=3·2^126 + sum_{i=1}^{63}
        // 2^(126-i) = 2^128 - 2^63; right = sum_{i=64}^{126} 2^(126-i) +
        // weight(127)=1 = 2^63.
        let left = weight_range(128, 0, 64);
        let right = weight_range(128, 64, 64);
        // `2^128 - 2^63` without ever forming `2^128` (which overflows
        // u128) as an intermediate value.
        assert_eq!(left, u128::MAX - (1u128 << 63) + 1);
        assert_eq!(right, 1u128 << 63);
    }

    #[test]
    fn geometric_seeded_differs_from_flat_seed() {
        // Guard against accidentally wiring the flat (Small) seed instead
        // of the geometric one: at the root node, a uniform magnitude
        // heavily favors "no leading zeros," so the geometric seed's root
        // context must differ from the balanced-tree default. The root
        // node for a power-of-two `bits` (= MAX + 1) lives at index
        // bits/2 - 1 (split = half(bits) = bits/2).
        assert_ne!(
            geometric_seeded::<15>()[16 / 2 - 1],
            AtMostContext::<15>::SEEDED[16 / 2 - 1]
        );
        assert_ne!(
            geometric_seeded::<31>()[32 / 2 - 1],
            AtMostContext::<31>::SEEDED[32 / 2 - 1]
        );
        assert_ne!(
            geometric_seeded::<63>()[64 / 2 - 1],
            AtMostContext::<63>::SEEDED[64 / 2 - 1]
        );
        assert_ne!(
            geometric_seeded::<127>()[128 / 2 - 1],
            AtMostContext::<127>::SEEDED[128 / 2 - 1]
        );
    }

    #[test]
    fn deep_nodes_keep_geometric_bias() {
        // Regression test for the precision collapse a single global
        // `shift_for(bits)` used to cause: with the old root-sized shift
        // (22 for bits=64), any node whose weight came from leaves with
        // `i > 40` (`lz > 41`, values below ~2^23) rounded both `lo` and
        // `hi` to 0, so `seed_context(0, 0)` silently fell back to the
        // unbiased `BitContext::True0False0` instead of the intended
        // geometric bias - exactly the region that distinguishes small
        // values from each other. With a per-node shift, deep splits like
        // leaf index 50 (single-leaf weight `2^12 = 4096`, well below the
        // old flat threshold of `2^22`) retain their bias.
        let seeded = geometric_seeded::<63>();
        assert_ne!(seeded[50], BitContext::True0False0);
    }
}
