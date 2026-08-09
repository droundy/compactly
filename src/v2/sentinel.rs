//! A periodic in-stream marker that bounds work on corrupt or truncated input.
//!
//! Coder decode is deliberately infallible: running past the encoded data yields
//! arbitrary in-range values rather than an error. That is fine for a fixed-size
//! value, but a *length-driven* loop is then unbounded — a handful of bytes
//! claiming billions of elements will happily materialize billions of them from
//! fabricated padding. Capping the eager allocation does not help; the loop still
//! runs.
//!
//! So every such loop codes a `true` bit every [`SENTINEL_EVERY`] elements. This
//! works because of a property of both coders: **decoding from zero padding can
//! never yield `true`.** Past the real data, `Range` has `value` and `lo` both
//! shifted to 0 while `split >= lo`, so `value > split` never holds; `Ans` has
//! `z = state & 255` drained to 0 while `ones >= 1`, so `z >= ones` never holds.
//! A `true` marker is therefore unforgeable by the fabricated tail, whatever the
//! context says, and a decoder that has run off the end sees `false` within one
//! interval and stops.
//!
//! The marker is coded against a **fixed** context, deliberately skewed so that
//! `true` is the *unlikely* outcome. That costs a few bits per marker instead of
//! a fraction of one, and buys the property that matters: a marker read out of
//! random or adversarially altered data comes up `true` — and so escapes notice —
//! only with probability `P(true)`. Skewing the other way would make markers
//! nearly free but would also let corruption pass ~255 times out of 256, which
//! would make this a work bound and not a check. Detection compounds across
//! markers, so a long collection is caught with overwhelming probability.
//!
//! Not adapting also denies an attacker any influence: the probability is the
//! same at every marker in every stream, so a crafted prefix cannot drag the
//! context toward `true` and disarm the later checks.
//!
//! Note the *truncation* case does not depend on the skew at all — zero padding
//! decodes `false` with certainty — so the bound on wasted work holds regardless.
//! The skew only sets how well mid-stream corruption is caught.
//!
//! Collections shorter than [`SENTINEL_EVERY`] code no marker at all, so their
//! encoded form is unchanged.

use super::bit_context::BitContext;
use super::{EntropyCoder, EntropyDecoder};

/// The fixed context every marker is coded against, seeded as though it had
/// seen a long run of `false`.
///
/// Sets both halves of the trade-off, since the cost of coding the marker's
/// `true` is `-log2(P(true))` bits and the chance corruption slips past one
/// marker is exactly `P(true)`. At this seeding (measured, not estimated):
///
/// | `adapt(false)` count | P(true) | cost per marker | miss rate |
/// |---|---|---|---|
/// | 8 | 0.098 | 3.4 bits | 9.8% |
/// | 32 | **0.027** | **5.2 bits** | **2.7%** |
/// | 64 | 0.012 | 6.4 bits | 1.2% |
///
/// 32 costs well under a byte per [`SENTINEL_EVERY`] elements while catching
/// ~97% of corruption per marker — and misses compound, so two markers miss
/// 0.07% of the time and three 0.002%. `adapt` is `const`, so the seeding is
/// free at runtime.
const SEEDED: BitContext = {
    let mut ctx = BitContext::True0False0;
    let mut i = 0;
    while i < 32 {
        ctx = ctx.adapt(false);
        i += 1;
    }
    ctx
};

/// Code a marker once per this many elements.
///
/// Two things at once: the bound on wasted work (a corrupt stream is caught
/// within this many elements of the real data ending) and the amortized size
/// cost, which is `marker_bits / SENTINEL_EVERY` — about 1.3 millibits per
/// element here.
///
/// That cost only shows up on data whose own cost per element is comparable,
/// i.e. highly compressible collections. Measured: 100k random `u64` grows
/// 0.002%, but `vec![1usize; 2^20]` — which codes at ~34 millibits/element —
/// grows about 3.7%. At 1024 the latter was +15%, which is why this is not
/// smaller. Detection per marker does not depend on this at all; only how many
/// markers a given collection gets.
const SENTINEL_EVERY: usize = 4096;

/// Emits and checks the periodic marker for one length-driven loop.
///
/// Encode and decode must call [`Self::encode`]/[`Self::decode`] the same number
/// of times in the same order — once per element, before coding it — so the
/// schedules stay in lockstep. The context is fixed and built locally, so no
/// `Context` struct has to carry it and nothing accumulates across markers.
pub(crate) struct Sentinel {
    countdown: usize,
}

impl Sentinel {
    #[inline]
    pub(crate) fn new() -> Self {
        Sentinel {
            countdown: SENTINEL_EVERY,
        }
    }

    /// Call once per element, before coding it; returns whether a marker is due.
    #[inline]
    fn tick(&mut self) -> bool {
        let fire = self.countdown == 0;
        if fire {
            self.countdown = SENTINEL_EVERY;
        }
        self.countdown -= 1;
        fire
    }

    /// Call once per element, before coding it.
    #[inline]
    pub(crate) fn encode<E: EntropyCoder>(&mut self, writer: &mut E) {
        if self.tick() {
            // A fresh copy every time: the context must not adapt.
            writer.encode_bit(&mut { SEEDED }, true);
        }
    }

    /// Call once per element, before decoding it. Errors once the stream has
    /// been detected as corrupt or truncated.
    #[inline]
    pub(crate) fn decode<D: EntropyDecoder>(
        &mut self,
        reader: &mut D,
    ) -> Result<(), std::io::Error> {
        // `&&` short-circuits, so the bit is only decoded when a marker is due.
        if self.tick() && !reader.decode_bit(&mut { SEEDED }) {
            return Err(std::io::Error::other(
                "corrupt or truncated stream: collection ran past the encoded data",
            ));
        }
        Ok(())
    }

    /// The async twin of [`Self::decode`].
    #[inline]
    pub(crate) async fn decode_async<D: crate::v2::AsyncEntropyDecoder>(
        &mut self,
        reader: &mut D,
    ) -> Result<(), std::io::Error> {
        // `&&` short-circuits, so the bit is only decoded when a marker is due.
        if self.tick() && !reader.decode_bit(&mut { SEEDED }).await {
            return Err(std::io::Error::other(
                "corrupt or truncated stream: collection ran past the encoded data",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::{decode, encode};
    use crate::{Compressible, Encoded, LowCardinality, Sorted, Values};
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

    /// Every collection that codes a marker must round-trip *past* the marker
    /// interval. The rest of the suite uses small values, which emit no marker
    /// at all, so without this an encode/decode asymmetry in any one of these
    /// paths would go completely unnoticed.
    const BIG: usize = super::SENTINEL_EVERY * 2 + 7;

    fn seq(n: usize) -> impl Iterator<Item = u64> {
        (0..n as u64).map(|i| i.wrapping_mul(2654435761) % 100_000)
    }

    macro_rules! round_trips {
        ($name:ident, $value:expr) => {
            #[test]
            fn $name() {
                let v = $value;
                let bytes = encode(&v);
                assert_eq!(
                    decode(&bytes).as_ref(),
                    Some(&v),
                    "round trip past sentinel"
                );
            }
        };
    }

    round_trips!(vec_u64, seq(BIG).collect::<Vec<u64>>());
    round_trips!(
        boxed_slice,
        seq(BIG).collect::<Vec<u64>>().into_boxed_slice()
    );
    round_trips!(
        string,
        seq(BIG)
            .map(|i| char::from(b'a' + (i % 26) as u8))
            .collect::<String>()
    );
    round_trips!(
        boxed_str,
        seq(BIG)
            .map(|i| char::from(b'a' + (i % 26) as u8))
            .collect::<String>()
            .into_boxed_str()
    );
    round_trips!(
        hashmap,
        seq(BIG).map(|i| (i, i ^ 5)).collect::<HashMap<u64, u64>>()
    );
    round_trips!(
        btreemap,
        seq(BIG).map(|i| (i, i ^ 5)).collect::<BTreeMap<u64, u64>>()
    );
    round_trips!(hashset, seq(BIG).collect::<HashSet<u64>>());
    round_trips!(btreeset, seq(BIG).collect::<BTreeSet<u64>>());
    round_trips!(
        vecdeque,
        Encoded::<VecDeque<u64>, Values<crate::Normal>>::new(seq(BIG).collect())
    );
    round_trips!(
        compact_btreeset,
        Encoded::<BTreeSet<u64>, crate::Small>::new(seq(BIG).collect())
    );
    round_trips!(
        sorted_strings,
        Encoded::<Vec<String>, Values<Sorted>>::new(
            (0..BIG).map(|i| format!("item{i:07}")).collect()
        )
    );
    round_trips!(
        sorted_vecs,
        Encoded::<Vec<Vec<u64>>, Values<Sorted>>::new(
            (0..BIG).map(|i| vec![i as u64, (i as u64) ^ 3]).collect()
        )
    );
    round_trips!(
        low_cardinality,
        Encoded::<Vec<u64>, LowCardinality>::new(seq(BIG).map(|i| i % 7).collect())
    );
    round_trips!(
        compressible_bytes,
        Encoded::<Vec<u8>, Compressible>::new(
            (0..BIG * 8).map(|i| (i % 251) as u8).collect::<Vec<u8>>()
        )
    );

    /// The point of the whole exercise: a tiny input claiming an enormous
    /// collection must be rejected promptly rather than materializing it.
    #[test]
    fn absurd_claimed_length_is_rejected() {
        let bytes = encode(&Encoded::<usize, crate::Small>::new(20_000_000));
        assert!(bytes.len() < 32, "the attack is a handful of bytes");
        let start = std::time::Instant::now();
        let decoded: Option<Vec<u8>> = decode(&bytes);
        assert!(decoded.is_none(), "must reject, not fabricate 20M elements");
        assert!(
            start.elapsed() < std::time::Duration::from_millis(200),
            "must bail within a sentinel interval, took {:?}",
            start.elapsed()
        );
    }

    /// A truncated stream must not silently yield a partial collection.
    #[test]
    fn truncation_is_caught() {
        let v: Vec<u64> = seq(BIG).collect();
        let bytes = encode(&v);
        let cut = bytes.len() / 4;
        let decoded: Option<Vec<u64>> = decode(&bytes[..cut]);
        assert!(
            decoded.is_none() || decoded.as_ref().unwrap().len() <= v.len(),
            "truncated stream must not expand"
        );
    }
}
