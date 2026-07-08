use super::{Encode, EncodingStrategy};
use crate::{Compressible, Normal, Small, Values};
use std::collections::VecDeque;

// mod buffer;

#[cfg(test)]
use expect_test::expect;

const MIN_MATCH: usize = 5;
const MAX_CHAIN: usize = 256;
const LZ77_HASH_SIZE: usize = 1 << 16; // 65536 buckets

#[inline]
fn lz77_hash(bytes: &[u8]) -> usize {
    let v = u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    (v.wrapping_mul(0x9E3779B9) >> 16) as usize
}

/// Bitset filter over 4-grams present in old strings (LZ77_HASH_SIZE bits = 8 KiB).
/// Bits are set when strings are added to `old` and never cleared, so the filter may
/// have false positives from evicted strings but never false negatives from live ones.
/// Cloning costs 8 KiB, which is far cheaper than scanning 25 KiB of old strings
/// per chunk at O(MAX_CHAIN) depth.
#[derive(Clone)]
struct OldFilter(Box<[u64; LZ77_HASH_SIZE / 64]>);

impl Default for OldFilter {
    fn default() -> Self {
        OldFilter(Box::new([0u64; LZ77_HASH_SIZE / 64]))
    }
}

impl OldFilter {
    #[inline]
    fn set(&mut self, h: usize) {
        self.0[h >> 6] |= 1u64 << (h & 63);
    }
    #[inline]
    fn test(&self, h: usize) -> bool {
        self.0[h >> 6] & (1u64 << (h & 63)) != 0
    }
}

#[derive(Default, Clone)]
pub struct Lz77 {
    old: VecDeque<Vec<u8>>,
    /// Bloom-style 4-gram filter over bytes in `old`. Bits are added on push_old and never
    /// removed — evictions cause false positives but never false negatives.
    old_filter: OldFilter,
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
    literal: Box<[u8]>,
    /// Number of bytes in the chunk.
    length: u8,
    /// Value of 0 indicates current string, otherwise count back in old.
    back: u8,
    /// Where in the string it is located.  Counts backwards if back==0 otherwise forwards.
    offset: usize,
}

impl Lz77 {
    fn push_old(&mut self, value: Vec<u8>) {
        for p in 0..value.len() {
            if p + 4 <= value.len() {
                self.old_filter.set(lz77_hash(&value[p..]));
            }
        }
        self.push_old_decode(value);
    }
    /// Decode-side variant of `push_old`: maintains the `old` deque (needed for
    /// back-references) but skips the `old_filter` upkeep, which is only read by
    /// the encode-side match scan (`eager`/`eager_chunk`) and never on decode.
    fn push_old_decode(&mut self, value: Vec<u8>) {
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
        let mut sofar: Vec<u8> = Vec::new();
        // Hash chain for fast sofar matching. hash_head[h] is the most recent position
        // in sofar whose first 4 bytes hash to h. hash_next[p] links to the previous
        // position with the same hash, forming a chain newest→oldest.
        let mut hash_head = vec![u32::MAX; LZ77_HASH_SIZE];
        let mut hash_next: Vec<u32> = Vec::new();
        let mut out = Vec::new();
        let mut ctx = self.clone();
        while let Some(chunk) =
            ctx.eager_chunk(&mut value, &mut sofar, &mut hash_head, &mut hash_next)
        {
            chunk.encode(&mut super::Millibits::new(0), &mut ctx);
            out.push(chunk);
        }
        out
    }
    fn eager_chunk(
        &mut self,
        value: &mut &[u8],
        sofar: &mut Vec<u8>,
        hash_head: &mut [u32],
        hash_next: &mut Vec<u32>,
    ) -> Option<Chunk> {
        const BACK_WINDOW: usize = 64 * 1024;
        let mut literal = Vec::with_capacity(value.len());
        while !value.is_empty() {
            let prefix = if value.len() > u8::MAX as usize {
                &value[..u8::MAX as usize]
            } else {
                *value
            };

            // Hash-chain lookup for sofar matches: O(MAX_CHAIN) instead of O(window).
            let mut best_sofar: Option<(usize, usize)> = None; // (sofar position, length)
            if prefix.len() >= MIN_MATCH {
                let h = lz77_hash(prefix);
                let min_valid = sofar.len().saturating_sub(BACK_WINDOW);
                let mut candidate = hash_head[h] as usize;
                let mut chain = 0;
                while candidate != u32::MAX as usize && candidate >= min_valid && chain < MAX_CHAIN
                {
                    let cand = &sofar[candidate..];
                    if cand.len() >= MIN_MATCH
                        && cand[0] == prefix[0]
                        && cand[1] == prefix[1]
                        && cand[2] == prefix[2]
                        && cand[3] == prefix[3]
                    {
                        let length = 4 + prefix[4..]
                            .iter()
                            .zip(&cand[4..])
                            .take_while(|(a, b)| a == b)
                            .count();
                        if length >= MIN_MATCH && best_sofar.is_none_or(|(_, bl)| length > bl) {
                            best_sofar = Some((candidate, length));
                            if length == prefix.len() {
                                break;
                            }
                        }
                    }
                    candidate = hash_next.get(candidate).copied().unwrap_or(u32::MAX) as usize;
                    chain += 1;
                }
            }

            // Linear scan in old strings; must beat best_sofar to be chosen.
            // The old_filter bloom filter lets us skip the entire scan when the
            // current 4-gram prefix is not present in any old string.
            let sofar_best_len = best_sofar.map_or(0, |(_, l)| l);
            let sofar_back_len = sofar.len().min(BACK_WINDOW);
            let mut old_budget = BACK_WINDOW.saturating_sub(sofar_back_len);
            let mut min_old = sofar_best_len;
            let mut best_old: Option<(u8, usize, usize)> = None; // (back, offset, length)
            let skip_old = prefix.len() >= MIN_MATCH && !self.old_filter.test(lz77_hash(prefix));
            if !skip_old {
                for (i, old_s) in self.old.iter().enumerate() {
                    if old_budget == 0 {
                        break;
                    }
                    let use_s = if old_s.len() > old_budget {
                        &old_s[..old_budget]
                    } else {
                        old_s.as_slice()
                    };
                    old_budget = old_budget.saturating_sub(use_s.len());
                    // When min_old is 0 (no prior match), use min_match=0 to allow
                    // matching short sequences (2-4 bytes) verbatim in old strings.
                    let search_min = if min_old == 0 { 0 } else { min_old + 1 };
                    if let Some((offset, length)) =
                        find_first_longest_prefix(use_s, prefix, search_min)
                    {
                        best_old = Some(((i + 1) as u8, offset, length));
                        min_old = length;
                    }
                }
            }

            // Choose best match: prefer longer; ties go to sofar (back=0, cheaper offset).
            let chosen = match (best_sofar, best_old) {
                (None, None) => None,
                (None, Some((back, offset, length))) => Some((back, offset, length)),
                (Some((sofar_pos, sofar_len)), old_opt) => {
                    if old_opt.is_some_and(|(_, _, ol)| ol > sofar_len) {
                        let (back, offset, length) = old_opt.unwrap();
                        Some((back, offset, length))
                    } else {
                        // Apply wrap-around extension when match reaches end of sofar.
                        let mut length = sofar_len;
                        if sofar_pos + length == sofar.len() {
                            let period = sofar.len() - sofar_pos;
                            while length < 255
                                && length < prefix.len()
                                && sofar[sofar_pos + length % period] == prefix[length]
                            {
                                length += 1;
                            }
                        }
                        Some((0u8, sofar.len() - sofar_pos - 1, length))
                    }
                }
            };

            if let Some((back, offset, length)) = chosen {
                *value = &value[length..];
                let new_start = sofar.len();
                sofar.extend_from_slice(&prefix[..length]);
                hash_next.resize(sofar.len(), u32::MAX);
                // Insert all positions that now have MIN_MATCH bytes for the first time.
                // This includes "seam" positions (new_start-(MIN_MATCH-1)..new_start-1)
                // that span old sofar bytes and new match bytes, plus positions within
                // the match itself.
                let first_insert = new_start.saturating_sub(MIN_MATCH - 1);
                let last_insert = sofar.len().saturating_sub(MIN_MATCH);
                for p in first_insert..=last_insert {
                    // Hash needs 4 bytes; skip if near end (e.g. short match < 4 bytes).
                    if p + 4 <= sofar.len() {
                        let h = lz77_hash(&sofar[p..]);
                        let old = hash_head[h];
                        hash_head[h] = p as u32;
                        hash_next[p] = old;
                    }
                }
                let chunk = Chunk {
                    literal: literal.into_boxed_slice(),
                    length: length as u8,
                    back,
                    offset,
                };
                self.shift_chunk(&chunk);
                return Some(chunk);
            }

            // No match: emit literal byte and update hash chain.
            let (&first, rest) = value.split_first()?;
            literal.push(first);
            sofar.push(first);
            hash_next.push(u32::MAX);
            if sofar.len() >= MIN_MATCH {
                let insert_pos = sofar.len() - MIN_MATCH;
                let h = lz77_hash(&sofar[insert_pos..]);
                let old = hash_head[h];
                hash_head[h] = insert_pos as u32;
                hash_next[insert_pos] = old;
            }
            *value = rest;
        }
        if literal.is_empty() {
            None
        } else {
            Some(Chunk {
                literal: literal.into_boxed_slice(),
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
        self.push_old_decode(out.clone());
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
                    literal: $s.to_vec().into_boxed_slice(),
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
        assert_eq!(millibits_of_literals, super::Millibits::new(22988));
        let mb_of_vec = Lz77::default().eager(b"aaa").millibits();
        assert_eq!(mb_of_vec, super::Millibits::new(25988));
        let mb_of_string = b"aaa".to_vec().millibits();
        assert_eq!(mb_of_string, super::Millibits::new(19988));
    }
    assert_eq!(
        Lz77::default().eager(b"aaaaaaaaaaaaaaaaaaaa"),
        vec![Chunk {
            literal: b"aaaaa".to_vec().into_boxed_slice(),
            length: 15,
            back: 0,
            offset: 4,
        },]
    );
}

/// Returns offset and length of the longest prefix of the needle prefering a later one
#[cfg(test)]
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
        let literal = <Box<[u8]> as Encode>::decode(reader, &mut ctx.literal)?;
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
    use super::encoded_bits;
    use crate::Encoded;

    expect!["3"].assert_eq(&encoded_bits!(b"".to_vec()));
    expect!["11"].assert_eq(&encoded_bits!(b"a".to_vec()));
    expect!["11"].assert_eq(&encoded_bits!(b"A".to_vec()));
    expect!["74"].assert_eq(&encoded_bits!(b"hello world".to_vec()));
    expect!["76"].assert_eq(&encoded_bits!(b"Hello world".to_vec()));
    expect!["35"].assert_eq(&encoded_bits!(b"hhhhhhhhhhh".to_vec()));

    fn compare_small_bits(value: &[u8]) -> String {
        let s = String::from_utf8_lossy(value);
        println!("comparing b{s:?}");
        format!(
            "normal: {} bits, small: {} bits",
            super::encoded_bits!(value.to_vec()),
            super::encoded_bits!(Encoded::<_, Compressible>::new(value.to_vec()))
        )
    }
    fn compare_vecs(value: &[&[u8]]) -> String {
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

        format!(
            "normal: {:?} ({} bits), small: {:?} ({} bits)",
            normal.millibits(),
            super::encoded_bits!(value.iter().map(|s| s.to_vec()).collect::<Vec<Vec<u8>>>()),
            small.millibits(),
            super::encoded_bits!(value
                .iter()
                .map(|s| Encoded::<_, Compressible>::new(s.to_vec()))
                .collect::<Vec<_>>())
        )
    }
    expect!["normal: 8985 bits, small: 7113 bits"]
        .assert_eq(&compare_small_bits(COMPRESSIBLE_TEXT));

    assert_eq!(true.millibits(), super::Millibits::bits(1));
    assert_eq!('a'.millibits(), super::Millibits::bits(8));
    assert_eq!(
        Chunk {
            literal: b"a".to_vec().into_boxed_slice(),
            length: 0,
            back: 0,
            offset: 0
        }
        .millibits(),
        super::Millibits::bits(14)
    );
    assert_eq!(
        Chunk {
            literal: Box::new([]),
            back: 0,
            offset: 0,
            length: 2
        }
        .millibits(),
        super::Millibits::bits(13)
    );
    expect!["normal: 3 bits, small: 3 bits"].assert_eq(&compare_small_bits(b""));
    expect!["normal: 11 bits, small: 17 bits"].assert_eq(&compare_small_bits(b"a"));
    expect!["normal: 17 bits, small: 23 bits"].assert_eq(&compare_small_bits(b"aa"));
    expect!["normal: 20 bits, small: 26 bits"].assert_eq(&compare_small_bits(b"aaa"));
    expect!["normal: 24 bits, small: 30 bits"].assert_eq(&compare_small_bits(b"aaaa"));
    expect!["normal: 31 bits, small: 37 bits"].assert_eq(&compare_small_bits(b"aaaaaaaa"));
    expect!["normal: 36 bits, small: 42 bits"].assert_eq(&compare_small_bits(b"hello"));
    expect!["normal: 122 bits, small: 116 bits"]
        .assert_eq(&compare_small_bits(b"hello world hello wood"));
    expect!["normal: 127 bits, small: 98 bits"]
        .assert_eq(&compare_small_bits(b"hello world hello world"));
    expect!["normal: 413 bits, small: 419 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.",
    ));
    expect!["normal: 1539 bits, small: 835 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
    ));
    expect!["normal: 1609 bits, small: 1005 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
    ));

    expect!["normal: Millibits(3000) (3 bits), small: Millibits(3000) (3 bits)"]
        .assert_eq(&compare_vecs(&[]));
    assert_eq!(
        b"h".to_vec().millibits(),
        super::Millibits::bits(11),
        "just h string"
    );

    let s = b"aaaaaaaaaaaaaaaa".to_vec();
    assert_eq!(s.millibits(), super::Millibits::new(39549), "just a string");
    expect!["40"].assert_eq(&encoded_bits!(s.clone()));

    let s = b"hello world this is a string".to_vec();
    assert_eq!(
        s.millibits(),
        super::Millibits::new(165201),
        "just a string"
    );
    expect!["165"].assert_eq(&encoded_bits!(s.clone()));

    expect!["normal: Millibits(14000) (14 bits), small: Millibits(20000) (20 bits)"]
        .assert_eq(&compare_vecs(&[b"h"]));
    expect!["normal: Millibits(76841) (77 bits), small: Millibits(82841) (83 bits)"]
        .assert_eq(&compare_vecs(&[b"hello world"]));
    expect!["normal: Millibits(128206) (128 bits), small: Millibits(101770) (102 bits)"]
        .assert_eq(&compare_vecs(&[b"hello world", b"hello world"]));
    expect!["normal: Millibits(172498) (173 bits), small: Millibits(112584) (113 bits)"].assert_eq(
        &compare_vecs(&[b"hello world", b"hello world", b"hello world"]),
    );
    expect!["normal: Millibits(262517) (263 bits), small: Millibits(145803) (146 bits)"].assert_eq(
        &compare_vecs(&[
            b"hello world",
            b"hello world",
            b"hello world",
            b"hello world hello world",
        ]),
    );
    expect!["normal: Millibits(496105) (496 bits), small: Millibits(413459) (414 bits)"].assert_eq(
        &compare_vecs(&[
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
        ]),
    );
}
