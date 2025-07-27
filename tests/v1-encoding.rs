use std::{collections::BTreeSet, fmt::Debug};

fn filename<T: compactly::v1::Encode + serde::Serialize>(value: &T) -> String {
    let value_hash = rapidhash::rapidhash(&bincode1::serialize(value).unwrap());
    let typename = std::any::type_name::<T>();
    let function_name = if let Some(typename) = typename.strip_prefix("v1_encoding::") {
        typename.split_once("::").unwrap().0.to_string()
    } else {
        typename
            .replace(']', "")
            .replace('[', "")
            .replace("; ", "-count-")
    };
    format!("tests/v1-encoding/{function_name}-{value_hash:016x}")
}

// Set this to `true` while creating new tests.
const AM_CREATING_NEW_TESTS: bool = false;

fn validate_encoding<T: Debug + compactly::v1::Encode + serde::Serialize>(value: &T) {
    let p = filename(value);
    if std::env::var("AM_CREATING_NEW_TESTS").ok() == Some("true".to_string())
        && !std::fs::exists(&p).expect("Unable to look for files?")
    {
        println!("Am creating a new encoded file at {p}");
        std::fs::write(filename(value), compactly::encode(value)).unwrap();
    }
    let bytes = std::fs::read(&p).expect(&format!(
        "file {p} does not exist for {value:?}
If you are creating a new test for which there is no comparison on disk, run this with

AM_CREATING_NEW_TESTS=true cargo test --test v1-encoding

"
    ));
    let encoded = compactly::v1::encode(value);
    assert_eq!(encoded, bytes, "encoded changed for {value:?}");
}

fn validate_eq<T: Debug + PartialEq + compactly::v1::Encode + serde::Serialize>(value: &T) {
    validate_encoding(value);
    let encoded = compactly::v1::encode(value);
    let decoded: T = compactly::v1::decode(&encoded).unwrap();
    assert_eq!(&decoded, value);
}

#[test]
fn remembered_to_stop_creating_new_tests() {
    assert!(!AM_CREATING_NEW_TESTS);
}

#[test]
fn unsigned() {
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, serde::Deserialize)]
    struct Unsigned {
        u64: u64,
        u32: u32,
        u16: u16,
        u8: u8,
        usize: usize,
    }
    impl From<u64> for Unsigned {
        fn from(value: u64) -> Self {
            Unsigned {
                u64: value,
                u32: value as u32,
                u16: value as u16,
                u8: value as u8,
                usize: value as usize,
            }
        }
    }
    for value in [
        0_u64,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        u64::MAX,
        u64::MAX - 1,
        u32::MAX as u64,
        u32::MAX as u64 - 1,
        u16::MAX as u64,
        u16::MAX as u64 - 1,
        u8::MAX as u64,
        u8::MAX as u64 - 1,
    ]
    .into_iter()
    .map(Unsigned::from)
    {
        validate_encoding(&value);
    }
}

#[test]
fn signed() {
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, serde::Deserialize)]
    struct Signed {
        i64: i64,
        i32: i32,
        i16: i16,
        i8: i8,
    }
    impl From<i64> for Signed {
        fn from(value: i64) -> Self {
            Self {
                i64: value,
                i32: value as i32,
                i16: value as i16,
                i8: value as i8,
            }
        }
    }
    for value in [
        0_i64,
        -1,
        -2,
        -3,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        i64::MAX,
        i64::MAX - 1,
        i64::MIN,
        i64::MIN + 1,
        i32::MAX as i64,
        i32::MAX as i64 - 1,
        i16::MAX as i64,
        i16::MAX as i64 - 1,
        i8::MAX as i64,
        i8::MAX as i64 - 1,
        i32::MIN as i64,
        i32::MIN as i64 + 1,
        i16::MIN as i64,
        i16::MIN as i64 + 1,
        i8::MIN as i64,
        i8::MIN as i64 + 1,
    ]
    .into_iter()
    .map(Signed::from)
    {
        validate_encoding(&value);
    }
}

#[test]
fn string_collections() {
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, serde::Deserialize)]
    struct Strings {
        vec: Vec<String>,
        #[compactly(Values<Sorted>)]
        sorted_vec: Vec<String>,
        #[compactly(Values<Compressible>)]
        compressible_vec: Vec<String>,
        #[compactly(Values<LowCardinality>)]
        lc_vec: Vec<String>,
        btree: BTreeSet<String>,
        #[compactly(Values<Normal>)]
        normal_btree: BTreeSet<String>,
        #[compactly(Values<LowCardinality>)]
        lc_btree: BTreeSet<String>,
        // hash: HashSet<String>, hashset doesn't work since our filename switches around.  :(
    }
    impl From<&[&str]> for Strings {
        fn from(value: &[&str]) -> Self {
            let vec: Vec<String> = value.iter().map(|s| String::from(*s)).collect();
            let btree: BTreeSet<String> = vec.clone().into_iter().collect();
            Self {
                normal_btree: btree.clone(),
                lc_btree: btree.clone(),
                btree,
                // hash: vec.clone().into_iter().collect(),
                compressible_vec: vec.clone(),
                sorted_vec: vec.clone(),
                lc_vec: vec.clone(),
                vec,
            }
        }
    }
    for value in [
        &[][..],
        &["hello"],
        &["hell", "world"],
        &["hellish", "hello", "world", "", ""],
        &["hellish", "", "hello", "world", "hellish", "", ""],
    ]
    .into_iter()
    .map(Strings::from)
    {
        validate_encoding(&value);
    }
}

#[test]
fn bytes_collections() {
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, serde::Deserialize)]
    struct ByteCollections {
        vec: Vec<Vec<u8>>,
        #[compactly(Values<Sorted>)]
        sorted_vec: Vec<Vec<u8>>,
        #[compactly(Values<Compressible>)]
        compressible_vec: Vec<Vec<u8>>,
        #[compactly(Values<LowCardinality>)]
        lc_vec: Vec<Vec<u8>>,
        btree: BTreeSet<Vec<u8>>,
        #[compactly(Values<Normal>)]
        normal_btree: BTreeSet<Vec<u8>>,
        #[compactly(Values<LowCardinality>)]
        lc_btree: BTreeSet<Vec<u8>>,
        // hash: HashSet<Vec<u8>>, hashset doesn't work since our filename switches around.  :(
    }
    impl From<&[&[u8]]> for ByteCollections {
        fn from(value: &[&[u8]]) -> Self {
            let vec: Vec<Vec<u8>> = value.iter().map(|s| (*s).to_vec()).collect();
            let btree: BTreeSet<Vec<u8>> = vec.clone().into_iter().collect();
            Self {
                normal_btree: btree.clone(),
                lc_btree: btree.clone(),
                btree,
                // hash: vec.clone().into_iter().collect(),
                compressible_vec: vec.clone(),
                sorted_vec: vec.clone(),
                lc_vec: vec.clone(),
                vec,
            }
        }
    }
    let all_possible_bytes = (0u8..255).collect::<Vec<_>>();
    for value in [
        &[][..],
        &[all_possible_bytes.as_slice()].as_slice(),
        &[all_possible_bytes.as_slice(), b"hello"],
        &[b"hello world", all_possible_bytes.as_slice()],
        &[b"hello".as_slice()],
        &[b"hell", b"world"],
        &[b"hellish", b"hello", b"world", b"", b""],
        &[b"hellish", b"", b"hello", b"world", b"hellish", b"", b""],
    ]
    .into_iter()
    .map(ByteCollections::from)
    {
        validate_encoding(&value);
    }
}

#[test]
fn small_bytes() {
    #[derive(Debug, PartialEq, compactly::v1::Encode, serde::Serialize, serde::Deserialize)]
    struct Bytes {
        vec: Vec<u8>,
        #[compactly(Values<Small>)]
        small_vec: Vec<u8>,
    }
    impl From<&[u8]> for Bytes {
        fn from(value: &[u8]) -> Self {
            let vec: Vec<u8> = value.to_vec();
            Self {
                small_vec: vec.clone(),
                vec,
            }
        }
    }
    let all_possible_bytes = (0u8..255).collect::<Vec<_>>();
    validate_eq(&Bytes::from(all_possible_bytes.as_slice()));
}

#[test]
fn arc_bytes() {
    use std::sync::Arc;
    #[derive(
        Debug, PartialEq, Eq, Hash, compactly::v1::Encode, serde::Serialize, serde::Deserialize,
    )]
    struct Bytes {
        vec: Vec<Arc<u8>>,
    }
    impl From<&[u8]> for Bytes {
        fn from(value: &[u8]) -> Self {
            let vec = value.iter().map(|&b| Arc::new(b)).collect();
            Self { vec }
        }
    }
    let all_possible_bytes = (0u8..255).chain((0u8..255).rev()).collect::<Vec<_>>();
    validate_eq(&Bytes::from(all_possible_bytes.as_slice()));
}

#[test]
fn arrays() {
    validate_eq(&[0u16, 1, u16::MAX]);
}
