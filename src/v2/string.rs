mod init;

use super::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
use crate::{Compressible, Small, Sorted};

#[cfg(test)]
use expect_test::expect;

/// Below this codepoint a non-ASCII char needs one continuation byte; the
/// leading byte then holds the top bits `x >> 8`, which fit in its 6 payload
/// bits exactly when `x < 1 << 14`.
const ONE_CHUNK_CUTOFF: u32 = 1 << 14;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharContext {
    /// Leading byte, UTF-8 style (big-endian): its top bits tag the length
    /// class (`[0,128)` ASCII, `[128,192)` one continuation byte, `[192,256)`
    /// two) and its low 6 bits hold the *high* bits of the codepoint. The low
    /// bytes go in the continuation chunks, so each script's char identity
    /// (its low byte) lands in a single adaptive `u8` tree.
    first: <u8 as Encode>::Context,
    /// Low byte of a one-continuation char.
    one_chunk: <u8 as Encode>::Context,
    /// Middle byte (`x >> 8`) of a two-continuation char.
    two_chunk_a: <u8 as Encode>::Context,
    /// Low byte of a two-continuation char.
    two_chunk_b: <u8 as Encode>::Context,
}

impl Default for CharContext {
    #[inline]
    fn default() -> Self {
        init::INITIAL_CHAR_CONTEXT
    }
}

impl Encode for char {
    type Context = CharContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let x = u32::from(*self);
        if x < 128 {
            (x as u8).encode(writer, &mut ctx.first);
        } else if x < ONE_CHUNK_CUTOFF {
            // Byte: `10` tag + high bits `x >> 8` (< 64); then the low byte.
            (0x80 | (x >> 8) as u8).encode(writer, &mut ctx.first);
            (x as u8).encode(writer, &mut ctx.one_chunk);
        } else {
            // Byte: `11` tag + top bits `x >> 16` (<= 16); then two low bytes.
            (0xc0 | (x >> 16) as u8).encode(writer, &mut ctx.first);
            ((x >> 8) as u8).encode(writer, &mut ctx.two_chunk_a);
            (x as u8).encode(writer, &mut ctx.two_chunk_b);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let byte = u8::decode(reader, &mut ctx.first)?;
        if byte < 128 {
            return Ok(char::from(byte));
        }
        let x = if byte < 192 {
            let high = (byte & 0x3f) as u32;
            let low = u8::decode(reader, &mut ctx.one_chunk)? as u32;
            (high << 8) | low
        } else {
            let top = (byte & 0x3f) as u32;
            let a = u8::decode(reader, &mut ctx.two_chunk_a)? as u32;
            let b = u8::decode(reader, &mut ctx.two_chunk_b)? as u32;
            (top << 16) | (a << 8) | b
        };
        char::from_u32(x).ok_or_else(|| std::io::Error::other("invalid char value"))
    }
}

#[derive(Default, Clone)]
pub struct Context {
    len: <Small as EncodingStrategy<usize>>::Context,
    chars: <char as Encode>::Context,
}

impl Encode for String {
    type Context = Context;
    #[inline]
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        Small::encode(&self.chars().count(), writer, &mut ctx.len);
        for b in self.chars() {
            b.encode(writer, &mut ctx.chars);
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let len = Small::decode(reader, &mut ctx.len)?;
        let mut out = String::with_capacity(len);
        for _ in 0..len {
            out.push(char::decode(reader, &mut ctx.chars)?);
        }
        Ok(out)
    }
}

pub(super) fn encode_str<E: EntropyCoder>(s: &str, writer: &mut E, ctx: &mut Context) {
    Small::encode(&s.chars().count(), writer, &mut ctx.len);
    for c in s.chars() {
        c.encode(writer, &mut ctx.chars);
    }
}

impl Encode for Box<str> {
    type Context = Context;
    #[inline]
    fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        encode_str(self.as_ref(), writer, ctx);
    }
    #[inline]
    fn decode<D: EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        String::decode(reader, ctx).map(String::into_boxed_str)
    }
}

#[derive(Default, Clone)]
pub struct SortedContext {
    previous: String,
    shared_prefix: <Small as EncodingStrategy<usize>>::Context,
    len: <Small as EncodingStrategy<usize>>::Context,
    chars: <char as Encode>::Context,
}

impl EncodingStrategy<String> for Sorted {
    type Context = SortedContext;
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        let len: usize = Small::decode(reader, &mut ctx.len)?;
        let mut out = String::new();
        if ctx.previous.is_empty() {
            out.reserve_exact(len);
        } else {
            let shared_prefix = Small::decode(reader, &mut ctx.shared_prefix)?;
            out.reserve_exact(shared_prefix + len);
            out.extend(ctx.previous.chars().take(shared_prefix));
            debug_assert!(shared_prefix <= ctx.previous.len());
        }
        for _ in 0..len {
            out.push(char::decode(reader, &mut ctx.chars)?);
        }
        ctx.previous.clone_from(&out);
        Ok(out)
    }
    fn encode<E: super::EntropyCoder>(value: &String, writer: &mut E, ctx: &mut Self::Context) {
        if ctx.previous.is_empty() {
            let len = value.chars().count();
            Small::encode(&len, writer, &mut ctx.len);
            for c in value.chars() {
                c.encode(writer, &mut ctx.chars);
            }
        } else {
            let shared_prefix = value
                .chars()
                .zip(ctx.previous.chars())
                .take_while(|(a, b)| a == b)
                .count();
            let len = value.chars().count() - shared_prefix;
            Small::encode(&len, writer, &mut ctx.len);
            Small::encode(&shared_prefix, writer, &mut ctx.shared_prefix);
            for c in value.chars().skip(shared_prefix) {
                c.encode(writer, &mut ctx.chars);
            }
        }
        ctx.previous.clone_from(value);
    }
}

#[cfg(test)]
const COMPRESSIBLE_TEXT: &str = "Lossless compression is a class of data compression that allows the original data to be perfectly reconstructed from the compressed data with no loss of information. Lossless compression is possible because most real-world data exhibits statistical redundancy.[1] By contrast, lossy compression permits reconstruction only of an approximation of the original data, though usually with greatly improved compression rates (and therefore reduced media sizes).

By operation of the pigeonhole principle, no lossless compression algorithm can shrink the size of all possible data: Some data will get longer by at least one symbol or bit.

Compression algorithms are usually effective for human- and machine-readable documents and cannot shrink the size of random data that contain no redundancy. Different algorithms exist that are designed either with a specific type of input data in mind or with specific assumptions about what kinds of redundancy the uncompressed data are likely to contain.

Lossless data compression is used in many applications. For example, it is used in the ZIP file format and in the GNU tool gzip. It is also often used as a component within lossy data compression technologies (e.g. lossless mid/side joint stereo preprocessing by MP3 encoders and other lossy audio encoders).[2]

Lossless compression is used in cases where it is important that the original and the decompressed data be identical, or where deviations from the original data would be unfavourable. Common examples are executable programs, text documents, and source code. Some image file formats, like PNG or GIF, use only lossless compression, while others like TIFF and MNG may use either lossless or lossy methods. Lossless audio formats are most often used for archiving or production purposes, while smaller lossy audio files are typically used on portable players and in other cases where storage space is limited or exact replication of the audio is unnecessary. ";

impl EncodingStrategy<String> for Compressible {
    type Context = super::bytes::Lz77;
    fn encode<E: super::EntropyCoder>(value: &String, writer: &mut E, ctx: &mut Self::Context) {
        ctx.encode(value.as_bytes(), writer)
    }

    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        let bytes = ctx.decode(reader)?;
        String::from_utf8(bytes).map_err(std::io::Error::other)
    }
}

#[test]
fn size() {
    use super::{assert_millibits, encoded_bits};
    use crate::Encoded;

    assert_millibits!("".to_string(), expect!["3 bits"]);
    assert_millibits!("a".to_string(), expect!["Millibits(7593)"]);
    assert_millibits!("A".to_string(), expect!["Millibits(10093)"]);
    assert_millibits!("É".to_string(), expect!["Millibits(19992)"]);
    assert_millibits!("😊".to_string(), expect!["Millibits(27038)"]);
    assert_millibits!("hello world".to_string(), expect!["Millibits(60601)"]);
    assert_millibits!("Hello world".to_string(), expect!["Millibits(63115)"]);
    assert_millibits!("hhhhhhhhhhh".to_string(), expect!["Millibits(31709)"]);

    fn compare_small_bits(value: &str) -> String {
        println!("comparing {value:?}");
        format!(
            "normal: {} bits, small: {} bits",
            super::encoded_bits!(value.to_string()),
            super::encoded_bits!(Encoded::<_, Compressible>::new(value.to_string()))
        )
    }
    fn compare_vecs(value: &[&str]) -> String {
        let normal = value.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let encoded_normal = super::encode(&normal);
        let decoded_normal: Vec<String> = super::decode(&encoded_normal).unwrap();
        assert_eq!(normal, decoded_normal);

        let small: Vec<Encoded<String, Compressible>> = value
            .iter()
            .map(|s| s.to_string().into())
            .collect::<Vec<_>>();
        let encoded_small = super::encode(&small);
        let decoded_small: Vec<Encoded<String, Compressible>> =
            super::decode(&encoded_small).unwrap();
        assert_eq!(small, decoded_small);

        format!(
            "normal: {:?} ({} bits), small: {:?} ({} bits)",
            normal.millibits(),
            super::encoded_bits!(value.iter().map(|s| s.to_string()).collect::<Vec<String>>()),
            small.millibits(),
            super::encoded_bits!(value
                .iter()
                .map(|s| Encoded::<_, Compressible>::new(s.to_string()))
                .collect::<Vec<_>>())
        )
    }
    expect!["normal: 8930 bits, small: 7113 bits"]
        .assert_eq(&compare_small_bits(COMPRESSIBLE_TEXT));

    expect!["1000 mb"].assert_eq(&true.millibits().to_string());
    expect!["4593 mb"].assert_eq(&'a'.millibits().to_string());
    expect!["24038 mb"].assert_eq(&'😊'.millibits().to_string());
    expect!["normal: 3 bits, small: 3 bits"].assert_eq(&compare_small_bits(""));
    expect!["normal: 8 bits, small: 17 bits"].assert_eq(&compare_small_bits("a"));
    expect!["normal: 12 bits, small: 23 bits"].assert_eq(&compare_small_bits("aa"));
    expect!["normal: 15 bits, small: 26 bits"].assert_eq(&compare_small_bits("aaa"));
    expect!["normal: 18 bits, small: 30 bits"].assert_eq(&compare_small_bits("aaaa"));
    expect!["normal: 25 bits, small: 37 bits"].assert_eq(&compare_small_bits("aaaaaaaa"));
    expect!["normal: 147 bits, small: 140 bits"]
        .assert_eq(&compare_small_bits("aaaa1★😊aaaaaaaa1★😊😊aa"));
    expect!["normal: 28 bits, small: 42 bits"].assert_eq(&compare_small_bits("hello"));
    expect!["normal: 105 bits, small: 116 bits"]
        .assert_eq(&compare_small_bits("hello world hello wood"));
    expect!["normal: 110 bits, small: 98 bits"]
        .assert_eq(&compare_small_bits("hello world hello world"));
    expect!["normal: 375 bits, small: 418 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.",
    ));
    expect!["normal: 1497 bits, small: 832 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
    ));
    expect!["normal: 1566 bits, small: 1001 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
    ));

    expect!["normal: Millibits(3000) (3 bits), small: Millibits(3000) (3 bits)"]
        .assert_eq(&compare_vecs(&[]));
    expect!["5866 mb"].assert_eq(&'h'.millibits().to_string());
    expect!["8866 mb"].assert_eq(&"h".to_string().millibits().to_string());

    let s = "aaaaaaaaaaaaaaaa".to_string();
    expect!["33974 mb"].assert_eq(&s.millibits().to_string());
    expect!["34"].assert_eq(&encoded_bits!(s.clone()));

    let s = "hello world this is a string".to_string();
    expect!["140933 mb"].assert_eq(&s.millibits().to_string());
    expect!["141"].assert_eq(&encoded_bits!(s.clone()));

    expect!["normal: Millibits(11866) (12 bits), small: Millibits(20000) (20 bits)"]
        .assert_eq(&compare_vecs(&["h"]));
    expect!["normal: Millibits(63601) (64 bits), small: Millibits(82841) (83 bits)"]
        .assert_eq(&compare_vecs(&["hello world"]));
    expect!["normal: Millibits(112312) (112 bits), small: Millibits(101770) (102 bits)"]
        .assert_eq(&compare_vecs(&["hello world", "hello world"]));
    expect!["normal: Millibits(155591) (156 bits), small: Millibits(112584) (113 bits)"].assert_eq(
        &compare_vecs(&["hello world", "hello world", "hello world"]),
    );
    expect!["normal: Millibits(244924) (245 bits), small: Millibits(145803) (146 bits)"].assert_eq(
        &compare_vecs(&[
            "hello world",
            "hello world",
            "hello world",
            "hello world hello world",
        ]),
    );
    expect!["normal: Millibits(199645) (200 bits), small: Millibits(198370) (198 bits)"]
        .assert_eq(&compare_vecs(&["hello world! 😊", "goodbye world! 😊"]));
    expect!["normal: Millibits(416130) (416 bits), small: Millibits(350885) (351 bits)"].assert_eq(
        &compare_vecs(&[
            "hello world! 😊",
            "greetings world! 😊",
            "goodbye world! 😊",
            "farewell sweet world! 😊",
        ]),
    );
    expect!["normal: Millibits(474216) (474 bits), small: Millibits(413459) (414 bits)"].assert_eq(
        &compare_vecs(&[
            "The quick brown fox jumps over the lazy dog.",
            "The",
            "quick",
            "brown",
            "fox",
            "jumps",
            "over",
            "the",
            "lazy",
            "dog",
        ]),
    );
}

#[test]
fn sorted() {
    use crate::{Encoded, Values};
    use std::collections::BTreeSet;

    let strings: Vec<String> = [
        "alpha",
        "all",
        "amortization",
        "amortize",
        "elegy",
        "elephant",
    ]
    .into_iter()
    .map(String::from)
    .collect::<BTreeSet<String>>()
    .into_iter()
    .collect::<Vec<_>>();
    let encoded_strings: Encoded<Vec<String>, Values<Sorted>> =
        crate::Encoded::new(strings.clone());
    expect!["223"].assert_eq(&estimated_bits!(strings.clone()));
    expect!["186"].assert_eq(&estimated_bits!(encoded_strings.clone()));

    let strings: Vec<String> = COMPRESSIBLE_TEXT
        .split(' ')
        .map(|s| s.to_string())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>();
    let encoded_strings: Encoded<Vec<String>, Values<Sorted>> =
        crate::Encoded::new(strings.clone());
    use super::estimated_bits;

    expect!["5933"].assert_eq(&estimated_bits!(strings.clone()));
    expect!["4937"].assert_eq(&estimated_bits!(encoded_strings.clone()));
}

#[test]
fn crash_from_bench() {
    use super::encoded_bits;
    use crate::{Encoded, Values};
    let names = ["Al", "Aïr"];
    let vec = names.iter().map(|n| n.to_string()).collect::<Vec<String>>();
    expect!["53"].assert_eq(&encoded_bits!(vec.clone()));
    expect!["53"].assert_eq(&encoded_bits!(super::Ans, vec.clone()));
    let compressible = Encoded::<Vec<String>, Values<Compressible>>::new(vec.clone());
    expect!["69"].assert_eq(&encoded_bits!(compressible.clone()));
    expect!["69"].assert_eq(&encoded_bits!(super::Ans, compressible.clone()));
    let sorted = Encoded::<Vec<String>, Values<Sorted>>::new(vec.clone());
    expect!["49"].assert_eq(&encoded_bits!(sorted.clone()));
    expect!["50"].assert_eq(&encoded_bits!(super::Ans, sorted.clone()));
}
