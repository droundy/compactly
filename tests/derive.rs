#[cfg(test)]
macro_rules! assert_bits {
    ($v:expr, $size:expr) => {
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
        let bytes = compactly::encode(&v);
        let decoded = compactly::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        assert_eq!((bytes.len() + 4) / 8, $size, "unexpected number of bits");
    };
}
#[cfg(test)]
pub(crate) use assert_bits;

#[test]
fn singlet_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple(usize);

    assert_bits!(Tuple(0), 1);
    assert_bits!(Tuple(1), 2);
    assert_bits!(Tuple(2), 4);
}

#[test]
fn pair_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple(usize, bool);

    assert_bits!(Tuple(0, false), 2);
    assert_bits!(Tuple(1, true), 3);
    assert_bits!(Tuple(2, false), 5);
    assert_bits!(Tuple(2048, false), 25);
}

#[test]
fn zero_size() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple;

    assert_bits!(Tuple, 0);
    assert_bits!([Tuple; 4], 0);
    assert_bits!([Tuple; 1024], 0);
}

#[test]
fn record() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
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
        14
    );
    assert_bits!(
        Tuple {
            size: 1024,
            happy: true,
            age: 51
        },
        35
    );
}

#[test]
fn simple_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    pub enum A {
        A,
        B,
        C,
        D,
    }

    assert_bits!(A::A, 2);
    assert_bits!(A::D, 2);
}

#[test]
fn bigger_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
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

    assert_bits!(A::A, 4);
    assert_bits!(A::D, 3);
    assert_bits!(compactly::URange::<10>::new(9), 4);
    assert_bits!(A::J, 4);
}

#[test]
fn weird_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    pub enum A {
        A { age: usize },
        B { age: bool },
    }

    assert_bits!(A::A { age: 51 }, 13);
    assert_bits!(A::B { age: false }, 2);
}

#[test]
fn fancy_enum() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    pub enum A {
        A { age: usize },
        B { big: bool },
    }

    assert_bits!(A::A { age: 51 }, 13);
    assert_bits!(A::B { big: false }, 2);
}

#[test]
fn simplest_generics() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    struct A<T> {
        value: T,
    }

    assert_bits!(A { value: 51_usize }, 12);
}
