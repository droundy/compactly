use super::Encode;

use super::Strategy;
use crate::Normal;
#[cfg(test)]
use expect_test::expect;

impl Encode for () {
    type Context = ();
    #[inline]
    fn encode<E: super::EntropyCoder>(_value: &Self, _writer: &mut E, _ctx: &mut Self::Context) {}
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        _reader: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(())
    }
}

impl<T1: Encode, T2: Encode> Encode for (T1, T2) {
    type Context = (T1::Context, T2::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode> Encode for (T1, T2, T3) {
    type Context = (T1::Context, T2::Context, T3::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode> Encode for (T1, T2, T3, T4) {
    type Context = (T1::Context, T2::Context, T3::Context, T4::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2);
        Normal::encode(&value.3, writer, &mut ctx.3)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
            Encode::decode(reader, &mut ctx.3)?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode> Encode for (T1, T2, T3, T4, T5) {
    type Context = (
        T1::Context,
        T2::Context,
        T3::Context,
        T4::Context,
        T5::Context,
    );

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2);
        Normal::encode(&value.3, writer, &mut ctx.3);
        Normal::encode(&value.4, writer, &mut ctx.4)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
            Encode::decode(reader, &mut ctx.3)?,
            Encode::decode(reader, &mut ctx.4)?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode> Encode
    for (T1, T2, T3, T4, T5, T6)
{
    type Context = (
        T1::Context,
        T2::Context,
        T3::Context,
        T4::Context,
        T5::Context,
        T6::Context,
    );

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2);
        Normal::encode(&value.3, writer, &mut ctx.3);
        Normal::encode(&value.4, writer, &mut ctx.4);
        Normal::encode(&value.5, writer, &mut ctx.5)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
            Encode::decode(reader, &mut ctx.3)?,
            Encode::decode(reader, &mut ctx.4)?,
            Encode::decode(reader, &mut ctx.5)?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode> Encode
    for (T1, T2, T3, T4, T5, T6, T7)
{
    type Context = (
        T1::Context,
        T2::Context,
        T3::Context,
        T4::Context,
        T5::Context,
        T6::Context,
        T7::Context,
    );

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2);
        Normal::encode(&value.3, writer, &mut ctx.3);
        Normal::encode(&value.4, writer, &mut ctx.4);
        Normal::encode(&value.5, writer, &mut ctx.5);
        Normal::encode(&value.6, writer, &mut ctx.6)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
            Encode::decode(reader, &mut ctx.3)?,
            Encode::decode(reader, &mut ctx.4)?,
            Encode::decode(reader, &mut ctx.5)?,
            Encode::decode(reader, &mut ctx.6)?,
        ))
    }
}

impl<
        T1: Encode,
        T2: Encode,
        T3: Encode,
        T4: Encode,
        T5: Encode,
        T6: Encode,
        T7: Encode,
        T8: Encode,
    > Encode for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Context = (
        T1::Context,
        T2::Context,
        T3::Context,
        T4::Context,
        T5::Context,
        T6::Context,
        T7::Context,
        T8::Context,
    );

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        Normal::encode(&value.1, writer, &mut ctx.1);
        Normal::encode(&value.2, writer, &mut ctx.2);
        Normal::encode(&value.3, writer, &mut ctx.3);
        Normal::encode(&value.4, writer, &mut ctx.4);
        Normal::encode(&value.5, writer, &mut ctx.5);
        Normal::encode(&value.6, writer, &mut ctx.6);
        Normal::encode(&value.7, writer, &mut ctx.7)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            Encode::decode(reader, &mut ctx.0)?,
            Encode::decode(reader, &mut ctx.1)?,
            Encode::decode(reader, &mut ctx.2)?,
            Encode::decode(reader, &mut ctx.3)?,
            Encode::decode(reader, &mut ctx.4)?,
            Encode::decode(reader, &mut ctx.5)?,
            Encode::decode(reader, &mut ctx.6)?,
            Encode::decode(reader, &mut ctx.7)?,
        ))
    }
}

#[test]
fn sizes() {
    use super::estimated_bits;

    expect!["2"].assert_eq(&estimated_bits!((false, false)));
    expect!["2"].assert_eq(&estimated_bits!((false, true)));
    expect!["2"].assert_eq(&estimated_bits!((true, true)));
    expect!["2"].assert_eq(&estimated_bits!((true, false)));

    expect!["3"].assert_eq(&estimated_bits!((true, true, true)));

    expect!["4"].assert_eq(&estimated_bits!((true, true, true, true)));

    expect!["3"].assert_eq(&estimated_bits!((false, false, false)));

    expect!["4"].assert_eq(&estimated_bits!((false, false, false, false)));

    expect!["5"].assert_eq(&estimated_bits!((false, false, false, false, false)));

    expect!["6"].assert_eq(&estimated_bits!((false, false, false, false, false, false)));

    expect!["7"].assert_eq(&estimated_bits!((
        false, false, false, false, false, false, false
    )));

    expect!["8"].assert_eq(&estimated_bits!((
        false, false, false, false, false, false, false, false
    )));
}
