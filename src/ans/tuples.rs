use super::{Encode, Reader};

impl Encode for () {
    type Context = ();
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, _writer: &mut E, _ctx: &mut Self::Context) {}
    #[inline]
    fn decode<R: std::io::Read>(
        _reader: &mut Reader<R>,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(())
    }
}

impl<T1: Encode, T2: Encode> Encode for (T1, T2) {
    type Context = (T1::Context, T2::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4);
        self.5.encode(writer, &mut ctx.5)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4);
        self.5.encode(writer, &mut ctx.5);
        self.6.encode(writer, &mut ctx.6)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4);
        self.5.encode(writer, &mut ctx.5);
        self.6.encode(writer, &mut ctx.6);
        self.7.encode(writer, &mut ctx.7)
    }

    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
    use super::assert_bits;

    assert_bits!((false, false), 2);
    assert_bits!((false, true), 2);
    assert_bits!((true, true), 1);
    assert_bits!((true, false), 2);

    assert_bits!((true, true, true), 1);

    assert_bits!((true, true, true, true), 1);

    assert_bits!((false, false, false), 3);

    assert_bits!((false, false, false, false), 4);

    assert_bits!((false, false, false, false, false), 5);

    assert_bits!((false, false, false, false, false, false), 6);

    assert_bits!((false, false, false, false, false, false, false), 7);

    assert_bits!((false, false, false, false, false, false, false, false), 8);
}
