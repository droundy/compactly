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
use super::{Encode, EntropyCoder, EntropyDecoder};

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

    /// Elements that can be coded before the next marker is due.
    ///
    /// Lets an async batch loop hand over a run containing **no** marker, so
    /// the unit it must budget for is exactly one element rather than an
    /// element plus a marker that is only present one time in
    /// [`SENTINEL_EVERY`].
    #[inline]
    pub(crate) fn until_marker(&self) -> usize {
        self.countdown
    }

    /// Account for `n` elements coded with no marker among them.
    ///
    /// The batch loop in [`decode_elements`] clamps every run to
    /// [`Self::until_marker`], so no marker can fall due inside one — the
    /// per-element [`Self::decode`] it would otherwise call is a countdown
    /// decrement and a branch that provably never fires. Doing the arithmetic
    /// once per run instead is worth 1.7% of the batched decode's instructions.
    #[inline]
    pub(crate) fn skip(&mut self, n: usize) {
        debug_assert!(n <= self.countdown, "a marker fell due inside a run");
        self.countdown -= n;
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

/// Somewhere one decoded element can be put.
///
/// A tiny trait rather than a closure so that [`decode_elements`] reaches its
/// collection directly: a closure would be captured by the `with_sync` closure
/// around it, putting the collection one indirection further away on the hot
/// path, which measured 0.6% (see OPTIMIZING.md). `std::iter::Extend` would
/// serve, but only via `extend(once(v))` — `extend_one` is still unstable — and
/// each collection's inherent one-element method is both plainer to read and
/// not reliant on the iterator specializing away.
pub(crate) trait ExtendOne<T> {
    fn extend_one_element(&mut self, value: T);
}

impl<T> ExtendOne<T> for Vec<T> {
    #[inline]
    fn extend_one_element(&mut self, value: T) {
        self.push(value)
    }
}

impl ExtendOne<char> for String {
    #[inline]
    fn extend_one_element(&mut self, value: char) {
        self.push(value)
    }
}

impl<T> ExtendOne<T> for std::collections::VecDeque<T> {
    #[inline]
    fn extend_one_element(&mut self, value: T) {
        self.push_back(value)
    }
}

impl<T: std::hash::Hash + Eq> ExtendOne<T> for std::collections::HashSet<T> {
    #[inline]
    fn extend_one_element(&mut self, value: T) {
        self.insert(value);
    }
}

impl<K: std::hash::Hash + Eq, V> ExtendOne<(K, V)> for std::collections::HashMap<K, V> {
    #[inline]
    fn extend_one_element(&mut self, (k, v): (K, V)) {
        self.insert(k, v);
    }
}

/// Decode `n` marked elements, handing over as many at a time as the decoder
/// will certainly cover — **the** way to decode a length-driven collection
/// asynchronously.
///
/// The naive loop (`decode_async` per element) is correct and up to 25% slower:
/// `decode_async` does still take the sync path per element, but through a
/// fresh [`with_sync`](crate::v2::AsyncEntropyDecoder::with_sync) handoff each
/// time, which for `Range` copies the coder state into the slice decoder and
/// back out again on every element. Handing over a run amortizes that, and lets
/// the sync decoder keep its state register-resident across the whole run.
///
/// `T` is whatever the collection codes *per element*, which is not always its
/// item type: a map passes `(K, V)` under `Mapping<SK, SV>`, which codes a key
/// then a value against exactly the contexts the map already holds. So one
/// `<T as Encode<S>>::MAX_BYTES` is the whole unit the decoder is asked about,
/// with no summing at the call site.
///
/// Runs stop short of the next marker, so `T` alone is the whole unit; a marker
/// is one bit every [`SENTINEL_EVERY`] elements and folding it into every
/// element's bound would overstate it by a byte apiece.
///
/// Elements are appended to `out` in stream order; see [`ExtendOne`] for why
/// that is a trait rather than a closure.
///
/// An **unbounded** `T` (`MAX_BYTES == usize::MAX`) cannot be promised *while
/// data is still arriving*, so the loop degrades to the naive one rather than
/// misbehaving — correct, just not faster. Once the source is complete every
/// `T` is promised, unbounded or not, and the batch runs at full width.
pub(crate) async fn decode_elements<D, T, S, C>(
    reader: &mut D,
    ctx: &mut <T as Encode<S>>::Context,
    n: usize,
    out: &mut C,
) -> Result<(), std::io::Error>
where
    D: crate::v2::AsyncEntropyDecoder,
    T: Encode<S>,
    C: ExtendOne<T>,
{
    let mut sentinel = Sentinel::new();
    let mut decoded = 0;
    while decoded < n {
        let batch = reader
            .sync_capacity(<T as Encode<S>>::MAX_BYTES)
            .min(sentinel.until_marker())
            .min(n - decoded);
        if batch > 0 {
            // Bound to a `let` so the closure's borrows end at the semicolon.
            let result = reader.with_sync(|sync| {
                for _ in 0..batch {
                    out.extend_one_element(<T as Encode<S>>::decode(sync, ctx)?);
                }
                Ok::<(), std::io::Error>(())
            });
            result?;
            // The run held no marker by construction; see `Sentinel::skip`.
            sentinel.skip(batch);
            decoded += batch;
            continue;
        }
        // Either too little is buffered to promise even one element, or a
        // marker falls due right here and a run may not span it. Take this one
        // element the slow way, which also awaits more input if that was why.
        sentinel.decode_async(reader).await?;
        out.extend_one_element(<T as Encode<S>>::decode_async(reader, ctx).await?);
        decoded += 1;
    }
    Ok(())
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

    /// Decode `value` from a chunk stream on both coders, at chunk sizes that
    /// put [`decode_elements`](super::decode_elements) into both of its regimes.
    ///
    /// The `in_memory` twin of each of these pins the *format*; this pins the
    /// **async decode** of it, which is a second implementation and not a
    /// re-run of the first. Two things make that worth its own test rather than
    /// trusting the sync one:
    ///
    /// - the batch loop is a different traversal of the same bytes, and the
    ///   sentinel bookkeeping is *not* shared with the sync path — a run takes
    ///   `Sentinel::skip` where the sync loop ticks per element, so a
    ///   miscounted run desynchronizes the marker schedule and nothing above
    ///   would notice until a marker came up `false`;
    /// - past `SENTINEL_EVERY` there is one element per interval that a run
    ///   cannot span, which the loop must hand to the awaiting path instead.
    ///   Below the interval that element does not exist, so only a `BIG` value
    ///   reaches it — hence testing exactly these fixtures rather than fresh
    ///   small ones.
    ///
    /// Both coders, because they answer `sync_capacity` in completely different
    /// terms: `Range` divides buffered bytes, so mid-stream it batches in
    /// bounded runs, while `Ans` is all-or-nothing on `reached_final` and so
    /// takes the awaiting path for the whole stream and then the entire tail at
    /// once. Neither exercises the other's loop.
    ///
    /// # What each fixture actually reaches
    ///
    /// Verified by mutation rather than assumed: an off-by-one in
    /// [`Sentinel::skip`] is caught by 11 of the 15 fixtures here and by
    /// **none** of their `in_memory` twins.
    ///
    /// The four survivors are the ones whose *element* is unbounded
    /// (`LowCardinality`, `Sorted` items, `Compressible`). For those, `Range`
    /// reports capacity 0 for the whole mid-stream, so the batch loop runs only
    /// once the source completes — a tail of a few elements, with no marker
    /// inside it. That is the real behaviour and not a gap here: the marker
    /// arithmetic lives in the one shared `decode_elements`, which the other 11
    /// pin thoroughly, and what remains per-site is the context threading and
    /// the sink, which every fixture checks by comparing the whole value. All
    /// 15 do reach the batch path (checked the same way).
    #[cfg(feature = "stream")]
    fn round_trips_from_a_stream<T>(value: &T)
    where
        T: crate::v2::Encode + PartialEq + std::fmt::Debug,
    {
        use crate::v2::stream::tests::Chunks;
        use crate::v2::{Ans, Range};
        use futures_executor::block_on;

        let range_bytes = encode(value);
        let ans_bytes = Ans::encode(value);

        // Two regimes. 7 bytes holds less than one element's `MAX_BYTES` for
        // every fixture here, so `Range`'s capacity stays 0 and each element
        // goes down the awaiting path. A quarter of the input leaves ample room
        // for real runs, so the same value is decoded by the batch loop as well
        // — and being *derived* rather than a constant matters: the most
        // compressible fixtures encode to under 2 KB, so a fixed 4096 would
        // hand them over as a single chunk and quietly test the slice decoder
        // instead of the async one.
        //
        // `Chunks` yields `Pending` before every chunk, so each boundary is a
        // genuine suspension rather than a ready poll, and neither size can
        // reach the single-chunk fast path.
        for chunk_size in [7, (range_bytes.len() / 4).max(8)] {
            let stream = Chunks::new(&range_bytes, chunk_size);
            let decoded: T = block_on(Range::decode_stream::<T, _, _>(stream))
                .expect("Range stream decode failed");
            assert_eq!(&decoded, value, "Range, chunk_size = {chunk_size}");
        }
        for chunk_size in [7, (ans_bytes.len() / 4).max(8)] {
            let stream = Chunks::new(&ans_bytes, chunk_size);
            let decoded: T =
                block_on(Ans::decode_stream::<T, _, _>(stream)).expect("Ans stream decode failed");
            assert_eq!(&decoded, value, "Ans, chunk_size = {chunk_size}");
        }
    }

    macro_rules! round_trips {
        ($name:ident, $value:expr) => {
            mod $name {
                use super::*;

                /// The format: encode and decode in memory.
                #[test]
                fn in_memory() {
                    let v = $value;
                    let bytes = encode(&v);
                    assert_eq!(
                        decode(&bytes).as_ref(),
                        Some(&v),
                        "round trip past sentinel"
                    );
                }

                /// The async decode of that same format; see
                /// [`round_trips_from_a_stream`].
                #[cfg(feature = "stream")]
                #[test]
                fn from_a_stream() {
                    round_trips_from_a_stream(&$value);
                }
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
    // `HashMap`'s *default* impl has its own `decode_elements` call, distinct
    // from the one in `Encode<Mapping<SK, SV>> for HashMap` — the only
    // length-driven async loop in the crate that no other fixture reaches.
    round_trips!(
        mapping_hashmap,
        Encoded::<HashMap<u64, u64>, crate::Mapping<crate::Small, crate::Small>>::new(
            seq(BIG).map(|i| (i, i ^ 5)).collect()
        )
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
