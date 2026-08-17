use super::sentinel::{decode_elements, Sentinel};
use super::{Encode, Strategy};
use crate::{Mapping, Normal, Sorted};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[cfg(test)]
use expect_test::expect;

pub struct MapContext<K: Encode<SK>, V: Encode<SV>, SK, SV> {
    len: <usize as Encode>::Context,
    /// The key's and value's contexts, held as the *entry*'s context rather
    /// than as two fields — `(K, V): Encode<Mapping<SK, SV>>` codes exactly a
    /// key then a value under exactly these, so a map can name one entry as a
    /// type and hand a whole run of them to the sync decoder at once. Nothing
    /// about the coding changes; this is only how the two contexts are spelled.
    entry: <(K, V) as Encode<Mapping<SK, SV>>>::Context,
}
impl<K: Encode<SK>, V: Encode<SV>, SK, SV> Default for MapContext<K, V, SK, SV> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            entry: Default::default(),
        }
    }
}
impl<K: Encode<SK>, V: Encode<SV>, SK, SV> Clone for MapContext<K, V, SK, SV> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            entry: self.entry.clone(),
        }
    }
}

impl<K: Encode + Hash + Eq, V: Encode> Encode for HashMap<K, V> {
    type Context = MapContext<K, V, Normal, Normal>;
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for (k, v) in value {
            sentinel.encode(writer);
            Normal::encode(k, writer, &mut ctx.entry.0);
            Normal::encode(v, writer, &mut ctx.entry.1);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len: usize = <usize as Encode>::decode(reader, &mut ctx.len)?;
        let mut map = Self::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            map.insert(
                Encode::decode(reader, &mut ctx.entry.0)?,
                Encode::decode(reader, &mut ctx.entry.1)?,
            );
        }
        Ok(map)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len = <usize as Encode>::decode_async(reader, &mut ctx.len).await?;
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        decode_elements::<_, (K, V), Mapping<Normal, Normal>, _>(
            reader,
            &mut ctx.entry,
            len,
            &mut map,
        )
        .await?;
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
    K: Encode<Sorted>,
{
    type Context = MapContext<K, V, Sorted, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Mapping::<Sorted, Normal>::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Mapping::<Sorted, Normal>::decode(reader, ctx)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = Result<BTreeMap<K, V>, std::io::Error>> {
        <BTreeMap<K, V> as Encode<Mapping<Sorted, Normal>>>::decode_awaiting(reader, ctx)
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

impl<K: Ord + Encode<SK>, SK, V: Encode<SV>, SV> Encode<Mapping<SK, SV>> for BTreeMap<K, V> {
    type Context = MapContext<K, V, SK, SV>;
    #[inline]
    fn encode<E: super::EntropyCoder>(
        value: &BTreeMap<K, V>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        Normal::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for (k, v) in value {
            sentinel.encode(writer);
            <K as Encode<SK>>::encode(k, writer, &mut ctx.entry.0);
            <V as Encode<SV>>::encode(v, writer, &mut ctx.entry.1);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeMap<K, V>, std::io::Error> {
        let len: usize = <usize as Encode>::decode(reader, &mut ctx.len)?;
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
                <K as Encode<SK>>::decode(reader, &mut ctx.entry.0)?,
                <V as Encode<SV>>::decode(reader, &mut ctx.entry.1)?,
            ));
        }
        Ok(pairs.into_iter().collect())
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<BTreeMap<K, V>, std::io::Error> {
        let len: usize = <usize as Encode>::decode_async(reader, &mut ctx.len).await?;
        // Stage + collect — see the sync `decode` above for why.
        let mut pairs = Vec::with_capacity(super::capacity_for::<(K, V)>(len));
        decode_elements::<_, (K, V), Mapping<SK, SV>, _>(reader, &mut ctx.entry, len, &mut pairs)
            .await?;
        Ok(pairs.into_iter().collect())
    }
}

impl<K: Hash + Eq + Encode<SK>, SK, V: Encode<SV>, SV> Encode<Mapping<SK, SV>> for HashMap<K, V> {
    type Context = MapContext<K, V, SK, SV>;
    #[inline]
    fn encode<E: super::EntropyCoder>(
        value: &HashMap<K, V>,
        writer: &mut E,
        ctx: &mut Self::Context,
    ) {
        Normal::encode(&value.len(), writer, &mut ctx.len);
        let mut sentinel = Sentinel::new();
        for (k, v) in value {
            sentinel.encode(writer);
            <K as Encode<SK>>::encode(k, writer, &mut ctx.entry.0);
            <V as Encode<SV>>::encode(v, writer, &mut ctx.entry.1);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len: usize = <usize as Encode>::decode(reader, &mut ctx.len)?;
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        let mut sentinel = Sentinel::new();
        for _ in 0..len {
            sentinel.decode(reader)?;
            map.insert(
                <K as Encode<SK>>::decode(reader, &mut ctx.entry.0)?,
                <V as Encode<SV>>::decode(reader, &mut ctx.entry.1)?,
            );
        }
        Ok(map)
    }

    /// Length-driven: an arbitrary number of entries.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<HashMap<K, V>, std::io::Error> {
        let len: usize = <usize as Encode>::decode_async(reader, &mut ctx.len).await?;
        let mut map = HashMap::with_capacity(super::capacity_for::<(K, V)>(len));
        decode_elements::<_, (K, V), Mapping<SK, SV>, _>(reader, &mut ctx.entry, len, &mut map)
            .await?;
        Ok(map)
    }
}
