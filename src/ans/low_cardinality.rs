use super::{Encode, EncodingStrategy, LowCardinality};
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct CacheContext<T: Encode + Clone + Hash + PartialEq + Eq> {
    cached: HashMap<T, usize>,
    cache: Vec<T>,
    is_cached: <bool as Encode>::Context,
    context: T::Context,
    index: <usize as Encode>::Context,
}

impl<T: Encode + Clone + Hash + PartialEq + Eq> Default for CacheContext<T> {
    #[inline]
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

macro_rules! impl_low_cardinality {
    ($t:ty, $mod:ident) => {
        mod $mod {
            use super::{CacheContext, Encode, EncodingStrategy, LowCardinality};
            impl EncodingStrategy<$t> for LowCardinality {
                type Context = CacheContext<$t>;
                #[inline]
                fn encode<E: super::super::EntropyCoder>(
                    value: &$t,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    let looked_up = ctx.cached.get(value).copied();
                    looked_up.is_some().encode(writer, &mut ctx.is_cached);
                    if let Some(idx) = looked_up {
                        idx.encode(writer, &mut ctx.index)
                    } else {
                        ctx.cached.insert(value.clone(), ctx.cached.len());
                        value.encode(writer, &mut ctx.context)
                    }
                }
                #[inline]
                fn decode<R: std::io::Read>(
                    reader: &mut super::super::Reader<R>,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
                    if is_cached {
                        let idx = usize::decode(reader, &mut ctx.index)?;
                        ctx.cache
                            .get(idx)
                            .cloned()
                            .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
                    } else {
                        let value = <$t>::decode(reader, &mut ctx.context)?;
                        ctx.cache.push(value.clone());
                        Ok(value)
                    }
                }
            }
        }
    };
}

impl_low_cardinality!(String, string);
impl_low_cardinality!(Vec<u8>, bytes);
impl_low_cardinality!(u64, mod_u64);

impl<T> EncodingStrategy<Vec<T>> for LowCardinality
where
    T: Encode,
    LowCardinality: EncodingStrategy<T>,
{
    type Context = (
        <usize as Encode>::Context,
        <LowCardinality as EncodingStrategy<T>>::Context,
    );
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        value.len().encode(writer, &mut ctx.0);
        for v in value {
            LowCardinality::encode(&v, writer, &mut ctx.1);
        }
    }
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let n = usize::decode(reader, &mut ctx.0)?;
        let mut x = Vec::with_capacity(n);
        for _ in 0..n {
            x.push(LowCardinality::decode(reader, &mut ctx.1)?);
        }
        Ok(x)
    }
}

#[test]
fn low_cardinality() {
    use super::assert_bits;
    use crate::Encoded;

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

    assert_bits!(v.clone(), 284470);
    assert_bits!(low.clone(), 1677);
    assert_bits!(strings.clone().to_vec(), 613);
    assert_bits!(
        strings
            .iter()
            .cloned()
            .map(|v| Encoded::<_, LowCardinality>::new(v))
            .collect::<Vec<_>>(),
        615
    );
}
