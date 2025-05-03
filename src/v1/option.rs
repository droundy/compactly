use super::Encode;

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
