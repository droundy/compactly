use super::sentinel::Sentinel;
use super::{Encode, EncodingStrategy};
use crate::{Mapping, Normal, Sorted};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

#[cfg(test)]
use expect_test::expect;

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
        let mut sentinel = Sentinel::new();
        for (k, v) in self {
            sentinel.encode(writer);
            k.encode(writer, &mut ctx.key);
            v.encode(writer, &mut ctx.value);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len = Encode::decode(reader, &mut ctx.len)?;
        let mut map = Self::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
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
    assert_size!(HashMap::<usize, usize>::new(), expect!["1"]);
    assert_size!(HashMap::from([(0_usize, 0_usize)]), expect!["1"]);
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
    assert_size!(BTreeMap::<usize, usize>::new(), expect!["1"]);
    assert_size!(BTreeMap::from([(0_usize, 0_usize)]), expect!["1"]);
    assert_size!(
        BTreeMap::from_iter((0_usize..2).map(|v| (v, v))),
        expect!["2"]
    );
    assert_size!(
        BTreeMap::from_iter((0_usize..1_000).map(|v| (v, v))),
        expect!["1018"]
    );
    assert_size!(
        BTreeMap::from_iter((1_000_usize..2_000).map(|v| (v, v))),
        expect!["1078"]
    );
    assert_size!(
        BTreeMap::from_iter((1_000_000_usize..1_001_000).map(|v| (v, v))),
        expect!["2044"]
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
        let mut sentinel = Sentinel::new();
        for (k, v) in value {
            sentinel.encode(writer);
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
        // Stage + collect: the keys arrive in sorted order, and `FromIterator`
        // bulk-builds packed nodes from sorted input in O(n) — see
        // `Values<S> for BTreeSet` in sets.rs. Identical to the old insert
        // loop for every valid stream (a `BTreeMap` has no duplicate keys to
        // emit) and for any key whose `Ord` agrees with its `Eq`; the two
        // diverge only on a corrupt stream carrying an Ord-equal key run of a
        // coarse-`Ord` key type, where `collect` keeps every Eq-distinct
        // entry. Not UB, which is all decode promises for corrupt input.
        let mut pairs = Vec::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            pairs.push((
                SK::decode(reader, &mut ctx.key)?,
                SV::decode(reader, &mut ctx.value)?,
            ));
        }
        Ok(pairs.into_iter().collect())
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
        let mut sentinel = Sentinel::new();
        for (k, v) in value {
            sentinel.encode(writer);
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
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            map.insert(
                SK::decode(reader, &mut ctx.key)?,
                SV::decode(reader, &mut ctx.value)?,
            );
        }
        Ok(map)
    }
}

impl<K: Encode + Hash + Eq, V: Encode> super::DecodeAsync<HashMap<K, V>> for Normal
where
    Normal: super::DecodeAsync<K> + super::EncodingStrategy<K, Context = <K as Encode>::Context>,
    Normal: super::DecodeAsync<V> + super::EncodingStrategy<V, Context = <V as Encode>::Context>,
{
    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len = <Normal as super::DecodeAsync<usize>>::decode_async(reader, &mut ctx.len).await?;
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode_async(reader).await?;
            let k = <Normal as super::DecodeAsync<K>>::decode_async(reader, &mut ctx.key).await?;
            let v = <Normal as super::DecodeAsync<V>>::decode_async(reader, &mut ctx.value).await?;
            map.insert(k, v);
        }
        Ok(map)
    }
}

impl<K: Ord, V: Encode> super::DecodeAsync<BTreeMap<K, V>> for Normal
where
    Sorted: super::DecodeAsync<K>,
    Normal: super::DecodeAsync<V> + super::EncodingStrategy<V, Context = <V as Encode>::Context>,
{
    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<BTreeMap<K, V>, std::io::Error>> {
        <Mapping<Sorted, Normal> as super::DecodeAsync<BTreeMap<K, V>>>::decode_awaiting(
            reader, ctx,
        )
    }
}

impl<K: Ord, SK: super::DecodeAsync<K>, V, SV: super::DecodeAsync<V>>
    super::DecodeAsync<BTreeMap<K, V>> for Mapping<SK, SV>
{
    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeMap<K, V>, std::io::Error> {
        let len: usize =
            <Normal as super::DecodeAsync<usize>>::decode_async(reader, &mut ctx.len).await?;
        // Stage + collect — see the sync `decode` above for why.
        let mut pairs = Vec::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode_async(reader).await?;
            let k = SK::decode_async(reader, &mut ctx.key).await?;
            let v = SV::decode_async(reader, &mut ctx.value).await?;
            pairs.push((k, v));
        }
        Ok(pairs.into_iter().collect())
    }
}

impl<K: Hash + Eq, SK: super::DecodeAsync<K>, V, SV: super::DecodeAsync<V>>
    super::DecodeAsync<HashMap<K, V>> for Mapping<SK, SV>
{
    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len: usize =
            <Normal as super::DecodeAsync<usize>>::decode_async(reader, &mut ctx.len).await?;
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode_async(reader).await?;
            let k = SK::decode_async(reader, &mut ctx.key).await?;
            let v = SV::decode_async(reader, &mut ctx.value).await?;
            map.insert(k, v);
        }
        Ok(map)
    }
}
