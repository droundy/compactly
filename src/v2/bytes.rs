use super::{Encode, EncodingStrategy};
use crate::{Compressible, Normal, Small, Values};
use std::collections::VecDeque;

// mod buffer;

#[derive(Default, Clone)]
pub struct Lz77 {
    old: VecDeque<Vec<u8>>,
    count: <Small as EncodingStrategy<usize>>::Context,
    literal: <Values<Normal> as EncodingStrategy<Vec<u8>>>::Context,
    back: <Small as EncodingStrategy<u8>>::Context,
    /// We use small encoding for offset, because we expect often to see small strings in total.
    offset: <Small as EncodingStrategy<usize>>::Context,
    self_offset: <Small as EncodingStrategy<usize>>::Context,
    length: <Small as EncodingStrategy<u8>>::Context,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Chunk {
    literal: Vec<u8>,
    /// Number of bytes in the chunk.
    length: u8,
    /// Value of 0 indicates current string, otherwise count back in old.
    back: u8,
    /// Where in the string it is located.  Counts backwards if back==0 otherwise forwards.
    offset: usize,
}

impl Lz77 {
    fn push_old(&mut self, value: Vec<u8>) {
        self.old.push_front(value);
        while self.old.len() > 254 {
            self.old.pop_back();
        }
    }
    fn shift_chunk(&mut self, Chunk { back, .. }: &Chunk) {
        if *back > 0 {
            // Whenever we use an old string, we move it to the front of the
            // queue.  This keeps "useful" strings from falling off the back,
            // and also makes smaller `back` values more common.
            let back = *back as usize - 1;
            if let Some(old) = self.old.remove(back) {
                self.old.push_front(old);
            }
        }
    }
    fn eager(&self, mut value: &[u8]) -> Vec<Chunk> {
        assert!(self.old.len() < 255);
        let mut sofar = Vec::with_capacity(value.len());
        let mut out = Vec::new();
        let mut ctx = self.clone();
        while let Some(chunk) = ctx.eager_chunk(&mut value, &mut sofar) {
            chunk.encode(&mut super::Millibits::new(0), &mut ctx);
            out.push(chunk);
        }
        out
    }
    fn eager_chunk(&mut self, value: &mut &[u8], sofar: &mut Vec<u8>) -> Option<Chunk> {
        let mut literal = Vec::with_capacity(value.len());
        while !value.is_empty() {
            let prefix = if value.len() > u8::MAX as usize {
                &value[..u8::MAX as usize]
            } else {
                *value
            };
            let sofar_clone = sofar.clone();
            let mut possible_chunks = Vec::new();
            let mut min_match = 0;
            let mut bytes_seen_so_far = 0;
            for (back, mut s) in std::iter::once(sofar_clone.as_slice())
                .chain(self.old.iter().map(|s| s.as_slice()))
                .enumerate()
                .map(|(back, s)| (back as u8, s))
            {
                const BACK_WINDOW: usize = 3 * 1024;
                if bytes_seen_so_far > BACK_WINDOW {
                    break;
                }
                if bytes_seen_so_far + s.len() > BACK_WINDOW {
                    let remaining = bytes_seen_so_far + s.len() - BACK_WINDOW;
                    if back > 0 {
                        if let Some((new_s, _)) = &s.split_at_checked(remaining) {
                            s = new_s;
                        }
                    } else {
                        if let Some((_, new_s)) = &s.split_at_checked(s.len() - remaining) {
                            s = new_s;
                        }
                    }
                }
                bytes_seen_so_far += s.len();
                let best_prefix = if back == 0 {
                    find_longest_latest_prefix(s, prefix)
                } else {
                    find_first_longest_prefix(s, prefix, min_match)
                };
                if let Some((mut offset, mut length)) = best_prefix {
                    min_match = length + 1;
                    if back == 0 {
                        if offset + length == s.len() {
                            // We could keep repeating maybe?
                            while length < 255
                                && length < prefix.len()
                                && s[offset + length % (s.len() - offset)] == prefix[length]
                            {
                                length += 1;
                            }
                        }
                        offset = s.len() - offset - 1;
                    }
                    let length = -(length as i16); // so we can minimize
                    possible_chunks.push((length, back, offset));
                }
            }
            if let Some((l, back, offset)) = possible_chunks.into_iter().min() {
                let length = (-l) as u8; // safe because l is negative and less than 256.
                *value = &value[length as usize..];
                sofar.extend_from_slice(&prefix[..length as usize]);
                let chunk = Chunk {
                    literal,
                    length,
                    back,
                    offset,
                };
                self.shift_chunk(&chunk);
                return Some(chunk);
            }
            // We are forced to emit a literal byte
            let (&first, rest) = value.split_first()?;
            literal.push(first);
            sofar.push(first);
            *value = rest;
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

    pub fn encode<E: super::EntropyCoder>(&mut self, value: &[u8], writer: &mut E) {
        let chunks = self.eager(value);
        Small::encode(&chunks.len(), writer, &mut self.count);
        for chunk in chunks {
            chunk.encode(writer, self);
            self.shift_chunk(&chunk);
        }
        self.push_old(value.to_vec());
    }

    pub fn decode<D: super::EntropyDecoder>(
        &mut self,
        reader: &mut D,
    ) -> Result<Vec<u8>, std::io::Error> {
        let count = <Small as EncodingStrategy<usize>>::decode(reader, &mut self.count)?;
        let mut out = Vec::with_capacity(5 * count);
        for _ in 0..count {
            let chunk @ Chunk {
                length,
                back,
                offset,
                ..
            } = <Chunk as Encode>::decode(reader, self)?;
            out.extend_from_slice(&chunk.literal);
            if length == 0 {
                // Nothing to do here.
            } else if back == 0 {
                // We are repeating our own string.  In this case offset
                // counts *backwards* and must be >= 1 so we shift it.
                let offset = out.len() - 1 - offset;
                out.reserve(length as usize);
                for i in offset..offset + length as usize {
                    out.push(out[i]);
                }
            } else {
                self.shift_chunk(&chunk);
                out.extend_from_slice(&self.old[0][offset..offset + length as usize]);
            }
        }
        self.push_old(out.clone());
        Ok(out)
    }
}

#[test]
fn eager() {
    assert_eq!(Lz77::default().eager(b""), Vec::new());
    macro_rules! assert_literal {
        ($s:literal) => {
            assert_eq!(
                Lz77::default().eager($s),
                vec![Chunk {
                    literal: $s.to_vec(),
                    length: 0,
                    back: 0,
                    offset: 0
                }]
            );
        };
    }
    assert_literal!(b"a");
    assert_literal!(b"aa");
    assert_literal!(b"aaa");
    {
        let mut ctx = Lz77::default();
        let mut millibits_of_literals = super::Millibits::new(0);
        for chunk in Lz77::default().eager(b"aaa") {
            chunk.encode(&mut millibits_of_literals, &mut ctx);
        }
        assert_eq!(millibits_of_literals, super::Millibits::new(22976));
        let mb_of_vec = Lz77::default().eager(b"aaa").millibits();
        assert_eq!(mb_of_vec, super::Millibits::new(25976));
        let mb_of_string = b"aaa".to_vec().millibits();
        assert_eq!(mb_of_string, super::Millibits::new(19976));
    }
    assert_eq!(
        Lz77::default().eager(b"aaaaaaaaaaaaaaaaaaaa"),
        vec![Chunk {
            literal: b"aaaaa".to_vec(),
            length: 15,
            back: 0,
            offset: 4,
        },]
    );
}

/// Returns offset and length of the longest prefix of the needle prefering a later one
pub(crate) fn find_longest_latest_prefix(haystack: &[u8], needle: &[u8]) -> Option<(usize, usize)> {
    const MIN_MATCH: usize = 5;
    let mut prefix = if needle.len() > 1 && needle.len() < MIN_MATCH {
        needle
    } else {
        needle.split_at_checked(MIN_MATCH)?.0
    };
    let mut best = None;
    for offset in (0..(haystack.len() + 1).saturating_sub(prefix.len())).rev() {
        let here = &haystack[offset..];
        if here.starts_with(prefix) {
            let mut length = prefix.len();
            if prefix.len() < needle.len() && here.len() > prefix.len() {
                length += needle[prefix.len()..]
                    .iter()
                    .zip(&haystack[offset + prefix.len()..])
                    .take_while(|(c1, c2)| c1 == c2)
                    .count();
            }
            best = Some((offset, length));
            if length == needle.len() {
                // We already found the whole needle!
                return best;
            }
            prefix = &needle[..length + 1];
        }
    }
    best
}

/// Returns offset and length of the longest prefix of the needle prefering one at the beginning
pub(crate) fn find_first_longest_prefix(
    haystack: &[u8],
    needle: &[u8],
    min_match: usize,
) -> Option<(usize, usize)> {
    if haystack.len() < min_match || needle.len() < min_match {
        return None;
    }
    let mut prefix = if min_match == 0 && needle.len() > 1 && needle.len() < 5 {
        needle
    } else {
        needle.split_at_checked(min_match.max(5))?.0
    };
    let mut best = None;
    for offset in 0..(haystack.len() + 1).saturating_sub(prefix.len()) {
        let here = &haystack[offset..];
        if here.starts_with(prefix) {
            let mut length = prefix.len();
            if prefix.len() < needle.len() && here.len() > prefix.len() {
                length += needle[prefix.len()..]
                    .iter()
                    .zip(&haystack[offset + prefix.len()..])
                    .take_while(|(c1, c2)| c1 == c2)
                    .count();
            }
            best = Some((offset, length));
            if length == needle.len() {
                // We already found the whole needle!
                return best;
            }
            prefix = &needle[..length + 1];
        }
    }
    best
}

#[test]
fn test_longest_prefix() {
    assert_eq!(
        find_longest_latest_prefix(b"hello world hello David", b"hello"),
        Some((12, 5))
    );
    assert_eq!(
        find_longest_latest_prefix(b"hello world hello David", b"hello Roundy"),
        Some((12, 6))
    );
    assert_eq!(
        find_longest_latest_prefix(b"hello world hello David hell is hellish", b"hello Roundy"),
        Some((12, 6))
    );
}

#[cfg(test)]
const COMPRESSIBLE_TEXT: &[u8] = b"Lossless compression is a class of data compression that allows the original data to be perfectly reconstructed from the compressed data with no loss of information. Lossless compression is possible because most real-world data exhibits statistical redundancy.[1] By contrast, lossy compression permits reconstruction only of an approximation of the original data, though usually with greatly improved compression rates (and therefore reduced media sizes).

By operation of the pigeonhole principle, no lossless compression algorithm can shrink the size of all possible data: Some data will get longer by at least one symbol or bit.

Compression algorithms are usually effective for human- and machine-readable documents and cannot shrink the size of random data that contain no redundancy. Different algorithms exist that are designed either with a specific type of input data in mind or with specific assumptions about what kinds of redundancy the uncompressed data are likely to contain.

Lossless data compression is used in many applications. For example, it is used in the ZIP file format and in the GNU tool gzip. It is also often used as a component within lossy data compression technologies (e.g. lossless mid/side joint stereo preprocessing by MP3 encoders and other lossy audio encoders).[2]

Lossless compression is used in cases where it is important that the original and the decompressed data be identical, or where deviations from the original data would be unfavourable. Common examples are executable programs, text documents, and source code. Some image file formats, like PNG or GIF, use only lossless compression, while others like TIFF and MNG may use either lossless or lossy methods. Lossless audio formats are most often used for archiving or production purposes, while smaller lossy audio files are typically used on portable players and in other cases where storage space is limited or exact replication of the audio is unnecessary. ";

impl Encode for Chunk {
    type Context = Lz77;
    fn encode<E: super::EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
        let Chunk {
            literal,
            length,
            back,
            offset,
        } = self;
        literal.encode(writer, &mut ctx.literal);
        Small::encode(length, writer, &mut ctx.length);
        if *length > 0 {
            Small::encode(back, writer, &mut ctx.back);
            if *back == 0 {
                Small::encode(offset, writer, &mut ctx.self_offset);
            } else {
                Small::encode(offset, writer, &mut ctx.offset);
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let literal = <Vec<u8> as Encode>::decode(reader, &mut ctx.literal)?;
        let length = <Small as EncodingStrategy<u8>>::decode(reader, &mut ctx.length)?;
        if length > 0 {
            let back = Small::decode(reader, &mut ctx.back)?;
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

impl EncodingStrategy<Vec<u8>> for Compressible {
    type Context = Lz77;
    fn encode<E: super::EntropyCoder>(value: &Vec<u8>, writer: &mut E, ctx: &mut Self::Context) {
        ctx.encode(value, writer)
    }

    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<u8>, std::io::Error> {
        ctx.decode(reader)
    }
}

#[test]
fn size() {
    use super::assert_bits;
    use crate::Encoded;

    assert_bits!(b"".to_vec(), 3);
    assert_bits!(b"a".to_vec(), 11);
    assert_bits!(b"A".to_vec(), 11);
    assert_bits!(b"hello world".to_vec(), 74);
    assert_bits!(b"Hello world".to_vec(), 76);
    assert_bits!(b"hhhhhhhhhhh".to_vec(), 35);

    fn compare_small_bits(value: &[u8], expected_normal: usize, expected_small: usize) {
        let s = String::from_utf8_lossy(value);
        assert_bits!(value.to_vec(), expected_normal, format!("normal b{s:?}"));
        assert_bits!(
            Encoded::<_, Compressible>::new(value.to_vec()),
            expected_small,
            format!("small b{s:?}")
        );
    }
    fn compare_vecs(value: &[&[u8]], expected_normal: usize, expected_small: usize) {
        let s = value
            .iter()
            .map(|b| String::from_utf8_lossy(b))
            .collect::<Vec<_>>();
        let normal = value.iter().map(|s| s.to_vec()).collect::<Vec<_>>();
        let encoded_normal = super::encode(&normal);
        let decoded_normal: Vec<Vec<u8>> = super::decode(&encoded_normal).unwrap();
        assert_eq!(normal, decoded_normal);

        let small: Vec<Encoded<Vec<u8>, Compressible>> =
            value.iter().map(|s| s.to_vec().into()).collect::<Vec<_>>();
        let encoded_small = super::encode(&small);
        let decoded_small: Vec<Encoded<Vec<u8>, Compressible>> =
            super::decode(&encoded_small).unwrap();
        assert_eq!(small, decoded_small);

        println!("normal millibits b{s:?}");
        assert_eq!(
            normal.millibits(),
            super::Millibits::new(expected_normal),
            "normal millibits b{s:?}"
        );
        println!("small millibits b{s:?}");
        assert_eq!(
            small.millibits(),
            super::Millibits::new(expected_small),
            "small millibits b{s:?}"
        );
        assert_bits!(
            value.iter().map(|s| s.to_vec()).collect::<Vec<Vec<u8>>>(),
            (expected_normal + 500) / 1000,
            format!("normal b{s:?}")
        );
        assert_bits!(
            value
                .iter()
                .map(|s| Encoded::<_, Compressible>::new(s.to_vec()))
                .collect::<Vec<_>>(),
            (expected_small + 500) / 1000,
            format!("small b{s:?}")
        );
    }
    compare_small_bits(COMPRESSIBLE_TEXT, 8979, 7116);

    assert_eq!(true.millibits(), super::Millibits::bits(1));
    assert_eq!('a'.millibits(), super::Millibits::bits(8));
    assert_eq!(
        Chunk {
            literal: b"a".to_vec(),
            length: 0,
            back: 0,
            offset: 0
        }
        .millibits(),
        super::Millibits::bits(14)
    );
    assert_eq!(
        Chunk {
            literal: Vec::new(),
            back: 0,
            offset: 0,
            length: 2
        }
        .millibits(),
        super::Millibits::bits(13)
    );
    compare_small_bits(b"", 3, 3);
    compare_small_bits(b"a", 11, 17);
    compare_small_bits(b"aa", 17, 23);
    compare_small_bits(b"aaa", 20, 26);
    compare_small_bits(b"aaaa", 24, 30);
    compare_small_bits(b"aaaaaaaa", 31, 39);
    compare_small_bits(b"hello", 36, 42);
    compare_small_bits(b"hello world hello wood", 122, 116);
    compare_small_bits(b"hello world hello world", 127, 98);
    compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.",
        415,
        421,
    );
    compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
        1537,
        839,
    );
    compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
        1607,
        1011,
    );

    compare_vecs(&[], 3000, 3000);
    assert_eq!(
        b"h".to_vec().millibits(),
        super::Millibits::bits(11),
        "just h string"
    );

    let s = b"aaaaaaaaaaaaaaaa".to_vec();
    assert_eq!(s.millibits(), super::Millibits::new(39424), "just a string");
    assert_bits!(s.clone(), 40);

    let s = b"hello world this is a string".to_vec();
    assert_eq!(
        s.millibits(),
        super::Millibits::new(165025),
        "just a string"
    );
    assert_bits!(s.clone(), 165);

    compare_vecs(&[b"h"], 14000, 20000);
    compare_vecs(&[b"hello world"], 76790, 82790);
    compare_vecs(&[b"hello world", b"hello world"], 128070, 101716);
    compare_vecs(
        &[b"hello world", b"hello world", b"hello world"],
        172264,
        112527,
    );
    compare_vecs(
        &[
            b"hello world",
            b"hello world",
            b"hello world",
            b"hello world hello world",
        ],
        262073,
        145730,
    );
    compare_vecs(
        &[
            b"The quick brown fox jumps over the lazy dog.",
            b"The",
            b"quick",
            b"brown",
            b"fox",
            b"jumps",
            b"over",
            b"the",
            b"lazy",
            b"dog",
        ],
        495559,
        413131,
    );
}
