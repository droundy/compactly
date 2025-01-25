use crate::Encode;
use cabac::{
    traits::{CabacReader, CabacWriter},
    vp8::VP8Context,
};
use std::io::{Read, Write};

pub struct UsizeContext {
    is_zero: VP8Context,
    bits: [BitContext; 64],
}

impl Default for UsizeContext {
    fn default() -> Self {
        Self {
            is_zero: Default::default(),
            bits: [Default::default(); 64],
        }
    }
}

#[derive(Default, Clone, Copy)]
struct BitContext {
    done_with_one: VP8Context,
    value: VP8Context,
}

impl Encode for usize {
    type Context = UsizeContext;
    fn encode<W: Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut value = *self;
        writer.put(value == 0, &mut ctx.is_zero)?;
        if value == 0 {
            return Ok(());
        }
        for i in 0..64 {
            writer.put(value == 1, &mut ctx.bits[i].done_with_one)?;
            if value == 1 {
                return Ok(());
            }
            writer.put((value & 1) == 1, &mut ctx.bits[i].value)?;
            value = value >> 1;
        }
        Ok(())
    }
    fn decode<R: Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut value = 0;
        if reader.get(&mut ctx.is_zero)? {
            return Ok(0);
        }
        for i in 0..64 {
            if reader.get(&mut ctx.bits[i].done_with_one)? {
                return Ok(value + (1 << i));
            }
            if reader.get(&mut ctx.bits[i].value)? {
                value += 1 << i;
            }
        }
        Ok(value)
    }
}

#[test]
fn size() {
    use crate::assert_bits;
    assert_bits!(0_usize, 1);
    assert_bits!(1_usize, 2);
    assert_bits!(2_usize, 4);
    assert_bits!(3_usize, 4);
    assert_bits!(4_usize, 6);
    assert_bits!(5_usize, 6);
    assert_bits!(6_usize, 6);
    assert_bits!(7_usize, 6);
    assert_bits!(8_usize, 8);
    assert_bits!(16_usize, 10);
    assert_bits!(32_usize, 12);
    assert_bits!(64_usize, 14);
    assert_bits!(128_usize, 16);
    assert_bits!(256_usize, 18);
    assert_bits!(512_usize, 20);
    assert_bits!(1024_usize, 22);
    assert_bits!(1024_usize * 1024, 42);
    assert_bits!(1024_usize * 1024 * 1024, 62);
    assert_bits!(u32::MAX as usize, 64);
    // Note the code will work for u32, but the following two tests will fail.
    assert_bits!(1024_usize * 1024 * 1024 * 1024, 82);
    assert_bits!(1024_usize * 1024 * 1024 * 1024 * 1024, 102);
    assert_bits!([0_usize; 128], 7);
    assert_bits!([1_usize; 19], 9);
    assert_bits!(
        [0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        11
    );
}
