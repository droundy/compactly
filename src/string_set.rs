//! An ordered set of `Arc<str>`/`Rc<str>` with fast longest-common-prefix /
//! suffix lookup.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

mod treap;
use treap::Treap;

/// The result of a miss in [`StringSet::insert_new`]: the best prefix and
/// best suffix match (if any) against the dictionary's existing members.
/// (The inserted string's newly assigned index is the set's length before
/// the insertion, so it is not repeated here.)
pub struct Miss<'a> {
    /// The dictionary member sharing the longest prefix with the inserted
    /// string, and that shared prefix (a slice of the inserted string).
    pub prefix: Option<(usize, &'a str)>,
    /// The dictionary member sharing the longest suffix with the inserted
    /// string, and that shared suffix (a slice of the inserted string).
    pub suffix: Option<(usize, &'a str)>,
}

/// An ordered set of ref-counted strings (`Arc<str>` or `Rc<str>`) supporting
/// O(1) exact-match lookups and O(log N) longest-common-prefix /
/// longest-common-suffix search fused with insertion.
///
/// Strings are kept in insertion order (accessible via [`StringSet::iter`] or
/// indexing), and are deduplicated: inserting a string that is already
/// present returns the index of the existing entry.
///
/// Generic over the pointer type `P` (`Arc<str>` or `Rc<str>`) so `v2`'s
/// dictionary-based `LowCardinality` encoding for `Arc<str>`/`Rc<str>`/
/// `String` can share one implementation; see `src/v2/low_cardinality.rs`.
#[derive(Clone)]
pub struct StringSet<P> {
    strings: Vec<P>,
    exact: HashMap<P, usize>,
    /// Ordered by string content, for prefix search. The treap stores no
    /// keys of its own -- entry `i` is `strings[i]`, compared through a
    /// closure at insert time (see [`treap`]).
    by_prefix: Treap,
    /// Ordered by the strings' *reversed* bytes ([`rev_cmp`]), for suffix
    /// search -- same index-keyed scheme, so no reversed copy of any
    /// string is ever materialized.
    by_suffix: Treap,
}

impl<P> Default for StringSet<P> {
    fn default() -> Self {
        Self {
            strings: Vec::new(),
            exact: HashMap::new(),
            by_prefix: Treap::default(),
            by_suffix: Treap::default(),
        }
    }
}

impl<P> StringSet<P>
where
    P: Clone + Ord + Hash + Borrow<str> + Deref<Target = str>,
{
    /// Create an empty `StringSet`.
    #[cfg(test)] // exercised only by tests; production callers use get/get_exact/insert_new/push
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of distinct strings in the set.
    #[cfg(test)] // exercised only by tests; production callers use get/get_exact/insert_new/push
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Whether the set contains no strings.
    #[cfg(test)] // exercised only by tests; production callers use get/get_exact/insert_new/push
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Iterate over the strings in insertion order.
    #[cfg(test)] // exercised only by tests; production callers use get/get_exact/insert_new/push
    pub fn iter(&self) -> impl Iterator<Item = &P> {
        self.strings.iter()
    }

    /// The string previously inserted at `index`, if any.
    pub fn get(&self, index: usize) -> Option<&P> {
        self.strings.get(index)
    }

    /// O(1) exact-match lookup: the index of `query`, if it's in the set.
    pub fn get_exact(&self, query: &str) -> Option<usize> {
        self.exact.get(query).copied()
    }

    /// Insert a string into the set, returning its index.
    ///
    /// If the string is already present, this is a no-op and returns the
    /// index of the existing entry. Prefer [`StringSet::insert_new`] when
    /// the caller has already confirmed (e.g. via [`StringSet::get_exact`])
    /// that the string is absent and wants the prefix/suffix match info
    /// too, since this method does a redundant exact-match check.
    #[cfg(test)] // exercised only by tests; production callers use get/get_exact/insert_new/push
    pub fn insert(&mut self, s: P) -> usize {
        if let Some(idx) = self.get_exact(&s) {
            return idx;
        }
        let idx = self.len();
        self.insert_new(&s);
        idx
    }

    /// Insert a string that is known not to already be in the set (callers
    /// typically confirm via [`StringSet::get_exact`] first -- this method
    /// panics if `s` is already present). Returns the best prefix and
    /// suffix matches against the dictionary's existing members, found in
    /// the same tree walk that performs the insertion (see [`treap`] for
    /// how).
    ///
    /// The shared prefix/suffix are trimmed to the nearest `char` boundary,
    /// so they are always valid to slice `s` with.
    pub fn insert_new<'a>(&mut self, s: &'a P) -> Miss<'a> {
        let idx = self.strings.len();
        // The hash-map insert reports a violated precondition for free, and
        // catches it before the treaps (whose insert would corrupt or panic
        // deeper in on a duplicate key) are touched.
        assert!(
            self.exact.insert(s.clone(), idx).is_none(),
            "StringSet::insert_new called with a string already in the set"
        );
        // The treaps identify entries by index (see [`treap`]): entry `i`
        // is `strings[i]`, which holds only if every string was added via
        // this method -- never mix `insert_new` and `push` on one set.
        debug_assert!(
            self.by_prefix.len() == idx && self.by_suffix.len() == idx,
            "StringSet::insert_new called on a set that was built with push"
        );
        let query: &str = s;
        let strings = &self.strings;
        let (prefix_pred, prefix_succ) = self
            .by_prefix
            .insert_and_find_neighbors(|i| query.cmp(&strings[i]));
        let (suffix_pred, suffix_succ) = self
            .by_suffix
            .insert_and_find_neighbors(|i| rev_cmp(query, &strings[i]));
        self.strings.push(s.clone());

        let s: &str = s;
        let mut prefix: Option<(usize, usize)> = None; // (index, byte len)
        for cand in [prefix_pred, prefix_succ].into_iter().flatten() {
            let len = common_prefix_len(s, &self.strings[cand]);
            if prefix.is_none_or(|(_, best_len)| len > best_len) {
                prefix = Some((cand, len));
            }
        }
        let mut suffix: Option<(usize, usize)> = None;
        for cand in [suffix_pred, suffix_succ].into_iter().flatten() {
            let len = common_suffix_len(s, &self.strings[cand]);
            if suffix.is_none_or(|(_, best_len)| len > best_len) {
                suffix = Some((cand, len));
            }
        }

        Miss {
            prefix: prefix.map(|(idx, len)| (idx, &s[..len])),
            suffix: suffix.map(|(idx, len)| (idx, &s[s.len() - len..])),
        }
    }

    /// Append a string for index-based retrieval only ([`StringSet::get`]),
    /// without updating the exact/prefix/suffix search structures.
    ///
    /// Cheaper than [`StringSet::insert`]/[`StringSet::insert_new`] when the
    /// caller only ever needs `get` -- e.g. reconstructing a dictionary
    /// while decoding a value that some other (encode-side) `StringSet`
    /// already deduplicated, so the index is known to be correct and in
    /// order without a lookup. Unlike `insert`, this does not check for or
    /// skip duplicates. A set built with `push` must never be handed to
    /// [`StringSet::insert_new`], which requires the treaps' entry indices
    /// to match the string indices.
    pub fn push(&mut self, s: P) -> usize {
        let idx = self.strings.len();
        self.strings.push(s);
        idx
    }
}

/// Compare `a` and `b` as if their byte sequences were reversed
/// (last byte first), without materializing the reversal; `Equal` only for
/// identical strings. This is the suffix treap's ordering: it clusters
/// strings sharing long suffixes, so a new string's reversed-order
/// neighbors are its best suffix-match candidates.
fn rev_cmp(a: &str, b: &str) -> Ordering {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let n = a.len().min(b.len());
    let mut i = 0;
    // Compare 8-byte blocks walking back from the end. A little-endian load
    // of such a block places the string's *later* bytes in *more*
    // significant positions, so a plain integer compare orders the block
    // exactly like the reversed byte sequence.
    while i + 8 <= n {
        let av = u64::from_le_bytes(a[a.len() - i - 8..a.len() - i].try_into().unwrap());
        let bv = u64::from_le_bytes(b[b.len() - i - 8..b.len() - i].try_into().unwrap());
        if av != bv {
            return av.cmp(&bv);
        }
        i += 8;
    }
    while i < n {
        let av = a[a.len() - 1 - i];
        let bv = b[b.len() - 1 - i];
        if av != bv {
            return av.cmp(&bv);
        }
        i += 1;
    }
    a.len().cmp(&b.len())
}

/// The length in bytes of the longest common prefix of `a` and `b`, trimmed
/// back to a `char` boundary in `a`.
fn common_prefix_len(a: &str, b: &str) -> usize {
    let mut len = a.bytes().zip(b.bytes()).take_while(|(x, y)| x == y).count();
    while !a.is_char_boundary(len) {
        len -= 1;
    }
    len
}

/// The length in bytes of the longest common suffix of `a` and `b`, trimmed
/// back to a `char` boundary in `a`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn set(strings: &[&str]) -> StringSet<Arc<str>> {
        let mut s = StringSet::new();
        for &string in strings {
            s.insert(Arc::from(string));
        }
        s
    }

    #[test]
    fn empty_set_returns_none() {
        let s: StringSet<Arc<str>> = StringSet::new();
        assert!(s.is_empty());
        assert_eq!(s.get_exact("anything"), None);
    }

    #[test]
    fn exact_match() {
        let s = set(&["hello", "world"]);
        assert!(!s.is_empty());
        assert_eq!(s.get_exact("hello"), Some(0));
        assert_eq!(s.get_exact("world"), Some(1));
        assert_eq!(s.get_exact("goodbye"), None);
    }

    #[test]
    fn neighbor_case_prefix() {
        let mut s = set(&["hello", "help"]);
        let query: Arc<str> = Arc::from("helicopter");
        let miss = s.insert_new(&query);
        let (_idx, prefix) = miss.prefix.unwrap();
        assert_eq!(prefix, "hel");
    }

    #[test]
    fn neighbor_case_suffix() {
        // reverse of the prefix case: shared trailing bytes
        let mut s = set(&["stello", "istello"]);
        let query: Arc<str> = Arc::from("nice_stello");
        let miss = s.insert_new(&query);
        let (_idx, suffix) = miss.suffix.unwrap();
        assert_eq!(suffix, "stello");
    }

    #[test]
    fn query_shorter_than_stored() {
        let mut s = set(&["hello world"]);
        let query: Arc<str> = Arc::from("hello");
        let miss = s.insert_new(&query);
        let (idx, prefix) = miss.prefix.unwrap();
        assert_eq!(idx, 0);
        assert_eq!(prefix, "hello");
    }

    #[test]
    fn query_longer_than_stored_prefix_match() {
        let mut s = set(&["hello"]);
        let query: Arc<str> = Arc::from("hello world");
        let miss = s.insert_new(&query);
        let (idx, prefix) = miss.prefix.unwrap();
        assert_eq!(idx, 0);
        assert_eq!(prefix, "hello");
    }

    #[test]
    fn no_common_prefix_or_suffix() {
        let mut s = set(&["zzz"]);
        let query: Arc<str> = Arc::from("aaa");
        let miss = s.insert_new(&query);
        // A neighbor exists (the only entry), but shares zero bytes.
        assert_eq!(miss.prefix, Some((0, "")));
        assert_eq!(miss.suffix, Some((0, "")));
    }

    #[test]
    fn duplicate_insert_returns_existing_index() {
        let mut s: StringSet<Arc<str>> = StringSet::new();
        let i1 = s.insert(Arc::from("hello"));
        let i2 = s.insert(Arc::from("hello"));
        assert_eq!(i1, i2);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn utf8_boundary_is_respected_for_prefix() {
        // "é" = 0xC3 0xA9, "è" = 0xC3 0xA8 -- share only the leading byte 0xC3,
        // which must not be sliced on its own.
        let mut s = set(&["éxyz"]);
        let query: Arc<str> = Arc::from("èabc");
        let miss = s.insert_new(&query);
        assert_eq!(miss.prefix, Some((0, "")));
    }

    #[test]
    fn utf8_boundary_is_respected_for_suffix() {
        let mut s = set(&["xyzé"]);
        let query: Arc<str> = Arc::from("abcè");
        let miss = s.insert_new(&query);
        assert_eq!(miss.suffix, Some((0, "")));
    }

    #[test]
    fn iter_preserves_insertion_order() {
        let s = set(&["c", "a", "b"]);
        let v: Vec<&str> = s.iter().map(|s| s.as_ref()).collect();
        assert_eq!(v, vec!["c", "a", "b"]);
    }

    #[test]
    fn works_with_rc_str() {
        use std::rc::Rc;
        let mut s: StringSet<Rc<str>> = StringSet::new();
        s.insert(Rc::from("hello world"));
        let query: Rc<str> = Rc::from("hello there");
        let miss = s.insert_new(&query);
        let (_idx, prefix) = miss.prefix.unwrap();
        assert_eq!(prefix, "hello ");
    }

    #[test]
    fn many_inserts_find_correct_neighbors() {
        // A larger, less contrived check: build up a dictionary of numeric
        // strings inserted in a scrambled (non-sorted) order, and confirm
        // every miss's reported prefix/suffix match is truly the best one
        // among *all* prior entries, checked by brute force.
        use std::sync::Arc;
        let mut inserted: Vec<Arc<str>> = Vec::new();
        let mut s: StringSet<Arc<str>> = StringSet::new();
        // A simple LCG to scramble insertion order deterministically.
        let mut state: u64 = 0x2545F4914F6CDD1D;
        let mut next = || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            (state >> 33) as u32
        };
        let mut values: Vec<u32> = (0..500).collect();
        for i in (1..values.len()).rev() {
            let j = (next() as usize) % (i + 1);
            values.swap(i, j);
        }
        for v in values {
            let text = format!("item-{v:04}-suffix");
            let arc: Arc<str> = Arc::from(text.as_str());
            if s.get_exact(&arc).is_some() {
                continue;
            }
            let miss = s.insert_new(&arc);

            let mut brute_prefix = 0;
            let mut brute_suffix = 0;
            for existing in &inserted {
                brute_prefix = brute_prefix.max(common_prefix_len(&arc, existing));
                brute_suffix = brute_suffix.max(common_suffix_len(&arc, existing));
            }
            let got_prefix = miss.prefix.map(|(_, p)| p.len()).unwrap_or(0);
            let got_suffix = miss.suffix.map(|(_, sfx)| sfx.len()).unwrap_or(0);
            assert_eq!(got_prefix, brute_prefix, "prefix mismatch for {arc}");
            assert_eq!(got_suffix, brute_suffix, "suffix mismatch for {arc}");

            inserted.push(arc);
        }
    }
}
