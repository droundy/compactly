use crate::Encode;
use std::io::{Read, Write};

#[derive(Default)]
pub struct U64Context {
    /// FIXME when Default is implemented for larger arrays, make this just be a
    /// 64-element array.
    less_significant: [<bool as Encode>::Context; 32],
    more_significant: [<bool as Encode>::Context; 32],
}

impl Encode for u64 {
    type Context = U64Context;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut v = *self;
        for i in 0..32 {
            (v & 1 == 1).encode(writer, &mut ctx.less_significant[i])?;
            v >>= 1;
        }
        for i in 0..32 {
            (v & 1 == 1).encode(writer, &mut ctx.more_significant[i])?;
            v >>= 1;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut v = 0;
        for i in 0..32 {
            if bool::decode(reader, &mut ctx.less_significant[i])? {
                v |= 1 << i;
            }
        }
        for i in 0..32 {
            if bool::decode(reader, &mut ctx.more_significant[i])? {
                v |= 1 << (i + 32);
            }
        }
        Ok(v)
    }
}

#[test]
fn size_u64() {
    use crate::assert_size;
    for sz in 0..1_000_000_u64 {
        println!("Trying with {sz}");
        assert_size!(sz, 6);
    }
    for sz in [u64::MAX] {
        println!("Trying with {sz}");
        assert_size!(sz, 9);
    }
    assert_size!([0_u64; 128], 61);
    assert_size!([1_u64; 2], 10);
    assert_size!([1_u64; 19], 36);
    assert_size!(
        [0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        40
    );
}

macro_rules! impl_uint {
    ($t:ident, $context:ident, $bits:literal) => {
        #[derive(Default)]
        pub struct $context {
            context: [<bool as Encode>::Context; $bits],
        }

        impl Encode for $t {
            type Context = $context;
            fn encode<W: Write>(
                &self,
                writer: &mut cabac::vp8::VP8Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let mut v = *self;
                for i in 0..$bits {
                    (v & 1 == 1).encode(writer, &mut ctx.context[i])?;
                    v >>= 1;
                }
                Ok(())
            }
            fn decode<R: Read>(
                reader: &mut cabac::vp8::VP8Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let mut v = 0;
                for i in 0..$bits {
                    if bool::decode(reader, &mut ctx.context[i])? {
                        v |= 1 << i;
                    }
                }
                Ok(v)
            }
        }
    };
}
impl_uint!(u32, U32Context, 32);
impl_uint!(u16, U16Context, 16);

#[test]
fn size_u32() {
    use crate::assert_size;
    for sz in 0..32768_u32 {
        println!("Trying with {sz}");
        assert_size!(sz, 2);
    }
    for sz in 32768_u32..1_000_000 {
        println!("Trying with {sz}");
        assert_size!(sz, 3);
    }
    for sz in [u32::MAX] {
        println!("Trying with {sz}");
        assert_size!(sz, 5);
    }
    assert_size!([0_u32; 128], 29);
    assert_size!([u32::MAX; 128], 30);
    assert_size!([1_u32; 2], 5);
    assert_size!([1_u32; 19], 18);
    assert_size!(
        [0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        21
    );
}
#[test]
fn size_u16() {
    use crate::assert_size;
    for sz in 0..16768_u16 {
        println!("Trying with {sz}");
        assert_size!(sz, 2);
    }
    for sz in [u16::MAX] {
        println!("Trying with {sz}");
        assert_size!(sz, 5);
    }
    assert_size!([0_u16; 128], 29);
    assert_size!([u16::MAX; 128], 30);
    assert_size!([1_u16; 2], 5);
    assert_size!([1_u16; 19], 18);
    assert_size!(
        [0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        21
    );
}
