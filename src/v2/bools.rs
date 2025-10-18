use crate::Sorted;

use super::Encode;
use super::{bit_context::BitContext, EncodingStrategy};

impl Encode for bool {
    type Context = BitContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        // println!("Encoding {self:?}");
        writer.encode_bit(ctx.probability(), *self);
        *ctx = ctx.adapt(*self);
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let b = reader.decode_bit(ctx)?;
        // println!("Decoding {b:?}");
        Ok(b)
    }
}

impl EncodingStrategy<bool> for Sorted {
    type Context = BitContext;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<bool, std::io::Error> {
        bool::decode(reader, ctx)
    }
    fn encode<E: super::EntropyCoder>(value: &bool, writer: &mut E, ctx: &mut Self::Context) {
        value.encode(writer, ctx)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    assert_bits!(true, 1);
    assert_bits!(false, 1);
    assert_bits!([false; 128], 7);
    assert_bits!([true; 2], 1);
    assert_bits!([true; 3], 1);
    assert_bits!([true; 16], 3);
    assert_bits!([true; 64], 5);
    assert_bits!([false, true], 3);
}

#[test]
fn millibits_required() {
    use super::Millibits;
    let mut bc = BitContext::default();
    assert_eq!(bc.probability().as_f64(), 0.5);

    assert_eq!(false.millibits(), Millibits::bits(1));
    assert_eq!(true.millibits(), Millibits::bits(1));

    macro_rules! assert_millibits {
        ($bit:literal, $ctx:expr, $expected:expr) => {{
            let mut mb = Millibits::new(0);
            $bit.encode(&mut mb, $ctx);
            assert_eq!(mb, $expected);
        }};
    }

    assert_millibits!(true, &mut bc, Millibits::bits(1));

    assert_eq!(bc, BitContext::True1False0);
    assert!(bc.probability().as_f64() < 0.5);

    assert_millibits!(true, &mut BitContext::True1False0, Millibits::new(582));
    assert_millibits!(false, &mut BitContext::True1False0, Millibits::new(1590));

    assert_millibits!(true, &mut bc, Millibits::new(582));
    assert_millibits!(true, &mut bc, Millibits::new(415));
    assert_millibits!(false, &mut bc, Millibits::new(2327));
    assert_millibits!(false, &mut bc, Millibits::new(1590));
    assert_millibits!(false, &mut bc, Millibits::new(1218));
    assert_millibits!(false, &mut bc, Millibits::new(1000));
}
