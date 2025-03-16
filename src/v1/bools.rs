use super::bit_context::BitContext;
use super::Encode;
use std::io::{Read, Write};

impl Encode for bool {
    type Context = BitContext;
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        writer.encode(*self, ctx)
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        reader.decode(ctx)
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
