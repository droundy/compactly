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
    pub struct SingletTuple(usize);

    assert_size!(SingletTuple(0), 1);
    assert_size!(SingletTuple(1), 1);
    assert_size!(SingletTuple(2), 1);
}
