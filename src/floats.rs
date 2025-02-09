use crate::{Encode, EncodingStrategy, Small};
use std::io::{Read, Write};

macro_rules! impl_float {
    ($t:ident, $intty:ident, $context:ident, $bits:literal) => {
        #[derive(Clone)]
        pub struct $context {
            is_int: <bool as Encode>::Context,
            int_context: <Small as EncodingStrategy<$intty>>::Context,
            context: [<bool as Encode>::Context; $bits],
        }
        impl Default for $context {
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
            fn encode<W: Write>(
                &self,
                writer: &mut crate::Writer<W>,
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
                    for i in (0..$bits).rev() {
                        (bits & 1 == 1).encode(writer, &mut ctx.context[i])?;
                        bits = bits >> 1;
                    }
                    Ok(())
                }
            }
            fn decode<R: Read>(
                reader: &mut crate::Reader<R>,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_int)? {
                    let intvalue =
                        <Small as EncodingStrategy<$intty>>::decode(reader, &mut ctx.int_context)?;
                    Ok(intvalue as $t)
                } else {
                    let mut bits: $intty = 0;
                    for i in (0..$bits).rev() {
                        bits = bits << 1;
                        if bool::decode(reader, &mut ctx.context[i])? {
                            bits = bits | 1;
                        }
                    }
                    Ok($t::from_le_bytes(bits.to_le_bytes()))
                }
            }
        }
    };
}
impl_float!(f64, u64, F64Context, 64);
impl_float!(f32, u32, F32Context, 32);
