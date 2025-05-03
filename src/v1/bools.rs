use super::bit_context::BitContext;
use super::Encode;
use std::io::{Read, Write};

impl Encode for bool {
    type Context = BitContext;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        // println!("Encoding {self:?}");
        writer.encode(*self, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let b = reader.decode(ctx)?;
        // println!("Decoding {b:?}");
        Ok(b)
    }
    #[inline]
    fn estimate_bits(&self, ctx: &mut Self::Context) -> usize {
        ctx.bits_required(*self) as usize
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(true, 1);
    assert_bits!(false, 1);
    assert_bits!([false; 128], 7);
    assert_bits!([true; 2], 1);
    assert_bits!([true; 3], 1);
    assert_bits!([true; 16], 3);
    assert_bits!([true; 64], 5);
    assert_bits!([false, true], 3);
}
