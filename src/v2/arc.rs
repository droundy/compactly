use super::{Encode, EntropyCoder, EntropyDecoder, Strategy};
use crate::{LowCardinality, Normal};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

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
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(value).copied();
        Normal::encode(&looked_up.is_some(), writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            Normal::encode(&idx, writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            Normal::encode(value.deref(), writer, &mut ctx.context)
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let is_cached = <bool as Encode>::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = <usize as Encode>::decode(reader, &mut ctx.index)?;
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

    /// A hit codes the cache index, a miss the value; the flag is coded either way.
    const MAX_BYTES: usize = {
        let index = <usize as Encode>::MAX_BYTES;
        let value = <T as Encode>::MAX_BYTES;
        let worst = if index > value { index } else { value };
        <bool as Encode>::MAX_BYTES.saturating_add(worst)
    };

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Arc<T>, std::io::Error> {
        let is_cached = <bool as Encode>::decode_async(reader, &mut ctx.is_cached).await?;
        if is_cached {
            let idx = <usize as Encode>::decode_async(reader, &mut ctx.index).await?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let value = Arc::new(<T as Encode>::decode_async(reader, &mut ctx.context).await?);
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

impl Encode for Arc<str> {
    type Context = <Arc<str> as Encode<LowCardinality>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        LowCardinality::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        LowCardinality::decode(reader, ctx)
    }

    /// Dictionary-coded strings: unbounded, like the `String` behind them.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Arc<str>, std::io::Error> {
        <Arc<str> as Encode<LowCardinality>>::decode_async(reader, ctx).await
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
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(value).copied();
        Normal::encode(&looked_up.is_some(), writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            Normal::encode(&idx, writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            Normal::encode(value.deref(), writer, &mut ctx.context)
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let is_cached = <bool as Encode>::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = <usize as Encode>::decode(reader, &mut ctx.index)?;
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

    /// A hit codes the cache index, a miss the value; the flag is coded either way.
    const MAX_BYTES: usize = {
        let index = <usize as Encode>::MAX_BYTES;
        let value = <T as Encode>::MAX_BYTES;
        let worst = if index > value { index } else { value };
        <bool as Encode>::MAX_BYTES.saturating_add(worst)
    };

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Rc<T>, std::io::Error> {
        let is_cached = <bool as Encode>::decode_async(reader, &mut ctx.is_cached).await?;
        if is_cached {
            let idx = <usize as Encode>::decode_async(reader, &mut ctx.index).await?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let value = Rc::new(<T as Encode>::decode_async(reader, &mut ctx.context).await?);
            ctx.cache.push(value.clone());
            Ok(value)
        }
    }
}

impl Encode for Rc<str> {
    type Context = <Rc<str> as Encode<LowCardinality>>::Context;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        LowCardinality::encode(value, writer, ctx)
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        LowCardinality::decode(reader, ctx)
    }

    /// Dictionary-coded strings: unbounded, like the `String` behind them.
    const MAX_BYTES: usize = usize::MAX;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Rc<str>, std::io::Error> {
        <Rc<str> as Encode<LowCardinality>>::decode_async(reader, ctx).await
    }
}
