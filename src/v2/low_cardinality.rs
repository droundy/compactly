use super::{Encode, EncodingStrategy, LowCardinality};
use crate::Small;
use std::{borrow::Borrow, collections::HashMap, hash::Hash, ops::Deref, rc::Rc, sync::Arc};

#[cfg(test)]
use expect_test::expect;

#[derive(Clone)]
pub struct CacheContext<T: Encode + Clone + Hash + PartialEq + Eq> {
    cached: HashMap<T, usize>,
    cache: Vec<T>,
    is_cached: <bool as Encode>::Context,
    context: T::Context,
    index: <usize as Encode>::Context,
}

impl<T: Encode + Clone + Hash + PartialEq + Eq> Default for CacheContext<T> {
    #[inline]
    fn default() -> Self {
        Self {
            cached: HashMap::new(),
            cache: Vec::new(),
            is_cached: Default::default(),
            context: Default::default(),
            index: Default::default(),
        }
    }
}

macro_rules! impl_low_cardinality {
    ($t:ty, $mod:ident) => {
        mod $mod {
            use super::{CacheContext, Encode, EncodingStrategy, LowCardinality};
            impl EncodingStrategy<$t> for LowCardinality {
                type Context = CacheContext<$t>;
                #[inline]
                fn encode<E: super::super::EntropyCoder>(
                    value: &$t,
                    writer: &mut E,
                    ctx: &mut Self::Context,
                ) {
                    let looked_up = ctx.cached.get(value).copied();
                    looked_up.is_some().encode(writer, &mut ctx.is_cached);
                    if let Some(idx) = looked_up {
                        idx.encode(writer, &mut ctx.index)
                    } else {
                        ctx.cached.insert(value.clone(), ctx.cached.len());
                        value.encode(writer, &mut ctx.context)
                    }
                }
                #[inline]
                fn decode<D: super::super::EntropyDecoder>(
                    reader: &mut D,
                    ctx: &mut Self::Context,
                ) -> Result<$t, std::io::Error> {
                    let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
                    if is_cached {
                        let idx = usize::decode(reader, &mut ctx.index)?;
                        ctx.cache
                            .get(idx)
                            .cloned()
                            .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
                    } else {
                        let value = <$t>::decode(reader, &mut ctx.context)?;
                        ctx.cache.push(value.clone());
                        Ok(value)
                    }
                }
            }
        }
    };
}

impl_low_cardinality!(Vec<u8>, bytes);
impl_low_cardinality!(u16, mod_u16);
impl_low_cardinality!(u32, mod_u32);
impl_low_cardinality!(u64, mod_u64);

// Arc<str>/Rc<str>/String share one dictionary-based implementation, generic
// over the ref-counted pointer type `P` (`Arc<str>` for `Arc<str>` fields,
// `Rc<str>` for both `Rc<str>` fields and -- internally -- `String` fields):
// rather than a plain content-addressed cache, a miss is encoded relative to
// the dictionary of strings seen so far — the longest prefix and longest
// suffix any dictionary member shares with the new string (each a length +
// index), plus only the literal "middle" characters not covered by either
// match. See `crate::StringSet` for the O(log N) prefix/suffix lookups this
// relies on. `String` gets the same miss-path savings as `Arc<str>`/`Rc<str>`,
// though (being an owned, non-shared type) it still pays a fresh allocation
// on every cache *hit*, unlike its ref-counted siblings.

/// A prefix/suffix match length on the wire: 0 means "no match"; any actual
/// length `L >= 2` is written as `L - 1`. Length 1 is never encoded (a
/// length plus an index costs at least as much as the one character it
/// would save), so this keeps the transmitted values contiguous
/// (0, 1, 2, …) instead of leaving a permanently-unused gap at 1.
#[inline]
fn encode_match_len<E: super::EntropyCoder>(
    len: usize,
    writer: &mut E,
    ctx: &mut <Small as EncodingStrategy<u16>>::Context,
) {
    debug_assert_ne!(len, 1, "length-1 matches must be rejected before encoding");
    let wire = if len == 0 { 0 } else { (len - 1) as u16 };
    Small::encode(&wire, writer, ctx);
}

#[inline]
fn decode_match_len<D: super::EntropyDecoder>(
    reader: &mut D,
    ctx: &mut <Small as EncodingStrategy<u16>>::Context,
) -> Result<usize, std::io::Error> {
    let wire: u16 = Small::decode(reader, ctx)?;
    Ok(if wire == 0 { 0 } else { wire as usize + 1 })
}

#[derive(Clone)]
pub struct DictContext<P> {
    dict: crate::StringSet<P>,
    is_cached: <bool as Encode>::Context,
    index: <Small as EncodingStrategy<usize>>::Context,
    prefix_len: <Small as EncodingStrategy<u16>>::Context,
    prefix_index: <Small as EncodingStrategy<usize>>::Context,
    suffix_len: <Small as EncodingStrategy<u16>>::Context,
    suffix_index: <Small as EncodingStrategy<usize>>::Context,
    middle_len: <Small as EncodingStrategy<usize>>::Context,
    chars: <char as Encode>::Context,
}

// Hand-written (not derived) so this doesn't require `P: Default` -- neither
// `Arc<str>` nor `Rc<str>` implement it, since `str` is unsized.
impl<P> Default for DictContext<P> {
    fn default() -> Self {
        Self {
            dict: Default::default(),
            is_cached: Default::default(),
            index: Default::default(),
            prefix_len: Default::default(),
            prefix_index: Default::default(),
            suffix_len: Default::default(),
            suffix_index: Default::default(),
            middle_len: Default::default(),
            chars: Default::default(),
        }
    }
}

/// Trait bound shorthand for the ref-counted pointer types `DictContext<P>`
/// works with (`Arc<str>`, `Rc<str>`): the `StringSet`/`Treap` bound, plus
/// what's needed to reconstruct a value on decode.
trait StrPtr: Clone + Ord + Hash + Borrow<str> + Deref<Target = str> {
    fn from_str(s: &str) -> Self;
}
impl StrPtr for Arc<str> {
    fn from_str(s: &str) -> Self {
        Arc::from(s)
    }
}
impl StrPtr for Rc<str> {
    fn from_str(s: &str) -> Self {
        Rc::from(s)
    }
}

/// The exact-match fast path, shared by `Arc<str>`/`Rc<str>`/`String`: O(1)
/// via `StringSet`'s internal hash map. Returns `true` (having already
/// written the `is_cached` bit and index) if `value` was found; the caller
/// should encode nothing further in that case. Otherwise writes the `false`
/// bit and the caller must proceed to [`encode_miss`].
fn encode_exact_or_bit<P: StrPtr, E: super::EntropyCoder>(
    value: &str,
    writer: &mut E,
    ctx: &mut DictContext<P>,
) -> bool {
    if let Some(idx) = ctx.dict.get_exact(value) {
        true.encode(writer, &mut ctx.is_cached);
        Small::encode(&idx, writer, &mut ctx.index);
        return true;
    }
    false.encode(writer, &mut ctx.is_cached);
    false
}

/// Encode a confirmed miss: insert `value` into the dictionary *and* find
/// its best prefix/suffix matches against the existing members, in exactly
/// one tree walk per ordering (see `crate::string_set::treap`), then encode
/// those matches plus the literal "middle" span.
fn encode_miss<P: StrPtr, E: super::EntropyCoder>(
    value: &P,
    writer: &mut E,
    ctx: &mut DictContext<P>,
) {
    let miss = ctx.dict.insert_new(value);
    let value: &str = value;

    let (mut prefix_len, mut prefix_idx) = match miss.prefix {
        Some((idx, matched)) if !matched.is_empty() => (matched.len(), Some(idx)),
        _ => (0, None),
    };
    // `StringSet` already guarantees `prefix_len` lands on a char
    // boundary in `value`; only capping at u16::MAX can change that, so
    // only re-trim in that (rare) case, not on every call.
    if prefix_len > u16::MAX as usize {
        prefix_len = u16::MAX as usize;
        while !value.is_char_boundary(prefix_len) {
            prefix_len -= 1;
        }
    }
    // A length-1 match costs a length plus an index -- at least as much
    // as just encoding the one character directly -- so it's never
    // worth it; treat as no match. Do this before computing the suffix
    // match, so a rejected prefix byte is free for the suffix to claim.
    if prefix_len <= 1 {
        prefix_len = 0;
        prefix_idx = None;
    }
    encode_match_len(prefix_len, writer, &mut ctx.prefix_len);
    if let Some(idx) = prefix_idx {
        Small::encode(&idx, writer, &mut ctx.prefix_index);
    }

    let (mut suffix_len, mut suffix_idx) = match miss.suffix {
        Some((idx, matched)) if !matched.is_empty() => (matched.len(), Some(idx)),
        _ => (0, None),
    };
    // Prefix and suffix matches can overlap on short strings; favor the
    // prefix. Then cap at u16::MAX. Only re-trim to a char boundary if
    // one of those two adjustments actually changed the length --
    // `StringSet` already guarantees the untouched length is safe.
    let mut suffix_adjusted = false;
    if prefix_len + suffix_len > value.len() {
        suffix_len = value.len() - prefix_len;
        suffix_adjusted = true;
    }
    if suffix_len > u16::MAX as usize {
        suffix_len = u16::MAX as usize;
        suffix_adjusted = true;
    }
    if suffix_adjusted {
        while !value.is_char_boundary(value.len() - suffix_len) {
            suffix_len -= 1;
        }
    }
    // Same length-1 rejection as the prefix, above.
    if suffix_len <= 1 {
        suffix_len = 0;
        suffix_idx = None;
    }
    encode_match_len(suffix_len, writer, &mut ctx.suffix_len);
    if let Some(idx) = suffix_idx {
        Small::encode(&idx, writer, &mut ctx.suffix_index);
    }

    let middle = &value[prefix_len..value.len() - suffix_len];
    Small::encode(&middle.chars().count(), writer, &mut ctx.middle_len);
    for c in middle.chars() {
        c.encode(writer, &mut ctx.chars);
    }
}

fn decode_generic<P: StrPtr, D: super::EntropyDecoder>(
    reader: &mut D,
    ctx: &mut DictContext<P>,
) -> Result<P, std::io::Error> {
    let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
    if is_cached {
        let idx: usize = Small::decode(reader, &mut ctx.index)?;
        return ctx
            .dict
            .get(idx)
            .cloned()
            .ok_or_else(|| std::io::Error::other("bad low_cardinality index"));
    }

    let prefix_len = decode_match_len(reader, &mut ctx.prefix_len)?;
    let mut out = String::new();
    if prefix_len > 0 {
        let idx: usize = Small::decode(reader, &mut ctx.prefix_index)?;
        let entry = ctx
            .dict
            .get(idx)
            .ok_or_else(|| std::io::Error::other("bad low_cardinality prefix index"))?;
        // The wire-supplied length is unrelated to whatever entry the index
        // resolved to, so slice fallibly: `get` rejects both out-of-bounds
        // and non-char-boundary lengths.
        let prefix = entry
            .get(..prefix_len)
            .ok_or_else(|| std::io::Error::other("bad low_cardinality prefix length"))?;
        out.push_str(prefix);
    }

    let suffix_len = decode_match_len(reader, &mut ctx.suffix_len)?;
    let suffix_idx = if suffix_len > 0 {
        Some(Small::decode(reader, &mut ctx.suffix_index)?)
    } else {
        None
    };

    let middle_len: usize = Small::decode(reader, &mut ctx.middle_len)?;
    for _ in 0..middle_len {
        out.push(char::decode(reader, &mut ctx.chars)?);
    }

    if let Some(idx) = suffix_idx {
        let entry = ctx
            .dict
            .get(idx)
            .ok_or_else(|| std::io::Error::other("bad low_cardinality suffix index"))?;
        // As with the prefix above: `checked_sub` rejects lengths longer
        // than the entry, `get` rejects non-char-boundary starts.
        let suffix = entry
            .len()
            .checked_sub(suffix_len)
            .and_then(|start| entry.get(start..))
            .ok_or_else(|| std::io::Error::other("bad low_cardinality suffix length"))?;
        out.push_str(suffix);
    }

    let value = P::from_str(&out);
    ctx.dict.push(value.clone());
    Ok(value)
}

impl EncodingStrategy<Arc<str>> for LowCardinality {
    type Context = DictContext<Arc<str>>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Arc<str>, writer: &mut E, ctx: &mut Self::Context) {
        if encode_exact_or_bit(value, writer, ctx) {
            return;
        }
        encode_miss(value, writer, ctx);
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Arc<str>, std::io::Error> {
        decode_generic(reader, ctx)
    }
}

impl EncodingStrategy<Rc<str>> for LowCardinality {
    type Context = DictContext<Rc<str>>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Rc<str>, writer: &mut E, ctx: &mut Self::Context) {
        if encode_exact_or_bit(value, writer, ctx) {
            return;
        }
        encode_miss(value, writer, ctx);
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Rc<str>, std::io::Error> {
        decode_generic(reader, ctx)
    }
}

// `String` reuses the `Rc<str>` dictionary internally: a miss allocates a
// fresh `Rc<str>` to insert (unavoidable -- `String` isn't itself shareable),
// but still benefits from prefix/suffix matching. A hit still costs a fresh
// `String` allocation on decode (unlike `Arc<str>`/`Rc<str>`, `String` can't
// share the cached buffer), so prefer `Arc<str>`/`Rc<str>` fields when you
// can; see `LowCardinality`'s docs.
impl EncodingStrategy<String> for LowCardinality {
    type Context = DictContext<Rc<str>>;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &String, writer: &mut E, ctx: &mut Self::Context) {
        if encode_exact_or_bit(value, writer, ctx) {
            return;
        }
        let value: Rc<str> = Rc::from(value.as_str());
        encode_miss(&value, writer, ctx);
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<String, std::io::Error> {
        decode_generic(reader, ctx).map(|rc: Rc<str>| rc.to_string())
    }
}

impl<T> EncodingStrategy<Vec<T>> for LowCardinality
where
    T: Encode,
    LowCardinality: EncodingStrategy<T>,
{
    type Context = (
        <usize as Encode>::Context,
        <LowCardinality as EncodingStrategy<T>>::Context,
    );
    fn encode<E: super::EntropyCoder>(value: &Vec<T>, writer: &mut E, ctx: &mut Self::Context) {
        value.len().encode(writer, &mut ctx.0);
        for v in value {
            LowCardinality::encode(v, writer, &mut ctx.1);
        }
    }
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Vec<T>, std::io::Error> {
        let n = usize::decode(reader, &mut ctx.0)?;
        let mut x = Vec::with_capacity(n);
        for _ in 0..n {
            x.push(LowCardinality::decode(reader, &mut ctx.1)?);
        }
        Ok(x)
    }
}

#[test]
fn low_cardinality() {
    use super::estimated_bits;
    use crate::Encoded;

    let strings = [
        b"hello world, this is the very first string".to_vec(),
        b"This is a second string, which is like unto the first, and yet quite different".to_vec(),
    ];
    let mut v = Vec::new();
    for i in 0..1024 {
        v.push(if i % 3 == 0 {
            strings[0].clone()
        } else {
            strings[1].clone()
        });
    }
    let low = v
        .iter()
        .cloned()
        .map(Encoded::<_, LowCardinality>::new)
        .collect::<Vec<_>>();

    expect!["284775"].assert_eq(&estimated_bits!(v.clone()));
    expect!["1673"].assert_eq(&estimated_bits!(low.clone()));
    expect!["611"].assert_eq(&estimated_bits!(strings.clone().to_vec()));
    expect!["612"].assert_eq(&estimated_bits!(strings
        .iter()
        .cloned()
        .map(Encoded::<_, LowCardinality>::new)
        .collect::<Vec<_>>()));
}

#[test]
fn arc_str_prefix_suffix_round_trip() {
    fn round_trip(values: Vec<Arc<str>>) {
        let encoded = super::encode(&values);
        let decoded: Vec<Arc<str>> = super::decode(&encoded).unwrap();
        assert_eq!(values, decoded);
    }

    // No dictionary overlap at all.
    round_trip(vec![
        Arc::from("alpha"),
        Arc::from("bravo"),
        Arc::from("charlie"),
    ]);

    // Exact repeats: the existing is_cached fast path.
    round_trip(vec![
        Arc::from("repeat"),
        Arc::from("repeat"),
        Arc::from("repeat"),
    ]);

    // Prefix-only match.
    round_trip(vec![Arc::from("hello world"), Arc::from("hello there")]);

    // Queries that are prefixes/suffixes of existing entries but whose only
    // matches have length 1, which is rejected as not worth encoding: these
    // take the no-match wire path and must not be mistaken for cache hits.
    round_trip(vec![Arc::from("ab"), Arc::from("a"), Arc::from("b")]);

    // Suffix-only match, where the query is *entirely* a suffix of an
    // existing entry (empty middle, and another exact-length-mismatch case
    // that must not be mistaken for a cache hit).
    round_trip(vec![Arc::from("xyzab"), Arc::from("ab")]);

    // Prefix match plus an exact repeat mixed in ("hello there" shares the
    // prefix "hello " with "hello world" but no suffix with anything).
    round_trip(vec![
        Arc::from("hello world"),
        Arc::from("goodbye world"),
        Arc::from("hello there"),
        Arc::from("hello world"),
    ]);

    // Both prefix and suffix match simultaneously, against *different*
    // dictionary entries: "abcab" matches prefix "abc" of "abcxy" and
    // suffix "cab" of "xxcab". 3 + 3 > len 5, so this also exercises the
    // overlap adjustment (suffix trimmed to "ab") and an empty middle.
    round_trip(vec![
        Arc::from("abcxy"),
        Arc::from("xxcab"),
        Arc::from("abcab"),
    ]);

    // UTF-8 boundary case: dictionary entries share only a leading byte of a
    // multi-byte character, which must not be sliced on its own.
    round_trip(vec![
        Arc::from("éxyz"),
        Arc::from("èabc"),
        Arc::from("xyzé"),
        Arc::from("abcè"),
    ]);

    // A shared prefix longer than u16::MAX bytes: the match must be capped,
    // not incorrect (the excess just isn't matched).
    let long_prefix = "a".repeat(u16::MAX as usize + 1);
    round_trip(vec![
        Arc::from(format!("{long_prefix}_one").as_str()),
        Arc::from(format!("{long_prefix}_two").as_str()),
    ]);

    // Same for a shared suffix longer than u16::MAX bytes.
    let long_suffix = "b".repeat(u16::MAX as usize + 1);
    round_trip(vec![
        Arc::from(format!("one_{long_suffix}").as_str()),
        Arc::from(format!("two_{long_suffix}").as_str()),
    ]);
}

/// Decoding corrupted bytes must fail cleanly (`Ok` with wrong contents or
/// `Err`), never panic: regression test for unchecked wire-supplied
/// prefix/suffix lengths being used to slice a dictionary entry.
#[test]
fn arc_str_corrupted_input_does_not_panic() {
    let values: Vec<Arc<str>> = vec![
        Arc::from("hello world"),
        Arc::from("hello there"),
        Arc::from("goodbye world"),
        Arc::from("hello worldwide"),
    ];
    let encoded = super::encode(&values);
    for byte in 0..encoded.len() {
        for bit in 0..8 {
            let mut corrupted = encoded.clone();
            corrupted[byte] ^= 1 << bit;
            let _ = super::decode::<Vec<Arc<str>>>(&corrupted);
        }
    }
}

#[test]
fn arc_str_prefix_suffix_size() {
    use super::estimated_bits;

    // Shared prefixes and suffixes typical of dictionary-like data (URLs).
    let values: Vec<Arc<str>> = vec![
        Arc::from("https://example.com/a"),
        Arc::from("https://example.com/b"),
        Arc::from("https://example.com/c"),
        Arc::from("https://example.org/a"),
    ];
    expect!["226"].assert_eq(&estimated_bits!(values));
}

// `Rc<str>` and `String` share the exact same dictionary algorithm as
// `Arc<str>` (see `StrPtr`/`encode_miss`/`decode_generic` above), so their
// prefix/suffix/UTF-8-boundary/overlap edge cases are already covered by
// `arc_str_prefix_suffix_round_trip` above and by `StringSet`'s own tests in
// `src/string_set.rs`. These just confirm the type-specific wiring (the
// `StrPtr` impls, and `String`'s `Rc<str>`-internally / `to_string()` glue)
// round-trips and gets the same size benefit.

#[test]
fn rc_str_prefix_suffix_round_trip() {
    use std::rc::Rc;
    let values: Vec<Rc<str>> = vec![
        Rc::from("hello world"),
        Rc::from("hello there"),
        Rc::from("goodbye world"),
        Rc::from("hello world"), // exact repeat
    ];
    let encoded = super::encode(&values);
    let decoded: Vec<Rc<str>> = super::decode(&encoded).unwrap();
    assert_eq!(values, decoded);
}

#[test]
fn string_low_cardinality_prefix_suffix_round_trip() {
    use crate::Encoded;

    let values: Vec<Encoded<String, LowCardinality>> = [
        "hello world",
        "hello there",
        "goodbye world",
        "hello world", // exact repeat
    ]
    .into_iter()
    .map(|s| Encoded::new(s.to_string()))
    .collect();
    let encoded = super::encode(&values);
    let decoded: Vec<Encoded<String, LowCardinality>> = super::decode(&encoded).unwrap();
    assert_eq!(values, decoded);
}

#[test]
fn string_low_cardinality_gets_prefix_suffix_size_benefit() {
    use super::estimated_bits;
    use crate::Encoded;

    // Same URL-like corpus as `arc_str_prefix_suffix_size`, but through
    // `LowCardinality`-on-`String` (internally `Rc<str>`): should benefit
    // the same way from the shared prefix/suffix.
    let values: Vec<Encoded<String, LowCardinality>> = [
        "https://example.com/a",
        "https://example.com/b",
        "https://example.com/c",
        "https://example.org/a",
    ]
    .into_iter()
    .map(|s| Encoded::new(s.to_string()))
    .collect();
    expect!["226"].assert_eq(&estimated_bits!(values));
}
