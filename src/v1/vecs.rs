use super::Encode;
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
    use super::assert_bits;
    assert_bits!(Vec::<usize>::new(), 3);
    for value in 0_usize..4 {
        assert_bits!(vec![dbg!(value)], 6);
    }
    assert_bits!(dbg!((0_usize..1).collect::<Vec<_>>()), 6);
    assert_bits!(dbg!((0_usize..2).collect::<Vec<_>>()), 9);
    assert_bits!(dbg!((0_usize..10).collect::<Vec<_>>()), 64);
}
