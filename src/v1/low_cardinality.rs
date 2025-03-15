use super::{Encode, EncodingStrategy, LowCardinality};
use std::{collections::HashMap, hash::Hash};

pub struct CacheContext<T: Encode + Clone + Hash + PartialEq + Eq> {
    cached: HashMap<T, usize>,
    cache: Vec<T>,
    is_cached: <bool as Encode>::Context,
    context: T::Context,
    index: <usize as Encode>::Context,
}

impl<T: Encode + Clone + Hash + PartialEq + Eq> Default for CacheContext<T> {
    fn default() -> Self {
        Self {
            cached: HashMap::new(),
            cache: Vec::new(),
            is_cached: Default::default(),
            context: Default::default(),
            index: Default::default(),
        }
    }
}

impl<T: Encode + Clone + Hash + PartialEq + Eq> EncodingStrategy<T> for LowCardinality {
    type Context = CacheContext<T>;
    fn encode<W: std::io::Write>(
        value: &T,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let looked_up = ctx.cached.get(value).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached)?;
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            value.encode(writer, &mut ctx.context)
        }
    }
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let value = T::decode(reader, &mut ctx.context)?;
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

#[test]
fn low_cardinality() {
    use super::{assert_bits, Encoded};

    let strings = [
        b"hello world, this is the very first string".to_vec(),
        b"This is a second string, which is like unto the first, and yet quite different".to_vec(),
    ];
    let mut v = Vec::new();
    for i in 0..1024 {
        v.push(if i % 3 == 0 {
            strings[0].clone()
        } else {
            strings[1].clone()
        });
    }
    let low = v
        .iter()
        .cloned()
        .map(|v| Encoded::<_, LowCardinality>::new(v))
        .collect::<Vec<_>>();

    assert_bits!(v.clone(), 276134);
    assert_bits!(low.clone(), 1631);
    assert_bits!(strings.clone().to_vec(), 612);
    assert_bits!(
        strings
            .iter()
            .cloned()
            .map(|v| Encoded::<_, LowCardinality>::new(v))
            .collect::<Vec<_>>(),
        614
    );
}
