use super::{Encode, EncodingStrategy};
use crate::{Decimal, Small};

macro_rules! impl_float {
    ($t:ident, $intty:ident, $sint:ident, $context:ident, $decimal:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            is_int: <bool as Encode>::Context,
            int_context: <Small as EncodingStrategy<$intty>>::Context,
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
            #[inline]
            fn default() -> Self {
                Self {
                    is_int: Default::default(),
                    int_context: Default::default(),
                    context: [Default::default(); $bits],
                }
            }
        }

        impl Encode for $t {
            type Context = $context;
            #[inline]
            fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let intvalue = *self as $intty;
                let is_int = intvalue as $t == *self;
                is_int.encode(writer, &mut ctx.is_int);
                if is_int {
                    <Small as EncodingStrategy<$intty>>::encode(
                        &intvalue,
                        writer,
                        &mut ctx.int_context,
                    )
                } else {
                    // The $bits raw bits are independent (one context per position),
                    // so encode them as a single batch.
                    let bits = $intty::from_le_bytes(self.to_le_bytes());
                    let pairs = std::array::from_fn::<_, $bits, _>(|i| {
                        let bit = (bits >> i) & 1 == 1;
                        let probability = ctx.context[i].probability();
                        ctx.context[i] = ctx.context[i].adapt(bit);
                        (bit, probability)
                    });
                    writer.encode_bits(pairs);
                }
            }
            #[inline]
            fn decode<D: super::EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$intty>>::decode(reader, &mut ctx.int_context)?;
                    Ok(intvalue as $t)
                } else {
                    // The $bits raw bits are independent (one context per position),
                    // so decode them as a single register-resident batch.
                    let decoded = reader.decode_bits(&mut ctx.context);
                    let mut bits: $intty = 0;
                    for i in 0..$bits {
                        if decoded[i] {
                            bits |= (1 << i);
                        }
                    }
                    Ok($t::from_le_bytes(bits.to_le_bytes()))
                }
            }
        }

        #[derive(Clone, Default)]
        pub struct $decimal {
            exponent: <Small as EncodingStrategy<i16>>::Context,
            int: <Small as EncodingStrategy<$sint>>::Context,
            is_int: <bool as Encode>::Context,
            integer: <Small as EncodingStrategy<$sint>>::Context,
        }
        impl EncodingStrategy<$t> for Decimal {
            type Context = $decimal;
            fn encode<E: super::EntropyCoder>(value: &$t, writer: &mut E, ctx: &mut Self::Context) {
                let intvalue = *value as $sint;
                let is_int = intvalue as $t == *value;
                is_int.encode(writer, &mut ctx.is_int);
                if is_int {
                    <Small as EncodingStrategy<$sint>>::encode(&intvalue, writer, &mut ctx.integer)
                } else {
                    // This is very hokey.
                    let d = format!("{value}");
                    let (power, int) = if let Some((a, b)) = d.split_once('.') {
                        let power = -(b.len() as i16);
                        let int = format!("{a}{b}");
                        (power, int.parse::<$sint>().expect("bad float {a}{b}"))
                    } else {
                        let int = d.trim_end_matches('0');
                        let power = (d.len() - int.len()) as i16;
                        if int.is_empty() {
                            (0, 0)
                        } else {
                            (power, int.parse::<$sint>().expect("bad float trimzeros"))
                        }
                    };
                    <Small as EncodingStrategy<i16>>::encode(&power, writer, &mut ctx.exponent);
                    <Small as EncodingStrategy<$sint>>::encode(&int, writer, &mut ctx.int)
                }
            }

            fn decode<D: super::EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$sint>>::decode(reader, &mut ctx.integer)?;
                    Ok(intvalue as $t)
                } else {
                    let power =
                        <Small as EncodingStrategy<i16>>::decode(reader, &mut ctx.exponent)?;
                    let int = <Small as EncodingStrategy<$sint>>::decode(reader, &mut ctx.int)?;
                    let s = if power > 0 {
                        let mut s = format!("{int}");
                        for _ in 0..power {
                            s.push('0');
                        }
                        s
                    } else {
                        format!("{int}.0e{power}")
                    };
                    Ok(s.parse().expect("bad decode str"))
                }
            }
        }
    };
}
impl_float!(f64, u64, i64, F64Context, F64Decimal, 64);
impl_float!(f32, u32, i32, F32Context, F32Decimal, 32);

#[test]
fn decimal_float() {
    use super::assert_bits;
    use crate::Encoded;

    fn test_value(v: f64, dec: usize, bin: usize) {
        println!("Testing {v}.");
        assert_bits!(v, bin);
        assert_bits!(Encoded::<f64, Decimal>::from(v), dec);
    }
    fn test32(v: f32, dec: usize, bin: usize) {
        println!("Testing {v}.");
        assert_bits!(v, bin);
        assert_bits!(Encoded::<f32, Decimal>::from(v), dec);
    }

    test_value(1.1, 16, 65);
    test_value(0.1, 13, 65);
    test_value(0.9, 16, 65);
    test_value(128.332, 31, 65);
    test_value(1.0_f64.exp(), 68, 65);
    test_value(0.0, 8, 3);
    test_value(8.0, 11, 10);
    test_value(8e200, 23, 65);
    test_value(8e300, 24, 65);

    test32(1.0_f32.exp(), 39, 33);
    test32(0.1, 12, 33);
    test32(0.0, 7, 3);
    test32(8.0, 10, 9);
}
