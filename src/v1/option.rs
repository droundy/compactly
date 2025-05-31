use super::{Encode, EncodingStrategy, LowCardinality};

pub struct OptionContext<T: Encode> {
    is_some: <bool as Encode>::Context,
    value: T::Context,
}
impl<T: Encode> Default for OptionContext<T> {
    #[inline]
    fn default() -> Self {
        Self {
            is_some: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T: Encode> Clone for OptionContext<T> {
    fn clone(&self) -> Self {
        Self {
            is_some: self.is_some.clone(),
            value: self.value.clone(),
        }
    }
}
impl<T: Encode> Encode for Option<T> {
    type Context = OptionContext<T>;
    #[inline]
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = self {
            true.encode(writer, &mut ctx.is_some)?;
            v.encode(writer, &mut ctx.value)
        } else {
            false.encode(writer, &mut ctx.is_some)
        }
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_some)? {
            Ok(Some(T::decode(reader, &mut ctx.value)?))
        } else {
            Ok(None)
        }
    }
}

macro_rules! option_encoding_strategy {
    ($t:ty, $strategy:ident, $mod:ident) => {
        mod $mod {
            use super::{Encode, EncodingStrategy};
            use crate::$strategy;
            #[derive(Default, Clone)]
            pub struct Context {
                is_some: <bool as Encode>::Context,
                value: <$strategy as EncodingStrategy<$t>>::Context,
            }

            impl EncodingStrategy<Option<$t>> for $strategy {
                type Context = Context;
                fn encode<W: std::io::Write>(
                    value: &Option<$t>,
                    writer: &mut crate::v1::Writer<W>,
                    ctx: &mut Self::Context,
                ) -> Result<(), std::io::Error> {
                    if let Some(v) = value {
                        true.encode(writer, &mut ctx.is_some)?;
                        $strategy::encode(v, writer, &mut ctx.value)
                    } else {
                        false.encode(writer, &mut ctx.is_some)
                    }
                }
                fn millibits(value: &Option<$t>, ctx: &mut Self::Context) -> Option<usize> {
                    if let Some(v) = value {
                        Some(
                            true.millibits(&mut ctx.is_some)?
                                + $strategy::millibits(v, &mut ctx.value)?,
                        )
                    } else {
                        false.millibits(&mut ctx.is_some)
                    }
                }
                fn decode<R: std::io::Read>(
                    reader: &mut crate::v1::Reader<R>,
                    ctx: &mut Self::Context,
                ) -> Result<Option<$t>, std::io::Error> {
                    if bool::decode(reader, &mut ctx.is_some)? {
                        Ok(Some($strategy::decode(reader, &mut ctx.value)?))
                    } else {
                        Ok(None)
                    }
                }
            }
        }
    };
}

option_encoding_strategy!(String, Compressible, compressible_string);
option_encoding_strategy!(u64, Small, small_u64);
option_encoding_strategy!(usize, Small, small_usize);

#[derive(Default, Clone)]
pub struct LowContext<C: Default> {
    is_some: <bool as Encode>::Context,
    value: C,
}

impl<T> EncodingStrategy<Option<T>> for LowCardinality
where
    LowCardinality: EncodingStrategy<T>,
{
    type Context = LowContext<<LowCardinality as EncodingStrategy<T>>::Context>;
    fn encode<W: std::io::Write>(
        value: &Option<T>,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if let Some(v) = value {
            true.encode(writer, &mut ctx.is_some)?;
            LowCardinality::encode(v, writer, &mut ctx.value)
        } else {
            false.encode(writer, &mut ctx.is_some)
        }
    }
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Option<T>, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_some)? {
            Ok(Some(LowCardinality::decode(reader, &mut ctx.value)?))
        } else {
            Ok(None)
        }
    }
}
