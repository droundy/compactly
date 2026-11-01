use super::sentinel::Sentinel;
use super::{Encode, Strategy};
use crate::{Compressible, Normal, Small, Values};
use std::cell::RefCell;
use std::collections::VecDeque;

// mod buffer;

#[cfg(test)]
use super::millibits;
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

/// Which 4-grams one old string holds, as a bitset over [`lz77_hash`], so the
/// match search can pass over a string that cannot contain what it wants without
/// reading it. Sized to the string, at about 32 bits per byte, which leaves few
/// bits set and so few strings read for nothing. Empty on the decode side, which
/// never searches, and an empty filter rules nothing out.
#[derive(Default, Clone)]
struct GramFilter(Box<[u64]>);

impl GramFilter {
    /// Keeps a long string's filter from growing without limit: past this it
    /// fills up and rules out less, which only costs a read of that string.
    const MAX_WORDS: usize = 512;

    fn new(bytes: &[u8]) -> Self {
        let words = (bytes.len() / 2).clamp(1, Self::MAX_WORDS);
        let mut filter = GramFilter(vec![0; words.next_power_of_two()].into_boxed_slice());
        for gram in bytes.windows(4) {
            let (word, bit) = filter.position(lz77_hash(gram));
            filter.0[word] |= bit;
        }
        filter
    }
    #[inline]
    fn position(&self, h: usize) -> (usize, u64) {
        ((h >> 6) & (self.0.len() - 1), 1u64 << (h & 63))
    }
    #[inline]
    fn test(&self, h: usize) -> bool {
        if self.0.is_empty() {
            return true;
        }
        let (word, bit) = self.position(h);
        self.0[word] & bit != 0
    }
}

/// A string kept for later values to match against.
#[derive(Clone)]
struct OldString {
    bytes: Vec<u8>,
    grams: GramFilter,
}

/// Hash chains over the value being searched: `head[h]` is the most recent
/// position whose first 4 bytes hash to `h`, and `next` links each position to
/// the previous one with the same hash, forming a chain newest→oldest.
///
/// The 256 KiB table is far bigger than most values, and filling it for each one
/// was most of the cost of encoding short strings. So there is one per thread
/// (see [`CHAINS`]), never cleared: positions are stored numbered from `base`,
/// which moves past each value, and an entry below `base` is left over from an
/// earlier one, which is to say empty.
#[derive(Default)]
struct Chains {
    head: Vec<u32>,
    next: Vec<u32>,
    base: u32,
}

impl Chains {
    /// Get ready for a new value of `len` bytes.
    fn start(&mut self, len: usize) {
        self.head.resize(LZ77_HASH_SIZE, 0);
        // Past the previous value's last position, and so never 0, which is
        // what an entry that has never been set holds.
        let base = self.base as usize + self.next.len() + 1;
        self.next.clear();
        if base.saturating_add(len) < u32::MAX as usize {
            self.base = base as u32;
        } else {
            self.head.fill(0);
            self.base = 1;
        }
    }

    /// Record that the 4 bytes at `position` hash to `h`.
    #[inline]
    fn insert(&mut self, h: usize, position: usize) {
        self.next[position] = self.head[h];
        self.head[h] = position as u32 + self.base;
    }
}

thread_local! {
    /// Per thread rather than per [`Lz77`] because there can be many of those:
    /// one for each `Compressible` field of a type, each encoding a value or
    /// two. Giving each its own table touches megabytes of cold memory per
    /// record, where sharing one keeps it in cache.
    static CHAINS: RefCell<Chains> = RefCell::default();
}

#[derive(Default, Clone)]
pub struct Lz77 {
    old: VecDeque<OldString>,
    /// Bloom-style 4-gram filter over bytes in `old`. Bits are added on push_old and never
    /// removed — evictions cause false positives but never false negatives.
    old_filter: OldFilter,
    count: <usize as Encode>::Context,
    literal: <Vec<u8> as Encode<Values<Normal>>>::Context,
    back: <u8 as Encode<Small>>::Context,
    /// The default `usize` encoding (tiny-seeded `U64Compact`) rather than
    /// `Small<usize>`: a Lz77 offset is usually large, and `Small<usize>`
    /// would spend an extra `AtMost<7>` bucket symbol before recursing into
    /// `Small<u64>`, while the default codes it directly (measured ~4-5%
    /// faster decode at ~neutral size). A `u32` offset would be tighter still
    /// — offsets are bounded by `BACK_WINDOW` (64 KiB) and strings rarely
    /// exceed `u32`, so `U32Compact`'s shorter length-of-length tree would fit
    /// with bits to spare; see OPTIMIZING.md.
    offset: <usize as Encode>::Context,
    self_offset: <usize as Encode>::Context,
    length: <u8 as Encode<Small>>::Context,
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
        for gram in value.windows(4) {
            self.old_filter.set(lz77_hash(gram));
        }
        let grams = GramFilter::new(&value);
        self.keep_old(OldString {
            bytes: value,
            grams,
        });
    }
    /// Decode-side variant of `push_old`: maintains the `old` deque (needed for
    /// back-references) but skips the filters, which are only read by the
    /// encode-side match scan (`eager`/`eager_chunk`) and never on decode.
    fn push_old_decode(&mut self, value: Vec<u8>) {
        self.keep_old(OldString {
            bytes: value,
            grams: GramFilter::default(),
        });
    }
    fn keep_old(&mut self, old: OldString) {
        self.old.push_front(old);
        while self.old.len() > 254 {
            self.old.pop_back();
        }
    }
    fn shift_chunk(&mut self, chunk: &Chunk) {
        shift_old(&mut self.old, chunk);
    }
    fn eager(&self, value: &[u8]) -> Vec<Chunk> {
        CHAINS.with_borrow_mut(|chains| self.eager_with(value, chains))
    }
    fn eager_with(&self, mut value: &[u8], chains: &mut Chains) -> Vec<Chunk> {
        assert!(self.old.len() < 255);
        let mut sofar: Vec<u8> = Vec::new();
        chains.start(value.len());
        let mut out = Vec::new();
        // Using an old string moves it to the front, which the search has to
        // see as it goes, without disturbing `self.old` until the chunks are
        // actually coded.
        let mut old: VecDeque<&OldString> = self.old.iter().collect();
        while let Some(chunk) =
            Self::eager_chunk(&mut old, &self.old_filter, &mut value, &mut sofar, chains)
        {
            out.push(chunk);
        }
        out
    }
    fn eager_chunk(
        old: &mut VecDeque<&OldString>,
        old_filter: &OldFilter,
        value: &mut &[u8],
        sofar: &mut Vec<u8>,
        chains: &mut Chains,
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
                let base = chains.base as usize;
                let min_valid = base + sofar.len().saturating_sub(BACK_WINDOW);
                let mut numbered = chains.head[h] as usize;
                let mut chain = 0;
                while numbered >= min_valid && chain < MAX_CHAIN {
                    let candidate = numbered - base;
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
                    numbered = chains.next[candidate] as usize;
                    chain += 1;
                }
            }

            // Linear scan in old strings; must beat best_sofar to be chosen.
            // The old_filter bloom filter lets us skip the entire scan when the
            // current 4-gram prefix is not present in any old string, and each
            // string's own filter skips that string. A prefix shorter than
            // MIN_MATCH can match in fewer than 4 bytes, so neither applies.
            let sofar_best_len = best_sofar.map_or(0, |(_, l)| l);
            let sofar_back_len = sofar.len().min(BACK_WINDOW);
            let mut old_budget = BACK_WINDOW.saturating_sub(sofar_back_len);
            let mut min_old = sofar_best_len;
            let mut best_old: Option<(u8, usize, usize)> = None; // (back, offset, length)
            let gram = (prefix.len() >= MIN_MATCH).then(|| lz77_hash(prefix));
            if gram.is_none_or(|h| old_filter.test(h)) {
                for (i, old_s) in old.iter().enumerate() {
                    if old_budget == 0 {
                        break;
                    }
                    let bytes = old_s.bytes.as_slice();
                    let use_s = if bytes.len() > old_budget {
                        &bytes[..old_budget]
                    } else {
                        bytes
                    };
                    old_budget = old_budget.saturating_sub(use_s.len());
                    if gram.is_some_and(|h| !old_s.grams.test(h)) {
                        continue;
                    }
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
                chains.next.resize(sofar.len(), 0);
                // Insert all positions that now have MIN_MATCH bytes for the first time.
                // This includes "seam" positions (new_start-(MIN_MATCH-1)..new_start-1)
                // that span old sofar bytes and new match bytes, plus positions within
                // the match itself.
                let first_insert = new_start.saturating_sub(MIN_MATCH - 1);
                let last_insert = sofar.len().saturating_sub(MIN_MATCH);
                for p in first_insert..=last_insert {
                    // Hash needs 4 bytes; skip if near end (e.g. short match < 4 bytes).
                    if p + 4 <= sofar.len() {
                        chains.insert(lz77_hash(&sofar[p..]), p);
                    }
                }
                let chunk = Chunk {
                    literal: literal.into_boxed_slice(),
                    length: length as u8,
                    back,
                    offset,
                };
                shift_old(old, &chunk);
                return Some(chunk);
            }

            // No match: emit literal byte and update hash chain.
            let (&first, rest) = value.split_first()?;
            literal.push(first);
            sofar.push(first);
            chains.next.push(0);
            if sofar.len() >= MIN_MATCH {
                let insert_pos = sofar.len() - MIN_MATCH;
                chains.insert(lz77_hash(&sofar[insert_pos..]), insert_pos);
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
        Normal::encode(&chunks.len(), writer, &mut self.count);
        let mut sentinel = Sentinel::new();
        for chunk in chunks {
            sentinel.encode(writer);
            Normal::encode(&chunk, writer, self);
            self.shift_chunk(&chunk);
        }
        self.push_old(value.to_vec());
    }

    pub fn decode<D: super::EntropyDecoder>(
        &mut self,
        reader: &mut D,
    ) -> Result<Vec<u8>, std::io::Error> {
        let count = <usize as Encode>::decode(reader, &mut self.count)?;
        let mut out = Vec::with_capacity(super::capacity_for::<u8>(count.saturating_mul(5)));
        let mut sentinel = Sentinel::new();
        for _ in 0..count {
            sentinel.decode(reader)?;
            let chunk = <Chunk as Encode>::decode(reader, self)?;
            self.apply_chunk(&chunk, &mut out)?;
        }
        self.push_old_decode(out.clone());
        Ok(out)
    }

    pub async fn decode_async<D: super::AsyncEntropyDecoder>(
        &mut self,
        reader: &mut D,
    ) -> Result<Vec<u8>, std::io::Error> {
        let count = <usize as Encode>::decode_async(reader, &mut self.count).await?;
        let mut out = Vec::with_capacity(super::capacity_for::<u8>(count.saturating_mul(5)));
        let mut sentinel = Sentinel::new();
        for _ in 0..count {
            sentinel.decode_async(reader).await?;
            let chunk = <Chunk as Encode>::decode_async(reader, self).await?;
            self.apply_chunk(&chunk, &mut out)?;
        }
        self.push_old_decode(out.clone());
        Ok(out)
    }

    /// Append what one decoded `chunk` stands for to `out`: its literal, then
    /// its match, copied from earlier in `out` or from a string in `old`.
    ///
    /// The chunk came off the wire, so a corrupt stream can name a match that
    /// does not exist: an `offset` reaching back before the start of `out`, a
    /// `back` past the strings in `old`, or a span running off the end of that
    /// string. The encoder never produces one, and indexing with one would
    /// panic, in release builds too, so each is an error here — checked once,
    /// for both decoders.
    ///
    /// `always`: this is not generic, so otherwise it is compiled once, in this
    /// crate, and called once per chunk by the generic decoders, which are
    /// compiled in the caller's crate. Plain `#[inline]` still left the call in
    /// place, measured +1.8% on `coders`' `compressible` decode; inlined, there
    /// is no detectable difference.
    #[inline(always)]
    fn apply_chunk(&mut self, chunk: &Chunk, out: &mut Vec<u8>) -> Result<(), std::io::Error> {
        out.extend_from_slice(&chunk.literal);
        let length = chunk.length as usize;
        if length == 0 {
            // Nothing to do here.
        } else if chunk.back == 0 {
            // We are repeating our own string.  In this case offset
            // counts *backwards* and must be >= 1 so we shift it.
            let start = chunk
                .offset
                .checked_add(1)
                .and_then(|distance| out.len().checked_sub(distance))
                .ok_or_else(|| {
                    std::io::Error::other("corrupt stream: Lz77 match starts before its string")
                })?;
            out.reserve(super::capacity_for::<u8>(length));
            for i in start..start + length {
                out.push(out[i]);
            }
        } else {
            if chunk.back as usize > self.old.len() {
                return Err(std::io::Error::other(
                    "corrupt stream: Lz77 match names a string not yet decoded",
                ));
            }
            self.shift_chunk(chunk);
            let matched = chunk
                .offset
                .checked_add(length)
                .and_then(|end| self.old[0].bytes.get(chunk.offset..end))
                .ok_or_else(|| {
                    std::io::Error::other("corrupt stream: Lz77 match runs past its string")
                })?;
            out.extend_from_slice(matched);
        }
        Ok(())
    }
}

/// Whenever we use an old string, we move it to the front of the queue. This
/// keeps "useful" strings from falling off the back, and also makes smaller
/// `back` values more common.
fn shift_old<T>(old: &mut VecDeque<T>, Chunk { back, .. }: &Chunk) {
    if *back > 0 {
        if let Some(string) = old.remove(*back as usize - 1) {
            old.push_front(string);
        }
    }
}

/// What one value leaves in the chains must not change what the next finds: a
/// stale entry read as a position would either match bytes of the wrong value or
/// cut a chain short. So the chunks must be what fresh chains give, including
/// when the numbering runs out and starts again.
#[test]
fn chains_kept_between_values_do_not_change_the_chunks() {
    let values: [&[u8]; 4] = [
        b"abcdefgh abcdefgh abcdefgh",
        b"abcdefgh xyzxyzxyz",
        b"",
        b"xyzxyzxyz abcdefgh abcdefgh",
    ];
    let mut ctx = Lz77::default();
    let mut kept = Chains::default();
    for start in [0, u32::MAX - 40] {
        kept.base = start;
        for value in values {
            let fresh = ctx.eager_with(value, &mut Chains::default());
            assert_eq!(ctx.eager_with(value, &mut kept), fresh, "{value:?}");
            ctx.push_old(value.to_vec());
        }
    }
}

/// A filter that wrongly ruled a string out would silently lose its matches, and
/// nothing else would notice: the output would still decode, just bigger.
#[test]
fn gram_filter_never_rules_out_a_gram_the_string_holds() {
    let mut x = 0x1234_5678_9abc_def0_u64;
    let mut byte = move || {
        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
        (x >> 56) as u8
    };
    // Past `MAX_WORDS` at the top, and shorter than one gram at the bottom.
    for len in [
        0,
        1,
        3,
        4,
        5,
        63,
        64,
        65,
        1000,
        4 * GramFilter::MAX_WORDS,
        100_000,
    ] {
        let bytes: Vec<u8> = (0..len).map(|_| byte()).collect();
        let filter = GramFilter::new(&bytes);
        assert!(filter.0.len() <= GramFilter::MAX_WORDS);
        for gram in bytes.windows(4) {
            assert!(filter.test(lz77_hash(gram)), "len {len}");
        }
    }
    // The decode side's empty filter must rule nothing out, not panic.
    assert!(GramFilter::default().test(lz77_hash(b"abcd")));
    // And a filter is worth having only if it does rule things out.
    let filter = GramFilter::new(b"the quick brown fox");
    assert!(!filter.test(lz77_hash(b"jump")));
}

/// Every match in an old string must still be found through the filters, so
/// look for one at every offset: a search that skipped a string it should have
/// read would lose some of them, for whichever 4-grams it got wrong.
#[test]
fn old_strings_are_matched_through_the_filters() {
    let wanted: &[u8] = b"the quick brown fox jumps over one lazy dog, then naps";
    let mut ctx = Lz77::default();
    ctx.push_old(b"nothing to see in this one".to_vec());
    ctx.push_old(wanted.to_vec());
    ctx.push_old(b"and nothing in this one either".to_vec());
    for offset in 0..wanted.len() - MIN_MATCH {
        let length = (wanted.len() - offset).min(12);
        assert_eq!(
            ctx.eager(&wanted[offset..offset + length]),
            vec![Chunk {
                literal: Box::new([]),
                length: length as u8,
                back: 2,
                offset,
            }],
            "offset {offset}"
        );
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
            Normal::encode(&chunk, &mut millibits_of_literals, &mut ctx);
        }
        assert_eq!(millibits_of_literals, super::Millibits::new(22988));
        let mb_of_vec = millibits(&Lz77::default().eager(b"aaa"));
        assert_eq!(mb_of_vec, super::Millibits::new(25988));
        let mb_of_string = millibits(&b"aaa".to_vec());
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

/// Every way a corrupt chunk can name a match that does not exist must be
/// rejected rather than indexed with: each of these used to panic, and the
/// ones that index `old` in release builds as well as debug. The boundary next
/// to each is legal and must still decode, so no check is off by one.
#[test]
fn decode_rejects_matches_that_do_not_exist() {
    use super::{assert_rejected, Range};
    use crate::Encoded;
    fn chunk(literal: &[u8], length: u8, back: u8, offset: usize) -> Chunk {
        Chunk {
            literal: literal.into(),
            length,
            back,
            offset,
        }
    }
    /// Code each value's chunks against the decoder's own context.
    fn code(values: &[&[Chunk]], writer: &mut Range) {
        let mut ctx = Lz77::default();
        for chunks in values {
            Normal::encode(&chunks.len(), writer, &mut ctx.count);
            for c in *chunks {
                Normal::encode(c, writer, &mut ctx);
            }
        }
    }
    fn decode<T: Encode>(values: &[&[Chunk]]) -> Option<T> {
        let mut writer = Range::default();
        code(values, &mut writer);
        super::decode(&writer.into_vec())
    }
    type One = Encoded<Vec<u8>, Compressible>;
    type Two = [Encoded<Vec<u8>, Compressible>; 2];

    // A repeat of the value's own bytes: `offset` counts back from the last
    // byte, so from "ab" an offset of 1 starts at the beginning and 2 is before
    // it, as is any offset at all with nothing decoded yet.
    let decoded = decode::<One>(&[&[chunk(b"ab", 3, 0, 1)]]);
    assert_eq!(decoded.map(Encoded::value), Some(b"ababa".to_vec()));
    assert_rejected::<One>(|w| code(&[&[chunk(b"ab", 3, 0, 2)]], w), "before the start");
    assert_rejected::<One>(|w| code(&[&[chunk(b"", 3, 0, 0)]], w), "repeat of nothing");

    // A match in an earlier value: there is none before the first.
    assert_rejected::<One>(|w| code(&[&[chunk(b"", 3, 1, 0)]], w), "no earlier value");

    // A match in an earlier value must lie inside it.
    let earlier: &[Chunk] = &[chunk(b"abc", 0, 0, 0)];
    let decoded = decode::<Two>(&[earlier, &[chunk(b"", 2, 1, 1)]]);
    assert_eq!(
        decoded.map(|[a, b]| [a.value(), b.value()]),
        Some([b"abc".to_vec(), b"bc".to_vec()])
    );
    assert_rejected::<Two>(
        |w| code(&[earlier, &[chunk(b"", 3, 1, 1)]], w),
        "past the end of an earlier value",
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
    // Almost every offset fails within a few bytes, and calling `memcmp` once per
    // offset to find that out was most of the cost of encoding. So compare the
    // first four bytes as a word, which every prefix tried here shares however
    // long it has grown, and then the last, which is the byte that would make
    // this the longest match so far and is the likeliest of the rest to differ.
    let first_word = prefix.first_chunk().map(|word| u32::from_ne_bytes(*word));
    let mut best = None;
    for offset in 0..(haystack.len() + 1).saturating_sub(prefix.len()) {
        let here = &haystack[offset..];
        if let (Some(word), Some(here_word)) = (first_word, here.first_chunk()) {
            if u32::from_ne_bytes(*here_word) != word {
                continue;
            }
        }
        if here.get(prefix.len() - 1) == prefix.last() && here.starts_with(prefix) {
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
    fn encode<E: super::EntropyCoder>(value: &Self, writer: &mut E, ctx: &mut Self::Context) {
        let Chunk {
            literal,
            length,
            back,
            offset,
        } = value;
        Normal::encode(literal, writer, &mut ctx.literal);
        Small::encode(length, writer, &mut ctx.length);
        if *length > 0 {
            Small::encode(back, writer, &mut ctx.back);
            if *back == 0 {
                Normal::encode(offset, writer, &mut ctx.self_offset);
            } else {
                Normal::encode(offset, writer, &mut ctx.offset);
            }
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let literal = <Box<[u8]> as Encode>::decode(reader, &mut ctx.literal)?;
        let length = <u8 as Encode<Small>>::decode(reader, &mut ctx.length)?;
        if length > 0 {
            let back = Small::decode(reader, &mut ctx.back)?;
            let offset = if back == 0 {
                <usize as Encode>::decode(reader, &mut ctx.self_offset)?
            } else {
                <usize as Encode>::decode(reader, &mut ctx.offset)?
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

    /// Unbounded: a literal run of arbitrary length.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Chunk, std::io::Error> {
        let literal = <Box<[u8]> as Encode>::decode_async(reader, &mut ctx.literal).await?;
        let length = <u8 as Encode<Small>>::decode_async(reader, &mut ctx.length).await?;
        if length > 0 {
            let back = <u8 as Encode<Small>>::decode_async(reader, &mut ctx.back).await?;
            let offset = if back == 0 {
                <usize as Encode>::decode_async(reader, &mut ctx.self_offset).await?
            } else {
                <usize as Encode>::decode_async(reader, &mut ctx.offset).await?
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

impl Encode<Compressible> for Vec<u8> {
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

    /// Length-driven: an arbitrary number of bytes.
    const MAX_BYTES: usize = usize::MAX;

    async fn decode_awaiting<D: super::AsyncEntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<u8>, std::io::Error> {
        ctx.decode_async(reader).await
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
            millibits(&normal),
            super::encoded_bits!(value.iter().map(|s| s.to_vec()).collect::<Vec<Vec<u8>>>()),
            millibits(&small),
            super::encoded_bits!(value
                .iter()
                .map(|s| Encoded::<_, Compressible>::new(s.to_vec()))
                .collect::<Vec<_>>())
        )
    }
    expect!["normal: 8989 bits, small: 7123 bits"]
        .assert_eq(&compare_small_bits(COMPRESSIBLE_TEXT));

    expect!["1000 mb"].assert_eq(&millibits(&true).to_string());
    expect!["4593 mb"].assert_eq(&millibits(&'a').to_string());
    expect!["14000 mb"].assert_eq(
        &millibits(&Chunk {
            literal: b"a".to_vec().into_boxed_slice(),
            length: 0,
            back: 0,
            offset: 0,
        })
        .to_string(),
    );
    expect!["11256 mb"].assert_eq(
        &millibits(&Chunk {
            literal: Box::new([]),
            back: 0,
            offset: 0,
            length: 2,
        })
        .to_string(),
    );
    expect!["normal: 3 bits, small: 1 bits"].assert_eq(&compare_small_bits(b""));
    expect!["normal: 11 bits, small: 16 bits"].assert_eq(&compare_small_bits(b"a"));
    expect!["normal: 17 bits, small: 22 bits"].assert_eq(&compare_small_bits(b"aa"));
    expect!["normal: 20 bits, small: 25 bits"].assert_eq(&compare_small_bits(b"aaa"));
    expect!["normal: 24 bits, small: 29 bits"].assert_eq(&compare_small_bits(b"aaaa"));
    expect!["normal: 31 bits, small: 37 bits"].assert_eq(&compare_small_bits(b"aaaaaaaa"));
    expect!["normal: 36 bits, small: 41 bits"].assert_eq(&compare_small_bits(b"hello"));
    expect!["normal: 122 bits, small: 118 bits"]
        .assert_eq(&compare_small_bits(b"hello world hello wood"));
    expect!["normal: 127 bits, small: 100 bits"]
        .assert_eq(&compare_small_bits(b"hello world hello world"));
    expect!["normal: 412 bits, small: 417 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.",
    ));
    expect!["normal: 1538 bits, small: 836 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?",
    ));
    expect!["normal: 1609 bits, small: 1006 bits"].assert_eq(&compare_small_bits(
        b"This sentence is pretty long and seems reflective of ordinary English to me.
           If I duplicate this sentence then I should get better compression, right?
           This sentence is pretty long but seems reflective of ordinary English to me.
           If I duplicate this sentence with tiny changes then I should get ok compression, right?",
    ));

    expect!["normal: Millibits(3000) (3 bits), small: Millibits(3000) (3 bits)"]
        .assert_eq(&compare_vecs(&[]));
    expect!["11000 mb"].assert_eq(&millibits(&b"h".to_vec()).to_string());

    let s = b"aaaaaaaaaaaaaaaa".to_vec();
    expect!["39549 mb"].assert_eq(&millibits(&s).to_string());
    expect!["40"].assert_eq(&encoded_bits!(s.clone()));

    let s = b"hello world this is a string".to_vec();
    expect!["165201 mb"].assert_eq(&millibits(&s).to_string());
    expect!["165"].assert_eq(&encoded_bits!(s.clone()));

    expect!["normal: Millibits(14000) (14 bits), small: Millibits(19264) (19 bits)"]
        .assert_eq(&compare_vecs(&[b"h"]));
    expect!["normal: Millibits(76841) (77 bits), small: Millibits(82105) (82 bits)"]
        .assert_eq(&compare_vecs(&[b"hello world"]));
    expect!["normal: Millibits(128206) (128 bits), small: Millibits(99082) (99 bits)"]
        .assert_eq(&compare_vecs(&[b"hello world", b"hello world"]));
    expect!["normal: Millibits(172498) (173 bits), small: Millibits(109043) (109 bits)"].assert_eq(
        &compare_vecs(&[b"hello world", b"hello world", b"hello world"]),
    );
    expect!["normal: Millibits(262517) (263 bits), small: Millibits(144296) (144 bits)"].assert_eq(
        &compare_vecs(&[
            b"hello world",
            b"hello world",
            b"hello world",
            b"hello world hello world",
        ]),
    );
    expect!["normal: Millibits(496104) (496 bits), small: Millibits(423048) (423 bits)"].assert_eq(
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
