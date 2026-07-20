use crate::{Normal, Small, Sorted, Values};

use super::{Encode, EncodingStrategy};
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    io::{Read, Write},
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
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Values::<Normal>::encode(self, writer, ctx)
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        Values::<Normal>::millibits(self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Values::<Normal>::decode(reader, ctx)
    }
}

#[test]
fn hashset() {
    use super::{assert_size, encoded_bits};
    assert_size!(HashSet::<usize>::new(), expect!["1"]);
    assert_size!(HashSet::from([0_usize]), expect!["1"]);
    assert_size!(HashSet::from([1_usize]), expect!["1"]);
    assert_size!(HashSet::from([5_usize]), expect!["2"]);
    expect!["6"].assert_eq(&encoded_bits!(HashSet::from([true, false])));
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
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Values::<Sorted>::encode(self, writer, ctx)
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        Values::<Sorted>::millibits(self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<W: Write>(
        value: &BTreeSet<u64>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.len().encode(writer, &mut ctx.size)?;
        let mut iter = value.iter().copied();
        if let Some(mut prev) = iter.next() {
            Small::encode(&prev, writer, &mut ctx.first)?;
            for v in iter {
                Small::encode(&(v - prev), writer, &mut ctx.diff)?;
                prev = v;
            }
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<u64>, std::io::Error> {
        let len = usize::decode(reader, &mut ctx.size)?;
        // Stage + collect: bulk-build from the sorted stream, as in
        // `Values<S> for BTreeSet` below.
        let mut values = Vec::with_capacity(len);
        if len > 0 {
            let mut prev = Small::decode(reader, &mut ctx.first)?;
            values.push(prev);
            for _ in 1..len {
                let diff: u64 = Small::decode(reader, &mut ctx.diff)?;
                prev += diff;
                values.push(prev);
            }
        }
        Ok(values.into_iter().collect())
    }
}

impl<T: Ord, S: EncodingStrategy<T>> EncodingStrategy<BTreeSet<T>> for Values<S> {
    type Context = SetContext<T, S>;
    fn encode<W: Write>(
        value: &BTreeSet<T>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.len().encode(writer, &mut ctx.len)?;
        for v in value {
            S::encode(v, writer, &mut ctx.values)?
        }
        Ok(())
    }
    fn millibits(value: &BTreeSet<T>, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = value.len().millibits(&mut ctx.len)?;
        for v in value {
            tot += S::millibits(v, &mut ctx.values)?;
        }
        Some(tot)
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<T>, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        // Stage in a Vec: the elements arrive in sorted order, and
        // `FromIterator` bulk-builds packed nodes from sorted input in O(n),
        // much faster than per-element `insert` (see the v2 twin of this
        // impl and OPTIMIZING.md's 2026-07-19 survey).
        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            values.push(S::decode(reader, &mut ctx.values)?);
        }
        Ok(values.into_iter().collect())
    }
}

impl<T: Hash + Eq, S: EncodingStrategy<T>> EncodingStrategy<HashSet<T>> for Values<S> {
    type Context = SetContext<T, S>;
    fn encode<W: Write>(
        value: &HashSet<T>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.len().encode(writer, &mut ctx.len)?;
        for v in value {
            S::encode(v, writer, &mut ctx.values)?
        }
        Ok(())
    }
    fn millibits(value: &HashSet<T>, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = value.len().millibits(&mut ctx.len)?;
        for v in value {
            tot += S::millibits(v, &mut ctx.values)?;
        }
        Some(tot)
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
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
    use super::encoded_bits;
    expect!["3"].assert_eq(&encoded_bits!(BTreeSet::<usize>::new()));
    expect!["6"].assert_eq(&encoded_bits!(BTreeSet::from([0_usize])));
    expect!["6"].assert_eq(&encoded_bits!(BTreeSet::from([1_usize])));
    expect!["8"].assert_eq(&encoded_bits!(BTreeSet::from([5_usize])));
    expect!["10"].assert_eq(&encoded_bits!(BTreeSet::from([0_usize, 1])));
    expect!["12"].assert_eq(&encoded_bits!(BTreeSet::from([0_usize, 1, 2])));
    expect!["40"].assert_eq(&encoded_bits!(BTreeSet::from_iter(0_usize..70)));
    expect!["86"].assert_eq(&encoded_bits!(BTreeSet::from_iter(0_usize..1024)));
    expect!["4"].assert_eq(&encoded_bits!(BTreeSet::from([false])));
    expect!["4"].assert_eq(&encoded_bits!(BTreeSet::from([true])));
    expect!["6"].assert_eq(&encoded_bits!(BTreeSet::from([false, true])));
    expect!["176"].assert_eq(&encoded_bits!(BTreeSet::from_iter(
        1_000_000_u64..1_001_024
    )));
    expect!["271"].assert_eq(&encoded_bits!(BTreeSet::from_iter(
        2_000_000_u64..2_002_048
    )));
}

#[test]
fn compact_btreeset() {
    use super::encoded_bits;
    use crate::Encoded;
    expect!["3"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::<u64>::new()
    )));
    expect!["10"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        0_u64
    ]))));
    expect!["10"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        1_u64
    ]))));
    expect!["11"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        5_u64
    ]))));
    expect!["40"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        u32::MAX as u64
    ]))));
    expect!["72"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        u64::MAX
    ]))));
    expect!["17"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        0_u64, 1
    ]))));
    expect!["21"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(BTreeSet::from([
        0_u64, 1, 2
    ]))));
    expect!["62"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(0_u64..70)
    )));
    expect!["141"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(0_u64..1024)
    )));
    expect!["159"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(1_000_000_u64..1_001_024)
    )));
    expect!["242"].assert_eq(&encoded_bits!(Encoded::<_, Small>::new(
        BTreeSet::from_iter(2_000_000_u64..2_002_048)
    )));
}
