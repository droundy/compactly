#![cfg(all(feature = "v1", feature = "v2"))]
#![allow(clippy::enum_variant_names)]
use expect_test::expect;

#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $expected:expr) => {
        let v = (
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
            ($v, $v, $v, $v, $v, $v, $v, $v),
        );
        let bytes = compactly::v1::encode(&v);
        let decoded = compactly::v1::decode(&bytes);
        let ans_bytes = compactly::v2::encode(&v);
        let ans_decoded = compactly::v2::decode(&ans_bytes);
        let some_v = Some(v);
        assert_eq!(decoded, some_v, "decoded value is incorrect");
        assert_eq!(ans_decoded, some_v, "ANS decoded value is incorrect");
        $expected.assert_eq(&format!(
            "v1: {} bits, v2: {} bits",
            (bytes.len() + 4) / 8,
            (ans_bytes.len() + 4) / 8
        ));
    };
}

#[test]
fn singlet_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize);

    assert_bits!(Tuple(0), expect!["v1: 3 bits, v2: 1 bits"]);
    assert_bits!(Tuple(1), expect!["v1: 3 bits, v2: 2 bits"]);
    assert_bits!(Tuple(2), expect!["v1: 3 bits, v2: 4 bits"]);
}

#[test]
fn pair_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize, bool);

    assert_bits!(Tuple(0, false), expect!["v1: 4 bits, v2: 2 bits"]);
    assert_bits!(Tuple(1, true), expect!["v1: 4 bits, v2: 3 bits"]);
    assert_bits!(Tuple(2, false), expect!["v1: 4 bits, v2: 5 bits"]);
    assert_bits!(Tuple(2048, false), expect!["v1: 18 bits, v2: 20 bits"]);
}

#[test]
fn zero_size() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple;

    assert_bits!(Tuple, expect!["v1: 0 bits, v2: 0 bits"]);
    assert_bits!([Tuple; 4], expect!["v1: 0 bits, v2: 0 bits"]);
    assert_bits!([Tuple; 1024], expect!["v1: 0 bits, v2: 0 bits"]);
}

#[test]
fn derive_strategy_for_newtype() {
    use compactly::{v1, v2, Compressible, Mapping, Small, Sorted};
    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, v2::Encode, v1::Encode)]
    #[compactly(Sorted)]
    pub struct NewType(u32);

    // assert_bits!(NewType(0), 32, 32);
    // assert_bits!(NewType(13), 32, 32);
    assert_eq!(v2::encode_with(Sorted, &NewType(0)).len(), 1);
    // assert_bits!(
    //     std::collections::BTreeSet::from([NewType(0), NewType(1)]),
    //     33,
    //     33
    // );

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, v2::Encode, v1::Encode)]
    #[compactly(Sorted)]
    #[compactly(Small)]
    pub struct Both(u32);

    assert_eq!(v2::encode_with(Sorted, &Both(0)).len(), 1);

    #[derive(Clone, Debug, PartialEq, v2::Encode, v1::Encode)]
    #[compactly(Mapping<Small, Compressible>)]
    pub struct Map(std::collections::BTreeMap<u32, String>);

    let strategy: Mapping<Small, Compressible> = Mapping::default();
    assert_eq!(v2::encode_with(strategy, &Map([].into())).len(), 1);
}

#[test]
fn record() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple {
        size: usize,
        happy: bool,
        age: usize,
    }

    assert_bits!(
        Tuple {
            size: 0,
            happy: false,
            age: 51
        },
        expect!["v1: 16 bits, v2: 15 bits"]
    );
    assert_bits!(
        Tuple {
            size: 1024,
            happy: true,
            age: 51
        },
        expect!["v1: 29 bits, v2: 31 bits"]
    );
}

#[test]
fn simple_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum A {
        A,
        B,
        C,
        D,
    }

    assert_bits!(A::A, expect!["v1: 2 bits, v2: 2 bits"]);
    assert_bits!(A::D, expect!["v1: 1 bits, v2: 2 bits"]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum Bool {
        True,
        False,
    }

    assert_bits!(Bool::True, expect!["v1: 1 bits, v2: 1 bits"]);
    assert_bits!(Bool::False, expect!["v1: 1 bits, v2: 1 bits"]);
}

#[test]
fn bigger_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum A {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
    }

    assert_bits!(A::A, expect!["v1: 3 bits, v2: 3 bits"]);
    assert_bits!(A::D, expect!["v1: 3 bits, v2: 3 bits"]);
    assert_bits!(A::J, expect!["v1: 1 bits, v2: 3 bits"]);
}

#[test]
fn weird_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum A {
        A { age: usize },
        B { age: bool },
    }

    assert_bits!(A::A { age: 51 }, expect!["v1: 13 bits, v2: 13 bits"]);
    assert_bits!(A::B { age: false }, expect!["v1: 2 bits, v2: 2 bits"]);
}

#[test]
fn fancy_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum A {
        A {
            #[compactly(Small)]
            age: u64,
        },
        B {
            big: bool,
        },
    }

    assert_bits!(A::A { age: 51 }, expect!["v1: 12 bits, v2: 11 bits"]);
    assert_bits!(A::B { big: false }, expect!["v1: 2 bits, v2: 2 bits"]);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum B {
        A { age: u64 },
        B { big: bool },
    }

    assert_bits!(B::A { age: 51 }, expect!["v1: 65 bits, v2: 11 bits"]);
    assert_bits!(B::B { big: false }, expect!["v1: 2 bits, v2: 2 bits"]);
}

#[test]
fn simplest_generics() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct A<T> {
        value: T,
    }

    assert_bits!(A { value: 51_usize }, expect!["v1: 12 bits, v2: 12 bits"]);
}

#[test]
fn low_cardinality() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Data {
        #[compactly(LowCardinality)]
        value: u64,
    }

    assert_bits!(Data { value: 51 }, expect!["v1: 65 bits, v2: 11 bits"]);
    assert_bits!(
        Data { value: u64::MAX },
        expect!["v1: 65 bits, v2: 66 bits"]
    );
    assert_bits!(
        (0..1024).map(|value| Data { value }).collect::<Vec<_>>(),
        expect!["v1: 8379 bits, v2: 8314 bits"]
    );
    // With three options, it takes less than two bits per value:
    assert_bits!(
        (0..1024)
            .map(|v| v % 3)
            .map(|value| Data { value })
            .collect::<Vec<_>>(),
        expect!["v1: 1903 bits, v2: 1799 bits"]
    );
}

#[test]
#[deny(deprecated)]
fn low_cardinality_string_allow_string() {
    // `allow_string` opts a `LowCardinality<String>` field out of the deprecation
    // warning that otherwise steers you toward `Arc<str>`. `#[deny(deprecated)]`
    // on this test makes the opt-out load-bearing: if a regression dropped the
    // flag, the derive-generated deprecation marker would become a hard error
    // right here. Deriving both `v1::Encode` and `v2::Encode` also proves the flag
    // is tolerated by the v1 derive (which never emits the warning) and that the
    // field still round-trips through both formats.
    #[derive(Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Data {
        #[compactly(LowCardinality, allow_string)]
        value: String,
    }

    assert_bits!(
        Data {
            value: String::from("hello")
        },
        expect!["v1: 37 bits, v2: 37 bits"]
    );
    assert_bits!(
        (0..1024)
            .map(|v| Data {
                value: format!("class-{}", v % 3)
            })
            .collect::<Vec<_>>(),
        expect!["v1: 1897 bits, v2: 1870 bits"]
    );
}

#[test]
fn unnamed_variants() {
    #[derive(compactly::v2::Encode, compactly::v1::Encode)]
    enum _SomeEnum {
        ValueHolder(String),
        OtherValue(u16),
        EvenMoreValues(String),
        FourthValue(_SubEnum),
    }

    #[derive(compactly::v2::Encode, compactly::v1::Encode)]
    enum _SubEnum {
        One,
        Two,
    }
}

#[test]
fn doc_comments_on_fields() {
    /// A struct with doc comments on the struct and its fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Documented {
        /// The value field.
        value: u8,
        /// A boolean flag.
        flag: bool,
    }

    for v in [
        Documented {
            value: 0,
            flag: false,
        },
        Documented {
            value: 42,
            flag: true,
        },
        Documented {
            value: 255,
            flag: false,
        },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

#[test]
fn const_generic_array_field() {
    // Const generic param used directly in a field type — requires the param
    // to be forwarded to DerivedContext (previously a compile error).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Buffer<const N: usize> {
        data: [u8; N],
    }

    for v in [
        Buffer::<4> { data: [0, 1, 2, 3] },
        Buffer::<4> {
            data: [255, 0, 128, 7],
        },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

#[test]
fn field_named_discriminant() {
    // A user field named `discriminant` used to collide with the hardcoded
    // `discriminant` field generated in DerivedContext. It should be renamed
    // automatically (to `discriminant_0`) to avoid the conflict.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct HasDiscriminant {
        discriminant: u8,
        value: bool,
    }

    for v in [
        HasDiscriminant {
            discriminant: 0,
            value: false,
        },
        HasDiscriminant {
            discriminant: 42,
            value: true,
        },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

#[test]
fn const_generic_independent() {
    // A struct with a const generic parameter that does NOT appear in any field type.
    // DerivedContext is generated without the const param, which is fine here
    // because none of the context field types reference it.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Tagged<const TAG: usize> {
        value: u32,
    }

    for v in [Tagged::<42> { value: 0 }, Tagged::<42> { value: 100 }] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

#[test]
fn multiple_type_params() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Pair<A, B> {
        first: A,
        second: B,
    }

    for v in [
        Pair {
            first: 3_u8,
            second: true,
        },
        Pair {
            first: 0_u8,
            second: false,
        },
        Pair {
            first: 255_u8,
            second: true,
        },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

#[test]
fn mixed_enum_variants() {
    // Enum mixing unit, tuple, and named-struct variants.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    enum Mixed {
        Unit,
        Tuple(u8, bool),
        Named { x: u8, y: u8 },
    }

    for v in [
        Mixed::Unit,
        Mixed::Tuple(0, false),
        Mixed::Tuple(42, true),
        Mixed::Named { x: 0, y: 0 },
        Mixed::Named { x: 1, y: 2 },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
        let bytes = compactly::v1::encode(&v);
        assert_eq!(
            compactly::v1::decode(&bytes),
            Some(v),
            "v1 roundtrip failed"
        );
    }
}

mod max_bytes {
    use compactly::v2::{AtMost, Encode};

    /// `Encode<S>::MAX_BYTES` is where a derived type's async-decode bound
    /// lives — the same trait as `encode`/`decode`, since there is no separate
    /// `DecodeAsync` twin.
    macro_rules! bound {
        ($t:ty) => {
            <$t as Encode>::MAX_BYTES
        };
        ($s:ty, $t:ty) => {
            <$t as Encode<$s>>::MAX_BYTES
        };
    }

    #[derive(compactly::v2::Encode)]
    struct Fields {
        flag: bool,
        byte: u8,
        wide: u64,
    }

    #[derive(compactly::v2::Encode)]
    enum Variants {
        Nothing,
        One(u8),
        Several { a: u64, b: u64, c: bool },
    }

    #[derive(compactly::v2::Encode)]
    struct WithStrategy {
        #[compactly(Small)]
        small: u64,
        plain: u64,
    }

    #[derive(compactly::v2::Encode)]
    struct HasUnbounded {
        bounded: u8,
        unbounded: String,
    }

    #[test]
    fn a_struct_sums_its_fields() {
        // A struct is one variant, so its `AtMost<0>` discriminant codes nothing.
        assert_eq!(bound!(AtMost<0>), 0);
        assert_eq!(bound!(Fields), bound!(bool) + bound!(u8) + bound!(u64));
    }

    #[test]
    fn an_enum_maxes_over_variants_atop_the_discriminant() {
        // Three variants, so an `AtMost<2>` discriminant, plus the fattest arm —
        // not the sum of all of them, since only one is ever coded.
        assert_eq!(
            bound!(Variants),
            bound!(AtMost<2>) + 2 * bound!(u64) + bound!(bool)
        );
    }

    #[test]
    fn a_field_uses_its_own_strategys_bound() {
        use compactly::Small;
        assert_eq!(bound!(WithStrategy), bound!(Small, u64) + bound!(u64));
    }

    #[test]
    fn one_unbounded_field_makes_the_whole_type_unbounded() {
        // Saturating, not wrapping: the safe direction, since an unbounded type
        // simply never takes the async decoder's sync fast path.
        assert_eq!(bound!(String), usize::MAX);
        assert_eq!(bound!(HasUnbounded), usize::MAX);
    }

    /// The bound has to hold against real decodes, not just arithmetic. Encoding
    /// one value alone yields its information plus the coder's final flush, so
    /// `MAX_BYTES` plus a settling allowance must cover it.
    #[test]
    fn encoded_values_fit_within_the_computed_bound() {
        const SETTLING: usize = 8;
        for wide in [0, 1, u64::MAX, 1 << 31] {
            for flag in [false, true] {
                let v = Fields {
                    flag,
                    byte: 200,
                    wide,
                };
                let n = compactly::v2::encode(&v).len();
                assert!(
                    n <= bound!(Fields) + SETTLING,
                    "Fields {{ {flag}, 200, {wide} }} encoded to {n} bytes, over \
                     its bound of {} (+{SETTLING} settling)",
                    bound!(Fields)
                );
            }
        }
    }
}

/// The derive emits the async-decode members (`MAX_BYTES`, `decode_awaiting`)
/// alongside `Encode`'s sync ones; this is the only place they can be
/// exercised, since the lib's own tests cannot use the derive (`extern crate
/// self as compactly` does not satisfy the generated `extern crate
/// compactly`).
#[cfg(feature = "stream")]
mod async_decode {
    use bytes::Bytes;
    use futures_executor::block_on;

    #[derive(compactly::v2::Encode, Debug, PartialEq)]
    struct Inner {
        flag: bool,
        wide: u64,
    }

    #[derive(compactly::v2::Encode, Debug, PartialEq)]
    enum Shape {
        Empty,
        Tuple(u8, String),
        Named {
            #[compactly(Small)]
            small: u64,
            inner: Inner,
            list: Vec<String>,
        },
    }

    #[derive(compactly::v2::Encode, Debug, PartialEq)]
    struct Generic<T> {
        first: T,
        rest: Vec<T>,
    }

    /// One `Bytes` per `chunk_size` bytes, so the decoder actually suspends.
    fn chunks(
        bytes: &[u8],
        chunk_size: usize,
    ) -> impl futures_core::Stream<Item = Result<Bytes, std::io::Error>> {
        struct Iter(std::vec::IntoIter<Bytes>);
        impl futures_core::Stream for Iter {
            type Item = Result<Bytes, std::io::Error>;
            fn poll_next(
                mut self: std::pin::Pin<&mut Self>,
                _: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Self::Item>> {
                std::task::Poll::Ready(self.0.next().map(Ok))
            }
        }
        Iter(
            bytes
                .chunks(chunk_size)
                .map(Bytes::copy_from_slice)
                .collect::<Vec<_>>()
                .into_iter(),
        )
    }

    #[track_caller]
    fn round_trips<T>(value: T)
    where
        T: compactly::v2::Encode + std::fmt::Debug + PartialEq,
    {
        let encoded = compactly::v2::encode(&value);
        assert_eq!(
            compactly::v2::decode::<T>(&encoded).as_ref(),
            Some(&value),
            "sync decode disagrees, so the fixture is wrong"
        );
        for chunk_size in [1, 2, 3, 7, 64, 4096] {
            let decoded: T =
                block_on(compactly::v2::decode_stream(chunks(&encoded, chunk_size))).unwrap();
            assert_eq!(decoded, value, "chunk_size = {chunk_size}");
        }
    }

    #[test]
    fn every_variant_shape_round_trips_from_a_stream() {
        round_trips(Shape::Empty);
        round_trips(Shape::Tuple(200, "hello 🦀".to_string()));
        round_trips(Shape::Named {
            small: u64::MAX,
            inner: Inner {
                flag: true,
                wide: 1 << 40,
            },
            list: vec!["a".to_string(), "x".repeat(300)],
        });
        round_trips(vec![
            Shape::Empty,
            Shape::Tuple(0, String::new()),
            Shape::Empty,
        ]);
    }

    #[test]
    fn a_generic_derived_type_round_trips_from_a_stream() {
        round_trips(Generic {
            first: 7_u64,
            rest: vec![0, 1, u64::MAX],
        });
        round_trips(Generic {
            first: "first".to_string(),
            rest: vec!["a".to_string(), "b".to_string()],
        });
    }
}

/// A hand-written `v2::Encode` impl must supply `MAX_BYTES`/`decode_awaiting`
/// too — `Encode<S>` is one trait with no opt-out, so a hand-written type used
/// as a field is exactly as stream-decodable as a derived one, with nothing
/// left to distinguish "sync-only" impls.
mod hand_written_encode_needs_the_async_members_too {
    use compactly::v2::{AsyncEntropyDecoder, Encode, EntropyCoder, EntropyDecoder};

    #[derive(Debug, PartialEq)]
    struct Manual(bool);

    impl Encode for Manual {
        type Context = <bool as Encode>::Context;
        fn encode<E: EntropyCoder>(value: &Self, encoder: &mut E, ctx: &mut Self::Context) {
            <bool as Encode>::encode(&value.0, encoder, ctx)
        }
        fn decode<D: EntropyDecoder>(
            decoder: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Self, std::io::Error> {
            Ok(Manual(<bool as Encode>::decode(decoder, ctx)?))
        }

        const MAX_BYTES: usize = <bool as Encode>::MAX_BYTES;

        async fn decode_awaiting<D: AsyncEntropyDecoder>(
            decoder: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Self, std::io::Error> {
            Ok(Manual(
                <bool as Encode>::decode_awaiting(decoder, ctx).await?,
            ))
        }
    }

    #[derive(Debug, PartialEq, compactly::v2::Encode)]
    struct Holder {
        field: Manual,
        count: u32,
    }

    #[test]
    fn round_trips() {
        let value = Holder {
            field: Manual(true),
            count: 42,
        };
        let bytes = compactly::v2::encode(&value);
        assert_eq!(compactly::v2::decode::<Holder>(&bytes), Some(value));
    }
}

/// Strategies now lift through the transparent wrappers `Option` and `Box`
/// automatically, for *any* strategy the inner type supports.
///
/// Before `Encode` took its strategy as a parameter, `Normal`'s blanket impl
/// covered every type and so overlapped any `impl<T, S> EncodingStrategy<W<T>>
/// for S`. Wrapper support had to be enumerated by hand — `src/v2/option.rs`
/// carried a macro listing `(type, strategy)` pairs — and every combination
/// nobody thought to add simply did not compile. These do now.
#[test]
fn strategies_lift_through_option_and_box() {
    #[derive(Debug, PartialEq, compactly::v2::Encode)]
    struct Wrapped {
        #[compactly(Small)]
        small_option: Option<u32>,
        #[compactly(Small)]
        small_box: Box<u64>,
        #[compactly(Small)]
        small_boxed_option: Option<Box<usize>>,
        #[compactly(Compressible)]
        compressible_option: Option<String>,
        #[compactly(LowCardinality)]
        low_cardinality_option: Option<u64>,
        #[compactly(Sorted)]
        sorted_option: Option<u8>,
    }

    for v in [
        Wrapped {
            small_option: None,
            small_box: Box::new(0),
            small_boxed_option: None,
            compressible_option: None,
            low_cardinality_option: None,
            sorted_option: None,
        },
        Wrapped {
            small_option: Some(7),
            small_box: Box::new(1_000_000),
            small_boxed_option: Some(Box::new(42)),
            compressible_option: Some("aaaaaaaaaaaaaaaaaaaa".to_string()),
            low_cardinality_option: Some(9),
            sorted_option: Some(3),
        },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(
            compactly::v2::decode(&bytes),
            Some(v),
            "v2 roundtrip failed"
        );
    }
}

/// A `Box<T>` costs nothing on the wire: it encodes exactly as the `T` inside,
/// under the default strategy and under a named one alike.
#[test]
fn box_is_transparent_on_the_wire() {
    assert_eq!(
        compactly::v2::encode(&Box::new(1234_u64)),
        compactly::v2::encode(&1234_u64),
    );
    assert_eq!(
        compactly::v2::encode_with(compactly::Small, &Box::new(1234_u64)),
        compactly::v2::encode_with(compactly::Small, &1234_u64),
    );
}
