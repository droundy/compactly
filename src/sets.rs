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

#[test]
fn btreeset() {
    use crate::assert_size;
    assert_size!(BTreeSet::<usize>::new(), 1);
    assert_size!(BTreeSet::from([0_usize]), 1);
    assert_size!(BTreeSet::from([1_usize]), 1);
    assert_size!(BTreeSet::from([5_usize]), 2);
    assert_size!(BTreeSet::from([0_usize, 1]), 2);
    assert_size!(BTreeSet::from([0_usize, 1, 2]), 3);
    assert_size!(BTreeSet::from_iter(0_usize..70), 64);
    assert_size!(BTreeSet::from_iter(0_usize..1024), 1272);
    assert_size!(BTreeSet::from_iter(0_usize..1_000_000), 1_392_024);
    assert_size!(BTreeSet::from([false]), 1);
    assert_size!(BTreeSet::from([true]), 1);
    assert_size!(BTreeSet::from([false, true]), 2);
}

pub struct CompactU64Set {
    first: <Option<u64> as Encode>::Context,
    diff: <usize as Encode>::Context,
}

// impl Encode for Compact<BTreeSet<u64>> {
//     type Context = CompactU64Set;
//     fn encode<W: Write>(
//         &self,
//         writer: &mut cabac::vp8::VP8Writer<W>,
//         ctx: &mut Self::Context,
//     ) -> Result<(), std::io::Error> {
//         let mut iter = self.0.iter();
//     }
// }
