//! Compares the new prefix/suffix dictionary `Arc<str>` encoding against the
//! old dictionary-only encoding it replaced, on a corpus of IPv4 addresses
//! read as strings (`ipv4.txt`) — a favorable case, since sorted-neighbor
//! addresses often share long prefixes ("192.168.0." for a whole subnet) and
//! dotted-decimal addresses often share suffixes too.
//!
//! Three variants: `old` (pre-`StringSet` plain content-addressed cache),
//! `btree` (the `BTreeMap`-based `StringSet`: `O(1)` exact-match `HashMap`
//! fast path, but two separate `O(log N)` searches plus two more `O(log N)`
//! inserts per miss), and `new` (current: the treap-based `StringSet`,
//! fusing search and insert into one `O(log N)` walk per ordering).
//!
//! Usage:
//! - `bench-arc-str` — human-readable size/ratio/wall-time summary for all
//!   three, on the `ipv4.txt` corpus.
//! - `bench-arc-str encode|decode old|btree|new [ipv4|repeated] [iterations]`
//!   (default `ipv4`, 200 iterations) — encode-only or decode-only loop for
//!   `bench perf stat -e cpu_core/cycles/ bench-arc-str encode|decode old|btree|new [ipv4|repeated]`,
//!   per OPTIMIZING.md's methodology (alternate variants, take the min of a
//!   few runs). `repeated` is the same string over and over (every
//!   `ipv4.txt` line is distinct, so that corpus is 100% dictionary misses;
//!   `repeated` is the opposite extreme, ~100% cache hits after the first
//!   value) -- useful for isolating the exact-match/cache-hit path's cost
//!   from the miss path's `StringSet` search cost.
//!
//! Everything lives behind the `v2` feature so that
//! `cargo check --no-default-features` (run by CI and the pre-commit hook)
//! still compiles this bin.
#[cfg(feature = "v2")]
mod imp {
    use compactly::v2::{decode, encode, Encode, EntropyCoder, EntropyDecoder};
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Instant;

    fn report<T: Encode + PartialEq>(label: &str, values: &Vec<T>, raw_bytes: usize) {
        let t0 = Instant::now();
        let encoded = encode(values);
        let encode_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let t1 = Instant::now();
        let decoded = decode::<Vec<T>>(&encoded).expect("decode failed");
        let decode_ms = t1.elapsed().as_secs_f64() * 1000.0;

        assert!(decoded == *values, "roundtrip mismatch");
        println!(
        "{label}: raw={raw_bytes}B encoded={}B ratio={:.2}x encode={encode_ms:.1}ms decode={decode_ms:.1}ms",
        encoded.len(),
        raw_bytes as f64 / encoded.len() as f64,
    );
    }

    /// A frozen copy of the pre-prefix/suffix `Arc<str>` `LowCardinality`
    /// encoding (see git history of `src/v2/low_cardinality.rs`): a plain
    /// content-addressed dictionary — one `is_cached` bit, then either an
    /// index or the full string encoded from scratch. Kept here, not in the
    /// library, purely so this benchmark can compare old vs. new without
    /// needing to check out an old commit.
    #[derive(Clone, PartialEq)]
    struct OldStyleArcStr(Arc<str>);

    #[derive(Default, Clone)]
    struct OldStyleContext {
        cached: HashMap<Arc<str>, usize>,
        cache: Vec<Arc<str>>,
        is_cached: <bool as Encode>::Context,
        string_ctx: <String as Encode>::Context,
        index: <usize as Encode>::Context,
    }

    impl Encode for OldStyleArcStr {
        type Context = OldStyleContext;
        fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
            let looked_up = ctx.cached.get(self.0.as_ref()).copied();
            looked_up.is_some().encode(writer, &mut ctx.is_cached);
            if let Some(idx) = looked_up {
                idx.encode(writer, &mut ctx.index)
            } else {
                ctx.cached.insert(self.0.clone(), ctx.cached.len());
                self.0.to_string().encode(writer, &mut ctx.string_ctx)
            }
        }
        fn decode<D: EntropyDecoder>(
            reader: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Self, std::io::Error> {
            let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
            if is_cached {
                let idx = usize::decode(reader, &mut ctx.index)?;
                ctx.cache
                    .get(idx)
                    .cloned()
                    .map(OldStyleArcStr)
                    .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
            } else {
                let s = String::decode(reader, &mut ctx.string_ctx)?;
                let value: Arc<str> = Arc::from(s.as_str());
                ctx.cache.push(value.clone());
                Ok(OldStyleArcStr(value))
            }
        }
    }

    /// A frozen copy of the pre-dictionary `String` `LowCardinality`
    /// encoding (the `impl_low_cardinality!(String, string)` macro
    /// expansion this crate used before `String` was switched to share the
    /// `Rc<str>` dictionary implementation): a plain content-addressed
    /// cache -- one `is_cached` bit, then either an index or the full
    /// string via `String`'s own char-by-char `Encode`. One allocation on
    /// an encode-side miss (the `String` clone stored as the `HashMap`
    /// key) and one more on a decode-side miss (`ctx.cache.push(value.clone())`).
    #[derive(Clone, PartialEq)]
    struct OldStyleString(String);

    #[derive(Default, Clone)]
    struct OldStyleStringContext {
        cached: HashMap<String, usize>,
        cache: Vec<String>,
        is_cached: <bool as Encode>::Context,
        string_ctx: <String as Encode>::Context,
        index: <usize as Encode>::Context,
    }

    impl Encode for OldStyleString {
        type Context = OldStyleStringContext;
        fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
            let looked_up = ctx.cached.get(&self.0).copied();
            looked_up.is_some().encode(writer, &mut ctx.is_cached);
            if let Some(idx) = looked_up {
                idx.encode(writer, &mut ctx.index)
            } else {
                ctx.cached.insert(self.0.clone(), ctx.cached.len());
                self.0.encode(writer, &mut ctx.string_ctx)
            }
        }
        fn decode<D: EntropyDecoder>(
            reader: &mut D,
            ctx: &mut Self::Context,
        ) -> Result<Self, std::io::Error> {
            let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
            if is_cached {
                let idx = usize::decode(reader, &mut ctx.index)?;
                ctx.cache
                    .get(idx)
                    .cloned()
                    .map(OldStyleString)
                    .ok_or_else(|| std::io::Error::other("bad low_cardinality index"))
            } else {
                let s = String::decode(reader, &mut ctx.string_ctx)?;
                ctx.cache.push(s.clone());
                Ok(OldStyleString(s))
            }
        }
    }

    /// A frozen copy of the `BTreeMap`-based prefix/suffix `StringSet` +
    /// `ArcStrCacheContext` (i.e. the version with the O(1) exact-match
    /// `HashMap` fast path and the allocation-free suffix query, but
    /// *before* the treap rewrite: two separate `O(log N)` searches plus
    /// two more `O(log N)` inserts per miss, six tree walks total). Kept
    /// here, not in the library, so this benchmark can compare the treap
    /// against exactly what it replaced without checking out an old commit.
    mod btree_variant {
        use compactly::v2::{Encode, EncodingStrategy, EntropyCoder, EntropyDecoder};
        use compactly::Small;
        use std::collections::{BTreeMap, HashMap};
        use std::ops::Bound;
        use std::sync::Arc;

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum PrefixMatch<'a> {
            Exact(usize),
            Prefix(usize, &'a str),
        }

        const SUFFIX_STACK_BUF: usize = 128;

        #[derive(Default, Clone)]
        struct StringSet {
            strings: Vec<Arc<str>>,
            exact: HashMap<Arc<str>, usize>,
            by_prefix: BTreeMap<Arc<str>, usize>,
            by_suffix: BTreeMap<Box<[u8]>, usize>,
        }

        impl StringSet {
            fn get(&self, index: usize) -> Option<&Arc<str>> {
                self.strings.get(index)
            }

            fn insert(&mut self, s: Arc<str>) -> usize {
                if let Some(&idx) = self.exact.get(s.as_ref()) {
                    return idx;
                }
                let idx = self.strings.len();
                self.exact.insert(s.clone(), idx);
                self.by_prefix.insert(s.clone(), idx);
                self.by_suffix.insert(s.bytes().rev().collect(), idx);
                self.strings.push(s);
                idx
            }

            fn push(&mut self, s: Arc<str>) -> usize {
                let idx = self.strings.len();
                self.strings.push(s);
                idx
            }

            fn prefix_match<'a>(&self, query: &'a str) -> Option<PrefixMatch<'a>> {
                if let Some(&idx) = self.exact.get(query) {
                    return Some(PrefixMatch::Exact(idx));
                }
                let mut best: Option<(usize, usize)> = None;
                if let Some((k, &idx)) = self
                    .by_prefix
                    .range::<str, _>((Bound::Included(query), Bound::Unbounded))
                    .next()
                {
                    let len = common_prefix_len(query, k);
                    best = Some((idx, len));
                }
                if let Some((k, &idx)) = self
                    .by_prefix
                    .range::<str, _>((Bound::Unbounded, Bound::Excluded(query)))
                    .next_back()
                {
                    let len = common_prefix_len(query, k);
                    if best.is_none_or(|(_, best_len)| len > best_len) {
                        best = Some((idx, len));
                    }
                }
                best.map(|(idx, len)| PrefixMatch::Prefix(idx, &query[..len]))
            }

            fn suffix_match<'a>(&self, query: &'a str) -> Option<(usize, &'a str)> {
                if self.strings.is_empty() {
                    return None;
                }
                let mut buf = [0u8; SUFFIX_STACK_BUF];
                let heap;
                let key: &[u8] = if query.len() <= SUFFIX_STACK_BUF {
                    for (dst, b) in buf.iter_mut().zip(query.bytes().rev()) {
                        *dst = b;
                    }
                    &buf[..query.len()]
                } else {
                    heap = query.bytes().rev().collect::<Vec<u8>>();
                    &heap
                };

                let mut best: Option<(usize, usize)> = None;
                if let Some((_, &idx)) = self
                    .by_suffix
                    .range::<[u8], _>((Bound::Included(key), Bound::Unbounded))
                    .next()
                {
                    let len = common_suffix_len(query, &self.strings[idx]);
                    best = Some((idx, len));
                }
                if let Some((_, &idx)) = self
                    .by_suffix
                    .range::<[u8], _>((Bound::Unbounded, Bound::Excluded(key)))
                    .next_back()
                {
                    let len = common_suffix_len(query, &self.strings[idx]);
                    if best.is_none_or(|(_, best_len)| len > best_len) {
                        best = Some((idx, len));
                    }
                }
                best.map(|(idx, len)| (idx, &query[query.len() - len..]))
            }
        }

        fn common_prefix_len(a: &str, b: &str) -> usize {
            let mut len = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
            while !a.is_char_boundary(len) {
                len -= 1;
            }
            len
        }

        fn common_suffix_len(a: &str, b: &str) -> usize {
            let mut len = a
                .bytes()
                .rev()
                .zip(b.bytes().rev())
                .take_while(|(x, y)| x == y)
                .count();
            while !a.is_char_boundary(a.len() - len) {
                len -= 1;
            }
            len
        }

        #[inline]
        fn encode_match_len<E: EntropyCoder>(
            len: usize,
            writer: &mut E,
            ctx: &mut <Small as EncodingStrategy<u16>>::Context,
        ) {
            let wire = if len == 0 { 0 } else { (len - 1) as u16 };
            Small::encode(&wire, writer, ctx);
        }

        #[inline]
        fn decode_match_len<D: EntropyDecoder>(
            reader: &mut D,
            ctx: &mut <Small as EncodingStrategy<u16>>::Context,
        ) -> Result<usize, std::io::Error> {
            let wire: u16 = Small::decode(reader, ctx)?;
            Ok(if wire == 0 { 0 } else { wire as usize + 1 })
        }

        #[derive(Default, Clone)]
        pub struct BTreeArcStrContext {
            dict: StringSet,
            is_cached: <bool as Encode>::Context,
            index: <Small as EncodingStrategy<usize>>::Context,
            prefix_len: <Small as EncodingStrategy<u16>>::Context,
            prefix_index: <Small as EncodingStrategy<usize>>::Context,
            suffix_len: <Small as EncodingStrategy<u16>>::Context,
            suffix_index: <Small as EncodingStrategy<usize>>::Context,
            middle_len: <Small as EncodingStrategy<usize>>::Context,
            chars: <char as Encode>::Context,
        }

        #[derive(Clone, PartialEq)]
        pub struct BTreeArcStr(pub Arc<str>);

        impl Encode for BTreeArcStr {
            type Context = BTreeArcStrContext;
            fn encode<E: EntropyCoder>(&self, writer: &mut E, ctx: &mut Self::Context) {
                let value = &self.0;
                let prefix_match = ctx.dict.prefix_match(value);

                if let Some(PrefixMatch::Exact(idx)) = prefix_match {
                    true.encode(writer, &mut ctx.is_cached);
                    return Small::encode(&idx, writer, &mut ctx.index);
                }
                false.encode(writer, &mut ctx.is_cached);

                let (mut prefix_len, mut prefix_idx) = match prefix_match {
                    Some(PrefixMatch::Prefix(idx, matched)) if !matched.is_empty() => {
                        (matched.len(), Some(idx))
                    }
                    _ => (0, None),
                };
                if prefix_len > u16::MAX as usize {
                    prefix_len = u16::MAX as usize;
                    while !value.is_char_boundary(prefix_len) {
                        prefix_len -= 1;
                    }
                }
                if prefix_len <= 1 {
                    prefix_len = 0;
                    prefix_idx = None;
                }
                encode_match_len(prefix_len, writer, &mut ctx.prefix_len);
                if let Some(idx) = prefix_idx {
                    Small::encode(&idx, writer, &mut ctx.prefix_index);
                }

                let suffix_match = ctx.dict.suffix_match(value);
                let (mut suffix_len, mut suffix_idx) = match suffix_match {
                    Some((idx, matched)) if !matched.is_empty() => (matched.len(), Some(idx)),
                    _ => (0, None),
                };
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

                ctx.dict.insert(value.clone());
            }
            fn decode<D: EntropyDecoder>(
                reader: &mut D,
                ctx: &mut Self::Context,
            ) -> Result<Self, std::io::Error> {
                let is_cached = bool::decode(reader, &mut ctx.is_cached)?;
                if is_cached {
                    let idx: usize = Small::decode(reader, &mut ctx.index)?;
                    return ctx
                        .dict
                        .get(idx)
                        .cloned()
                        .map(BTreeArcStr)
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
                Ok(BTreeArcStr(value))
            }
        }
    }
    use btree_variant::BTreeArcStr;

    fn read_ips() -> Vec<Arc<str>> {
        std::fs::read_to_string("ipv4.txt")
            .expect("ipv4.txt")
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| Arc::from(l.trim()))
            .collect()
    }

    /// Every `ipv4.txt` line is distinct (checked with `sort -u`), so that
    /// corpus never exercises the exact-match/cache-hit path at all -- every
    /// value is a genuine dictionary miss. This corpus is the opposite
    /// extreme: the same string over and over, so (after the first value)
    /// every encode call is a cache hit. Same length as `ipv4.txt` for a
    /// comparable cycle count.
    fn repeated_corpus() -> Vec<Arc<str>> {
        let s: Arc<str> = Arc::from("192.168.1.1");
        std::iter::repeat_n(s, 20629).collect()
    }

    fn read_corpus(name: &str) -> Vec<Arc<str>> {
        match name {
            "ipv4" => read_ips(),
            "repeated" => repeated_corpus(),
            _ => unreachable!(),
        }
    }

    fn read_string_corpus(name: &str) -> Vec<String> {
        read_corpus(name).iter().map(|s| s.to_string()).collect()
    }

    /// Encode- or decode-only loop for `bench perf stat -e cpu_core/cycles/`:
    /// repeat the chosen operation with `black_box` so the compiler can't
    /// hoist it out, matching `just-compress-strings`/`just-decompress-strings`'s
    /// convention. No internal wall-clock timing — per OPTIMIZING.md, cycle
    /// counts via `perf` (alternating old/new, taking the min of a few runs)
    /// are far less noisy than `Instant`-based wall time on this machine.
    fn timing_loop(op: &str, mode: &str, corpus: &str, iterations: usize) {
        let ips = read_corpus(corpus);
        let mut total = 0usize;
        match (op, mode) {
            ("encode", "old") => {
                let old: Vec<OldStyleArcStr> = ips.iter().cloned().map(OldStyleArcStr).collect();
                for _ in 0..iterations {
                    total += std::hint::black_box(encode(&old)).len();
                }
                println!("total encoded bytes {total}");
            }
            ("encode", "new") => {
                for _ in 0..iterations {
                    total += std::hint::black_box(encode(&ips)).len();
                }
                println!("total encoded bytes {total}");
            }
            ("encode", "btree") => {
                let bt: Vec<BTreeArcStr> = ips.iter().cloned().map(BTreeArcStr).collect();
                for _ in 0..iterations {
                    total += std::hint::black_box(encode(&bt)).len();
                }
                println!("total encoded bytes {total}");
            }
            ("encode", "string-old") => {
                let strs: Vec<OldStyleString> = read_string_corpus(corpus)
                    .into_iter()
                    .map(OldStyleString)
                    .collect();
                for _ in 0..iterations {
                    total += std::hint::black_box(encode(&strs)).len();
                }
                println!("total encoded bytes {total}");
            }
            ("encode", "string-new") => {
                let strs: Vec<compactly::Encoded<String, compactly::LowCardinality>> =
                    read_string_corpus(corpus)
                        .into_iter()
                        .map(compactly::Encoded::new)
                        .collect();
                for _ in 0..iterations {
                    total += std::hint::black_box(encode(&strs)).len();
                }
                println!("total encoded bytes {total}");
            }
            ("decode", "string-old") => {
                let strs: Vec<OldStyleString> = read_string_corpus(corpus)
                    .into_iter()
                    .map(OldStyleString)
                    .collect();
                let encoded = encode(&strs);
                println!("string-old encoded size {}", encoded.len());
                for _ in 0..iterations {
                    total += std::hint::black_box(decode::<Vec<OldStyleString>>(&encoded).unwrap())
                        .len();
                }
                println!("total decoded {total}");
            }
            ("decode", "string-new") => {
                let strs: Vec<compactly::Encoded<String, compactly::LowCardinality>> =
                    read_string_corpus(corpus)
                        .into_iter()
                        .map(compactly::Encoded::new)
                        .collect();
                let encoded = encode(&strs);
                println!("string-new encoded size {}", encoded.len());
                for _ in 0..iterations {
                    total += std::hint::black_box(
                        decode::<Vec<compactly::Encoded<String, compactly::LowCardinality>>>(
                            &encoded,
                        )
                        .unwrap(),
                    )
                    .len();
                }
                println!("total decoded {total}");
            }
            ("decode", "old") => {
                let old: Vec<OldStyleArcStr> = ips.iter().cloned().map(OldStyleArcStr).collect();
                let encoded = encode(&old);
                println!("old encoded size {}", encoded.len());
                for _ in 0..iterations {
                    total += std::hint::black_box(decode::<Vec<OldStyleArcStr>>(&encoded).unwrap())
                        .len();
                }
                println!("total decoded {total}");
            }
            ("decode", "new") => {
                let encoded = encode(&ips);
                println!("new encoded size {}", encoded.len());
                for _ in 0..iterations {
                    total += std::hint::black_box(decode::<Vec<Arc<str>>>(&encoded).unwrap()).len();
                }
                println!("total decoded {total}");
            }
            ("decode", "btree") => {
                let bt: Vec<BTreeArcStr> = ips.iter().cloned().map(BTreeArcStr).collect();
                let encoded = encode(&bt);
                println!("btree encoded size {}", encoded.len());
                for _ in 0..iterations {
                    total +=
                        std::hint::black_box(decode::<Vec<BTreeArcStr>>(&encoded).unwrap()).len();
                }
                println!("total decoded {total}");
            }
            _ => unreachable!(),
        }
    }

    pub(crate) fn run() {
        let args: Vec<String> = std::env::args().collect();
        let op = args
            .iter()
            .find(|a| a.as_str() == "encode" || a.as_str() == "decode")
            .map(String::as_str);
        let mode = args
            .iter()
            .find(|a| {
                matches!(
                    a.as_str(),
                    "old" | "new" | "btree" | "string-old" | "string-new"
                )
            })
            .map(String::as_str);
        let corpus = args
            .iter()
            .find(|a| a.as_str() == "ipv4" || a.as_str() == "repeated")
            .map(String::as_str)
            .unwrap_or("ipv4");
        if let (Some(op), Some(mode)) = (op, mode) {
            let iterations: usize = args
                .iter()
                .filter_map(|a| a.parse().ok())
                .next()
                .unwrap_or(200);
            timing_loop(op, mode, corpus, iterations);
            return;
        }

        let ips = read_ips();
        let raw_bytes: usize = ips.iter().map(|s| s.len()).sum();

        println!("── IPv4 addresses as strings ({} lines) ──", ips.len());
        let old: Vec<OldStyleArcStr> = ips.iter().cloned().map(OldStyleArcStr).collect();
        report("  old (dictionary only)     ", &old, raw_bytes);
        let bt: Vec<BTreeArcStr> = ips.iter().cloned().map(BTreeArcStr).collect();
        report("  btree (prefix/suffix dict)", &bt, raw_bytes);
        report("  treap (prefix/suffix dict)", &ips, raw_bytes);
    }
}

#[cfg(feature = "v2")]
fn main() {
    imp::run()
}

#[cfg(not(feature = "v2"))]
fn main() {
    eprintln!("bench-arc-str requires the v2 feature");
}
