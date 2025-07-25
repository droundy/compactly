use super::{Encode, EncodingStrategy};
use crate::{Mapping, Normal, Sorted};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

pub struct MapContext<K, V, SK: EncodingStrategy<K>, SV: EncodingStrategy<V>> {
    len: <usize as Encode>::Context,
    key: SK::Context,
    value: SV::Context,
}
impl<K, V, SK: EncodingStrategy<K>, SV: EncodingStrategy<V>> Default for MapContext<K, V, SK, SV> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            key: Default::default(),
            value: Default::default(),
        }
    }
}
impl<K, V, SK: EncodingStrategy<K>, SV: EncodingStrategy<V>> Clone for MapContext<K, V, SK, SV> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl<K: Encode + Hash + Eq, V: Encode> Encode for HashMap<K, V> {
    type Context = MapContext<K, V, Normal, Normal>;
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.len().encode(writer, &mut ctx.len);
        for (k, v) in self {
            k.encode(writer, &mut ctx.key);
            v.encode(writer, &mut ctx.value);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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
    use super::assert_size;
    assert_size!(HashMap::<usize, usize>::new(), 1);
    assert_size!(HashMap::from([(0_usize, 0_usize)]), 1);
    // Sizes of larger hash maps are unpredictable because the values come out
    // in arbitrary orders.
}

impl<K: Ord, V: Encode> Encode for BTreeMap<K, V>
where
    Sorted: EncodingStrategy<K>,
{
    type Context = MapContext<K, V, Sorted, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        Mapping::<Sorted, Normal>::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Mapping::<Sorted, Normal>::decode(reader, ctx)
    }
}

#[test]
fn btreemap() {
    use super::assert_size;
    assert_size!(BTreeMap::<usize, usize>::new(), 1);
    assert_size!(BTreeMap::from([(0_usize, 0_usize)]), 1);
    assert_size!(BTreeMap::from_iter((0_usize..2).map(|v| (v, v))), 2);
    assert_size!(BTreeMap::from_iter((0_usize..1_000).map(|v| (v, v))), 934);
    assert_size!(
        BTreeMap::from_iter((1_000_usize..2_000).map(|v| (v, v))),
        975
    );
    assert_size!(
        BTreeMap::from_iter((1_000_000_usize..1_001_000).map(|v| (v, v))),
        1005
    );
}

impl<K: Ord, SK: EncodingStrategy<K>, V, SV: EncodingStrategy<V>> EncodingStrategy<BTreeMap<K, V>>
    for Mapping<SK, SV>
{
    type Context = MapContext<K, V, SK, SV>;
    #[inline]
    fn encode<E: super::EntropyCoder>(
        value: &BTreeMap<K, V>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        value.len().encode(writer, &mut ctx.len);
        for (k, v) in value {
            SK::encode(k, writer, &mut ctx.key);
            SV::encode(v, writer, &mut ctx.value);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeMap<K, V>, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut map = BTreeMap::new();
        for _ in 0..len {
            map.insert(
                SK::decode(reader, &mut ctx.key)?,
                SV::decode(reader, &mut ctx.value)?,
            );
        }
        Ok(map)
    }
}

impl<K: Hash + Eq, SK: EncodingStrategy<K>, V, SV: EncodingStrategy<V>>
    EncodingStrategy<HashMap<K, V>> for Mapping<SK, SV>
{
    type Context = MapContext<K, V, SK, SV>;
    #[inline]
    fn encode<E: super::EntropyCoder>(
        value: &HashMap<K, V>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        value.len().encode(writer, &mut ctx.len);
        for (k, v) in value {
            SK::encode(k, writer, &mut ctx.key);
            SV::encode(v, writer, &mut ctx.value);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len: usize = Encode::decode(reader, &mut ctx.len)?;
        let mut map = HashMap::with_capacity(len);
        for _ in 0..len {
            map.insert(
                SK::decode(reader, &mut ctx.key)?,
                SV::decode(reader, &mut ctx.value)?,
            );
        }
        Ok(map)
    }
}
