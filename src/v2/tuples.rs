use super::Encode;

#[cfg(test)]
use expect_test::expect;

impl Encode for () {
    type Context = ();
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, _writer: &mut E, _ctx: &mut Self::Context) {}
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1)
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2)
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3)
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4)
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
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        self.0.encode(writer, &mut ctx.0);
        self.1.encode(writer, &mut ctx.1);
        self.2.encode(writer, &mut ctx.2);
        self.3.encode(writer, &mut ctx.3);
        self.4.encode(writer, &mut ctx.4);
        self.5.encode(writer, &mut ctx.5)
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

impl super::DecodeAsync<()> for crate::Normal {
    /// Carries no information, so nothing is coded.
    const MAX_BYTES: usize = 0;

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        _reader: &mut D,
        _ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl<T1: Encode, T2: Encode> super::DecodeAsync<(T1, T2)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode> super::DecodeAsync<(T1, T2, T3)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode> super::DecodeAsync<(T1, T2, T3, T4)>
    for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T4> + super::EncodingStrategy<T4, Context = <T4 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T4>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
            <crate::Normal as super::DecodeAsync<T4>>::decode_async(reader, &mut ctx.3).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode>
    super::DecodeAsync<(T1, T2, T3, T4, T5)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T4> + super::EncodingStrategy<T4, Context = <T4 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T5> + super::EncodingStrategy<T5, Context = <T5 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T4>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T5>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
            <crate::Normal as super::DecodeAsync<T4>>::decode_async(reader, &mut ctx.3).await?,
            <crate::Normal as super::DecodeAsync<T5>>::decode_async(reader, &mut ctx.4).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode>
    super::DecodeAsync<(T1, T2, T3, T4, T5, T6)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T4> + super::EncodingStrategy<T4, Context = <T4 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T5> + super::EncodingStrategy<T5, Context = <T5 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T6> + super::EncodingStrategy<T6, Context = <T6 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T4>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T5>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T6>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
            <crate::Normal as super::DecodeAsync<T4>>::decode_async(reader, &mut ctx.3).await?,
            <crate::Normal as super::DecodeAsync<T5>>::decode_async(reader, &mut ctx.4).await?,
            <crate::Normal as super::DecodeAsync<T6>>::decode_async(reader, &mut ctx.5).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode, T5: Encode, T6: Encode, T7: Encode>
    super::DecodeAsync<(T1, T2, T3, T4, T5, T6, T7)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T4> + super::EncodingStrategy<T4, Context = <T4 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T5> + super::EncodingStrategy<T5, Context = <T5 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T6> + super::EncodingStrategy<T6, Context = <T6 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T7> + super::EncodingStrategy<T7, Context = <T7 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T4>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T5>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T6>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T7>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
            <crate::Normal as super::DecodeAsync<T4>>::decode_async(reader, &mut ctx.3).await?,
            <crate::Normal as super::DecodeAsync<T5>>::decode_async(reader, &mut ctx.4).await?,
            <crate::Normal as super::DecodeAsync<T6>>::decode_async(reader, &mut ctx.5).await?,
            <crate::Normal as super::DecodeAsync<T7>>::decode_async(reader, &mut ctx.6).await?,
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
    > super::DecodeAsync<(T1, T2, T3, T4, T5, T6, T7, T8)> for crate::Normal
where
    crate::Normal:
        super::DecodeAsync<T1> + super::EncodingStrategy<T1, Context = <T1 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T2> + super::EncodingStrategy<T2, Context = <T2 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T3> + super::EncodingStrategy<T3, Context = <T3 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T4> + super::EncodingStrategy<T4, Context = <T4 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T5> + super::EncodingStrategy<T5, Context = <T5 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T6> + super::EncodingStrategy<T6, Context = <T6 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T7> + super::EncodingStrategy<T7, Context = <T7 as Encode>::Context>,
    crate::Normal:
        super::DecodeAsync<T8> + super::EncodingStrategy<T8, Context = <T8 as Encode>::Context>,
{
    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <crate::Normal as super::DecodeAsync<T1>>::MAX_BYTES
        .saturating_add(<crate::Normal as super::DecodeAsync<T2>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T3>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T4>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T5>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T6>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T7>>::MAX_BYTES)
        .saturating_add(<crate::Normal as super::DecodeAsync<T8>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8), std::io::Error> {
        Ok((
            <crate::Normal as super::DecodeAsync<T1>>::decode_async(reader, &mut ctx.0).await?,
            <crate::Normal as super::DecodeAsync<T2>>::decode_async(reader, &mut ctx.1).await?,
            <crate::Normal as super::DecodeAsync<T3>>::decode_async(reader, &mut ctx.2).await?,
            <crate::Normal as super::DecodeAsync<T4>>::decode_async(reader, &mut ctx.3).await?,
            <crate::Normal as super::DecodeAsync<T5>>::decode_async(reader, &mut ctx.4).await?,
            <crate::Normal as super::DecodeAsync<T6>>::decode_async(reader, &mut ctx.5).await?,
            <crate::Normal as super::DecodeAsync<T7>>::decode_async(reader, &mut ctx.6).await?,
            <crate::Normal as super::DecodeAsync<T8>>::decode_async(reader, &mut ctx.7).await?,
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
