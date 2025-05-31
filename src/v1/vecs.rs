use super::{Encode, EncodingStrategy};
use crate::{Normal, Small};
use std::io::{Read, Write};

impl<T: Encode> Encode for Vec<T> {
    type Context = Context<T, Normal>;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        crate::Values::<Normal>::encode(self, writer, ctx)
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        crate::Values::<Normal>::millibits(self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        crate::Values::<Normal>::decode(reader, ctx)
    }
}
#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(Vec::<usize>::new(), 3);
    for value in 0_usize..4 {
        assert_bits!(vec![dbg!(value)], 6);
    }
    assert_bits!(dbg!((0_usize..1).collect::<Vec<_>>()), 6);
    assert_bits!(dbg!((0_usize..2).collect::<Vec<_>>()), 10);
    assert_bits!(dbg!((0_usize..10).collect::<Vec<_>>()), 61);
}

pub struct Context<T, S: EncodingStrategy<T>> {
    len: <Small as EncodingStrategy<usize>>::Context,
    values: S::Context,
}
impl<T, S: EncodingStrategy<T>> Default for Context<T, S> {
    fn default() -> Self {
        Self {
            len: Default::default(),
            values: Default::default(),
        }
    }
}
impl<T, S: EncodingStrategy<T>> Clone for Context<T, S> {
    fn clone(&self) -> Self {
        Self {
            len: self.len.clone(),
            values: self.values.clone(),
        }
    }
}

impl<T, S: EncodingStrategy<T>> EncodingStrategy<Vec<T>> for crate::Values<S> {
    type Context = Context<T, S>;
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let n = Small::decode(reader, &mut ctx.len)?;
        let mut x = Vec::with_capacity(n);
        for _ in 0..n {
            x.push(S::decode(reader, &mut ctx.values)?);
        }
        Ok(x)
    }
    fn encode<W: Write>(
        value: &Vec<T>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Small::encode(&value.len(), writer, &mut ctx.len)?;
        for v in value {
            S::encode(v, writer, &mut ctx.values)?;
        }
        Ok(())
    }
    fn millibits(value: &Vec<T>, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = Small::millibits(&value.len(), &mut ctx.len)?;
        for v in value {
            tot += S::millibits(v, &mut ctx.values)?;
        }
        Some(tot)
    }
}
