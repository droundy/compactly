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

use super::arith::{Decoder, Range, SETTLING_BYTES};
use super::{AtMost, DecodeAsync, EncodingStrategy, EntropyDecoder};
use crate::{Normal, Small};

/// Code `values` back to back under one context, then decode them one at a
/// time, asserting each round-trips. Returns the largest number of bytes any
/// single value consumed, and reports it so looseness stays visible.
///
/// Does not check a bound — a strategy only declares one once it has a
/// [`DecodeAsync`] impl, since that is the only thing that reads it. Types
/// still waiting for their async twin are measured here; [`worst_bytes`] is the
/// checking version.
#[track_caller]
fn measure<T, S>(what: &str, values: &[T]) -> usize
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
        worst = worst.max(before - reader.bytes_remaining());
    }
    worst
}

/// [`measure`], plus the assertion that nothing exceeded the strategy's
/// declared bound.
#[track_caller]
fn worst_bytes<T, S>(what: &str, values: &[T]) -> usize
where
    T: PartialEq + std::fmt::Debug,
    S: DecodeAsync<T>,
{
    let worst = measure::<T, S>(what, values);
    let allowed = S::MAX_BYTES.saturating_add(SETTLING_BYTES);
    assert!(
        worst <= allowed,
        "{what}: a value consumed {worst} bytes, over the {allowed} its MAX_BYTES \
         of {} allows (plus {SETTLING_BYTES} settling)",
        S::MAX_BYTES
    );
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
    measure::<_, Small>("Small<u8> exhaustive", &(0..=255u8).collect::<Vec<_>>());
    measure::<_, Normal>("i8 exhaustive", &(-128..=127i8).collect::<Vec<_>>());
    atmost_cases!(0, 1, 2, 3, 7, 15, 63, 255, 1000);

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
    measure::<_, Normal>("u32 skewed", &skewed(0u32, [u32::MAX, 0, u32::MAX]));
    measure::<_, Normal>("u128 skewed", &skewed(0u128, [u128::MAX, 0, u128::MAX]));
    worst_bytes::<_, Normal>("usize skewed", &skewed(0usize, [usize::MAX, 0, usize::MAX]));
    measure::<_, Normal>("isize skewed", &skewed(0isize, [isize::MIN, 0, isize::MAX]));

    // Unbounded types still get measured, so the numbers are on record if we
    // ever want to bound them.
    let strings: Vec<String> = (0..200).map(|i| format!("value number {i}")).collect();
    worst_bytes::<_, Normal>("String", &strings);
    worst_bytes::<_, Normal>("char skewed", &skewed('a', ['a', 'Z', 'é', '日', '🦀']));
}

/// Every type that declares a *finite* bound, checked the same way. Slower than
/// the focused test above — it walks a lot of values — but an unchecked bound is
/// the one failure mode that is silent, so the coverage is worth the seconds.
#[test]
fn every_bounded_type_stays_within_its_bound() {
    use crate::{Compressible, Incompressible, Sorted};
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
    use std::num::{NonZeroI16, NonZeroI64, NonZeroU16, NonZeroU64, NonZeroUsize};

    // Integers, every strategy that declares a bound.
    measure::<_, Sorted>(
        "Sorted<u64>",
        &(0..500u64).map(|i| i * 7).collect::<Vec<_>>(),
    );
    measure::<_, Sorted>(
        "Sorted<u64> unsorted",
        &skewed(0u64, [u64::MAX, 0, u64::MAX]),
    );
    measure::<_, Incompressible>("Incompressible<u64>", &skewed(0u64, [u64::MAX, 7]));
    measure::<_, Sorted>("Sorted<i64>", &skewed(0i64, [i64::MIN, i64::MAX, 0]));
    measure::<_, Small>("Small<i64>", &skewed(0i64, [i64::MIN, i64::MAX, 0]));
    measure::<_, Normal>("u16", &(0..=u16::MAX).step_by(7).collect::<Vec<_>>());
    measure::<_, Small>("Small<u16>", &(0..=u16::MAX).step_by(7).collect::<Vec<_>>());
    measure::<_, Normal>("i16", &(i16::MIN..=i16::MAX).step_by(7).collect::<Vec<_>>());
    measure::<_, Small>("Small<i16>", &skewed(0i16, [i16::MIN, i16::MAX, 0]));
    measure::<_, Normal>("i32 skewed", &skewed(0i32, [i32::MIN, i32::MAX, 0]));
    measure::<_, Normal>("i128 skewed", &skewed(0i128, [i128::MIN, i128::MAX, 0]));
    measure::<_, Small>("Small<i8>", &(-128..=127i8).collect::<Vec<_>>());
    measure::<_, Sorted>("Sorted<u8>", &(0..=255u8).collect::<Vec<_>>());
    measure::<_, Sorted>("Sorted<i8>", &(-128..=127i8).collect::<Vec<_>>());
    measure::<_, Incompressible>("Incompressible<u8>", &(0..=255u8).collect::<Vec<_>>());
    measure::<_, Sorted>("Sorted<bool>", &skewed(false, [true, false]));

    // NonZero: coded through the plain integer behind them.
    let nz = |v: u64| NonZeroU64::new(v).unwrap();
    measure::<_, Normal>("NonZeroU64", &skewed(nz(1), [nz(u64::MAX), nz(1)]));
    measure::<_, Small>("Small<NonZeroU64>", &skewed(nz(1), [nz(u64::MAX), nz(1)]));
    let nzi = |v: i64| NonZeroI64::new(v).unwrap();
    measure::<_, Normal>(
        "NonZeroI64",
        &skewed(nzi(1), [nzi(i64::MIN), nzi(i64::MAX)]),
    );
    let nzu16 = |v: u16| NonZeroU16::new(v).unwrap();
    measure::<_, Normal>("NonZeroU16", &skewed(nzu16(1), [nzu16(u16::MAX)]));
    let nzi16 = |v: i16| NonZeroI16::new(v).unwrap();
    measure::<_, Normal>(
        "NonZeroI16",
        &skewed(nzi16(1), [nzi16(i16::MIN), nzi16(i16::MAX)]),
    );
    let nzus = |v: usize| NonZeroUsize::new(v).unwrap();
    measure::<_, Normal>("NonZeroUsize", &skewed(nzus(1), [nzus(usize::MAX)]));

    // Floats: several tiers behind selector bits, so probe each.
    let f64s: Vec<f64> = skewed(
        0.0f64,
        [
            1.0,
            -1.0,
            0.5,
            1e300,
            -1e-300,
            f64::MAX,
            f64::MIN,
            f64::EPSILON,
            1234.5678,
        ],
    );
    measure::<_, Normal>("f64", &f64s);
    measure::<_, crate::Decimal>("Decimal<f64>", &f64s);
    let f32s: Vec<f32> = skewed(0.0f32, [1.0, -1.0, 1e30, f32::MAX, f32::MIN, 1234.5678]);
    measure::<_, Normal>("f32", &f32s);
    measure::<_, crate::Decimal>("Decimal<f32>", &f32s);

    // Fixed-size composites.
    measure::<_, Normal>("()", &[(); 8]);
    measure::<_, Normal>("(u8, u64)", &skewed((0u8, 0u64), [(255, u64::MAX)]));
    measure::<_, Normal>(
        "(bool, u8, u16, u32)",
        &skewed((false, 0u8, 0u16, 0u32), [(true, 255, u16::MAX, u32::MAX)]),
    );
    measure::<_, Normal>("[u8; 4]", &skewed([0u8; 4], [[255u8; 4]]));
    measure::<_, Normal>("Option<u64>", &skewed(None::<u64>, [Some(u64::MAX), None]));
    measure::<_, Normal>("Box<u64>", &skewed(Box::new(0u64), [Box::new(u64::MAX)]));
    measure::<_, Normal>("PhantomData", &[std::marker::PhantomData::<u64>; 8]);

    // Network addresses: fixed size, so all bounded.
    measure::<_, Normal>(
        "Ipv4Addr",
        &skewed(
            Ipv4Addr::new(0, 0, 0, 0),
            [Ipv4Addr::new(255, 255, 255, 255)],
        ),
    );
    let v6 = |a| Ipv6Addr::from(a);
    measure::<_, Normal>(
        "Ipv6Addr",
        &skewed(
            v6([0u16; 8]),
            [v6([0xffff; 8]), v6([1, 0, 2, 0, 3, 0, 4, 0])],
        ),
    );
    measure::<_, Normal>(
        "IpAddr",
        &skewed(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            [IpAddr::V6(v6([0xffff; 8]))],
        ),
    );
    measure::<_, Normal>(
        "SocketAddrV4",
        &skewed(
            SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0),
            [SocketAddrV4::new(
                Ipv4Addr::new(255, 255, 255, 255),
                u16::MAX,
            )],
        ),
    );
    measure::<_, Normal>(
        "SocketAddrV6",
        &skewed(
            SocketAddrV6::new(v6([0u16; 8]), 0, 0, 0),
            [SocketAddrV6::new(
                v6([0xffff; 8]),
                u16::MAX,
                u32::MAX,
                u32::MAX,
            )],
        ),
    );
    measure::<_, Normal>(
        "SocketAddr",
        &skewed(
            SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)),
            [SocketAddr::V6(SocketAddrV6::new(
                v6([0xffff; 8]),
                u16::MAX,
                u32::MAX,
                u32::MAX,
            ))],
        ),
    );

    // Unbounded ones are exercised too — the assertion is vacuous, but the
    // round-trip and the reported worst case are not.
    measure::<_, Normal>("Vec<u64>", &vec![vec![1u64, 2, 3]; 8]);
    measure::<_, Compressible>(
        "Compressible<String>",
        &vec!["the quick brown fox jumps".to_string(); 8],
    );
}
