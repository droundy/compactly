use std::io::{Read, Write};

use super::Encode;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitsContext<const N: usize>([<bool as Encode>::Context; N]);
impl<const N: usize> Default for BitsContext<N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bits<const N: usize> {
    value: u8,
}
impl<const N: usize> From<Bits<N>> for u8 {
    fn from(value: Bits<N>) -> Self {
        value.value
    }
}
impl<const N: usize> Bits<N> {
    const MAX: u8 = (N - 1) as u8;
    const N_BITS: u32 = N.ilog2();
    #[inline]
    pub fn take_from(source: &mut u32) -> Self {
        let value = (*source as u8) & Self::MAX;
        *source = *source >> Self::N_BITS;
        Self { value }
    }
}
impl<const N: usize> TryFrom<u8> for Bits<N> {
    type Error = ();
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::MAX {
            Ok(Self { value })
        } else {
            Err(())
        }
    }
}

impl<const N: usize> Encode for Bits<N> {
    type Context = BitsContext<N>;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        debug_assert_eq!(N, 1 << Self::N_BITS);
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        for i in 0..Self::N_BITS {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = (self.value >> (Self::N_BITS - 1 - i)) & 1 == 1;
            bit.encode(writer, ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
        Ok(())
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        for i in 0..Self::N_BITS {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = bool::decode(reader, ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
        Ok(Self {
            value: accumulated_value as u8,
        })
    }
}

#[test]
fn size() {
    use super::assert_bits;

    assert_eq!(Bits::<4>::MAX, 3);
    assert_eq!(Bits::<8>::MAX, 7);
    assert_bits!(Bits::<4>::try_from(2u8).unwrap(), 2);
    assert_bits!(Bits::<8>::try_from(7u8).unwrap(), 1);
    assert_bits!(Bits::<8>::try_from(6u8).unwrap(), 3);
}
