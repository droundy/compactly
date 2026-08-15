use super::Encode;
use super::EncodeExt;

/// Context for an `Option<T>` encoded with strategy `S`: one bit saying whether
/// the value is present, plus whatever context `T` needs under `S`.
pub struct OptionContext<T: Encode<S>, S> {
    is_some: <bool as Encode>::Context,
    value: <T as Encode<S>>::Context,
}
impl<T: Encode<S>, S> Default for OptionContext<T, S> {
    #[inline]
    fn default() -> Self {
        Self {
            is_some: Default::default(),
            value: Default::default(),
        }
    }
}
impl<T: Encode<S>, S> Clone for OptionContext<T, S> {
    fn clone(&self) -> Self {
        Self {
            is_some: self.is_some,
            value: self.value.clone(),
        }
    }
}

/// `Option<T>` is transparent to the strategy: whatever strategy the field asks
/// for is applied to the value inside. This one impl covers `Option<T>` under
/// every strategy `T` itself supports — including strategies defined in other
/// crates — so `#[compactly(Small)] x: Option<u32>` needs nothing added here.
#[diagnostic::do_not_recommend]
impl<T: Encode<S>, S> Encode<S> for Option<T> {
    type Context = OptionContext<T, S>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(v) = value {
            true.encode(writer, &mut ctx.is_some);
            <T as Encode<S>>::encode(v, writer, &mut ctx.value)
        } else {
            false.encode(writer, &mut ctx.is_some)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if <bool as Encode>::decode(reader, &mut ctx.is_some)? {
            Ok(Some(<T as Encode<S>>::decode(reader, &mut ctx.value)?))
        } else {
            Ok(None)
        }
    }
}
