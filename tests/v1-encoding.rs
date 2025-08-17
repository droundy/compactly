use std::{collections::BTreeSet, fmt::Debug};

use compactly::Small;

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

fn validate_strategy<
    S: compactly::v1::EncodingStrategy<T>,
    T: Debug + PartialEq + compactly::v1::Encode + serde::Serialize + Clone,
>(
    strategy: S,
    value: &T,
) {
    validate_encoding(value);
    let encoded = compactly::v1::encode_with(strategy, value);
    let decoded: T = compactly::v1::decode_with(strategy, &encoded).unwrap();
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
    let textgen = CompressibleText { x: 13 };
    let big_text = textgen.take(512).collect::<Vec<_>>();
    let ref_text = big_text.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    for value in [
        &[][..],
        &["hello"],
        &["hell", "world"],
        &["hellish", "hello", "world", "", ""],
        &["hellish", "", "hello", "world", "hellish", "", ""],
        ref_text.as_slice(),
    ]
    .into_iter()
    .map(Strings::from)
    {
        validate_encoding(&value);
    }
    validate_encoding(&String::from_utf8(vec![b'a'; 1024 * 1024]).unwrap());
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
    validate_eq(&[0u16, 1, 2, 3, 0, u16::MAX]);
    validate_eq(&BTreeSet::from([0u16, 1, 2, 3, 0, u16::MAX]));
}

#[test]
fn bools() {
    validate_eq(&[true, true, true, true, false, false]);
    validate_eq(&[false, true]);
    validate_eq(&BTreeSet::from([false, true]));
    validate_eq(&BTreeSet::from([false]));
}

#[test]
fn byte() {
    validate_eq(&i8::MIN);
    validate_eq(&i8::MAX);
    validate_eq(&-1_i8);
    validate_eq(&0_i8);
    validate_eq(&3_i8);
    validate_strategy(Small, &3i16);
}

#[test]
fn floats() {
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, Clone, Copy)]
    struct F32 {
        v: f32,
        #[compactly(Decimal)]
        decimal: f32,
    }
    impl F32 {
        fn new(value: f32) -> Self {
            F32 {
                v: value,
                decimal: value,
            }
        }
    }
    impl PartialEq for F32 {
        fn eq(&self, other: &Self) -> bool {
            self.v.to_be_bytes() == other.v.to_be_bytes()
                && self.decimal.to_be_bytes() == other.decimal.to_be_bytes()
        }
    }
    #[derive(Debug, compactly::v1::Encode, serde::Serialize, Clone, Copy)]
    struct F64 {
        v: f64,
        #[compactly(Decimal)]
        decimal: f64,
    }
    impl F64 {
        fn new(value: f64) -> Self {
            F64 {
                v: value,
                decimal: value,
            }
        }
    }
    impl PartialEq for F64 {
        fn eq(&self, other: &Self) -> bool {
            self.v.to_be_bytes() == other.v.to_be_bytes()
                && self.decimal.to_be_bytes() == other.decimal.to_be_bytes()
        }
    }
    let f32s = [
        0.0_f32,
        1.0,
        2.0,
        std::f32::consts::PI,
        0.1,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        std::f32::NAN,
        std::f32::INFINITY,
        std::f32::NEG_INFINITY,
    ]
    .into_iter()
    .map(F32::new)
    .collect::<Vec<_>>();
    let f64s = [
        0.0_f64,
        1.0,
        2.0,
        std::f64::consts::PI,
        0.1,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        std::f64::NAN,
        std::f64::INFINITY,
        std::f64::NEG_INFINITY,
    ]
    .into_iter()
    .map(F64::new)
    .collect::<Vec<_>>();
    for &value in &f32s {
        validate_eq(&value);
    }
    validate_eq(&f32s);
    for &value in &f64s {
        validate_eq(&value);
    }
    validate_eq(&f64s);
}

struct CompressibleText {
    x: u64,
}

impl CompressibleText {
    fn rng(&mut self) -> usize {
        self.x = self.x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31)) as usize
    }
    fn string(&mut self) -> String {
        let mut out = String::new();
        while out.len() < 1024 && self.rng() % 13 != 0 {
            const WORDS: &[&str] = &[
                "hello", "hell", "world", "the", "is", "this", "of", "goodbye", "farewell",
            ];
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(WORDS[self.rng() % WORDS.len()]);
        }
        out
    }
}
impl Iterator for CompressibleText {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.string())
    }
}
