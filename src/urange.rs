use crate::Encode;
use cabac::traits::{CabacReader, CabacWriter};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct URange<const N: usize>(usize);

impl<const N: usize> URange<N> {
    pub const fn new(value: usize) -> Self {
        if value < N {
            URange(value)
        } else {
            panic!("Invalid value in compactly::URange")
        }
    }
}

impl<const N: usize> From<URange<N>> for usize {
    fn from(value: URange<N>) -> Self {
        value.0
    }
}

impl<const N: usize> TryFrom<usize> for URange<N> {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < N {
            Ok(URange(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct URangeContext<const N: usize> {
    bits: [<bool as Encode>::Context; N],
}

impl<const N: usize> Default for URangeContext<N> {
    fn default() -> Self {
        Self {
            bits: [Default::default(); N],
        }
    }
}

impl<const N: usize> Encode for URange<N> {
    type Context = URangeContext<N>;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let nbits = (N + 1).ilog2();
        for i in 0..nbits {
            if filled_up + accumulated_value > N {
                break;
            }
            let ctx = &mut ctx.bits[filled_up + accumulated_value];
            let bit = (self.0 >> (nbits - i - 1)) & 1 == 1;
            println!("bit is {bit:?}");
            writer.put(bit, ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
            println!("E {i} ==> {filled_up} -> {accumulated_value}");
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let nbits = (N + 1).ilog2();
        for i in 0..nbits {
            if 2 * accumulated_value + 1 > N {
                accumulated_value *= 2;
                break;
            }
            let ctx = &mut ctx.bits[filled_up + accumulated_value];
            let bit = reader.get(ctx)?;
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
            println!("D {i} ==> {filled_up} -> {accumulated_value}");
        }
        Ok(Self(accumulated_value))
    }
}

#[test]
fn size() {
    use crate::assert_bits;
    assert_bits!(URange::<3>::try_from(0).unwrap(), 2);
    assert_bits!(URange::<3>::try_from(1).unwrap(), 2);
    assert_bits!(URange::<3>::try_from(2).unwrap(), 2);

    assert_bits!(URange::<128>::try_from(0).unwrap(), 7);
    assert_bits!(URange::<128>::try_from(1).unwrap(), 7);
    assert_bits!(URange::<128>::try_from(127).unwrap(), 7);

    assert_bits!(URange::<256>::try_from(0).unwrap(), 8);
    assert_bits!(URange::<256>::try_from(1).unwrap(), 8);
    assert_bits!(URange::<256>::try_from(255).unwrap(), 8);
}
