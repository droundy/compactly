use super::{Encode, Strategy};
use crate::{Mapping, Normal};

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

impl<T1: Encode, T2: Encode> Encode for (T1, T2) {
    type Context = (T1::Context, T2::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES.saturating_add(<T2 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
        ))
    }
}

/// A pair coded under [`Mapping`], each half with its own strategy — which is
/// exactly one entry of a `Mapping` collection.
///
/// Spelling the entry as a type is what lets a map's async decode ask
/// [`sync_capacity`](super::AsyncEntropyDecoder::sync_capacity) about the unit
/// it actually hands over, and so share the same batching helper as every other
/// length-driven collection rather than needing a two-value variant of it.
/// Coding order and contexts are the key's then the value's, identically to the
/// map impls, so a map is free to decode its entries through this.
impl<K: Encode<SK>, SK, V: Encode<SV>, SV> Encode<Mapping<SK, SV>> for (K, V) {
    type Context = (<K as Encode<SK>>::Context, <V as Encode<SV>>::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        <K as Encode<SK>>::encode(&value.0, writer, &mut ctx.0);
        super::split_unless_atomic(writer, <Self as Encode<Mapping<SK, SV>>>::MAX_BYTES);
        <V as Encode<SV>>::encode(&value.1, writer, &mut ctx.1)
    }

    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok((
            <K as Encode<SK>>::decode(reader, &mut ctx.0)?,
            <V as Encode<SV>>::decode(reader, &mut ctx.1)?,
        ))
    }

    /// The key then the value, each under its own strategy.
    const MAX_BYTES: usize =
        <K as Encode<SK>>::MAX_BYTES.saturating_add(<V as Encode<SV>>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(K, V), std::io::Error> {
        Ok((
            <K as Encode<SK>>::decode_async(reader, &mut ctx.0).await?,
            <V as Encode<SV>>::decode_async(reader, &mut ctx.1).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode> Encode for (T1, T2, T3) {
    type Context = (T1::Context, T2::Context, T3::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
        ))
    }
}

impl<T1: Encode, T2: Encode, T3: Encode, T4: Encode> Encode for (T1, T2, T3, T4) {
    type Context = (T1::Context, T2::Context, T3::Context, T4::Context);

    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        Normal::encode(&value.0, writer, &mut ctx.0);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.2, writer, &mut ctx.2);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES)
        .saturating_add(<T4 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
            <T4 as Encode>::decode_async(reader, &mut ctx.3).await?,
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
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.2, writer, &mut ctx.2);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.3, writer, &mut ctx.3);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES)
        .saturating_add(<T4 as Encode>::MAX_BYTES)
        .saturating_add(<T5 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
            <T4 as Encode>::decode_async(reader, &mut ctx.3).await?,
            <T5 as Encode>::decode_async(reader, &mut ctx.4).await?,
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
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.2, writer, &mut ctx.2);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.3, writer, &mut ctx.3);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.4, writer, &mut ctx.4);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES)
        .saturating_add(<T4 as Encode>::MAX_BYTES)
        .saturating_add(<T5 as Encode>::MAX_BYTES)
        .saturating_add(<T6 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
            <T4 as Encode>::decode_async(reader, &mut ctx.3).await?,
            <T5 as Encode>::decode_async(reader, &mut ctx.4).await?,
            <T6 as Encode>::decode_async(reader, &mut ctx.5).await?,
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
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.2, writer, &mut ctx.2);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.3, writer, &mut ctx.3);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.4, writer, &mut ctx.4);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.5, writer, &mut ctx.5);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES)
        .saturating_add(<T4 as Encode>::MAX_BYTES)
        .saturating_add(<T5 as Encode>::MAX_BYTES)
        .saturating_add(<T6 as Encode>::MAX_BYTES)
        .saturating_add(<T7 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
            <T4 as Encode>::decode_async(reader, &mut ctx.3).await?,
            <T5 as Encode>::decode_async(reader, &mut ctx.4).await?,
            <T6 as Encode>::decode_async(reader, &mut ctx.5).await?,
            <T7 as Encode>::decode_async(reader, &mut ctx.6).await?,
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
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.1, writer, &mut ctx.1);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.2, writer, &mut ctx.2);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.3, writer, &mut ctx.3);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.4, writer, &mut ctx.4);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.5, writer, &mut ctx.5);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
        Normal::encode(&value.6, writer, &mut ctx.6);
        super::split_unless_atomic(writer, <Self as Encode>::MAX_BYTES);
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

    /// The elements, coded in sequence.
    const MAX_BYTES: usize = <T1 as Encode>::MAX_BYTES
        .saturating_add(<T2 as Encode>::MAX_BYTES)
        .saturating_add(<T3 as Encode>::MAX_BYTES)
        .saturating_add(<T4 as Encode>::MAX_BYTES)
        .saturating_add(<T5 as Encode>::MAX_BYTES)
        .saturating_add(<T6 as Encode>::MAX_BYTES)
        .saturating_add(<T7 as Encode>::MAX_BYTES)
        .saturating_add(<T8 as Encode>::MAX_BYTES);

    #[inline]
    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8), std::io::Error> {
        Ok((
            <T1 as Encode>::decode_async(reader, &mut ctx.0).await?,
            <T2 as Encode>::decode_async(reader, &mut ctx.1).await?,
            <T3 as Encode>::decode_async(reader, &mut ctx.2).await?,
            <T4 as Encode>::decode_async(reader, &mut ctx.3).await?,
            <T5 as Encode>::decode_async(reader, &mut ctx.4).await?,
            <T6 as Encode>::decode_async(reader, &mut ctx.5).await?,
            <T7 as Encode>::decode_async(reader, &mut ctx.6).await?,
            <T8 as Encode>::decode_async(reader, &mut ctx.7).await?,
        ))
    }
}

/// A pair under [`Mapping`] must code exactly what a map's own key-then-value
/// loop codes, since that is what lets a map decode its entries through this
/// impl — and, in passing, `Values<Mapping<..>>` gives a `Vec` of pairs
/// per-half strategies, which nothing else offers.
#[test]
fn mapping_pair_round_trips() {
    use crate::{Encoded, Small, Sorted, Values};
    let pairs: Vec<(u64, u64)> = (0..500).map(|i| (i * 3, i * i)).collect();

    let v = Encoded::<Vec<(u64, u64)>, Values<Mapping<Small, Normal>>>::new(pairs.clone());
    let encoded = super::encode(&v);
    assert_eq!(super::decode(&encoded).as_ref(), Some(&v));

    // A sorted-key pair is what `BTreeMap`'s default `Mapping<Sorted, Normal>`
    // uses, so this is the shape its batch loop asks `sync_capacity` about.
    let v = Encoded::<Vec<(u64, u64)>, Values<Mapping<Sorted, Normal>>>::new(pairs);
    let encoded = super::encode(&v);
    assert_eq!(super::decode(&encoded).as_ref(), Some(&v));
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
