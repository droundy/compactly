use super::sentinel::Sentinel;
use crate::{Normal, Small, Sorted, Values};

use super::{Encode, Strategy};
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

#[cfg(test)]
use expect_test::expect;

pub struct SetContext<T: Encode<S>, S> {
    len: <usize as Encode>::Context,
    values: <T as Encode<S>>::Context,
}
impl<T: Encode<S>, S> Default for SetContext<T, S> {
    #[inline]
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}
impl<T: Encode<S>, S> Clone for SetContext<T, S> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            values: self.values.clone(),
        }
    }
}

impl<T: Encode + Hash + Eq> Encode for HashSet<T> {
    type Context = SetContext<T, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Values::<Normal>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Values::<Normal>::decode(reader, ctx)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<HashSet<T>, std::io::Error>> {
        <HashSet<T> as Encode<Values<Normal>>>::decode_awaiting(reader, ctx)
    }
}

#[test]
fn hashset() {
    use super::{assert_size, estimated_bits};
    assert_size!(HashSet::<usize>::new(), expect!["1"]);
    assert_size!(HashSet::from([0_usize]), expect!["1"]);
    assert_size!(HashSet::from([1_usize]), expect!["1"]);
    assert_size!(HashSet::from([5_usize]), expect!["1"]);
    expect!["7"].assert_eq(&estimated_bits!(HashSet::from([true, false])));
    // assert_size!(HashSet::from([0_usize, 1, 2]), 3);
    // assert_size!(HashSet::from([0_usize, 1]), 1);
    // Sizes of larger hash sets are unpredictable because the values come out
    // in arbitrary orders.
}

impl<T: Ord> Encode for BTreeSet<T>
where
    T: Encode<Sorted>,
{
    type Context = SetContext<T, Sorted>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Values::<Sorted>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Values::<Sorted>::decode(reader, ctx)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<BTreeSet<T>, std::io::Error>> {
        <BTreeSet<T> as Encode<Values<Sorted>>>::decode_awaiting(reader, ctx)
    }
}

#[derive(Default, Clone)]
pub struct CompactU64Set {
    size: <usize as Encode>::Context,
    first: <u64 as Encode<Small>>::Context,
    diff: <u64 as Encode<Small>>::Context,
}

impl Encode<super::Small> for BTreeSet<u64> {
    type Context = CompactU64Set;
    fn encode<E: super::EntropyCoder>(
        value: &BTreeSet<u64>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        Normal::encode(&value.len(), writer, &mut ctx.size);
        let mut iter = value.iter().copied();
        if let Some(mut prev) = iter.next() {
            Small::encode(&prev, writer, &mut ctx.first);
            let mut sentinel = Sentinel::new();
            for v in iter {
                sentinel.encode(writer);
                Small::encode(&(v - prev), writer, &mut ctx.diff);
                prev = v;
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<u64>, std::io::Error> {
        let len = <usize as Encode>::decode(reader, &mut ctx.size)?;
        // Stage + collect: bulk-build from the sorted stream, as in
        // `Values<S> for BTreeSet` below.
        let mut values = Vec::with_capacity(super::capacity_for::<u64>(len));
        if len > 0 {
            let mut prev = Small::decode(reader, &mut ctx.first)?;
            values.push(prev);
            let mut sentinel = Sentinel::new();
            for _ in 1..len {
                sentinel.decode(reader)?;
                let diff: u64 = Small::decode(reader, &mut ctx.diff)?;
                prev += diff;
                values.push(prev);
            }
        }
        Ok(values.into_iter().collect())
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<u64>, std::io::Error> {
        let len = <usize as Encode>::decode_async(reader, &mut ctx.size).await?;
        // Stage + collect: bulk-build from the sorted stream, as in
        // `Values<S> for BTreeSet` below.
        let mut values = Vec::with_capacity(super::capacity_for::<u64>(len));
        if len > 0 {
            let mut prev = <u64 as Encode<Small>>::decode_async(reader, &mut ctx.first).await?;
            values.push(prev);
            let mut sentinel = Sentinel::new();
            for _ in 1..len {
                sentinel.decode_async(reader).await?;
                let diff: u64 = <u64 as Encode<Small>>::decode_async(reader, &mut ctx.diff).await?;
                prev += diff;
                values.push(prev);
            }
        }
        Ok(values.into_iter().collect())
    }
}

impl<T: Ord + Encode<S>, S> Encode<Values<S>> for BTreeSet<T> {
    type Context = SetContext<T, S>;
    fn encode<E: super::EntropyCoder>(
        value: &BTreeSet<T>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        Normal::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for v in value {
            sentinel.encode(writer);
            <T as Encode<S>>::encode(v, writer, &mut ctx.values);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<T>, std::io::Error> {
        let len: usize = <usize as Encode>::decode(reader, &mut ctx.len)?;
        // Stage in a Vec: the elements arrive in sorted order, and
        // `FromIterator` bulk-builds packed nodes from sorted input in O(n) —
        // measured ~4.7x faster than per-element `insert` on 38k strings
        // (OPTIMIZING.md, 2026-07-19 survey). For any valid stream this is
        // identical to the old insert loop: a `BTreeSet` never holds
        // Ord-equal elements, so encode emits none and there is nothing to
        // dedup. The two diverge only on a *corrupt* stream carrying an
        // Ord-equal run of a type whose `Ord` is coarser than its `Eq`:
        // `collect` keeps every Eq-distinct element (a technically-malformed
        // set) where `insert` dedups by `Ord`. Decode makes no guarantee
        // about corrupt input beyond not being UB, and this isn't; pinned by
        // `btreeset_bulk_build_keeps_ord_equal_dupes` below.
        let mut values = Vec::with_capacity(super::capacity_for::<T>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            values.push(<T as Encode<S>>::decode(reader, &mut ctx.values)?);
        }
        Ok(values.into_iter().collect())
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<T>, std::io::Error> {
        let len: usize = <usize as Encode>::decode_async(reader, &mut ctx.len).await?;
        // Stage in a Vec — see the sync `decode` above for why `collect`
        // rather than a per-element `insert` loop.
        let mut values = Vec::with_capacity(super::capacity_for::<T>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode_async(reader).await?;
            values.push(<T as Encode<S>>::decode_async(reader, &mut ctx.values).await?);
        }
        Ok(values.into_iter().collect())
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OrdOnFirstField(i32, char);

#[cfg(test)]
impl PartialOrd for OrdOnFirstField {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
impl Ord for OrdOnFirstField {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

#[cfg(test)]
impl Encode for OrdOnFirstField {
    type Context = (<i32 as Encode>::Context, <char as Encode>::Context);
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self(
            <i32 as Encode>::decode(reader, &mut ctx.0)?,
            <char as Encode>::decode(reader, &mut ctx.1)?,
        ))
    }

    const MAX_BYTES: usize = <i32 as Encode>::MAX_BYTES.saturating_add(<char as Encode>::MAX_BYTES);
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self(
            <i32 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <char as Encode>::decode_async(reader, &mut ctx.1).await?,
        ))
    }
}

#[test]
fn btreeset_bulk_build_keeps_ord_equal_dupes() {
    // `OrdOnFirstField`'s `Ord` only looks at field 0, so `.1` distinguishes
    // values that compare Ord-equal. A genuine `BTreeSet<OrdOnFirstField>`
    // can never hold two such elements (`insert` treats them as the same
    // key), so build the raw decode stream by hand to simulate a
    // corrupt/adversarial input carrying a duplicate run: `[(1, 'a'), (1,
    // 'b')]`. This documents the one way the bulk-build decode differs from
    // the old insert loop — a corrupt-input-only edge, not reachable from any
    // valid stream.
    let mut writer = super::Range::default();
    let mut ctx = SetContext::<OrdOnFirstField, crate::Normal>::default();
    Normal::encode(&2_usize, &mut writer, &mut ctx.len);
    Normal::encode(&OrdOnFirstField(1, 'a'), &mut writer, &mut ctx.values);
    Normal::encode(&OrdOnFirstField(1, 'b'), &mut writer, &mut ctx.values);
    let bytes = writer.into_vec();

    use super::EntropyDecoder;
    let mut reader = super::arith::Decoder::new(&bytes);
    let mut ctx = SetContext::<OrdOnFirstField, crate::Normal>::default();
    let decoded =
        <BTreeSet<OrdOnFirstField> as Encode<Values<crate::Normal>>>::decode(&mut reader, &mut ctx)
            .unwrap();

    // `collect`'s `FromIterator` keeps *every* Eq-distinct element of the
    // Ord-equal run (a technically-malformed set of len 2); the old
    // per-element `insert` loop would have kept only the first (len 1). Both
    // are acceptable "garbage in" results for a corrupt stream.
    assert_eq!(decoded.len(), 2);
    assert_eq!(
        decoded,
        [OrdOnFirstField(1, 'a'), OrdOnFirstField(1, 'b')]
            .into_iter()
            .collect()
    );
}

impl<T: Hash + Eq + Encode<S>, S> Encode<Values<S>> for HashSet<T> {
    type Context = SetContext<T, S>;
    fn encode<E: super::EntropyCoder>(value: &HashSet<T>, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for v in value {
            sentinel.encode(writer);
            <T as Encode<S>>::encode(v, writer, &mut ctx.values);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashSet<T>, std::io::Error> {
        let len: usize = <usize as Encode>::decode(reader, &mut ctx.len)?;
        let mut set = HashSet::with_capacity(super::capacity_for::<T>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            set.insert(<T as Encode<S>>::decode(reader, &mut ctx.values)?);
        }
        Ok(set)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashSet<T>, std::io::Error> {
        let len: usize = <usize as Encode>::decode_async(reader, &mut ctx.len).await?;
        let mut set = HashSet::with_capacity(super::capacity_for::<T>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode_async(reader).await?;
            set.insert(<T as Encode<S>>::decode_async(reader, &mut ctx.values).await?);
        }
        Ok(set)
    }
}

#[test]
fn btreeset() {
    use super::{encoded_bits, estimated_bits};
    expect!["1"].assert_eq(&estimated_bits!(BTreeSet::<usize>::new()));
    expect!["5"].assert_eq(&estimated_bits!(BTreeSet::from([0_usize])));
    expect!["5"].assert_eq(&estimated_bits!(BTreeSet::from([1_usize])));
    expect!["7"].assert_eq(&estimated_bits!(BTreeSet::from([5_usize])));
    expect!["11"].assert_eq(&estimated_bits!(BTreeSet::from([0_usize, 1])));
    expect!["14"].assert_eq(&estimated_bits!(BTreeSet::from([0_usize, 1, 2])));
    expect!["41"].assert_eq(&estimated_bits!(BTreeSet::from_iter(0_usize..70)));
    expect!["82"].assert_eq(&estimated_bits!(BTreeSet::from_iter(0_usize..1024)));
    expect!["3"].assert_eq(&estimated_bits!(BTreeSet::from([false])));
    expect!["3"].assert_eq(&estimated_bits!(BTreeSet::from([true])));
    expect!["7"].assert_eq(&estimated_bits!(BTreeSet::from([false, true])));
    expect!["105"].assert_eq(&estimated_bits!(BTreeSet::from_iter(
        1_000_000_u64..1_001_024
    )));
    expect!["148"].assert_eq(&encoded_bits!(BTreeSet::from_iter(
        2_000_000_u64..2_002_048
    )));
    expect!["152"].assert_eq(&encoded_bits!(
        super::Ans,
        BTreeSet::from_iter(2_000_000_u64..2_002_048)
    ));
}

#[test]
fn compact_btreeset() {
    use super::{encoded_bits, estimated_bits};
    use crate::Encoded;
    expect!["1"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(
        BTreeSet::<u64>::new()
    )));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [0_u64]
    ))));
    expect!["5"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [1_u64]
    ))));
    expect!["8"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [5_u64]
    ))));
    expect!["41"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [u32::MAX as u64]
    ))));
    expect!["68"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [u64::MAX]
    ))));
    expect!["10"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [0_u64, 1]
    ))));
    expect!["12"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(BTreeSet::from(
        [0_u64, 1, 2]
    ))));
    expect!["35"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(0_u64..70)
    )));
    expect!["66"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(0_u64..1024)
    )));
    expect!["89"].assert_eq(&estimated_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(1_000_000_u64..1_001_024)
    )));
    expect!["119"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(2_000_000_u64..2_002_048)
    )));
    expect!["122"].assert_eq(&encoded_bits!(
        super::Ans,
        Encoded::<_, Small>::new(BTreeSet::from_iter(2_000_000_u64..2_002_048))
    ));
}
