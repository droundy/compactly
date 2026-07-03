#![cfg(all(feature = "v1", feature = "v2"))]
#![allow(clippy::enum_variant_names)]
#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $size:expr, $ans:expr,) => {
        assert_bits!($v, $size, $ans)
    };
    ($v:expr, $size:expr, $ans:expr) => {
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
        assert_eq!((bytes.len() + 4) / 8, $size, "unexpected number of bits");
        assert_eq!(
            (ans_bytes.len() + 4) / 8,
            $ans,
            "unexpected number of bits for ANS"
        );
    };
}

#[test]
fn singlet_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize);

    assert_bits!(Tuple(0), 3, 3);
    assert_bits!(Tuple(1), 3, 3);
    assert_bits!(Tuple(2), 3, 3);
}

#[test]
fn pair_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize, bool);

    assert_bits!(Tuple(0, false), 4, 4);
    assert_bits!(Tuple(1, true), 4, 4);
    assert_bits!(Tuple(2, false), 4, 4);
    assert_bits!(Tuple(2048, false), 18, 18);
}

#[test]
fn zero_size() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub struct Tuple;

    assert_bits!(Tuple, 0, 0);
    assert_bits!([Tuple; 4], 0, 0);
    assert_bits!([Tuple; 1024], 0, 0);
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
        16,
        16,
    );
    assert_bits!(
        Tuple {
            size: 1024,
            happy: true,
            age: 51
        },
        29,
        29,
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

    assert_bits!(A::A, 2, 2);
    assert_bits!(A::D, 1, 1);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum Bool {
        True,
        False,
    }

    assert_bits!(Bool::True, 1, 1);
    assert_bits!(Bool::False, 1, 1);
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

    assert_bits!(A::A, 3, 3);
    assert_bits!(A::D, 3, 3);
    assert_bits!(A::J, 1, 1);
}

#[test]
fn weird_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum A {
        A { age: usize },
        B { age: bool },
    }

    assert_bits!(A::A { age: 51 }, 13, 13);
    assert_bits!(A::B { age: false }, 2, 2);
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

    assert_bits!(A::A { age: 51 }, 12, 12);
    assert_bits!(A::B { big: false }, 2, 2);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    pub enum B {
        A { age: u64 },
        B { big: bool },
    }

    assert_bits!(B::A { age: 51 }, 65, 65);
    assert_bits!(B::B { big: false }, 2, 2);
}

#[test]
fn simplest_generics() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct A<T> {
        value: T,
    }

    assert_bits!(A { value: 51_usize }, 12, 12);
}

#[test]
fn low_cardinality() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::v2::Encode, compactly::v1::Encode)]
    struct Data {
        #[compactly(LowCardinality)]
        value: u64,
    }

    assert_bits!(Data { value: 51 }, 65, 65);
    assert_bits!(Data { value: u64::MAX }, 65, 66);
    assert_bits!(
        (0..1024).map(|value| Data { value }).collect::<Vec<_>>(),
        8379,
        9222,
    );
    // With three options, it takes less than two bits per value:
    assert_bits!(
        (0..1024)
            .map(|v| v % 3)
            .map(|value| Data { value })
            .collect::<Vec<_>>(),
        1903,
        1902,
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
        Documented { value: 0, flag: false },
        Documented { value: 42, flag: true },
        Documented { value: 255, flag: false },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
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
        Buffer::<4> { data: [255, 0, 128, 7] },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
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
        HasDiscriminant { discriminant: 0, value: false },
        HasDiscriminant { discriminant: 42, value: true },
    ] {
        let bytes = compactly::v2::encode(&v);
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
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
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
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
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
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
        assert_eq!(compactly::v2::decode(&bytes), Some(v), "v2 roundtrip failed");
        let bytes = compactly::v1::encode(&v);
        assert_eq!(compactly::v1::decode(&bytes), Some(v), "v1 roundtrip failed");
    }
}
