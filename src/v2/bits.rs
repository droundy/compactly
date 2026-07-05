use super::Encode;

#[cfg(test)]
use expect_test::expect;

/// Adaptive context for [`Bits<N>`] encoding; holds one bit context per node in
/// the log2(N)-level binary tree.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BitsContext<const N: usize>([<bool as Encode>::Context; N]);
impl<const N: usize> Default for BitsContext<N> {
    fn default() -> Self {
        Self([Default::default(); N])
    }
}
/// An N-ary value encoded as log2(N) bits using an adaptive binary tree.
///
/// `N` must be a power of two. `Bits<32>` encodes 5-bit values (0..31),
/// `Bits<128>` encodes 7-bit values (0..127), etc.
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
    /// Extract the low log2(N) bits from `source`, shifting it right.
    #[inline]
    pub fn take_from(source: &mut u32) -> Self {
        let value = (*source as u8) & Self::MAX;
        *source >>= Self::N_BITS;
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        debug_assert_eq!(N, 1 << Self::N_BITS);
        writer.encode_tree(&mut ctx.0, self.value as usize)
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Self {
            value: reader.decode_tree(&mut ctx.0) as u8,
        })
    }
}

#[test]
fn size() {
    use super::assert_bits;

    assert_eq!(Bits::<4>::MAX, 3);
    assert_eq!(Bits::<8>::MAX, 7);
    assert_bits!(Bits::<4>::try_from(2u8).unwrap(), expect!["2"]);
    assert_bits!(Bits::<8>::try_from(7u8).unwrap(), expect!["3"]);
    assert_bits!(Bits::<8>::try_from(6u8).unwrap(), expect!["3"]);
}
