use super::Encode;

/// An unsigned integer value fitting in `log2(SIZE+1)` bits, with a stack-allocated
/// adaptive binary-tree context.
///
/// The const generic `SIZE` must equal `(1 << N) - 1` for some `N` in 1..=8:
/// use SIZE=1 for 1-bit, 3 for 2-bit, 7 for 3-bit, 15 for 4-bit, 31 for 5-bit,
/// 63 for 6-bit, 127 for 7-bit, or 255 for 8-bit values.
///
/// Each of the `SIZE+1` possible values has its own distinct path through the
/// binary tree, so the encoder adapts independently for every value — just like
/// `ULessThan`, but without heap allocation and with a fixed iteration count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AFewBits<const SIZE: usize>(u8);

impl<const SIZE: usize> AFewBits<SIZE> {
    const N_BITS: usize = (SIZE + 1).ilog2() as usize;

    #[inline]
    pub fn new(value: u8) -> Self {
        debug_assert!(value as usize <= SIZE);
        AFewBits(value)
    }
}

impl<const SIZE: usize> From<AFewBits<SIZE>> for u8 {
    #[inline]
    fn from(v: AFewBits<SIZE>) -> u8 {
        v.0
    }
}

/// Stack-allocated context for [`AFewBits<SIZE>`].
///
/// Holds `SIZE` independent adaptive bit contexts, one per node in the binary
/// tree. Because `SIZE = 2^N - 1`, the full tree for `N` bits fits exactly.
#[derive(Clone)]
pub struct AFewBitsContext<const SIZE: usize>([<bool as Encode>::Context; SIZE]);

impl<const SIZE: usize> Default for AFewBitsContext<SIZE> {
    #[inline]
    fn default() -> Self {
        AFewBitsContext([Default::default(); SIZE])
    }
}

impl<const SIZE: usize> Encode for AFewBits<SIZE> {
    type Context = AFewBitsContext<SIZE>;

    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let mut filled_up = 0;
        let mut accumulated_value = 0;
        for i in 0..Self::N_BITS {
            let ctx = &mut ctx.0[filled_up + accumulated_value];
            let bit = (self.0 as usize >> (Self::N_BITS - 1 - i)) & 1 == 1;
            bit.encode(writer, ctx);
            filled_up += 1 << i;
            accumulated_value = 2 * accumulated_value + bit as usize;
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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
        Ok(AFewBits(accumulated_value as u8))
    }
}

#[test]
fn afewbits_roundtrip() {
    fn test<const SIZE: usize>() {
        for value in 0..=SIZE {
            let v = AFewBits::<SIZE>::new(value as u8);
            let encoded = super::encode(&v);
            let decoded = super::decode::<AFewBits<SIZE>>(&encoded).unwrap();
            assert_eq!(u8::from(decoded), value as u8, "SIZE={SIZE} value={value}");
        }
    }
    test::<1>();
    test::<3>();
    test::<7>();
    test::<15>();
    test::<31>();
    test::<63>();
    test::<127>();
    test::<255>();
}
