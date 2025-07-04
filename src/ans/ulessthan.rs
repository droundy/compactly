use super::Encode;
use std::io::{Read, Write};

/// A unsigned integer with a value less than `N`.
///
/// This type is unseful if you want to compactly encode a value like the
/// variant of an enum with a range that is known at compil time to be limited,
/// with a range that may not be an integer number of bits.
///
/// Some values will take fewer bits than others, and *all* values will be
/// adapted independently so if e.g. any two variants are exclusively present
/// and equally likely, they will each take only a tiny bit more than a single
/// bit to encode (eventually).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ULessThan<const N: usize>(usize);

impl<const N: usize> ULessThan<N> {
    /// Construct a new `ULessThan<N>`.
    ///
    /// Panics if the value is not less than `N`.
    #[inline]
    pub const fn new(value: usize) -> Self {
        if value < N {
            ULessThan(value)
        } else {
            panic!("Invalid value in compactly::ULessThan")
        }
    }
}

impl<const N: usize> From<ULessThan<N>> for usize {
    #[inline]
    fn from(value: ULessThan<N>) -> Self {
        value.0
    }
}

impl<const N: usize> TryFrom<usize> for ULessThan<N> {
    type Error = ();
    #[inline]
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < N {
            Ok(ULessThan(value))
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ULessThanContext<const N: usize> {
    /// This uses way more context than is needed, because I couldn't find an
    /// elegant way to map the N needed context to the possible bit sequences.  :(
    bits: Vec<<bool as Encode>::Context>,
}

impl<const N: usize> ULessThanContext<N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut <bool as Encode>::Context {
        if self.bits.len() <= index {
            self.bits.reserve(index - self.bits.len());
            while self.bits.len() <= index {
                self.bits.push(Default::default());
            }
        }
        &mut self.bits[index]
    }
}

#[inline]
fn half(i: usize) -> usize {
    let half = i / 2;
    if half > 1 {
        1 << half.ilog(2)
    } else {
        half
    }
}

impl<const N: usize> Encode for ULessThan<N> {
    type Context = ULessThanContext<N>;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let mut bits_chosen = 0;
        // println!("N={N} and value {}", self.0);
        let mut possible_values_left = N;
        let mut value_considered = half(possible_values_left); // big endian bits, splitting values by half each time
        let mut i = 1;
        while accumulated_value + value_considered < N && possible_values_left > 1 {
            let bit = self.0 >= accumulated_value + value_considered;
            // println!(
            //     "bit {i} is {bit:?} == {} >= {}",
            //     self.0,
            //     accumulated_value + value_considered
            // );
            // println!(
            //     "{}: bit {i} is {bit:?} with context {} considering {value_considered} with {possible_values_left} values left to consider",
            //     self.0,
            //     filled_up + bits_chosen
            // );
            let ctx = ctx.index_mut(filled_up + bits_chosen);
            bit.encode(writer, ctx)?;
            filled_up += i;
            if bit {
                bits_chosen += 1 << i;
                accumulated_value += value_considered;
                possible_values_left -= value_considered;
            } else {
                possible_values_left = value_considered;
            }
            // println!("E {i} ==> {filled_up} -> {accumulated_value}");
            value_considered = half(possible_values_left);
            i += 1;
        }
        Ok(())
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        let mut bits_chosen = 0;
        // println!("N={N} and value {}", self.0);
        let mut possible_values_left = N;
        let mut value_considered = half(possible_values_left); // big endian bits, splitting values by half each time
        let mut i = 1;
        let mut tot = 0;
        while accumulated_value + value_considered < N && possible_values_left > 1 {
            let bit = self.0 >= accumulated_value + value_considered;
            // println!(
            //     "bit {i} is {bit:?} == {} >= {}",
            //     self.0,
            //     accumulated_value + value_considered
            // );
            // println!(
            //     "{}: bit {i} is {bit:?} with context {} considering {value_considered} with {possible_values_left} values left to consider",
            //     self.0,
            //     filled_up + bits_chosen
            // );
            let ctx = ctx.index_mut(filled_up + bits_chosen);
            tot += bit.millibits(ctx)?;
            filled_up += i;
            if bit {
                bits_chosen += 1 << i;
                accumulated_value += value_considered;
                possible_values_left -= value_considered;
            } else {
                possible_values_left = value_considered;
            }
            // println!("E {i} ==> {filled_up} -> {accumulated_value}");
            value_considered = half(possible_values_left);
            i += 1;
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
        let mut bits_chosen = 0;
        let mut possible_values_left = N;
        let mut value_considered = half(possible_values_left); // big endian bits, splitting values by half each time
        let mut i = 1;
        while accumulated_value + value_considered < N && possible_values_left > 1 {
            let ctx = ctx.index_mut(filled_up + bits_chosen);
            let bit = bool::decode(reader, ctx)?;
            filled_up += i;
            if bit {
                bits_chosen += 1 << i;
                accumulated_value += value_considered;
                possible_values_left -= value_considered;
            } else {
                possible_values_left = value_considered;
            }
            // println!("D {i} ==> {filled_up} -> {accumulated_value}");
            value_considered = half(possible_values_left);
            i += 1;
        }
        Ok(Self(accumulated_value))
    }
}

#[test]
fn size() {
    use super::assert_bits;
    fn test_urange<const N: usize>() {
        for i in 0..N {
            let v = ULessThan::<N>::new(i);
            println!("Testing ULessThan::<{N}>::new({i})");
            let encoded = super::encode(&v);
            let decoded = super::decode::<ULessThan<N>>(&encoded).unwrap();
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

    assert_bits!(ULessThan::<3>::try_from(0).unwrap(), 1);
    assert_bits!(ULessThan::<3>::try_from(1).unwrap(), 2);
    assert_bits!(ULessThan::<3>::try_from(2).unwrap(), 1);

    assert_bits!(ULessThan::<5>::try_from(0).unwrap(), 2);
    assert_bits!(ULessThan::<5>::try_from(1).unwrap(), 2);
    assert_bits!(ULessThan::<5>::try_from(2).unwrap(), 2);
    assert_bits!(ULessThan::<5>::try_from(3).unwrap(), 3);
    assert_bits!(ULessThan::<5>::try_from(4).unwrap(), 1);

    assert_bits!(ULessThan::<6>::try_from(0).unwrap(), 2);
    assert_bits!(ULessThan::<6>::try_from(1).unwrap(), 2);
    assert_bits!(ULessThan::<6>::try_from(2).unwrap(), 3);
    assert_bits!(ULessThan::<6>::try_from(3).unwrap(), 3);
    assert_bits!(ULessThan::<6>::try_from(4).unwrap(), 3);
    assert_bits!(ULessThan::<6>::try_from(5).unwrap(), 1);

    assert_bits!(ULessThan::<128>::try_from(0).unwrap(), 7);
    assert_bits!(ULessThan::<128>::try_from(1).unwrap(), 7);
    assert_bits!(ULessThan::<128>::try_from(127).unwrap(), 3);

    assert_bits!(ULessThan::<256>::try_from(0).unwrap(), 8);
    assert_bits!(ULessThan::<256>::try_from(1).unwrap(), 8);
    assert_bits!(ULessThan::<256>::try_from(255).unwrap(), 3);
}
