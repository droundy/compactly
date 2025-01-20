use crate::Encode;

impl Encode for () {
    type Context = ();
    fn encode<W: std::io::Write>(
        &self,
        _writer: &mut cabac::vp8::VP8Writer<W>,
        _ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn decode<R: std::io::Read>(
        _reader: &mut cabac::vp8::VP8Reader<R>,
        _ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(())
    }
}

impl<T1: Encode, T2: Encode> Encode for (T1, T2) {
    type Context = (T1::Context, T2::Context);

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)?;
        self.3.encode(writer, &mut ctx.3)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)?;
        self.3.encode(writer, &mut ctx.3)?;
        self.4.encode(writer, &mut ctx.4)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)?;
        self.3.encode(writer, &mut ctx.3)?;
        self.4.encode(writer, &mut ctx.4)?;
        self.5.encode(writer, &mut ctx.5)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)?;
        self.3.encode(writer, &mut ctx.3)?;
        self.4.encode(writer, &mut ctx.4)?;
        self.5.encode(writer, &mut ctx.5)?;
        self.6.encode(writer, &mut ctx.6)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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

    fn encode<W: std::io::Write>(
        &self,
        writer: &mut cabac::vp8::VP8Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        self.0.encode(writer, &mut ctx.0)?;
        self.1.encode(writer, &mut ctx.1)?;
        self.2.encode(writer, &mut ctx.2)?;
        self.3.encode(writer, &mut ctx.3)?;
        self.4.encode(writer, &mut ctx.4)?;
        self.5.encode(writer, &mut ctx.5)?;
        self.6.encode(writer, &mut ctx.6)?;
        self.7.encode(writer, &mut ctx.7)
    }

    fn decode<R: std::io::Read>(
        reader: &mut cabac::vp8::VP8Reader<R>,
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
    use crate::assert_size;

    assert_size!((false, false), 0);
    assert_size!((false, true), 1);
    assert_size!((true, true), 1);
    assert_size!((true, false), 1);

    assert_size!((true, true, true), 1);

    assert_size!((true, true, true, true), 1);

    assert_size!((true, true, true, true, true), 1);

    assert_size!((true, true, true, true, true, true), 1);

    assert_size!((true, true, true, true, true, true, true), 1);

    assert_size!((true, true, true, true, true, true, true, true), 2);
}