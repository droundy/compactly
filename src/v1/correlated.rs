use std::{
    collections::HashMap,
    io::{Read, Write},
};

use super::Encode;

pub struct Correlated;

pub trait EncodeCorrelated: Encode + std::hash::Hash + Eq + Clone {
    fn correlated_encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        base_ctx: &mut <Self as Encode>::Context,
        correlated_ctx: &mut <Self as Encode>::Context,
    ) -> Result<(), std::io::Error>;

    fn correlated_decode<R: Read>(
        reader: &mut super::Reader<R>,
        base_ctx: &mut <Self as Encode>::Context,
        correlated_ctx: &mut <Self as Encode>::Context,
    ) -> Result<Self, std::io::Error>;
}

pub struct CorrelatedContext<T: Encode + std::hash::Hash + Eq> {
    base: <T as Encode>::Context,
    correlated: HashMap<T, <T as Encode>::Context>,
    previous: Option<T>,
}
impl<T: Encode + std::hash::Hash + Eq> Default for CorrelatedContext<T> {
    fn default() -> Self {
        Self {
            base: Default::default(),
            correlated: Default::default(),
            previous: None,
        }
    }
}

impl<T: EncodeCorrelated> super::EncodingStrategy<T> for Correlated {
    type Context = CorrelatedContext<T>;

    fn encode<W: Write>(
        value: &T,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if let Some(previous) = std::mem::take(&mut ctx.previous) {
            let correlated_ctx = ctx.correlated.entry(previous).or_default();
            value.correlated_encode(writer, &mut ctx.base, correlated_ctx)?;
        } else {
            value.encode(writer, &mut ctx.base)?;
        }
        ctx.previous = Some(value.clone());
        Ok(())
    }

    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<T, std::io::Error> {
        let out = if let Some(previous) = std::mem::take(&mut ctx.previous) {
            let correlated_ctx = ctx.correlated.entry(previous).or_default();
            T::correlated_decode(reader, &mut ctx.base, correlated_ctx)?
        } else {
            T::decode(reader, &mut ctx.base)?
        };
        ctx.previous = Some(out.clone());
        Ok(out)
    }
}
