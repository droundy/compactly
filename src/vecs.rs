use crate::Encode;
use std::io::{Read, Write};

impl<T: Encode> Encode for Vec<T> {
    type Context = (<usize as Encode>::Context, T::Context);
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.len().encode(writer, &mut ctx.0)?;
        for v in self {
            v.encode(writer, &mut ctx.1)?;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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
    use crate::assert_size;
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
        assert_size!(dbg!(value), 5);
    }
}
