use crate::{Normal, Small, Sorted, Values};

use super::{Encode, EncodingStrategy};
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

#[cfg(test)]
use expect_test::expect;

pub struct SetContext<T, S: EncodingStrategy<T>> {
    len: <usize as Encode>::Context,
    values: S::Context,
}
impl<T, S: EncodingStrategy<T>> Default for SetContext<T, S> {
    #[inline]
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}
impl<T, S: EncodingStrategy<T>> Clone for SetContext<T, S> {
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        Values::<Normal>::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Values::<Normal>::decode(reader, ctx)
    }
}

#[test]
fn hashset() {
    use super::{assert_bits, assert_size};
    assert_size!(HashSet::<usize>::new(), expect!["1"]);
    assert_size!(HashSet::from([0_usize]), expect!["1"]);
    assert_size!(HashSet::from([1_usize]), expect!["1"]);
    assert_size!(HashSet::from([5_usize]), expect!["2"]);
    assert_bits!(HashSet::from([true, false]), expect!["6"]);
    // assert_size!(HashSet::from([0_usize, 1, 2]), 3);
    // assert_size!(HashSet::from([0_usize, 1]), 1);
    // Sizes of larger hash sets are unpredictable because the values come out
    // in arbitrary orders.
}

impl<T: Ord> Encode for BTreeSet<T>
where
    Sorted: EncodingStrategy<T>,
{
    type Context = SetContext<T, Sorted>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        Values::<Sorted>::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Values::<Sorted>::decode(reader, ctx)
    }
}

#[derive(Default, Clone)]
pub struct CompactU64Set {
    size: <usize as Encode>::Context,
    first: <Small as EncodingStrategy<u64>>::Context,
    diff: <Small as EncodingStrategy<u64>>::Context,
}

impl EncodingStrategy<BTreeSet<u64>> for super::Small {
    type Context = CompactU64Set;
    fn encode<E: super::EntropyCoder>(
        value: &BTreeSet<u64>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        value.len().encode(writer, &mut ctx.size);
        let mut iter = value.iter().copied();
        if let Some(mut prev) = iter.next() {
            Small::encode(&prev, writer, &mut ctx.first);
            for v in iter {
                Small::encode(&(v - prev), writer, &mut ctx.diff);
                prev = v;
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<u64>, std::io::Error> {
        let mut out = BTreeSet::new();
        let len = usize::decode(reader, &mut ctx.size)?;
        if len > 0 {
            let mut prev = Small::decode(reader, &mut ctx.first)?;
            out.insert(prev);
            for _ in 1..len {
                let diff: u64 = Small::decode(reader, &mut ctx.diff)?;
                prev += diff;
                out.insert(prev);
            }
        }
        Ok(out)
    }
}

impl<T: Ord, S: EncodingStrategy<T>> EncodingStrategy<BTreeSet<T>> for Values<S> {
    type Context = SetContext<T, S>;
    fn encode<E: super::EntropyCoder>(
        value: &BTreeSet<T>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        value.len().encode(writer, &mut ctx.len);
        for v in value {
            S::encode(v, writer, &mut ctx.values);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<T>, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut set = BTreeSet::new();
        for _ in 0..len {
            set.insert(S::decode(reader, &mut ctx.values)?);
        }
        Ok(set)
    }
}

impl<T: Hash + Eq, S: EncodingStrategy<T>> EncodingStrategy<HashSet<T>> for Values<S> {
    type Context = SetContext<T, S>;
    fn encode<E: super::EntropyCoder>(value: &HashSet<T>, writer: &mut E, ctx: &mut Self::Context) {
        value.len().encode(writer, &mut ctx.len);
        for v in value {
            S::encode(v, writer, &mut ctx.values);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashSet<T>, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut set = HashSet::with_capacity(len);
        for _ in 0..len {
            set.insert(S::decode(reader, &mut ctx.values)?);
        }
        Ok(set)
    }
}

#[test]
fn btreeset() {
    use super::{assert_ans_bits, assert_bits};
    assert_bits!(BTreeSet::<usize>::new(), expect!["3"]);
    assert_bits!(BTreeSet::from([0_usize]), expect!["6"]);
    assert_bits!(BTreeSet::from([1_usize]), expect!["6"]);
    assert_bits!(BTreeSet::from([5_usize]), expect!["8"]);
    assert_bits!(BTreeSet::from([0_usize, 1]), expect!["10"]);
    assert_bits!(BTreeSet::from([0_usize, 1, 2]), expect!["12"]);
    assert_bits!(BTreeSet::from_iter(0_usize..70), expect!["40"]);
    assert_bits!(BTreeSet::from_iter(0_usize..1024), expect!["87"]);
    assert_bits!(BTreeSet::from([false]), expect!["4"]);
    assert_bits!(BTreeSet::from([true]), expect!["4"]);
    assert_bits!(BTreeSet::from([false, true]), expect!["6"]);
    assert_bits!(
        BTreeSet::from_iter(1_000_000_u64..1_001_024),
        expect!["160"]
    );
    assert_bits!(
        BTreeSet::from_iter(2_000_000_u64..2_002_048),
        expect!["243"]
    );
    assert_ans_bits!(
        BTreeSet::from_iter(2_000_000_u64..2_002_048),
        expect!["243"]
    );
}

#[test]
fn compact_btreeset() {
    use super::{assert_ans_bits, assert_bits};
    use crate::Encoded;
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::<u64>::new()),
        expect!["3"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([0_u64])),
        expect!["9"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([1_u64])),
        expect!["9"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([5_u64])),
        expect!["11"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([u32::MAX as u64])),
        expect!["41"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([u64::MAX])),
        expect!["74"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([0_u64, 1])),
        expect!["15"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from([0_u64, 1, 2])),
        expect!["19"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from_iter(0_u64..70)),
        expect!["55"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from_iter(0_u64..1024)),
        expect!["124"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from_iter(1_000_000_u64..1_001_024)),
        expect!["143"]
    );
    assert_bits!(
        Encoded::<_, Small>::new(BTreeSet::from_iter(2_000_000_u64..2_002_048)),
        expect!["214"]
    );
    assert_ans_bits!(
        Encoded::<_, Small>::new(BTreeSet::from_iter(2_000_000_u64..2_002_048)),
        expect!["214"]
    );
}
