use super::{Encode, EncodingStrategy, LowCardinality};
use crate::{Compressible, Normal, Small};

pub struct OptionContext<T, S: EncodingStrategy<T>> {
    is_some: <bool as Encode>::Context,
    value: S::Context,
}
impl<T, S: EncodingStrategy<T>> Default for OptionContext<T, S> {
    #[inline]
    fn default() -> Self {
        Self {
            is_some: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T, S: EncodingStrategy<T>> Clone for OptionContext<T, S> {
    fn clone(&self) -> Self {
        Self {
            is_some: self.is_some.clone(),
            value: self.value.clone(),
        }
    }
}
impl<T: Encode> Encode for Option<T> {
    type Context = OptionContext<T, Normal>;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(v) = self {
            true.encode(writer, &mut ctx.is_some);
            v.encode(writer, &mut ctx.value)
        } else {
            false.encode(writer, &mut ctx.is_some)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
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
    ($t:ty, $strategy:ident) => {
        impl EncodingStrategy<Option<$t>> for $strategy {
            type Context = OptionContext<$t, $strategy>;
            fn encode<E: super::EntropyCoder>(
                value: &Option<$t>,
                writer: &mut E,
                ctx: &mut Self::Context,
            ) {
                if let Some(v) = value {
                    true.encode(writer, &mut ctx.is_some);
                    $strategy::encode(v, writer, &mut ctx.value)
                } else {
                    false.encode(writer, &mut ctx.is_some)
                }
            }
            fn decode<D: super::EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Option<$t>, std::io::Error> {
                if bool::decode(reader, &mut ctx.is_some)? {
                    Ok(Some($strategy::decode(reader, &mut ctx.value)?))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

option_encoding_strategy!(String, Compressible);
option_encoding_strategy!(u64, Small);
option_encoding_strategy!(usize, Small);

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
    fn encode<E: super::EntropyCoder>(value: &Option<T>, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(v) = value {
            true.encode(writer, &mut ctx.is_some);
            LowCardinality::encode(v, writer, &mut ctx.value)
        } else {
            false.encode(writer, &mut ctx.is_some)
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Option<T>, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_some)? {
            Ok(Some(LowCardinality::decode(reader, &mut ctx.value)?))
        } else {
            Ok(None)
        }
    }
}
