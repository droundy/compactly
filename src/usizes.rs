use crate::Encode;
use cabac::{
    traits::{CabacReader, CabacWriter},
    vp8::VP8Context,
};
use std::io::{Read, Write};

#[derive(Default)]
pub struct UsizeContext {
    is_zero: VP8Context,
    /// FIXME when Default is implemented for larger arrays, make this just be a
    /// 64-element array.
    less_significant: [BitContext; 32],
    more_significant: [BitContext; 32],
}

#[derive(Default)]
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
        for i in 0..32 {
            writer.put(value == 1, &mut ctx.less_significant[i].done_with_one)?;
            if value == 1 {
                return Ok(());
            }
            writer.put((value & 1) == 1, &mut ctx.less_significant[i].value)?;
            value = value >> 1;
        }
        for i in 0..32 {
            writer.put(value == 1, &mut ctx.less_significant[i].done_with_one)?;
            if value == 1 {
                return Ok(());
            }
            writer.put((value & 1) == 1, &mut ctx.more_significant[i].value)?;
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
        for i in 0..32 {
            if reader.get(&mut ctx.less_significant[i].done_with_one)? {
                return Ok(value + (1 << i));
            }
            if reader.get(&mut ctx.less_significant[i].value)? {
                value += 1 << i;
            }
        }
        for i in 0..32 {
            if reader.get(&mut ctx.more_significant[i].done_with_one)? {
                return Ok(value + (1 << i));
            }
            if reader.get(&mut ctx.more_significant[i].value)? {
                value += 1 << (i + 32);
            }
        }

        Ok(value)
    }
}

#[test]
fn size() {
    use crate::assert_size;
    for sz in 0_usize..8_usize {
        println!("Trying with {sz}");
        assert_size!(sz, 1);
    }
    for sz in 8_usize..128 {
        println!("Trying with {sz}");
        assert_size!(sz, 2);
    }
    for sz in 128_usize..2048 {
        println!("Trying with {sz}");
        assert_size!(sz, 3);
    }
    assert_size!([0_usize; 128], 2);
    assert_size!([1_usize; 19], 2);
    assert_size!(
        [0_usize, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        2
    );
}
