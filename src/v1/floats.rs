use super::{Encode, EncodingStrategy};
use crate::{Decimal, LowCardinality, Small};
use std::io::{Read, Write};

#[cfg(test)]
use expect_test::expect;

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
            fn encode<W: Write>(
                &self,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let intvalue = *self as $intty;
                let is_int = intvalue as $t == *self;
                is_int.encode(writer, &mut ctx.is_int)?;
                if is_int {
                    <Small as EncodingStrategy<$intty>>::encode(
                        &intvalue,
                        writer,
                        &mut ctx.int_context,
                    )
                } else {
                    let mut bits = $intty::from_le_bytes(self.to_le_bytes());
                    for i in 0..$bits {
                        (bits & 1 == 1).encode(writer, &mut ctx.context[i])?;
                        bits >>= 1;
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
            /// Normal means not infinity, NaN, etc.
            is_normal: <bool as Encode>::Context,
            /// In case of abnormal numbers, we just use a LowCardinality
            /// encoding of the bytes
            ///
            /// This is a bit lazy, but ensures that we get a bitwise identical
            /// NaN in case that matters.
            abnormal:
                <LowCardinality as EncodingStrategy<[u8; std::mem::size_of::<$t>()]>>::Context,
        }
        impl EncodingStrategy<$t> for Decimal {
            type Context = $decimal;
            fn encode<W: Write>(
                value: &$t,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                let intvalue = *value as $sint;
                let is_int = intvalue as $t == *value;
                is_int.encode(writer, &mut ctx.is_int)?;
                if is_int {
                    <Small as EncodingStrategy<$sint>>::encode(&intvalue, writer, &mut ctx.integer)
                } else {
                    // This is very hokey.
                    let d = format!("{value}");
                    if let Some((power, int)) = if let Some((a, b)) = d.split_once('.') {
                        let power = -(b.len() as i16);
                        let int = format!("{a}{b}");
                        Some((power, int.parse::<$sint>().expect("bad float {a}{b}")))
                    } else {
                        let int = d.trim_end_matches('0');
                        let power = (d.len() - int.len()) as i16;
                        if int.is_empty() {
                            Some((0, 0))
                        } else if let Ok(mantissa) = int.parse::<$sint>() {
                            Some((power, mantissa))
                        } else {
                            None
                        }
                    } {
                        true.encode(writer, &mut ctx.is_normal)?;
                        <Small as EncodingStrategy<i16>>::encode(
                            &power,
                            writer,
                            &mut ctx.exponent,
                        )?;
                        <Small as EncodingStrategy<$sint>>::encode(&int, writer, &mut ctx.int)
                    } else {
                        false.encode(writer, &mut ctx.is_normal)?;
                        <LowCardinality as EncodingStrategy<
                                                    [u8; std::mem::size_of::<$t>()],
                                                >>::encode(
                                                    &value.to_be_bytes(), writer, &mut ctx.abnormal
                                                )
                    }
                }
            }

            fn decode<R: Read>(
                reader: &mut super::Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<$t, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$sint>>::decode(reader, &mut ctx.integer)?;
                    Ok(intvalue as $t)
                } else if bool::decode(reader, &mut ctx.is_normal)? {
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
                } else {
                    let bytes = <LowCardinality as EncodingStrategy<
                        [u8; std::mem::size_of::<$t>()],
                    >>::decode(reader, &mut ctx.abnormal)?;
                    Ok($t::from_be_bytes(bytes))
                }
            }
        }
    };
}
impl_float!(f64, u64, i64, F64Context, F64Decimal, 64);
impl_float!(f32, u32, i32, F32Context, F32Decimal, 32);

#[test]
fn decimal_float() {
    use crate::Encoded;

    fn sizes(v: f64) -> String {
        println!("Testing {v}.");
        format!(
            "decimal: {} bits, binary: {} bits",
            super::encoded_bits!(Encoded::<f64, Decimal>::from(v)),
            super::encoded_bits!(v)
        )
    }
    fn sizes32(v: f32) -> String {
        println!("Testing {v}.");
        format!(
            "decimal: {} bits, binary: {} bits",
            super::encoded_bits!(Encoded::<f32, Decimal>::from(v)),
            super::encoded_bits!(v)
        )
    }

    expect!["decimal: 18 bits, binary: 65 bits"].assert_eq(&sizes(1.1));
    expect!["decimal: 16 bits, binary: 65 bits"].assert_eq(&sizes(0.1));
    expect!["decimal: 18 bits, binary: 65 bits"].assert_eq(&sizes(0.9));
    expect!["decimal: 31 bits, binary: 65 bits"].assert_eq(&sizes(128.332));
    expect!["decimal: 68 bits, binary: 65 bits"].assert_eq(&sizes(1.0_f64.exp()));
    expect!["decimal: 9 bits, binary: 3 bits"].assert_eq(&sizes(0.0));
    expect!["decimal: 11 bits, binary: 10 bits"].assert_eq(&sizes(8.0));
    expect!["decimal: 24 bits, binary: 65 bits"].assert_eq(&sizes(8e200));
    expect!["decimal: 25 bits, binary: 65 bits"].assert_eq(&sizes(8e300));

    expect!["decimal: 39 bits, binary: 33 bits"].assert_eq(&sizes32(1.0_f32.exp()));
    expect!["decimal: 15 bits, binary: 33 bits"].assert_eq(&sizes32(0.1));
    expect!["decimal: 8 bits, binary: 3 bits"].assert_eq(&sizes32(0.0));
    expect!["decimal: 10 bits, binary: 9 bits"].assert_eq(&sizes32(8.0));
}
