use crate::{Compact, Encode};
use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    io::{Read, Write},
};

pub struct SetContext<T: Encode> {
    len: <usize as Encode>::Context,
    values: T::Context,
}
impl<T: Encode> Default for SetContext<T> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}

impl<T: Encode + Hash + Eq> Encode for HashSet<T> {
    type Context = SetContext<T>;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.len)?;
        for v in self {
            v.encode(writer, &mut ctx.values)?
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len = Encode::decode(reader, &mut ctx.len)?;
        let mut set = HashSet::with_capacity(len);
        for _ in 0..len {
            set.insert(Encode::decode(reader, &mut ctx.values)?);
        }
        Ok(set)
    }
}

#[test]
fn hashset() {
    use crate::assert_size;
    assert_size!(HashSet::<usize>::new(), 1);
    assert_size!(HashSet::from([0_usize]), 1);
    assert_size!(HashSet::from([1_usize]), 1);
    assert_size!(HashSet::from([5_usize]), 2);
    assert_size!(HashSet::from([0_usize, 1]), 2);
    assert_size!(HashSet::from([0_usize, 1, 2]), 3);
    // Sizes of larger hash sets are unpredictable because the values come out
    // in arbitrary orders.
}

impl<T: Encode + Ord> Encode for BTreeSet<T> {
    type Context = SetContext<T>;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.len)?;
        for v in self {
            v.encode(writer, &mut ctx.values)?
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

#[derive(Default)]
pub struct CompactU64Set {
    size: <usize as Encode>::Context,
    first: <Compact<u64> as Encode>::Context,
    diff: <Compact<u64> as Encode>::Context,
}

impl Encode for Compact<BTreeSet<u64>> {
    type Context = CompactU64Set;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
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
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

#[test]
fn btreeset() {
    use crate::assert_bits;
    assert_bits!(BTreeSet::<usize>::new(), 1);
    assert_bits!(BTreeSet::from([0_usize]), 3);
    assert_bits!(BTreeSet::from([1_usize]), 4);
    assert_bits!(BTreeSet::from([5_usize]), 8);
    assert_bits!(BTreeSet::from([0_usize, 1]), 8);
    assert_bits!(BTreeSet::from([0_usize, 1, 2]), 12);
    assert_bits!(BTreeSet::from_iter(0_usize..70), 503);
    assert_bits!(BTreeSet::from_iter(0_usize..1024), 10168);
    assert_bits!(BTreeSet::from([false]), 3);
    assert_bits!(BTreeSet::from([true]), 3);
    assert_bits!(BTreeSet::from([false, true]), 7);
}

#[test]
fn compact_btreeset() {
    use crate::assert_bits;
    assert_bits!(Compact(BTreeSet::<u64>::new()), 1);
    assert_bits!(Compact(BTreeSet::from([0_u64])), 9);
    assert_bits!(Compact(BTreeSet::from([1_u64])), 8);
    assert_bits!(Compact(BTreeSet::from([5_u64])), 10);
    assert_bits!(Compact(BTreeSet::from([u32::MAX as u64])), 39);
    assert_bits!(Compact(BTreeSet::from([u64::MAX])), 72);
    assert_bits!(Compact(BTreeSet::from([0_u64, 1])), 17);
    assert_bits!(Compact(BTreeSet::from([0_u64, 1, 2])), 21);
    assert_bits!(Compact(BTreeSet::from_iter(0_u64..70)), 58);
    assert_bits!(Compact(BTreeSet::from_iter(0_u64..1024)), 115);
    assert_bits!(Compact(BTreeSet::from_iter(1_000_000_u64..1_001_024)), 133);
}
