//! Does any single value ever consume more stream than its `MAX_BYTES` promises?
//!
//! The async decoder hands a value to the *sync* decoder whenever its bound
//! fits in the bytes already buffered. An understated bound therefore decodes
//! past the end of the buffer, which does not fail loudly — it zero-pads and
//! returns a plausible wrong value, and leaves the stream position wrong for
//! everything after. So the bounds need checking, not just deriving.
//!
//! [`worst_bytes`] measures the real thing: how far one value's decode advances
//! the cursor. Values are coded back to back through **one shared context**, so
//! every value after the first is measured with *adapted* contexts — which is
//! where the worst case lives, since a well-skewed context makes the unlikely
//! branch expensive. A run of one value followed by extremes is the shape that
//! probes it hardest, which [`skewed`] builds.

use super::arith::{Decoder, Range};
use super::{AtMost, EncodingStrategy, EntropyDecoder};
use crate::{Normal, Small};

/// Code `values` back to back under one context, then decode them one at a
/// time, asserting each round-trips and that none consumes more than
/// `S::MAX_BYTES`. Returns the largest consumption seen, so looseness stays
/// visible rather than drifting.
#[track_caller]
fn worst_bytes<T, S>(what: &str, values: &[T]) -> usize
where
    T: PartialEq + std::fmt::Debug,
    S: EncodingStrategy<T>,
{
    let mut coder = Range::default();
    let mut ctx = S::Context::default();
    for v in values {
        S::encode(v, &mut coder, &mut ctx);
    }
    let bytes = coder.into_vec();

    let mut reader = Decoder::new(&bytes);
    let mut ctx = S::Context::default();
    let mut worst = 0;
    for (i, expected) in values.iter().enumerate() {
        let before = reader.bytes_remaining();
        let got = S::decode(&mut reader, &mut ctx)
            .unwrap_or_else(|e| panic!("{what}: value {i} failed to decode: {e}"));
        assert_eq!(&got, expected, "{what}: value {i} did not round-trip");
        let consumed = before - reader.bytes_remaining();
        assert!(
            consumed <= S::MAX_BYTES,
            "{what}: value {i} ({expected:?}) consumed {consumed} bytes, over its \
             declared MAX_BYTES of {}",
            S::MAX_BYTES
        );
        worst = worst.max(consumed);
    }
    println!(
        "{what:34} worst {worst:>3} of {} declared",
        if S::MAX_BYTES == usize::MAX {
            "unbounded".to_string()
        } else {
            S::MAX_BYTES.to_string()
        }
    );
    worst
}

/// A long run of `common` — enough to drive the contexts hard toward it — then
/// the values that context makes expensive.
fn skewed<T: Clone>(common: T, rare: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut values = vec![common; 500];
    values.extend(rare);
    values
}

/// Exhaustive over every `AtMost<MAX>` value, for a spread of `MAX` covering
/// both tree layouts (power-of-two value count and not) and both walks
/// (whole-symbol and, at the top, the per-bit fallback).
macro_rules! atmost_cases {
    ($($max:literal),*) => {$({
        let all: Vec<AtMost<$max>> = (0..=$max).map(AtMost::<$max>::new).collect();
        worst_bytes::<_, Normal>(concat!("AtMost<", $max, "> exhaustive"), &all);
        worst_bytes::<_, Normal>(
            concat!("AtMost<", $max, "> skewed"),
            &skewed(AtMost::<$max>::new(0), (0..=$max).rev().map(AtMost::<$max>::new)),
        );
    })*};
}

#[test]
fn no_value_exceeds_its_declared_max_bytes() {
    // Exhaustive where the domain is small enough to be certain.
    worst_bytes::<_, Normal>("bool exhaustive", &[false, true]);
    worst_bytes::<_, Normal>("bool skewed", &skewed(false, [true, true, false]));
    worst_bytes::<_, Normal>("u8 exhaustive", &(0..=255u8).collect::<Vec<_>>());
    worst_bytes::<_, Normal>("u8 skewed", &skewed(0u8, 0..=255u8));
    worst_bytes::<_, Small>("Small<u8> exhaustive", &(0..=255u8).collect::<Vec<_>>());
    worst_bytes::<_, Normal>("i8 exhaustive", &(-128..=127i8).collect::<Vec<_>>());
    atmost_cases!(1, 2, 3, 7, 15, 63, 255, 1000);

    // Wide types: extremes, every bit-length regime, and a hard skew. Every
    // power of two and its neighbours covers each `blbl` bucket and each
    // mantissa width the hierarchical encoding can produce.
    fn bit_length_regimes<T>(from_shift: fn(u32) -> T) -> Vec<T> {
        (0..64)
            .flat_map(|s| [from_shift(s), from_shift(s)])
            .collect()
    }
    let u64s: Vec<u64> = bit_length_regimes(|s| {
        let base = 1u64 << s;
        base | (0x5a5a_5a5a_5a5a_5a5a >> (63 - s))
    });
    worst_bytes::<_, Normal>("u64 bit-length regimes", &u64s);
    worst_bytes::<_, Normal>("u64 skewed", &skewed(0u64, [u64::MAX, 0, u64::MAX, 1]));
    worst_bytes::<_, Small>("Small<u64> skewed", &skewed(0u64, [u64::MAX, 0, u64::MAX]));
    worst_bytes::<_, Normal>("u32 skewed", &skewed(0u32, [u32::MAX, 0, u32::MAX]));
    worst_bytes::<_, Normal>("u128 skewed", &skewed(0u128, [u128::MAX, 0, u128::MAX]));
    worst_bytes::<_, Normal>("usize skewed", &skewed(0usize, [usize::MAX, 0, usize::MAX]));
    worst_bytes::<_, Normal>("isize skewed", &skewed(0isize, [isize::MIN, 0, isize::MAX]));

    // Unbounded types still get measured, so the numbers are on record if we
    // ever want to bound them.
    let strings: Vec<String> = (0..200).map(|i| format!("value number {i}")).collect();
    worst_bytes::<_, Normal>("String", &strings);
    worst_bytes::<_, Normal>("char skewed", &skewed('a', ['a', 'Z', 'é', '日', '🦀']));
}
