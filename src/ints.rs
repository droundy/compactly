use crate::{Compact, Encode, URange};
use std::io::{Read, Write};

macro_rules! impl_uint {
    ($t:ident, $context:ident, $bits:literal) => {
        #[derive(Clone, Copy)]
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

macro_rules! impl_compact {
    ($t:ident, $context:ident, $bits:literal) => {
        #[derive(Clone, Copy)]
        pub struct $context {
            leading_zeros: <URange<{ $bits + 1 }> as Encode>::Context,
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
            fn default() -> Self {
                Self {
                    leading_zeros: Default::default(),
                    context: [Default::default(); $bits],
                }
            }
        }

        impl Encode for Compact<$t> {
            type Context = $context;
            fn encode<W: Write>(
                &self,
                writer: &mut cabac::vp8::VP8Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let uleading = self.leading_zeros() as usize;
                let leading_zeros = URange::<{ $bits + 1 }>::new(uleading);
                leading_zeros.encode(writer, &mut ctx.leading_zeros)?;
                if uleading >= $bits - 1 {
                    return Ok(());
                }
                for i in 0..($bits - 1) - uleading {
                    ((self.0 >> i) & 1 == 1).encode(writer, &mut ctx.context[i])?;
                }
                Ok(())
            }
            fn decode<R: Read>(
                reader: &mut cabac::vp8::VP8Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let leading_zeros =
                    URange::<{ $bits + 1 }>::decode(reader, &mut ctx.leading_zeros)?;
                let uleading = usize::from(leading_zeros);
                if uleading >= $bits - 1 {
                    if uleading == $bits {
                        return Ok(Compact(0));
                    } else {
                        return Ok(Compact(1));
                    }
                }
                let mut out = 1 << ($bits - 1 - uleading);
                for i in 0..($bits - 1) - uleading {
                    if bool::decode(reader, &mut ctx.context[i])? {
                        out |= 1 << i;
                    }
                }
                Ok(Compact(out))
            }
        }
    };
}

impl_compact!(u64, U64Compact, 64);
impl_compact!(u32, U32Compact, 32);
impl_compact!(u16, U16Compact, 16);

#[test]
fn compact_u16() {
    use crate::assert_bits;
    assert_bits!(Compact(0_u16), 5);
    assert_bits!(Compact(1_u16), 4);
    assert_bits!(Compact(2_u16), 5);
    assert_bits!(Compact(3_u16), 5);
    assert_bits!(Compact(4_u16), 6);
    assert_bits!(Compact(5_u16), 6);
    assert_bits!(Compact(6_u16), 6);
    assert_bits!(Compact(7_u16), 6);
    assert_bits!(Compact(8_u16), 7);
    assert_bits!(Compact(u16::MAX), 20);
    assert_bits!([Compact(0_u16); 128], 39);
    assert_bits!([Compact(u16::MAX); 128], 148);
    assert_bits!([Compact(1_u16); 2], 7);
    assert_bits!([Compact(1_u16); 19], 18);
    assert_bits!(
        [0_u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            .map(Compact),
        36
    );
}

#[test]
fn compact_u32() {
    use crate::assert_bits;
    assert_bits!(Compact(0_u32), 6);
    assert_bits!(Compact(1_u32), 5);
    assert_bits!(Compact(2_u32), 6);
    assert_bits!(Compact(3_u32), 6);
    assert_bits!(Compact(4_u32), 7);
    assert_bits!(Compact(5_u32), 7);
    assert_bits!(Compact(6_u32), 7);
    assert_bits!(Compact(7_u32), 7);
    assert_bits!(Compact(8_u32), 8);
    assert_bits!(Compact(u32::MAX), 37);
    assert_bits!([Compact(0_u32); 128], 47);
    assert_bits!([Compact(u32::MAX); 128], 271);
    assert_bits!([Compact(1_u32); 2], 8);
    assert_bits!([Compact(1_u32); 19], 22);
    assert_bits!(
        [0_u32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
            .map(Compact),
        41
    );
}
