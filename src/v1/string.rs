use super::{bits::Bits, Encode, EncodingStrategy, Small, URange};

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CharContext {
    is_ascii: <bool as Encode>::Context,
    ascii: <Bits<128> as Encode>::Context,
    n_chunks: <URange<3> as Encode>::Context,
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
            let n_chunks = URange::<3>::try_from(n_chunks).unwrap();
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
            let n_chunks = URange::<3>::try_from(n_chunks).unwrap();
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
            let n_chunks = URange::<3>::decode(reader, &mut ctx.n_chunks)?;
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
pub struct Lz77 {
    old: Vec<String>,
    count: <Small as EncodingStrategy<usize>>::Context,
    literal: <String as Encode>::Context,
    back: <usize as Encode>::Context,
    /// We use small encoding for offset, because we expect often to see small strings in total.
    offset: <Small as EncodingStrategy<usize>>::Context,
    self_offset: <Small as EncodingStrategy<usize>>::Context,
    length: <Small as EncodingStrategy<u8>>::Context,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Chunk {
    literal: String,
    /// Number of bytes in the chunk.
    length: u8,
    /// Value of 0 indicates current string, otherwise count back in old.
    back: usize,
    /// Where in the string it is located.  Counts backwards if back==0 otherwise forwards.
    offset: usize,
}

fn split_prefix(s: &str, mut len: usize) -> Option<&str> {
    while len > 1 {
        if let Some((p, _)) = s.split_at_checked(len) {
            return Some(p);
        }
        len -= 1;
    }
    None
}

impl Lz77 {
    fn eager(&self, mut value: &str) -> Vec<Chunk> {
        let mut sofar = String::new();
        let mut out = Vec::new();
        let mut ctx = self.clone();
        while let Some(chunk) = ctx.eager_chunk(&mut value, &mut sofar) {
            chunk.millibits(&mut ctx);
            out.push(chunk);
        }
        // println!("{out:?} with old {:?}", self.old);
        out
    }
    fn eager_chunk(&self, value: &mut &str, sofar: &mut String) -> Option<Chunk> {
        let mut literal = String::with_capacity(value.len());
        while !value.is_empty() {
            let prefix = if value.len() > u8::MAX as usize {
                &value[..u8::MAX as usize]
            } else {
                *value
            };
            if let Some(prefix_start) = split_prefix(prefix, 5) {
                let sofar_clone = sofar.clone();
                let mut possible_chunks = Vec::new();
                for (back, s) in std::iter::once(sofar_clone.as_str())
                    .chain(self.old.iter().map(|s| s.as_str()).rev())
                    .enumerate()
                {
                    if let Some(mut offset) = s.find(prefix_start) {
                        let length = prefix
                            .bytes()
                            .zip(s[offset..].bytes())
                            .take_while(|(c1, c2)| c1 == c2)
                            .count();
                        let length = -(length as i16); // so we can minimize
                        if back == 0 {
                            offset = s.len() - offset - 1;
                        }
                        possible_chunks.push((length, back, offset));
                    }
                }
                if let Some((l, back, offset)) = possible_chunks.into_iter().min() {
                    let length = (-l) as u8; // safe because l is negative and less than 256.
                    *value = &value[length as usize..];
                    sofar.push_str(&prefix[..length as usize]);
                    return Some(Chunk {
                        literal,
                        length,
                        back,
                        offset,
                    });
                }
            }
            // We are forced to emit a literal character
            let mut chars = value.char_indices();
            let first = chars.next()?.1;
            literal.push(first);
            sofar.push(first);
            let sz = chars
                .next()
                .map(|(sz, _)| sz)
                .unwrap_or_else(|| value.len());
            *value = &value[sz..];
        }
        if literal.is_empty() {
            None
        } else {
            Some(Chunk {
                literal,
                length: 0,
                back: 0,
                offset: 0,
            })
        }
    }
}

#[test]
fn eager() {
    assert_eq!(Lz77::default().eager(""), Vec::new());
    macro_rules! assert_literal {
        ($s:literal) => {
            assert_eq!(
                Lz77::default().eager($s),
                vec![Chunk {
                    literal: $s.to_string(),
                    length: 0,
                    back: 0,
                    offset: 0
                }]
            );
        };
    }
    assert_literal!("a");
    assert_literal!("aa");
    assert_literal!("aaa");
    {
        let mut ctx = Lz77::default();
        let millibits_of_literals = Lz77::default()
            .eager("aaa")
            .into_iter()
            .map(|c| c.millibits(&mut ctx).unwrap())
            .sum::<usize>();
        assert_eq!(millibits_of_literals, 22976);
        let mb_of_vec = Lz77::default()
            .eager("aaa")
            .millibits(&mut Default::default())
            .unwrap();
        assert_eq!(mb_of_vec, 25976);
        let mb_of_string = "aaa"
            .to_string()
            .millibits(&mut Default::default())
            .unwrap();
        assert_eq!(mb_of_string, 19976);
    }
    assert_eq!(
        Lz77::default().eager("aaaa"),
        vec![Chunk {
            literal: "aa".to_string(),
            length: 2,
            back: 0,
            offset: 1,
        }]
    );
    assert_eq!(
        Lz77::default().eager("aaaaaaaaaaaaaaaaaaaa"),
        vec![
            Chunk {
                literal: "aaaaa".to_string(),
                length: 5,
                back: 0,
                offset: 4,
            },
            Chunk {
                literal: "".to_string(),
                length: 10,
                back: 0,
                offset: 9,
            }
        ]
    );
    // assert_eq!(
    //     Lz77::default().eager(COMPRESSIBLE_TEXT),
    //     vec![
    //         Chunk::Literal('a'),
    //         Chunk::Literal('a'),
    //         Chunk::Chunk {
    //             back: 0,
    //             offset: 1,
    //             length: 2
    //         }
    //     ]
    // );
}

#[cfg(test)]
const COMPRESSIBLE_TEXT: &str = "Lossless compression is a class of data compression that allows the original data to be perfectly reconstructed from the compressed data with no loss of information. Lossless compression is possible because most real-world data exhibits statistical redundancy.[1] By contrast, lossy compression permits reconstruction only of an approximation of the original data, though usually with greatly improved compression rates (and therefore reduced media sizes).

By operation of the pigeonhole principle, no lossless compression algorithm can shrink the size of all possible data: Some data will get longer by at least one symbol or bit.

Compression algorithms are usually effective for human- and machine-readable documents and cannot shrink the size of random data that contain no redundancy. Different algorithms exist that are designed either with a specific type of input data in mind or with specific assumptions about what kinds of redundancy the uncompressed data are likely to contain.

Lossless data compression is used in many applications. For example, it is used in the ZIP file format and in the GNU tool gzip. It is also often used as a component within lossy data compression technologies (e.g. lossless mid/side joint stereo preprocessing by MP3 encoders and other lossy audio encoders).[2]

Lossless compression is used in cases where it is important that the original and the decompressed data be identical, or where deviations from the original data would be unfavourable. Common examples are executable programs, text documents, and source code. Some image file formats, like PNG or GIF, use only lossless compression, while others like TIFF and MNG may use either lossless or lossy methods. Lossless audio formats are most often used for archiving or production purposes, while smaller lossy audio files are typically used on portable players and in other cases where storage space is limited or exact replication of the audio is unnecessary. ";

impl Encode for Chunk {
    type Context = Lz77;
    fn encode<W: std::io::Write>(
        &self,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let Chunk {
            literal,
            length,
            back,
            offset,
        } = self;
        literal.encode(writer, &mut ctx.literal)?;
        Small::encode(length, writer, &mut ctx.length)?;
        if *length > 0 {
            back.encode(writer, &mut ctx.back)?;
            if *back == 0 {
                Small::encode(offset, writer, &mut ctx.self_offset)?;
            } else {
                Small::encode(offset, writer, &mut ctx.offset)?;
            }
        }
        Ok(())
    }
    fn millibits(&self, ctx: &mut Self::Context) -> Option<usize> {
        let Chunk {
            literal,
            length,
            back,
            offset,
        } = self;
        let mut tot = literal.millibits(&mut ctx.literal)?;
        tot += Small::millibits(length, &mut ctx.length)?;
        if *length > 0 {
            tot += back.millibits(&mut ctx.back)?;
            tot += if *back == 0 {
                Small::millibits(offset, &mut ctx.self_offset)?
            } else {
                Small::millibits(offset, &mut ctx.offset)?
            };
        }
        Some(tot)
    }
    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let literal = <String as Encode>::decode(reader, &mut ctx.literal)?;
        let length = <Small as EncodingStrategy<u8>>::decode(reader, &mut ctx.length)?;
        if length > 0 {
            let back = <usize as Encode>::decode(reader, &mut ctx.back)?;
            let offset = if back == 0 {
                <Small as EncodingStrategy<usize>>::decode(reader, &mut ctx.self_offset)?
            } else {
                <Small as EncodingStrategy<usize>>::decode(reader, &mut ctx.offset)?
            };
            Ok(Chunk {
                literal,
                back,
                offset,
                length,
            })
        } else {
            Ok(Chunk {
                literal,
                length,
                back: 0,
                offset: 0,
            })
        }
    }
}

impl EncodingStrategy<String> for Small {
    type Context = Lz77;
    fn encode<W: std::io::Write>(
        value: &String,
        writer: &mut super::Writer<W>,
        ctx: &mut Self::Context,
    ) -> Result<(), std::io::Error> {
        let chunks = ctx.eager(value);
        Small::encode(&chunks.len(), writer, &mut ctx.count)?;
        for chunk in chunks {
            chunk.encode(writer, ctx)?;
        }
        ctx.old.push(value.clone());
        Ok(())
    }
    fn millibits(value: &String, ctx: &mut Self::Context) -> Option<usize> {
        let chunks = ctx.eager(value);
        let mut tot = Small::millibits(&chunks.len(), &mut ctx.count)?;
        for chunk in chunks {
            tot += chunk.millibits(ctx)?;
        }
        ctx.old.push(value.clone());
        Some(tot)
    }

    fn decode<R: std::io::Read>(
        reader: &mut super::Reader<R>,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        let count = <Small as EncodingStrategy<usize>>::decode(reader, &mut ctx.count)?;
        let mut out = String::with_capacity(5 * count);
        for _ in 0..count {
            let Chunk {
                literal,
                length,
                back,
                offset,
            } = <Chunk as Encode>::decode(reader, ctx)?;
            out.push_str(&literal);
            if back == 0 {
                // We are repeating our own string.  In this case offset
                // counts *backwards* and must be >= 1 so we shift it.
                let offset = out.len() - 1 - offset;
                if offset + length as usize <= out.len() {
                    // println!("We have from {offset} to {}", offset + length);
                    let x = String::from(&out[offset..offset + length as usize]);
                    out.push_str(&x);
                } else {
                    // println!("We are run length encoding");
                    // With extra length this means we are using run length
                    // encoding in effect, which is kind of a pain.
                    let chunk = String::from(&out[offset..]);
                    let final_length = out.len() + length as usize;
                    while out.len() < final_length {
                        out.push_str(&chunk);
                    }
                    while out.len() > final_length {
                        out.pop();
                    }
                }
            } else {
                out.push_str(&ctx.old[ctx.old.len() - back][offset..offset + length as usize]);
            }
        }
        ctx.old.push(out.clone());
        Ok(out)
    }
}

#[test]
fn size() {
    use super::{assert_bits, Encoded};

    assert_bits!("".to_string(), 3);
    assert_bits!("a".to_string(), 11);
    assert_bits!("A".to_string(), 11);
    assert_bits!("É".to_string(), 16);
    assert_bits!("😊".to_string(), 23);
    assert_bits!("hello world".to_string(), 74);
    assert_bits!("Hello world".to_string(), 76);
    assert_bits!("hhhhhhhhhhh".to_string(), 35);

    fn compare_small_bits(value: &str, expected_normal: usize, expected_small: usize) {
        assert_bits!(
            value.to_string(),
            expected_normal,
            format!("normal {value:?}")
        );
        assert_bits!(
            Encoded::<_, Small>::new(value.to_string()),
            expected_small,
            format!("small {value:?}")
        );
    }
    fn compare_vecs(value: &[&str], expected_normal: usize, expected_small: usize) {
        println!("normal millibits {value:?}");
        assert_eq!(
            value
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .millibits(&mut Default::default()),
            Some(expected_normal),
            "normal millibits {value:?}"
        );
        println!("small millibits {value:?}");
        assert_eq!(
            value
                .iter()
                .map(|s| super::Compact::new(s.to_string()))
                .collect::<Vec<_>>()
                .millibits(&mut Default::default()),
            Some(expected_small),
            "small millibits {value:?}"
        );
        assert_bits!(
            value.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            (expected_normal + 500) / 1000,
            format!("normal {value:?}")
        );
        assert_bits!(
            value
                .iter()
                .map(|s| super::Compact::new(s.to_string()))
                .collect::<Vec<_>>(),
            (expected_small + 500) / 1000,
            format!("small {value:?}")
        );
    }
    compare_small_bits(COMPRESSIBLE_TEXT, 8979, 7276);

    assert_eq!(true.millibits(&mut Default::default()), Some(1000));
    assert_eq!('a'.millibits(&mut Default::default()), Some(8000));
    assert_eq!(
        Chunk {
            literal: "a".to_string(),
            length: 0,
            back: 0,
            offset: 0
        }
        .millibits(&mut Default::default()),
        Some(14000)
    );
    assert_eq!(
        Chunk {
            literal: String::new(),
            back: 0,
            offset: 0,
            length: 2
        }
        .millibits(&mut Default::default()),
        Some(13000)
    );
    assert_eq!('😊'.millibits(&mut Default::default()), Some(20000));
    compare_small_bits("", 3, 3);
    compare_small_bits("a", 11, 17);
    compare_small_bits("aa", 17, 23);
    compare_small_bits("aaa", 20, 26);
    compare_small_bits("aaaa", 24, 30);
    compare_small_bits("aaaaaaaa", 31, 39);
    compare_small_bits("aaaa1★😊aaaaaaaa1★😊😊aa", 133, 119);
    compare_small_bits("hello", 36, 42);
    compare_small_bits("hello world hello wood", 122, 116);
    compare_small_bits("hello world hello world", 127, 98);
    compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.",
        415,
        421,
    );
    compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
        1537,
        842,
    );
    compare_small_bits(
        "This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
        1607,
        1013,
    );

    compare_vecs(&[], 3000, 3000);
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
    assert_bits!(s.clone(), 40);

    let s = "hello world this is a string".to_string();
    assert_eq!(
        s.millibits(&mut Default::default()),
        Some(165025),
        "just a string"
    );
    assert_bits!(s.clone(), 165);

    compare_vecs(&["h"], 14000, 20000);
    compare_vecs(&["hello world"], 76790, 82790);
    compare_vecs(&["hello world", "hello world"], 127070, 100716);
    compare_vecs(
        &["hello world", "hello world", "hello world"],
        171264,
        111527,
    );
    compare_vecs(
        &[
            "hello world",
            "hello world",
            "hello world",
            "hello world hello world",
        ],
        265073,
        148730,
    );
}
