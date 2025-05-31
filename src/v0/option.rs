use super::{Encode, EncodingStrategy};
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
impl<T: Encode> Encode for Option<T> {
    type Context = OptionContext<T, Normal>;
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
    ($t:ty, $strategy:ident) => {
        impl EncodingStrategy<Option<$t>> for $strategy {
            type Context = OptionContext<$t, $strategy>;
            fn encode<W: std::io::Write>(
                value: &Option<$t>,
                writer: &mut super::Writer<W>,
                ctx: &mut Self::Context,
            ) -> Result<(), std::io::Error> {
                if let Some(v) = value {
                    true.encode(writer, &mut ctx.is_some)?;
                    $strategy::encode(v, writer, &mut ctx.value)
                } else {
                    false.encode(writer, &mut ctx.is_some)
                }
            }
            fn decode<R: std::io::Read>(
                reader: &mut super::Reader<R>,
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
