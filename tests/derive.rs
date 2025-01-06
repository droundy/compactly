#[cfg(test)]
macro_rules! assert_size {
    ($v:expr, $size:expr) => {
        let v = $v;
        let bytes = compactly::encode(&v);
        println!("bytes are {bytes:?}");
        let decoded = compactly::decode(&bytes);
        assert_eq!(decoded, Some(v), "decoded value is incorrect");
        assert_eq!(bytes.len(), $size, "unexpected size");
    };
}
#[cfg(test)]
pub(crate) use assert_size;

#[test]
fn singlet_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple(usize);

    assert_size!(Tuple(0), 1);
    assert_size!(Tuple(1), 1);
    assert_size!(Tuple(2), 1);
}

#[test]
fn pair_tuple() {
    #[derive(Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple(usize, bool);

    assert_size!(Tuple(0, false), 1);
    assert_size!(Tuple(1, true), 1);
    assert_size!(Tuple(2, false), 1);
    assert_size!(Tuple(2048, false), 4);
}

#[test]
fn zero_size() {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, compactly::Encode)]
    pub struct Tuple;

    assert_size!(Tuple, 1);
    assert_size!([Tuple; 4], 2);
    assert_size!([Tuple; 1024], 3);
}
