use super::{Compact, Encode, EncodingStrategy, Small, URange};
use std::io::{Read, Write};

#[derive(Default, Clone)]
pub struct UsizeContext {
    less_than_four: <bool as Encode>::Context,
    small: <URange<4> as Encode>::Context,
    big: <Compact<u64> as Encode>::Context,
}

impl Encode for usize {
    type Context = UsizeContext;
    #[inline]
    fn encode<W: Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if let Ok(r) = URange::<4>::try_from(*self) {
            true.encode(writer, &mut ctx.less_than_four)?;
            r.encode(writer, &mut ctx.small)
        } else {
            false.encode(writer, &mut ctx.less_than_four)?;
            Compact((*self - 4) as u64).encode(writer, &mut ctx.big)
        }
    }
    #[inline]
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.less_than_four)? {
            URange::<4>::decode(reader, &mut ctx.small).map(usize::from)
        } else {
            let Compact(v) = Compact::<u64>::decode(reader, &mut ctx.big)?;
            usize::try_from(v + 4).map_err(std::io::Error::other)
        }
    }
}

impl EncodingStrategy<usize> for Small {
    type Context = UsizeContext;
    fn encode<W: Write>(
        value: &usize,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        value.encode(writer, ctx)
    }
    fn decode<R: Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<usize, std::io::Error> {
        usize::decode(reader, ctx)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    use super::Compact;
    assert_bits!(Compact(0_u64), 7);
    assert_bits!(0_usize, 3);
    assert_bits!(Compact(1_u64), 7);
    assert_bits!(1_usize, 3);
    assert_bits!(Compact(2_u64), 7);
    assert_bits!(2_usize, 3);
    assert_bits!(3_usize, 3);
    assert_bits!(4_usize, 8);
    assert_bits!(5_usize, 8);
    assert_bits!(6_usize, 8);
    assert_bits!(7_usize, 8);
    assert_bits!(8_usize, 9);
    assert_bits!(Compact(16_u64), 10);
    assert_bits!(16_usize, 10);
    assert_bits!(Compact(32_u64), 11);
    assert_bits!(32_usize, 11);
    assert_bits!(Compact(64_u64), 12);
    assert_bits!(64_usize, 12);
    assert_bits!(Compact(128_u64), 13);
    assert_bits!(128_usize, 13);
    assert_bits!(Compact(256_u64), 14);
    assert_bits!(256_usize, 14);
    assert_bits!(512_usize, 15);
    assert_bits!(Compact(1024_u64), 16);
    assert_bits!(1024_usize, 16);
    assert_bits!(Compact(1024_u64 * 1024), 26);
    assert_bits!(1024_usize * 1024, 26);
    assert_bits!(1024_usize * 1024 * 1024, 36);
    assert_bits!(u32::MAX as usize, 38);
    // Note the code will work for u32, but the following two tests will fail.
    assert_bits!(1024_usize * 1024 * 1024 * 1024, 46);
    assert_bits!(1024_usize * 1024 * 1024 * 1024 * 1024, 56);
    assert_bits!([0_usize; 128], 23);
    assert_bits!([1_usize; 19], 13);
    assert_bits!(
        [0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        19
    );
}
