use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
use crate::LowCardinality;
use std::{collections::HashMap, hash::Hash, ops::Deref, rc::Rc, sync::Arc};

pub struct CacheContext<T: Encode + Hash + PartialEq + Eq> {
    cached: HashMap<Arc<T>, usize>,
    cache: Vec<Arc<T>>,
    is_cached: <bool as Encode>::Context,
    context: T::Context,
    index: <usize as Encode>::Context,
}

impl<T: Encode + Hash + PartialEq + Eq> Default for CacheContext<T> {
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

impl<T: Encode + Hash + PartialEq + Eq> Clone for CacheContext<T> {
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
            cache: self.cache.clone(),
            is_cached: self.is_cached,
            context: self.context.clone(),
            index: self.index.clone(),
        }
    }
}

impl<T: Encode + Hash + PartialEq + Eq> Encode for Arc<T> {
    type Context = CacheContext<T>;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(self).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(self.clone(), ctx.cached.len());
            self.deref().encode(writer, &mut ctx.context)
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let value = Arc::new(T::decode(reader, &mut ctx.context)?);
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

impl Encode for Arc<str> {
    type Context = <LowCardinality as EncodingStrategy<Arc<str>>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        LowCardinality::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        LowCardinality::decode(reader, ctx)
    }
}

pub struct RcCacheContext<T: Encode + Hash + PartialEq + Eq> {
    cached: HashMap<Rc<T>, usize>,
    cache: Vec<Rc<T>>,
    is_cached: <bool as Encode>::Context,
    context: T::Context,
    index: <usize as Encode>::Context,
}

impl<T: Encode + Hash + PartialEq + Eq> Default for RcCacheContext<T> {
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

impl<T: Encode + Hash + PartialEq + Eq> Clone for RcCacheContext<T> {
    fn clone(&self) -> Self {
        Self {
            cached: self.cached.clone(),
            cache: self.cache.clone(),
            is_cached: self.is_cached,
            context: self.context.clone(),
            index: self.index.clone(),
        }
    }
}

impl<T: Encode + Hash + PartialEq + Eq> Encode for Rc<T> {
    type Context = RcCacheContext<T>;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(self).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(self.clone(), ctx.cached.len());
            self.deref().encode(writer, &mut ctx.context)
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let value = Rc::new(T::decode(reader, &mut ctx.context)?);
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

impl Encode for Rc<str> {
    type Context = <LowCardinality as EncodingStrategy<Rc<str>>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        LowCardinality::encode(self, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        LowCardinality::decode(reader, ctx)
    }
}
