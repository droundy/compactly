use crate::Encode;
use cabac::{
    traits::{CabacReader, CabacWriter},
    vp8::VP8Context,
};
use std::io::{Read, Write};

pub struct ByteContext([VP8Context; 512]);
impl Default for ByteContext {
    fn default() -> Self {
        ByteContext([VP8Context::new(); 512])
    }
}

impl Encode for u8 {
    type Context = ByteContext;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut sofar = 0;
        for i in 0..8 {
            let ctx = &mut ctx.0[sofar];
            let bit = (*self >> (7 - i)) & 1 == 1;
            writer.put(bit, ctx)?;
            sofar += 1 << i;
            if bit {
                sofar = sofar + (1 << (7 - i));
            }
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut sofar = 0;
        let mut value = 0;
        for i in 0..8 {
            let ctx = &mut ctx.0[sofar];
            let bit = reader.get(ctx)?;
            sofar += 1 << i;
            if bit {
                sofar = sofar + (1 << (7 - i));
                value |= 1 << (7 - i);
            }
        }
        Ok(value)
    }
}

#[test]
fn size() {
    use crate::assert_size;
    assert_size!(0_u8, 0);
    for b in 3_u8..=255 {
        println!("Byte {b}");
        assert_size!(b, (b & 1) as usize + 1);
    }
    assert_size!(*b"hello", 5);
    assert_size!(*b"hello world", 10);
    assert_size!(*b"hello world, hello world", 18);
    assert_size!(*b"hello hello, hello hello", 15);
    assert_size!(*b"hello hello, hello hello, hello hello, hello hello", 26);
    assert_size!(*b"hhhhhhhhhhhhhhhhhhhhhhhh", 6);
    assert_size!(*b"hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh", 7);
    assert_size!(*b"\0", 0);
    assert_size!(*b"\x01", 2);
    assert_size!(*b"\x01\x01", 3);
    assert_size!(*b"\x01\x01\x01\x01", 4);
}
