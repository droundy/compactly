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
        let ans_bytes = compactly::ans::encode(&v);
        let ans_decoded = compactly::ans::decode(&ans_bytes);
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

#[cfg(test)]
pub(crate) use assert_bits;

#[test]
fn singlet_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize);

    assert_bits!(Tuple(0), 3, 3);
    assert_bits!(Tuple(1), 3, 3);
    assert_bits!(Tuple(2), 3, 3);
}

#[test]
fn pair_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub struct Tuple(usize, bool);

    assert_bits!(Tuple(0, false), 4, 4);
    assert_bits!(Tuple(1, true), 4, 4);
    assert_bits!(Tuple(2, false), 4, 4);
    assert_bits!(Tuple(2048, false), 18, 18);
}

#[test]
fn zero_size() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub struct Tuple;

    assert_bits!(Tuple, 0, 0);
    assert_bits!([Tuple; 4], 0, 0);
    assert_bits!([Tuple; 1024], 0, 0);
}

#[test]
fn record() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub enum A {
        A,
        B,
        C,
        D,
    }

    assert_bits!(A::A, 2, 2);
    assert_bits!(A::D, 1, 1);

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub enum Bool {
        True,
        False,
    }

    assert_bits!(Bool::True, 1, 1);
    assert_bits!(Bool::False, 1, 1);
}

#[test]
fn bigger_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub enum A {
        A { age: usize },
        B { age: bool },
    }

    assert_bits!(A::A { age: 51 }, 13, 13);
    assert_bits!(A::B { age: false }, 2, 2);
}

#[test]
fn fancy_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    pub enum B {
        A { age: u64 },
        B { big: bool },
    }

    assert_bits!(B::A { age: 51 }, 65, 65);
    assert_bits!(B::B { big: false }, 2, 2);
}

#[test]
fn simplest_generics() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    struct A<T> {
        value: T,
    }

    assert_bits!(A { value: 51_usize }, 12, 12);
}

#[test]
fn low_cardinality() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::ans::Encode, compactly::v1::Encode)]
    struct Data {
        #[compactly(LowCardinality)]
        value: u64,
    }

    assert_bits!(Data { value: 51 }, 65, 65);
    assert_bits!(Data { value: u64::MAX }, 65, 65);
    assert_bits!(
        (0..1024).map(|value| Data { value }).collect::<Vec<_>>(),
        8379,
        8379,
    );
    // With three options, it takes less than two bits per value:
    assert_bits!(
        (0..1024)
            .map(|v| v % 3)
            .map(|value| Data { value })
            .collect::<Vec<_>>(),
        1903,
        1903,
    );
}
