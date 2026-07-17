use super::{Encode, EncodingStrategy, LowCardinality};
use crate::Small;
use std::{collections::HashMap, hash::Hash, rc::Rc, sync::Arc};

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

impl_low_cardinality!(String, string);
impl_low_cardinality!(Vec<u8>, bytes);
impl_low_cardinality!(u16, mod_u16);
impl_low_cardinality!(u32, mod_u32);
impl_low_cardinality!(u64, mod_u64);

// Arc<str> needs its own context: rather than a plain content-addressed
// cache, a miss is encoded relative to the dictionary of strings seen so
// far — the longest prefix and longest suffix any dictionary member shares
// with the new string (each a length + index), plus only the literal
// "middle" characters not covered by either match. See
// `crate::StringSet` for the O(log N) prefix/suffix lookups this relies on.

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

#[derive(Default, Clone)]
pub struct ArcStrCacheContext {
    dict: crate::StringSet,
    is_cached: <bool as Encode>::Context,
    index: <Small as EncodingStrategy<usize>>::Context,
    prefix_len: <Small as EncodingStrategy<u16>>::Context,
    prefix_index: <Small as EncodingStrategy<usize>>::Context,
    suffix_len: <Small as EncodingStrategy<u16>>::Context,
    suffix_index: <Small as EncodingStrategy<usize>>::Context,
    middle_len: <Small as EncodingStrategy<usize>>::Context,
    chars: <char as Encode>::Context,
}

impl EncodingStrategy<Arc<str>> for LowCardinality {
    type Context = ArcStrCacheContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Arc<str>, writer: &mut E, ctx: &mut Self::Context) {
        if let Some(idx) = ctx.dict.get_exact(value) {
            true.encode(writer, &mut ctx.is_cached);
            return Small::encode(&idx, writer, &mut ctx.index);
        }
        false.encode(writer, &mut ctx.is_cached);

        // Inserts `value` into the dictionary *and* finds its best
        // prefix/suffix matches against the existing members, in exactly
        // one tree walk per ordering (see `crate::string_set::treap`).
        let miss = ctx.dict.insert_new(value);

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
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Arc<str>, std::io::Error> {
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
            out.push_str(&entry[..prefix_len]);
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
            out.push_str(&entry[entry.len() - suffix_len..]);
        }

        let value: Arc<str> = Arc::from(out.as_str());
        ctx.dict.push(value.clone());
        Ok(value)
    }
}

#[derive(Default, Clone)]
pub struct RcStrCacheContext {
    cached: HashMap<Rc<str>, usize>,
    cache: Vec<Rc<str>>,
    is_cached: <bool as Encode>::Context,
    string_ctx: <String as Encode>::Context,
    index: <usize as Encode>::Context,
}

impl EncodingStrategy<Rc<str>> for LowCardinality {
    type Context = RcStrCacheContext;
    #[inline]
    fn encode<E: super::EntropyCoder>(value: &Rc<str>, writer: &mut E, ctx: &mut Self::Context) {
        let looked_up = ctx.cached.get(value.as_ref()).copied();
        looked_up.is_some().encode(writer, &mut ctx.is_cached);
        if let Some(idx) = looked_up {
            idx.encode(writer, &mut ctx.index)
        } else {
            ctx.cached.insert(value.clone(), ctx.cached.len());
            value.to_string().encode(writer, &mut ctx.string_ctx)
        }
    }
    #[inline]
    fn decode<D: super::EntropyDecoder>(
        reader: &mut D,
        ctx: &mut Self::Context,
    ) -> Result<Rc<str>, std::io::Error> {
        let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
        if is_cached {
            let idx = usize::decode(reader, &mut ctx.index)?;
            ctx.cache
                .get(idx)
                .cloned()
                .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
        } else {
            let s = String::decode(reader, &mut ctx.string_ctx)?;
            let value: Rc<str> = Rc::from(s.as_str());
            ctx.cache.push(value.clone());
            Ok(value)
        }
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

    // Suffix-only match, and a query entirely a suffix of an existing entry
    // (an exact-length-mismatch case that must not be mistaken for a cache hit).
    round_trip(vec![Arc::from("ab"), Arc::from("a"), Arc::from("b")]);

    // Both prefix and suffix match, against different dictionary entries,
    // plus an exact repeat mixed in.
    round_trip(vec![
        Arc::from("hello world"),
        Arc::from("goodbye world"),
        Arc::from("hello there"),
        Arc::from("hello world"),
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
