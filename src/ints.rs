use crate::Encode;
use std::io::{Read, Write};

macro_rules! impl_uint {
    ($t:ident, $context:ident, $bits:literal) => {
        pub struct $context {
            leading_zero: [<bool as Encode>::Context; $bits],
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
            fn default() -> Self {
                Self {
                    leading_zero: [Default::default(); $bits],
                    context: [Default::default(); $bits],
                }
            }
        }

        impl Encode for $t {
            type Context = $context;
            fn encode<W: Write>(
                &self,
                writer: &mut cabac::vp8::VP8Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let mut am_leading = true;
                for i in (0..$bits).rev() {
                    let bit = (*self & (1 << i)) != 0;
                    if am_leading {
                        bit.encode(writer, &mut ctx.leading_zero[i])?;
                        am_leading = !bit;
                    } else {
                        bit.encode(writer, &mut ctx.context[i])?;
                    }
                }
                Ok(())
            }
            fn decode<R: Read>(
                reader: &mut cabac::vp8::VP8Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let mut v = 0;
                let mut am_leading = true;
                for i in (0..$bits).rev() {
                    let bit = if am_leading {
                        let bit = bool::decode(reader, &mut ctx.leading_zero[i])?;
                        am_leading = !bit;
                        bit
                    } else {
                        bool::decode(reader, &mut ctx.context[i])?
                    };
                    if bit {
                        v |= 1 << i;
                    }
                }
                Ok(v)
            }
        }
    };
}
impl_uint!(u64, U64Context, 64);
impl_uint!(u32, U32Context, 32);
impl_uint!(u16, U16Context, 16);

#[test]
fn size_u64() {
    use crate::assert_bits;
    for sz in 0..1024_u64 {
        println!("Trying with {sz}");
        assert_bits!(sz, 64);
    }
    for sz in [1_000_000_u64, u64::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 64);
    }
    assert_bits!([0_u64; 128], 503);
    assert_bits!([1_u64; 2], 102);
    assert_bits!([1_u64; 19], 284);
    assert_bits!(
        [0_u64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        320
    );
}

#[test]
fn size_u32() {
    use crate::assert_bits;
    for sz in 0..32768_u32 {
        println!("Trying with {sz}");
        assert_bits!(sz, 32);
    }
    for sz in 999_990_u32..1_000_000 {
        println!("Trying with {sz}");
        assert_bits!(sz, 32);
    }
    for sz in [u32::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 32);
    }
    assert_bits!([0_u32; 128], 251);
    assert_bits!([u32::MAX; 128], 231);
    assert_bits!([1_u32; 2], 51);
    assert_bits!([1_u32; 19], 142);
    assert_bits!(
        [0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        162
    );
}

#[test]
fn size_u16() {
    use crate::assert_bits;
    for sz in 0..1_u16 {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    for sz in 1..128_u16 {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    for sz in 128..32768_u16 {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    for sz in [u16::MAX] {
        println!("Trying with {sz}");
        assert_bits!(sz, 16);
    }
    assert_bits!([0_u16; 128], 126);
    assert_bits!([u16::MAX; 128], 115);
    assert_bits!([1_u16; 2], 26);
    assert_bits!([1_u16; 19], 71);
    assert_bits!(
        [0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        83
    );
}
