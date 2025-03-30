use super::Encode;
use super::{bit_context::BitContext, EncodeCorrelated};
use std::io::{Read, Write};

impl Encode for bool {
    type Context = BitContext;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        writer.encode(*self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        reader.decode(ctx)
    }
}

impl EncodeCorrelated for bool {
    fn correlated_encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        base_ctx: &mut <Self as Encode>::Context,
        correlated_ctx: &mut <Self as Encode>::Context,
    ) -> Result<(), std::io::Error> {
        let (ctx, extra) = if *correlated_ctx >= BitContext::CONFIDENT {
            (correlated_ctx, base_ctx)
        } else {
            (base_ctx, correlated_ctx)
        };
        extra.adapt(*self);
        self.encode(writer, ctx)
    }
    fn correlated_decode<R: Read>(
        reader: &mut super::Reader<R>,
        base_ctx: &mut <Self as Encode>::Context,
        correlated_ctx: &mut <Self as Encode>::Context,
    ) -> Result<Self, std::io::Error> {
        let (ctx, extra) = if *correlated_ctx >= BitContext::CONFIDENT {
            (correlated_ctx, base_ctx)
        } else {
            (base_ctx, correlated_ctx)
        };
        let bit = Self::decode(reader, ctx)?;
        extra.adapt(bit);
        Ok(bit)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(true, 1);
    assert_bits!(false, 1);
    assert_bits!([false; 128], 5);
    assert_bits!([true; 2], 1);
    assert_bits!([true; 3], 2);
    assert_bits!([true; 16], 4);
    assert_bits!([true; 64], 5);
    assert_bits!([false, true], 3);
}
