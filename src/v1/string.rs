use super::{bits::Bits, Encode, EncodingStrategy, ULessThan};
use crate::{Compressible, Small, Sorted};

#[cfg(test)]
use expect_test::expect;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CharContext {
    is_ascii: <bool as Encode>::Context,
    ascii: <Bits<128> as Encode>::Context,
    n_chunks: <ULessThan<3> as Encode>::Context,
    chunk1: <Bits<32> as Encode>::Context,
    chunks: [<Bits<64> as Encode>::Context; 3],
}

impl Encode for char {
    type Context = CharContext;
    #[inline]
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let mut x = u32::from(*self);
        let is_ascii = x < 128;
        is_ascii.encode(writer, &mut ctx.is_ascii)?;
        if is_ascii {
            Bits::<128>::take_from(&mut x).encode(writer, &mut ctx.ascii)
        } else {
            let n_chunks = if x < 32 * 64 {
                0
            } else if x < 32 * 64 * 64 {
                1
            } else {
                2
            };
            let n_chunks = ULessThan::<3>::try_from(n_chunks).unwrap();
            n_chunks.encode(writer, &mut ctx.n_chunks)?;
            Bits::<32>::take_from(&mut x).encode(writer, &mut ctx.chunk1)?;
            for i in 0_usize..1 + usize::from(n_chunks) {
                Bits::<64>::take_from(&mut x).encode(writer, &mut ctx.chunks[i])?;
            }
            Ok(())
        }
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let mut x = u32::from(*self);
        let is_ascii = x < 128;
        let mut tot = is_ascii.millibits(&mut ctx.is_ascii)?;
        if is_ascii {
            Some(tot + Bits::<128>::take_from(&mut x).millibits(&mut ctx.ascii)?)
        } else {
            let n_chunks = if x < 32 * 64 {
                0
            } else if x < 32 * 64 * 64 {
                1
            } else {
                2
            };
            let n_chunks = ULessThan::<3>::try_from(n_chunks).unwrap();
            tot += n_chunks.millibits(&mut ctx.n_chunks)?;
            tot += Bits::<32>::take_from(&mut x).millibits(&mut ctx.chunk1)?;
            for i in 0_usize..1 + usize::from(n_chunks) {
                tot += Bits::<64>::take_from(&mut x).millibits(&mut ctx.chunks[i])?;
            }
            Some(tot)
        }
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        if bool::decode(reader, &mut ctx.is_ascii)? {
            let v: u8 = Bits::<128>::decode(reader, &mut ctx.ascii)?.into();
            Ok(char::from(v))
        } else {
            let n_chunks = ULessThan::<3>::decode(reader, &mut ctx.n_chunks)?;
            let mut out: u32 = u8::from(Bits::<32>::decode(reader, &mut ctx.chunk1)?) as u32;
            for i in 0_usize..1 + usize::from(n_chunks) {
                let chunk = u8::from(Bits::<64>::decode(reader, &mut ctx.chunks[i])?) as u32;
                out |= chunk << (5 + 6 * i);
            }
            char::from_u32(out).ok_or_else(|| std::io::Error::other("invalid char value"))
        }
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
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        Small::encode(&self.chars().count(), writer, &mut ctx.len)?;
        for b in self.chars() {
            b.encode(writer, &mut ctx.chars)?;
        }
        Ok(())
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = Small::millibits(&self.chars().count(), &mut ctx.len)?;
        for b in self.chars() {
            tot += b.millibits(&mut ctx.chars)?;
        }
        // println!("{self:?}: {tot}");
        Some(tot)
    }
    #[inline]
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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

#[derive(Default, Clone)]
pub struct SortedContext {
    previous: String,
    shared_prefix: <Small as EncodingStrategy<usize>>::Context,
    len: <Small as EncodingStrategy<usize>>::Context,
    chars: <char as Encode>::Context,
}

impl EncodingStrategy<String> for Sorted {
    type Context = SortedContext;
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
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
        ctx.previous = out.clone();
        Ok(out)
    }
    fn encode<W: std::io::Write>(
        value: &String,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        if ctx.previous.is_empty() {
            let len = value.chars().count();
            Small::encode(&len, writer, &mut ctx.len)?;
            for c in value.chars() {
                c.encode(writer, &mut ctx.chars)?;
            }
        } else {
            let shared_prefix = value
                .chars()
                .zip(ctx.previous.chars())
                .take_while(|(a, b)| a == b)
                .count();
            let len = value.chars().count() - shared_prefix;
            Small::encode(&len, writer, &mut ctx.len)?;
            Small::encode(&shared_prefix, writer, &mut ctx.shared_prefix)?;
            for c in value.chars().skip(shared_prefix) {
                c.encode(writer, &mut ctx.chars)?;
            }
        }
        ctx.previous = value.clone();
        Ok(())
    }
    fn millibits(value: &String, ctx: &mut Self::Context) -> Option<usize> {
        let mut tot = 0;
        if ctx.previous.is_empty() {
            let len = value.len();
            tot += Small::millibits(&len, &mut ctx.len)?;
            for c in value.chars() {
                tot += c.millibits(&mut ctx.chars)?;
            }
        } else {
            let shared_prefix = value
                .chars()
                .zip(ctx.previous.chars())
                .take_while(|(a, b)| a == b)
                .count();
            let len = value.len() - shared_prefix;
            tot += Small::millibits(&len, &mut ctx.len)?;
            tot += Small::millibits(&shared_prefix, &mut ctx.shared_prefix)?;
            for c in value.chars().skip(shared_prefix) {
                tot += c.millibits(&mut ctx.chars)?;
            }
        }
        ctx.previous = value.clone();
        Some(tot)
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
    fn encode<W: std::io::Write>(
        value: &String,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        ctx.encode(value.as_bytes(), writer)
    }
    fn millibits(value: &String, ctx: &mut Self::Context) -> Option<usize> {
        ctx.millibits(value.as_bytes())
    }

    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        let bytes = ctx.decode(reader)?;
        String::from_utf8(bytes).map_err(std::io::Error::other)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    use crate::Encoded;

    assert_bits!("".to_string(), expect!["3"]);
    assert_bits!("a".to_string(), expect!["11"]);
    assert_bits!("A".to_string(), expect!["11"]);
    assert_bits!("É".to_string(), expect!["16"]);
    assert_bits!("😊".to_string(), expect!["23"]);
    assert_bits!("hello world".to_string(), expect!["74"]);
    assert_bits!("Hello world".to_string(), expect!["76"]);
    assert_bits!("hhhhhhhhhhh".to_string(), expect!["35"]);

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
            "normal: {:?} millibits ({} bits), small: {:?} millibits ({} bits)",
            normal.millibits(&mut Default::default()),
            super::encoded_bits!(value.iter().map(|s| s.to_string()).collect::<Vec<String>>()),
            small.millibits(&mut Default::default()),
            super::encoded_bits!(value
                .iter()
                .map(|s| Encoded::<_, Compressible>::new(s.to_string()))
                .collect::<Vec<_>>())
        )
    }
    expect!["normal: 8979 bits, small: 7116 bits"]
        .assert_eq(&compare_small_bits(COMPRESSIBLE_TEXT));

    assert_eq!(true.millibits(&mut Default::default()), Some(1000));
    assert_eq!('a'.millibits(&mut Default::default()), Some(8000));
    assert_eq!('😊'.millibits(&mut Default::default()), Some(20000));
    expect!["normal: 3 bits, small: 3 bits"].assert_eq(&compare_small_bits(""));
    expect!["normal: 11 bits, small: 17 bits"].assert_eq(&compare_small_bits("a"));
    expect!["normal: 17 bits, small: 23 bits"].assert_eq(&compare_small_bits("aa"));
    expect!["normal: 20 bits, small: 26 bits"].assert_eq(&compare_small_bits("aaa"));
    expect!["normal: 24 bits, small: 30 bits"].assert_eq(&compare_small_bits("aaaa"));
    expect!["normal: 31 bits, small: 39 bits"].assert_eq(&compare_small_bits("aaaaaaaa"));
    expect!["normal: 133 bits, small: 140 bits"]
        .assert_eq(&compare_small_bits("aaaa1★😊aaaaaaaa1★😊😊aa"));
    expect!["normal: 36 bits, small: 42 bits"].assert_eq(&compare_small_bits("hello"));
    expect!["normal: 122 bits, small: 116 bits"]
        .assert_eq(&compare_small_bits("hello world hello wood"));
    expect!["normal: 127 bits, small: 98 bits"]
        .assert_eq(&compare_small_bits("hello world hello world"));
    expect!["normal: 415 bits, small: 421 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.",
    ));
    expect!["normal: 1537 bits, small: 839 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
    ));
    expect!["normal: 1607 bits, small: 1011 bits"].assert_eq(&compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
    ));

    expect!["normal: Some(3000) millibits (3 bits), small: Some(3000) millibits (3 bits)"]
        .assert_eq(&compare_vecs(&[]));
    assert_eq!('h'.millibits(&mut Default::default()), Some(8000), "just h");
    assert_eq!(
        "h".to_string().millibits(&mut Default::default()),
        Some(11000),
        "just h string"
    );

    let s = "aaaaaaaaaaaaaaaa".to_string();
    assert_eq!(
        s.millibits(&mut Default::default()),
        Some(39424),
        "just a string"
    );
    assert_bits!(s.clone(), expect!["40"]);

    let s = "hello world this is a string".to_string();
    assert_eq!(
        s.millibits(&mut Default::default()),
        Some(165025),
        "just a string"
    );
    assert_bits!(s.clone(), expect!["165"]);

    expect!["normal: Some(14000) millibits (14 bits), small: Some(20000) millibits (20 bits)"]
        .assert_eq(&compare_vecs(&["h"]));
    expect!["normal: Some(76790) millibits (77 bits), small: Some(82790) millibits (83 bits)"]
        .assert_eq(&compare_vecs(&["hello world"]));
    expect!["normal: Some(128070) millibits (128 bits), small: Some(101716) millibits (102 bits)"]
        .assert_eq(&compare_vecs(&["hello world", "hello world"]));
    expect!["normal: Some(172264) millibits (172 bits), small: Some(112527) millibits (113 bits)"]
        .assert_eq(&compare_vecs(&[
            "hello world",
            "hello world",
            "hello world",
        ]));
    expect!["normal: Some(262073) millibits (262 bits), small: Some(145730) millibits (146 bits)"]
        .assert_eq(&compare_vecs(&[
            "hello world",
            "hello world",
            "hello world",
            "hello world hello world",
        ]));
    expect!["normal: Some(209885) millibits (210 bits), small: Some(198308) millibits (198 bits)"]
        .assert_eq(&compare_vecs(&["hello world! 😊", "goodbye world! 😊"]));
    expect!["normal: Some(424130) millibits (424 bits), small: Some(350634) millibits (351 bits)"]
        .assert_eq(&compare_vecs(&[
            "hello world! 😊",
            "greetings world! 😊",
            "goodbye world! 😊",
            "farewell sweet world! 😊",
        ]));
    expect!["normal: Some(495559) millibits (496 bits), small: Some(413131) millibits (413 bits)"]
        .assert_eq(&compare_vecs(&[
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
        ]));
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
    assert_bits!(strings.clone(), expect!["242"]);
    assert_bits!(encoded_strings.clone(), expect!["204"]);

    let strings: Vec<String> = COMPRESSIBLE_TEXT
        .split(' ')
        .map(|s| s.to_string())
        .collect::<BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>();
    let encoded_strings: Encoded<Vec<String>, Values<Sorted>> =
        crate::Encoded::new(strings.clone());
    use super::assert_bits;

    assert_bits!(strings.clone(), expect!["5958"]);
    assert_bits!(encoded_strings.clone(), expect!["4961"]);
}

#[test]
fn crash_from_bench() {
    use super::assert_bits;
    use crate::{Encoded, Values};
    let names = ["Al", "Aïr"];
    let vec = names.iter().map(|n| n.to_string()).collect::<Vec<String>>();
    assert_bits!(vec.clone(), expect!["54"]);
    let compressible = Encoded::<Vec<String>, Values<Compressible>>::new(vec.clone());
    assert_bits!(compressible.clone(), expect!["69"]);
    let sorted = Encoded::<Vec<String>, Values<Sorted>>::new(vec.clone());
    assert_bits!(sorted.clone(), expect!["51"]);
}
