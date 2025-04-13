use super::{Decimal, Encode, EncodingStrategy, Small};
use std::io::{Read, Write};

macro_rules! impl_float {
    ($t:ident, $intty:ident, $context:ident, $decimal:ident, $bits:literal) => {
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
            fn encode<W: Write>(
                &self,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let intvalue = *self as $intty;
                let is_int = intvalue as $t == *self;
                is_int.encode(writer, &mut ctx.is_int)?;
                if is_int {
                    println!("encoding as small int");
                    <Small as EncodingStrategy<$intty>>::encode(
                        &intvalue,
                        writer,
                        &mut ctx.int_context,
                    )
                } else {
                    let mut bits = $intty::from_le_bytes(self.to_le_bytes());
                    println!("encoding {bits:x?}");
                    for i in 0..$bits {
                        (bits & 1 == 1).encode(writer, &mut ctx.context[i])?;
                        bits = bits >> 1;
                    }
                    Ok(())
                }
            }
            #[inline]
            fn decode<R: Read>(
                reader: &mut super::Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$intty>>::decode(reader, &mut ctx.int_context)?;
                    Ok(intvalue as $t)
                } else {
                    let mut bits: $intty = 0;
                    for i in 0..$bits {
                        if bool::decode(reader, &mut ctx.context[i])? {
                            bits = bits | (1 << i);
                        }
                    }
                    println!("decoding {bits:x?}");
                    Ok($t::from_le_bytes(bits.to_le_bytes()))
                }
            }
        }

        #[derive(Clone, Default)]
        pub struct $decimal {
            exponent: <Small as EncodingStrategy<i16>>::Context,
            int: <Small as EncodingStrategy<$intty>>::Context,
        }
        impl EncodingStrategy<$t> for Decimal {
            type Context = $decimal;
            fn encode<W: Write>(
                value: &$t,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                // This is very hokey.
                let d = format!("{value}");
                let (power, int) = if let Some((a, b)) = d.split_once('.') {
                    let power = -(b.len() as i16);
                    let int = format!("{a}{b}");
                    println!("parsing {int} as int?");
                    (power, int.parse::<$intty>().unwrap())
                } else {
                    let int = d.trim_end_matches('0');
                    let power = (d.len() - int.len()) as i16;
                    println!("parsing {int} as int");
                    if int.is_empty() {
                        (0, 0)
                    } else {
                        (power, int.parse::<$intty>().unwrap())
                    }
                };
                <Small as EncodingStrategy<i16>>::encode(&power, writer, &mut ctx.exponent)?;
                <Small as EncodingStrategy<$intty>>::encode(&int, writer, &mut ctx.int)
            }

            fn decode<R: Read>(
                reader: &mut super::Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                let power = <Small as EncodingStrategy<i16>>::decode(reader, &mut ctx.exponent)?;
                let int = <Small as EncodingStrategy<$intty>>::decode(reader, &mut ctx.int)?;
                let s = if power > 0 {
                    format!("{int}{0:0$}", power as usize)
                } else {
                    format!("{int}.0e{power}")
                };
                println!("parsing {s} as float");
                Ok(s.parse().unwrap())
            }
        }
    };
}
impl_float!(f64, u64, F64Context, F64Decimal, 64);
impl_float!(f32, u32, F32Context, F32Decimal, 32);

#[test]
fn decimal_float() {
    use super::{assert_bits, Encoded};

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

    test_value(1.1, 15, 65);
    test_value(0.1, 13, 65);
    test_value(0.9, 15, 65);
    test_value(128.332, 28, 65);
    test_value(1.0_f64.exp(), 65, 65);
    test_value(0.0, 13, 3);
    test_value(8.0, 15, 10);

    test32(1.0_f32.exp(), 36, 33);
    test32(0.1, 12, 33);
    test32(0.0, 12, 3);
    test32(8.0, 14, 9);
}
