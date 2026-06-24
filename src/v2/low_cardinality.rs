use super::{Encode, EncodingStrategy, LowCardinality};
use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};

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
                fn decode<D: super::super::EntropyDecoder>(
                    reader: &mut D,
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
impl_low_cardinality!(u16, mod_u16);
impl_low_cardinality!(u32, mod_u32);
impl_low_cardinality!(u64, mod_u64);

// Arc<str> needs its own context: the encode-side HashMap uses Arc<str> as keys
// (deduplication by content) and the decode-side cache holds Arc<str> values so
// that cache hits are just a cheap refcount increment rather than a new allocation.
#[derive(Default, Clone)]
pub struct ArcStrCacheContext {
    cached: HashMap<Arc<str>, usize>,
    cache: Vec<Arc<str>>,
    is_cached: <bool as Encode>::Context,
    string_ctx: <String as Encode>::Context,
    index: <usize as Encode>::Context,
}

impl EncodingStrategy<Arc<str>> for LowCardinality {
    type Context = ArcStrCacheContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Arc<str>, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(value.as_ref()).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            value.to_string().encode(writer, &mut ctx.string_ctx)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Arc<str>, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let s = String::decode(reader, &mut ctx.string_ctx)?;
            let value: Arc<str> = Arc::from(s.as_str());
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

#[derive(Default, Clone)]
pub struct RcStrCacheContext {
    cached: HashMap<Rc<str>, usize>,
    cache: Vec<Rc<str>>,
    is_cached: <bool as Encode>::Context,
    string_ctx: <String as Encode>::Context,
    index: <usize as Encode>::Context,
}

impl EncodingStrategy<Rc<str>> for LowCardinality {
    type Context = RcStrCacheContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Rc<str>, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(value.as_ref()).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            value.to_string().encode(writer, &mut ctx.string_ctx)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Rc<str>, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let s = String::decode(reader, &mut ctx.string_ctx)?;
            let value: Rc<str> = Rc::from(s.as_str());
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

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
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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

    assert_bits!(v.clone(), 284430);
    assert_bits!(low.clone(), 1673);
    assert_bits!(strings.clone().to_vec(), 610);
    assert_bits!(
        strings
            .iter()
            .cloned()
            .map(|v| Encoded::<_, LowCardinality>::new(v))
            .collect::<Vec<_>>(),
        612
    );
}
