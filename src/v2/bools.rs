use crate::{Normal, Sorted};

use super::bit_context::BitContext;
use super::{Encode, Strategy};

#[cfg(test)]
use super::millibits;
#[cfg(test)]
use expect_test::expect;

impl Encode for bool {
    type Context = BitContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        writer.encode_bit(ctx, *value);
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let b = reader.decode_bit(ctx);
        // println!("Decoding {b:?}");
        Ok(b)
    }
}

impl Encode<Sorted> for bool {
    type Context = BitContext;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<bool, std::io::Error> {
        <bool as Encode>::decode(reader, ctx)
    }
    fn encode<E: super::EntropyCoder>(value: &bool, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(value, writer, ctx)
    }
}

#[test]
fn size() {
    use super::estimated_bits;
    expect!["1"].assert_eq(&estimated_bits!(true));
    expect!["1"].assert_eq(&estimated_bits!(false));
    expect!["7"].assert_eq(&estimated_bits!([false; 128]));
    expect!["2"].assert_eq(&estimated_bits!([true; 2]));
    expect!["2"].assert_eq(&estimated_bits!([true; 3]));
    expect!["4"].assert_eq(&estimated_bits!([true; 16]));
    expect!["6"].assert_eq(&estimated_bits!([true; 64]));
    expect!["3"].assert_eq(&estimated_bits!([false, true]));
}

#[test]
fn millibits_required() {
    use super::Millibits;
    let mut bc = BitContext::default();
    assert_eq!(bc.probability().as_f64(), 0.5);

    assert_eq!(millibits(&false), Millibits::bits(1));
    assert_eq!(millibits(&true), Millibits::bits(1));

    macro_rules! assert_millibits {
        ($bit:literal, $ctx:expr, $expected:expr) => {{
            let mut mb = Millibits::new(0);
            Normal::encode(&$bit, &mut mb, $ctx);
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
