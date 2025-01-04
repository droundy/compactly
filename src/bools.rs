use crate::Encode;
use cabac::traits::{CabacReader, CabacWriter};
use cabac::vp8::VP8Context;
use std::io::{Read, Write};

impl Encode for bool {
    type Context = VP8Context;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        writer.put(*self, ctx)
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        reader.get(ctx)
    }
}

#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:literal) => {
        let v = $v;
        let bytes = crate::encode(&v);
        println!("bytes are {bytes:?}");
        let decoded = crate::decode(&bytes);
        assert_eq!(decoded, Some(v));
        assert_eq!(bytes.len(), $size);
    };
}

#[test]
fn bools_size() {
    assert_size!(true, 1);
    assert_size!(false, 0);
    assert_size!([false; 128], 0);
    assert_size!([true; 2], 2);
    assert_size!([true; 7], 2);
    assert_size!([true; 16], 2);
    assert_size!([true; 64], 2);
    assert_size!([false, true], 2);
}
