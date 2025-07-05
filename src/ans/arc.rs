use super::Encode;
use std::{collections::HashMap, hash::Hash, ops::Deref, sync::Arc};

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
            is_cached: self.is_cached.clone(),
            context: self.context.clone(),
            index: self.index.clone(),
        }
    }
}

impl<T: Encode + Hash + PartialEq + Eq> Encode for Arc<T> {
    type Context = CacheContext<T>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
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
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
