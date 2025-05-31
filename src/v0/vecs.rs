use super::{Encode, EncodingStrategy};
use crate::Small;
use std::io::{Read, Write};

impl<T: Encode> Encode for Vec<T> {
    type Context = (<usize as Encode>::Context, T::Context);
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.0)?;
        for v in self {
            v.encode(writer, &mut ctx.1)?;
        }
        Ok(())
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let n = usize::decode(reader, &mut ctx.0)?;
        let mut x = Vec::with_capacity(n);
        for _ in 0..n {
            x.push(T::decode(reader, &mut ctx.1)?);
        }
        Ok(x)
    }
}
#[test]
fn size() {
    use super::assert_size;
    assert_size!(Vec::<usize>::new(), 1);
    for value in 0_usize..4 {
        assert_size!(vec![dbg!(value)], 1);
    }
    for value in 4_usize..64 {
        assert_size!(vec![dbg!(value)], 2);
    }
    for num in 0_usize..2 {
        let value = (0..num).collect::<Vec<_>>();
        assert_size!(dbg!(value), 1);
    }
    for num in 2_usize..5 {
        let value = (0..num).collect::<Vec<_>>();
        assert_size!(dbg!(value), num);
    }
    for num in 5_usize..6 {
        let value = (0..num).collect::<Vec<_>>();
        assert_size!(dbg!(value), 4);
    }
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
}
