use super::{Compact, Encode, EncodingStrategy};
use crate::{Normal, Values};
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    io::{Read, Write},
};

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
    use super::assert_size;
    assert_size!(HashSet::<usize>::new(), 1);
    assert_size!(HashSet::from([0_usize]), 1);
    assert_size!(HashSet::from([1_usize]), 1);
    assert_size!(HashSet::from([5_usize]), 2);
    assert_size!(HashSet::from([0_usize, 1]), 2);
    // assert_size!(HashSet::from([0_usize, 1, 2]), 3);
    // Sizes of larger hash sets are unpredictable because the values come out
    // in arbitrary orders.
}

impl<T: Encode + Ord> Encode for BTreeSet<T> {
    type Context = SetContext<T, Normal>;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.len)?;
        for v in self {
            v.encode(writer, &mut ctx.values)?
        }
        Ok(())
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut set = Self::new();
        for _ in 0..len {
            set.insert(Encode::decode(reader, &mut ctx.values)?);
        }
        Ok(set)
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
        for value in value {
            S::encode(value, writer, &mut ctx.values)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<W: Write>(
        value: &HashSet<T>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.len().encode(writer, &mut ctx.len)?;
        for value in value {
            S::encode(value, writer, &mut ctx.values)?;
        }
        Ok(())
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

#[derive(Default)]
pub struct CompactU64Set {
    size: <usize as Encode>::Context,
    first: <Compact<u64> as Encode>::Context,
    diff: <Compact<u64> as Encode>::Context,
}

impl Encode for Compact<BTreeSet<u64>> {
    type Context = CompactU64Set;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.size)?;
        let mut iter = self.0.iter().copied();
        if let Some(mut prev) = iter.next() {
            Compact(prev).encode(writer, &mut ctx.first)?;
            for v in iter {
                let diff = Compact(v - prev);
                diff.encode(writer, &mut ctx.diff)?;
                prev = v;
            }
        }
        Ok(())
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut out = BTreeSet::new();
        let len = usize::decode(reader, &mut ctx.size)?;
        if len > 0 {
            let Compact(mut prev) = Compact::<u64>::decode(reader, &mut ctx.first)?;
            out.insert(prev);
            for _ in 1..len {
                let Compact(diff) = Compact::<u64>::decode(reader, &mut ctx.diff)?;
                prev += diff;
                out.insert(prev);
            }
        }
        Ok(Compact(out))
    }
}

impl super::EncodingStrategy<BTreeSet<u64>> for super::Small {
    type Context = CompactU64Set;
    fn encode<W: Write>(
        value: &BTreeSet<u64>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.len().encode(writer, &mut ctx.size)?;
        let mut iter = value.iter().copied();
        if let Some(mut prev) = iter.next() {
            Compact(prev).encode(writer, &mut ctx.first)?;
            for v in iter {
                let diff = Compact(v - prev);
                diff.encode(writer, &mut ctx.diff)?;
                prev = v;
            }
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<BTreeSet<u64>, std::io::Error> {
        let mut out = BTreeSet::new();
        let len = usize::decode(reader, &mut ctx.size)?;
        if len > 0 {
            let Compact(mut prev) = Compact::<u64>::decode(reader, &mut ctx.first)?;
            out.insert(prev);
            for _ in 1..len {
                let Compact(diff) = Compact::<u64>::decode(reader, &mut ctx.diff)?;
                prev += diff;
                out.insert(prev);
            }
        }
        Ok(out)
    }
}

#[test]
fn btreeset() {
    use super::assert_bits;
    assert_bits!(BTreeSet::<usize>::new(), 3);
    assert_bits!(BTreeSet::from([0_usize]), 6);
    assert_bits!(BTreeSet::from([1_usize]), 6);
    assert_bits!(BTreeSet::from([5_usize]), 11);
    assert_bits!(BTreeSet::from([0_usize, 1]), 9);
    assert_bits!(BTreeSet::from([0_usize, 1, 2]), 12);
    assert_bits!(BTreeSet::from_iter(0_usize..70), 505);
    assert_bits!(BTreeSet::from_iter(0_usize..1024), 10183);
    assert_bits!(BTreeSet::from([false]), 4);
    assert_bits!(BTreeSet::from([true]), 4);
    assert_bits!(BTreeSet::from([false, true]), 6);
}

#[test]
fn compact_btreeset() {
    use super::assert_bits;
    assert_bits!(Compact(BTreeSet::<u64>::new()), 3);
    assert_bits!(Compact(BTreeSet::from([0_u64])), 10);
    assert_bits!(Compact(BTreeSet::from([1_u64])), 10);
    assert_bits!(Compact(BTreeSet::from([5_u64])), 11);
    assert_bits!(Compact(BTreeSet::from([u32::MAX as u64])), 40);
    assert_bits!(Compact(BTreeSet::from([u64::MAX])), 72);
    assert_bits!(Compact(BTreeSet::from([0_u64, 1])), 17);
    assert_bits!(Compact(BTreeSet::from([0_u64, 1, 2])), 21);
    assert_bits!(Compact(BTreeSet::from_iter(0_u64..70)), 64);
    assert_bits!(Compact(BTreeSet::from_iter(0_u64..1024)), 125);
    assert_bits!(Compact(BTreeSet::from_iter(1_000_000_u64..1_001_024)), 143);
}
