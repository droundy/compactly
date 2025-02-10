use crate::Encode;
use cabac::traits::{CabacReader, CabacWriter};
use cabac::vp8::VP8Context;
use std::io::{Read, Write};

impl Encode for bool {
    type Context = VP8Context;
    fn encode<W: Write>(
        &self,
        writer: &mut crate::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        writer.put(*self, ctx)
    }
    fn decode<R: Read>(
        reader: &mut crate::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        reader.get(ctx)
    }
}

#[test]
fn size() {
    use crate::assert_bits;
    assert_bits!(true, 1);
    assert_bits!(false, 1);
    assert_bits!([false; 128], 8);
    assert_bits!([true; 2], 2);
    assert_bits!([true; 7], 3);
    assert_bits!([true; 16], 4);
    assert_bits!([true; 64], 6);
    assert_bits!([false, true], 3);
}
