use crate::Encode;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    io::{Read, Write},
};

pub struct MapContext<K: Encode, V: Encode> {
    len: <usize as Encode>::Context,
    key: K::Context,
    value: V::Context,
}
impl<K: Encode, V: Encode> Default for MapContext<K, V> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            key: Default::default(),
            value: Default::default(),
        }
    }
}

impl<K: Encode + Hash + Eq, V: Encode> Encode for HashMap<K, V> {
    type Context = MapContext<K, V>;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.len)?;
        for (k, v) in self {
            k.encode(writer, &mut ctx.key)?;
            v.encode(writer, &mut ctx.value)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len = Encode::decode(reader, &mut ctx.len)?;
        let mut map = Self::with_capacity(len);
        for _ in 0..len {
            map.insert(
                Encode::decode(reader, &mut ctx.key)?,
                Encode::decode(reader, &mut ctx.value)?,
            );
        }
        Ok(map)
    }
}

#[test]
fn hashmap() {
    use crate::assert_size;
    assert_size!(HashMap::<usize, usize>::new(), 1);
    assert_size!(HashMap::from([(0_usize, 0_usize)]), 1);
    // Sizes of larger hash maps are unpredictable because the values come out
    // in arbitrary orders.
}

impl<K: Encode + Ord, V: Encode> Encode for BTreeMap<K, V> {
    type Context = MapContext<K, V>;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.len)?;
        for (k, v) in self {
            k.encode(writer, &mut ctx.key)?;
            v.encode(writer, &mut ctx.value)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut map = Self::new();
        for _ in 0..len {
            map.insert(
                Encode::decode(reader, &mut ctx.key)?,
                Encode::decode(reader, &mut ctx.value)?,
            );
        }
        Ok(map)
    }
}

#[test]
fn btreemap() {
    use crate::assert_size;
    assert_size!(BTreeMap::<usize, usize>::new(), 1);
    assert_size!(BTreeMap::from([(0_usize, 0_usize)]), 1);
    assert_size!(BTreeMap::from_iter((0_usize..2).map(|v| (v, v))), 3);
    assert_size!(BTreeMap::from_iter((0_usize..1_000).map(|v| (v, v))), 2486);
    assert_size!(
        BTreeMap::from_iter((1_000_usize..2_000).map(|v| (v, v))),
        2472
    );
    assert_size!(
        BTreeMap::from_iter((1_000_000_usize..1_001_000).map(|v| (v, v))),
        2720
    );
}
