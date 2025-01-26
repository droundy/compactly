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

#[derive(Debug, Clone, Copy)]
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
        // println!("N={N} and value {}", self.0);
        let mut value_considered = 1;
        let mut i = 1;
        while accumulated_value + value_considered < N {
            let ctx = &mut ctx.bits[filled_up + accumulated_value];
            let bit = self.0 & value_considered != 0;
            // println!(
            //     "{}: bit {i} is {bit:?} with context {}",
            //     self.0,
            //     filled_up + accumulated_value
            // );
            writer.put(bit, ctx)?;
            filled_up += i;
            if bit {
                accumulated_value += value_considered;
            }
            // println!("E {i} ==> {filled_up} -> {accumulated_value}");
            value_considered *= 2;
            i += 1;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let mut value_considered = 1;
        let mut i = 1;
        while accumulated_value + value_considered < N {
            let ctx = &mut ctx.bits[filled_up + accumulated_value];
            let bit = reader.get(ctx)?;
            filled_up += i;
            if bit {
                accumulated_value += value_considered;
            }
            // println!("D {i} ==> {filled_up} -> {accumulated_value}");
            value_considered *= 2;
            i += 1;
        }
        Ok(Self(accumulated_value))
    }
}

#[test]
fn size() {
    use crate::assert_bits;
    fn test_urange<const N: usize>() {
        for i in 0..N {
            let v = URange::<N>::new(i);
            let encoded = crate::encode(&v);
            let decoded = crate::decode::<URange<N>>(&encoded).unwrap();
            assert_eq!(decoded, v);
        }
    }
    test_urange::<1>();
    test_urange::<2>();
    test_urange::<3>();
    test_urange::<4>();
    test_urange::<5>();
    test_urange::<6>();
    test_urange::<7>();
    test_urange::<8>();
    test_urange::<9>();
    test_urange::<10>();
    test_urange::<255>();
    test_urange::<256>();
    test_urange::<257>();

    assert_bits!(URange::<3>::try_from(0).unwrap(), 2);
    assert_bits!(URange::<3>::try_from(1).unwrap(), 1);
    assert_bits!(URange::<3>::try_from(2).unwrap(), 2);

    assert_bits!(URange::<128>::try_from(0).unwrap(), 7);
    assert_bits!(URange::<128>::try_from(1).unwrap(), 7);
    assert_bits!(URange::<128>::try_from(127).unwrap(), 7);

    assert_bits!(URange::<256>::try_from(0).unwrap(), 8);
    assert_bits!(URange::<256>::try_from(1).unwrap(), 8);
    assert_bits!(URange::<256>::try_from(255).unwrap(), 8);

    assert_bits!(URange::<3>::new(2), 2);
    assert_bits!(URange::<4>::new(3), 2);
    println!("Looking at 4 as max in urange");
    assert_bits!(URange::<5>::new(4), 3);
    assert_bits!(URange::<10>::new(9), 4);
}
