//! An ordered set of `Arc<str>` with fast longest-common-prefix / suffix lookup.
//!
//! Not yet consumed elsewhere in the crate, hence `allow(dead_code)` below.
#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::ops::Bound;
use std::sync::Arc;

/// An ordered set of `Arc<str>` supporting O(log N) longest-common-prefix and
/// longest-common-suffix lookups against a query string.
///
/// Strings are kept in insertion order (accessible via [`StringSet::iter`] or
/// indexing), and are deduplicated: inserting a string that is already
/// present returns the index of the existing entry.
#[derive(Default, Clone)]
pub struct StringSet {
    strings: Vec<Arc<str>>,
    by_prefix: BTreeMap<Arc<str>, usize>,
    by_suffix: BTreeMap<SuffixKey, usize>,
}

/// Wrapper giving `Arc<str>` a reverse-byte-order `Ord`, so a `BTreeMap`
/// keyed on this type is ordered by suffix instead of by prefix.
#[derive(Clone, Eq, PartialEq)]
struct SuffixKey(Arc<str>);

impl Ord for SuffixKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.bytes().rev().cmp(other.0.bytes().rev())
    }
}

impl PartialOrd for SuffixKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl StringSet {
    /// Create an empty `StringSet`.
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of distinct strings in the set.
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Whether the set contains no strings.
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Iterate over the strings in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &Arc<str>> {
        self.strings.iter()
    }

    /// The string previously inserted at `index`, if any.
    pub fn get(&self, index: usize) -> Option<&Arc<str>> {
        self.strings.get(index)
    }

    /// Insert a string into the set, returning its index.
    ///
    /// If the string is already present, this is a no-op and returns the
    /// index of the existing entry.
    pub fn insert(&mut self, s: Arc<str>) -> usize {
        if let Some(&idx) = self.by_prefix.get(s.as_ref()) {
            return idx;
        }
        let idx = self.strings.len();
        self.by_prefix.insert(s.clone(), idx);
        self.by_suffix.insert(SuffixKey(s.clone()), idx);
        self.strings.push(s);
        idx
    }

    /// Find the string in the set that shares the longest prefix with
    /// `query`, returning its index and the shared prefix (a slice of
    /// `query`).
    ///
    /// Returns `None` if the set is empty. The shared prefix is trimmed to
    /// the nearest `char` boundary, so it is always valid to slice `query`
    /// with it.
    pub fn longest_common_prefix<'a>(&self, query: &'a str) -> Option<(usize, &'a str)> {
        let mut best: Option<(usize, usize)> = None; // (index, byte len)
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
        best.map(|(idx, len)| (idx, &query[..len]))
    }

    /// Find the string in the set that shares the longest suffix with
    /// `query`, returning its index and the shared suffix (a slice of
    /// `query`).
    ///
    /// Returns `None` if the set is empty. The shared suffix is trimmed to
    /// the nearest `char` boundary, so it is always valid to slice `query`
    /// with it.
    pub fn longest_common_suffix<'a>(&self, query: &'a str) -> Option<(usize, &'a str)> {
        if self.strings.is_empty() {
            return None;
        }
        let probe = SuffixKey(Arc::from(query));
        let mut best: Option<(usize, usize)> = None; // (index, byte len)
        if let Some((k, &idx)) = self.by_suffix.range(probe.clone()..).next() {
            let len = common_suffix_len(query, &k.0);
            best = Some((idx, len));
        }
        if let Some((k, &idx)) = self.by_suffix.range(..probe).next_back() {
            let len = common_suffix_len(query, &k.0);
            if best.is_none_or(|(_, best_len)| len > best_len) {
                best = Some((idx, len));
            }
        }
        best.map(|(idx, len)| (idx, &query[query.len() - len..]))
    }
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

    fn set(strings: &[&str]) -> StringSet {
        let mut s = StringSet::new();
        for &string in strings {
            s.insert(Arc::from(string));
        }
        s
    }

    #[test]
    fn empty_set_returns_none() {
        let s = StringSet::new();
        assert_eq!(s.longest_common_prefix("anything"), None);
        assert_eq!(s.longest_common_suffix("anything"), None);
    }

    #[test]
    fn exact_match_prefix_and_suffix() {
        let s = set(&["hello", "world"]);
        let (idx, prefix) = s.longest_common_prefix("hello").unwrap();
        assert_eq!(idx, 0);
        assert_eq!(prefix, "hello");
        let (idx, suffix) = s.longest_common_suffix("world").unwrap();
        assert_eq!(idx, 1);
        assert_eq!(suffix, "world");
    }

    #[test]
    fn neighbor_case_prefix() {
        let s = set(&["hello", "help"]);
        let (_idx, prefix) = s.longest_common_prefix("helicopter").unwrap();
        assert_eq!(prefix, "hel");
    }

    #[test]
    fn neighbor_case_suffix() {
        // reverse of the prefix case: shared trailing bytes
        let s = set(&["stello", "istello"]);
        let (_idx, suffix) = s.longest_common_suffix("nice_stello").unwrap();
        assert_eq!(suffix, "stello");
    }

    #[test]
    fn query_shorter_than_stored() {
        let s = set(&["hello world"]);
        let (idx, prefix) = s.longest_common_prefix("hello").unwrap();
        assert_eq!(idx, 0);
        assert_eq!(prefix, "hello");
    }

    #[test]
    fn query_longer_than_stored_prefix_match() {
        let s = set(&["hello"]);
        let (idx, prefix) = s.longest_common_prefix("hello world").unwrap();
        assert_eq!(idx, 0);
        assert_eq!(prefix, "hello");
    }

    #[test]
    fn no_common_prefix_or_suffix() {
        let s = set(&["zzz"]);
        let (_idx, prefix) = s.longest_common_prefix("aaa").unwrap();
        assert_eq!(prefix, "");
        let (_idx, suffix) = s.longest_common_suffix("aaa").unwrap();
        assert_eq!(suffix, "");
    }

    #[test]
    fn duplicate_insert_returns_existing_index() {
        let mut s = StringSet::new();
        let i1 = s.insert(Arc::from("hello"));
        let i2 = s.insert(Arc::from("hello"));
        assert_eq!(i1, i2);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn utf8_boundary_is_respected_for_prefix() {
        // "é" = 0xC3 0xA9, "è" = 0xC3 0xA8 -- share only the leading byte 0xC3,
        // which must not be sliced on its own.
        let s = set(&["éxyz"]);
        let (_idx, prefix) = s.longest_common_prefix("èabc").unwrap();
        assert_eq!(prefix, "");
        assert!(prefix.is_char_boundary(prefix.len()));
    }

    #[test]
    fn utf8_boundary_is_respected_for_suffix() {
        let s = set(&["xyzé"]);
        let (_idx, suffix) = s.longest_common_suffix("abcè").unwrap();
        assert_eq!(suffix, "");
        assert!(suffix.is_char_boundary(0));
    }

    #[test]
    fn iter_preserves_insertion_order() {
        let s = set(&["c", "a", "b"]);
        let v: Vec<&str> = s.iter().map(|s| s.as_ref()).collect();
        assert_eq!(v, vec!["c", "a", "b"]);
    }
}
