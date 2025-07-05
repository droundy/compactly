use super::{Encode, EncodingStrategy};
use crate::Small;
use std::io::Read;

#[derive(Clone)]
pub struct ByteContext([<bool as Encode>::Context; 256]);
impl Default for ByteContext {
    #[inline]
    fn default() -> Self {
        ByteContext([Default::default(); 256])
    }
}

impl Encode for u8 {
    type Context = ByteContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        for i in 0..8 {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = (*self >> (7 - i)) & 1 == 1;
            bit.encode(writer, ctx);
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let mut tot = 0;
        for i in 0..8 {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = (*self >> (7 - i)) & 1 == 1;
            tot += bit.millibits(ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
        Some(tot)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        for i in 0..8 {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = bool::decode(reader, ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
        Ok(accumulated_value as u8)
    }
}

macro_rules! small_num {
    ($t:ty, $nbits:literal, $maxval:literal, $doublemax:literal, $testname:ident) => {
        mod $testname {
            use super::{Encode, Read, UBits};

            #[derive(Clone)]
            pub struct Context([<bool as Encode>::Context; $doublemax]);

            impl Default for Context {
                #[inline]
                fn default() -> Self {
                    Self([Default::default(); $doublemax])
                }
            }

            impl Encode for $t {
                type Context = Context;
                #[inline]
                fn encode<E: super::super::EntropyCoder>(
                    &self,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    let value = u8::from(*self);
                    let mut filled_up = 0;
                    let mut accumulated_value = 0;
                    for i in 0..$nbits {
                        let ctx = &mut ctx.0[filled_up + accumulated_value];
                        let bit = (value >> ($nbits - 1 - i)) & 1 == 1;
                        bit.encode(writer, ctx);
                        filled_up += 1 << i;
                        accumulated_value = 2 * accumulated_value + bit as usize;
                    }
                }
                fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
                    let value = u8::from(*self);
                    let mut filled_up = 0;
                    let mut accumulated_value = 0;
                    let mut tot = 0;
                    for i in 0..$nbits {
                        let ctx = &mut ctx.0[filled_up + accumulated_value];
                        let bit = (value >> ($nbits - 1 - i)) & 1 == 1;
                        tot += bit.millibits(ctx)?;
                        filled_up += 1 << i;
                        accumulated_value = 2 * accumulated_value + bit as usize;
                    }
                    Some(tot)
                }
                #[inline]
                fn decode<R: Read>(
                    reader: &mut super::super::Reader<R>,
                    ctx: &mut Self::Context,
                ) -> Result<Self, std::io::Error> {
                    let mut filled_up = 0;
                    let mut accumulated_value = 0;
                    for i in 0..$nbits {
                        let ctx = &mut ctx.0[filled_up + accumulated_value];
                        let bit = bool::decode(reader, ctx)?;
                        filled_up += 1 << i;
                        accumulated_value = 2 * accumulated_value + bit as usize;
                    }
                    Ok((accumulated_value as u8).try_into().unwrap())
                }
            }

            #[test]
            fn test() {
                for value in 0u8..=$maxval {
                    println!("Testing {value}");
                    let v: $t = value.try_into().unwrap();
                    let encoded = super::super::encode(&v);
                    let decoded = super::super::decode::<$t>(&encoded).unwrap();
                    assert_eq!(v, decoded);
                    assert_eq!(v.millibits(&mut Default::default()), Some($nbits * 1000));
                }
            }
        }
    };
}

/// An N-Bit unsigned number that fits into a `u8`.
///
/// This number is tracked precisely, like `u8` itself.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UBits<const N: u8>(u8);

impl<const N: u8> From<UBits<N>> for u8 {
    fn from(value: UBits<N>) -> u8 {
        value.0
    }
}

impl<const N: u8> TryFrom<u8> for UBits<N> {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if N == 8 {
            Ok(Self(value))
        } else if value >> N == 0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

small_num!(UBits<1>, 1, 1, 2, ub1);
small_num!(UBits<2>, 2, 3, 4, ub2);
small_num!(UBits<3>, 3, 7, 8, ub3);
small_num!(UBits<4>, 4, 15, 16, ub4);
small_num!(UBits<5>, 5, 31, 32, ub5);
small_num!(UBits<6>, 6, 63, 64, ub6);
small_num!(UBits<7>, 7, 127, 128, ub7);
small_num!(UBits<8>, 8, 255, 256, ub8);

impl Encode for i8 {
    type Context = <u8 as Encode>::Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        (*self as u8).encode(writer, ctx)
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        <u8 as Encode>::decode(reader, ctx).map(|v| v as i8)
    }
}

#[derive(Default, Clone)]
pub struct SmallContext {
    nonzero: <UBits<3> as Encode>::Context,
    b1: <UBits<1> as Encode>::Context,
    b2: <UBits<2> as Encode>::Context,
    b3: <UBits<3> as Encode>::Context,
    b4: <UBits<4> as Encode>::Context,
    b5: <UBits<5> as Encode>::Context,
    need_seven_bits: <bool as Encode>::Context,
    b6: <UBits<6> as Encode>::Context,
    b7: <UBits<7> as Encode>::Context,
}

impl EncodingStrategy<u8> for Small {
    type Context = SmallContext;
    fn encode<E: super::EntropyCoder>(value: &u8, writer: &mut E, ctx: &mut Self::Context) {
        let nonzero: UBits<3>;
        match *value {
            0 => {
                nonzero = 0.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero)
            }
            1 => {
                nonzero = 1.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero)
            }
            2..4 => {
                nonzero = 2.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b1: UBits<1> = (*value - 2).try_into().unwrap();
                b1.encode(writer, &mut ctx.b1)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b2: UBits<2> = (*value - 4).try_into().unwrap();
                b2.encode(writer, &mut ctx.b2)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b3: UBits<3> = (*value - 8).try_into().unwrap();
                b3.encode(writer, &mut ctx.b3)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b4: UBits<4> = (*value - 16).try_into().unwrap();
                b4.encode(writer, &mut ctx.b4)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                let b5: UBits<5> = (*value - 32).try_into().unwrap();
                b5.encode(writer, &mut ctx.b5)
            }
            64..128 => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                false.encode(writer, &mut ctx.need_seven_bits);
                let b6: UBits<6> = (*value - 64).try_into().unwrap();
                b6.encode(writer, &mut ctx.b6)
            }
            128..=255 => {
                nonzero = 7.try_into().unwrap();
                nonzero.encode(writer, &mut ctx.nonzero);
                true.encode(writer, &mut ctx.need_seven_bits);
                let b7: UBits<7> = (*value - 128).try_into().unwrap();
                b7.encode(writer, &mut ctx.b7)
            }
        }
    }
    fn millibits(value: &u8, ctx: &mut Self::Context) -> Option<usize> {
        let nonzero: UBits<3>;
        match *value {
            0 => {
                nonzero = 0.try_into().unwrap();
                nonzero.millibits(&mut ctx.nonzero)
            }
            1 => {
                nonzero = 1.try_into().unwrap();
                nonzero.millibits(&mut ctx.nonzero)
            }
            2..4 => {
                nonzero = 2.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.nonzero)?;
                let b1: UBits<1> = (*value - 2).try_into().unwrap();
                Some(tot + b1.millibits(&mut ctx.b1)?)
            }
            4..8 => {
                nonzero = 3.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.nonzero)?;
                let b2: UBits<2> = (*value - 4).try_into().unwrap();
                Some(tot + b2.millibits(&mut ctx.b2)?)
            }
            8..16 => {
                nonzero = 4.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.nonzero)?;
                let b3: UBits<3> = (*value - 8).try_into().unwrap();
                Some(tot + b3.millibits(&mut ctx.b3)?)
            }
            16..32 => {
                nonzero = 5.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.nonzero)?;
                let b4: UBits<4> = (*value - 16).try_into().unwrap();
                Some(tot + b4.millibits(&mut ctx.b4)?)
            }
            32..64 => {
                nonzero = 6.try_into().unwrap();
                let tot = nonzero.millibits(&mut ctx.nonzero)?;
                let b5: UBits<5> = (*value - 32).try_into().unwrap();
                Some(tot + b5.millibits(&mut ctx.b5)?)
            }
            64..128 => {
                nonzero = 7.try_into().unwrap();
                let mut tot = nonzero.millibits(&mut ctx.nonzero)?;
                tot += false.millibits(&mut ctx.need_seven_bits)?;
                let b6: UBits<6> = (*value - 64).try_into().unwrap();
                Some(tot + b6.millibits(&mut ctx.b6)?)
            }
            128..=255 => {
                nonzero = 7.try_into().unwrap();
                let mut tot = nonzero.millibits(&mut ctx.nonzero)?;
                tot += true.millibits(&mut ctx.need_seven_bits)?;
                let b7: UBits<7> = (*value - 128).try_into().unwrap();
                Some(tot + b7.millibits(&mut ctx.b7)?)
            }
        }
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<u8, std::io::Error> {
        let nonzero: u8 = <UBits<3> as Encode>::decode(reader, &mut ctx.nonzero)?.into();
        match nonzero {
            0 => Ok(0),
            1 => Ok(1),
            2 => {
                let rest: u8 = <UBits<1> as Encode>::decode(reader, &mut ctx.b1)?.into();
                Ok(rest + 2)
            }
            3 => {
                let rest: u8 = <UBits<2> as Encode>::decode(reader, &mut ctx.b2)?.into();
                Ok(rest + 4)
            }
            4 => {
                let rest: u8 = <UBits<3> as Encode>::decode(reader, &mut ctx.b3)?.into();
                Ok(rest + 8)
            }
            5 => {
                let rest: u8 = <UBits<4> as Encode>::decode(reader, &mut ctx.b4)?.into();
                Ok(rest + 16)
            }
            6 => {
                let rest: u8 = <UBits<5> as Encode>::decode(reader, &mut ctx.b5)?.into();
                Ok(rest + 32)
            }
            7 => {
                if <bool as Encode>::decode(reader, &mut ctx.need_seven_bits)? {
                    let rest: u8 = <UBits<7> as Encode>::decode(reader, &mut ctx.b7)?.into();
                    Ok(rest + 128)
                } else {
                    let rest: u8 = <UBits<6> as Encode>::decode(reader, &mut ctx.b6)?.into();
                    Ok(rest + 64)
                }
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(u8::MAX, 3);
    assert_bits!(0_u8, 8);
    for b in 3_u8..255 {
        println!("Byte {b}");
        assert_bits!(b, 8);
    }
    assert_bits!(*b"hello", 31);
    assert_bits!(*b"hello world", 68);
    assert_bits!(*b"hello world, hello world", 129);
    assert_bits!(*b"hello hello, hello hello", 111);
    assert_bits!(*b"hello hello, hello hello, hello hello, hello hello", 195);
    assert_bits!(*b"hhhhhhhhhhhhhhhhhhhhhhhh", 37);
    assert_bits!(*b"hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh", 44);
    assert_bits!(*b"\0", 8);
    assert_bits!(*b"\x01", 8);
    assert_bits!(*b"\x01\x01", 13);
    assert_bits!(*b"\x01\x01\x01\x01", 19);
    assert_bits!(*b"\x01\x01\x01\x01\x01", 21);
    assert_bits!(*b"\x01\x01\x01\x01\x01\x01", 22);
    assert_bits!(*b"\x01\x02\x03\x04", 25);
    assert_bits!(*b"\x01\x02\x03\x04\x05", 30);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06", 36);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06\x07", 40);
    assert_bits!(*b"\x01\x02\x03\x04\x05\x06\x07\x08", 47);

    assert_bits!(i8::MAX, 8);
    assert_bits!(0_i8, 8);
}

#[test]
fn small() {
    use super::{assert_bits, Small};
    use crate::Encoded;
    fn check_size(v: u8, expected: usize) {
        println!("Checking {v}");
        assert_eq!(
            Encoded::<u8, Small>::new(v).millibits(&mut Default::default()),
            Some(1000 * expected)
        );
        assert_bits!(Encoded::<u8, Small>::new(v), expected);
    }

    for x in 0..2 {
        check_size(x, 3);
    }
    for x in 2..4 {
        check_size(x, 4);
    }
    for x in 4..8 {
        check_size(x, 5);
    }
    for x in 8..16 {
        check_size(x, 6);
    }
    for x in 16..32 {
        check_size(x, 7);
    }
    for x in 32..64 {
        check_size(x, 8);
    }
    for x in 64..128 {
        check_size(x, 10);
    }
    for x in 128..255 {
        check_size(x, 11);
    }
    assert_eq!(
        Encoded::<u8, Small>::new(255u8).millibits(&mut Default::default()),
        Some(11000)
    );
}
