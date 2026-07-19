//! A treap supporting a single-descent find-or-insert-with-neighbors
//! operation, used by [`super::StringSet`] to fuse the "find the longest
//! prefix/suffix match" search with the insertion of a new dictionary
//! entry: one tree walk instead of a search followed by a separate insert.
//!
//! Neighbor tracking works during any ordinary BST insert-by-key descent,
//! independent of the balancing scheme: every time the search goes *right*
//! (the current node's key is less than the key being inserted), that node
//! becomes the tightest known predecessor seen so far; every time it goes
//! *left*, that node becomes the tightest known successor. By the time the
//! descent reaches the empty slot where the new key is inserted, the last
//! updated predecessor/successor *are* its true immediate neighbors.
//!
//! Balancing is via the treap scheme (BST by key, max-heap by an
//! independent random priority per node): expected `O(log N)` depth with
//! high probability, via simple priority-based rotations on the way back up
//! -- no color/case-based rebalancing logic to get wrong, unlike a
//! red-black tree.
//!
//! # Arena storage and implicit keys
//!
//! Nodes live in one `Vec` and link by `u32` index, not `Box` pointers --
//! no per-node allocation, contiguous memory for the walk, and dropping
//! the treap is one `Vec` free. The treap does not store keys or values
//! at all: callers insert entries in index order (entry *n* is the *n*-th
//! insert, matching `StringSet`'s string indices) and supply the ordering
//! as a comparison closure over those indices, so the node's position in
//! the arena *is* its key's identity. This lets one treap keyed on the
//! strings themselves and another keyed on their reversed bytes share the
//! `StringSet`'s single string storage instead of each owning a copy.
//! Consequence: measured on the `bench-arc-str` all-miss encode workload,
//! the boxed-node, owned-key predecessor of this design spent most of its
//! time in malloc/free and node drop glue; this layout removes all of it.

use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

/// A small, fast, non-cryptographic PRNG (SplitMix64) for treap priorities.
/// Seeded once per `Treap` from `RandomState` (already linked in via std's
/// `HashMap`, avoiding a `rand` dependency) so priorities aren't predictable
/// across runs/instances, which is all a treap needs for its expected
/// `O(log N)` balance guarantee -- no cryptographic strength required.
#[derive(Clone)]
struct SplitMix64(u64);

impl SplitMix64 {
    fn new() -> Self {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_u8(0);
        Self(hasher.finish())
    }

    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Sentinel "no child" link. Also caps the arena at `u32::MAX` entries,
/// enforced by the `assert!` in [`Treap::insert_and_find_neighbors`].
const NIL: u32 = u32::MAX;

/// 12 bytes; the key is implicit (this node's own arena index) and the
/// priority is truncated to 32 bits -- rotations fire only on *strictly*
/// greater priority, so the rare tie merely forgoes one rebalancing
/// rotation, costing nothing in correctness.
#[derive(Clone)]
struct Node {
    priority: u32,
    left: u32,
    right: u32,
}

/// An ordered set of externally keyed entries supporting
/// [`Treap::insert_and_find_neighbors`]: insert a new entry and report its
/// immediate predecessor and successor (as arena/entry indices) in one
/// tree walk.
#[derive(Clone)]
pub(super) struct Treap {
    nodes: Vec<Node>,
    root: u32,
    rng: SplitMix64,
}

impl Default for Treap {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            root: NIL,
            rng: SplitMix64::new(),
        }
    }
}

impl Treap {
    /// Insert the next entry (its index is the number of prior inserts),
    /// returning the indices of its immediate predecessor and successor in
    /// the order defined by `cmp`, if any.
    ///
    /// `cmp(i)` must order the *new* entry's key against existing entry
    /// `i`'s key (`new_key.cmp(&key_of(i))`), consistently with every
    /// earlier insert's ordering, and must never return
    /// [`Ordering::Equal`] -- the new key is required to be absent (see
    /// [`super::StringSet::insert_new`]).
    /// The number of entries inserted so far (the next insert's index).
    pub(super) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(super) fn insert_and_find_neighbors(
        &mut self,
        mut cmp: impl FnMut(usize) -> Ordering,
    ) -> (Option<usize>, Option<usize>) {
        let new = self.nodes.len() as u32;
        assert!(new < NIL, "Treap arena is full");
        self.nodes.push(Node {
            priority: (self.rng.next() >> 32) as u32,
            left: NIL,
            right: NIL,
        });
        let mut pred = None;
        let mut succ = None;
        self.root = self.insert(self.root, new, &mut cmp, &mut pred, &mut succ);
        (pred, succ)
    }

    fn insert(
        &mut self,
        node: u32,
        new: u32,
        cmp: &mut impl FnMut(usize) -> Ordering,
        pred: &mut Option<usize>,
        succ: &mut Option<usize>,
    ) -> u32 {
        if node == NIL {
            return new;
        }
        let i = node as usize;
        match cmp(i) {
            Ordering::Less => {
                *succ = Some(i);
                let child = self.insert(self.nodes[i].left, new, cmp, pred, succ);
                self.nodes[i].left = child;
                if self.nodes[child as usize].priority > self.nodes[i].priority {
                    return self.rotate_right(node);
                }
            }
            Ordering::Greater => {
                *pred = Some(i);
                let child = self.insert(self.nodes[i].right, new, cmp, pred, succ);
                self.nodes[i].right = child;
                if self.nodes[child as usize].priority > self.nodes[i].priority {
                    return self.rotate_left(node);
                }
            }
            Ordering::Equal => {
                unreachable!("Treap::insert_and_find_neighbors requires an absent key")
            }
        }
        node
    }

    /// Standard BST right rotation: promotes `node`'s left child to the
    /// root of this subtree, demoting `node` to be that child's right
    /// child. Returns the promoted child.
    fn rotate_right(&mut self, node: u32) -> u32 {
        let left = self.nodes[node as usize].left;
        self.nodes[node as usize].left = self.nodes[left as usize].right;
        self.nodes[left as usize].right = node;
        left
    }

    /// Mirror of [`Treap::rotate_right`].
    fn rotate_left(&mut self, node: u32) -> u32 {
        let right = self.nodes[node as usize].right;
        self.nodes[node as usize].right = self.nodes[right as usize].left;
        self.nodes[right as usize].left = node;
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Drive the index-based API the way `StringSet` does: keys live in an
    /// external `Vec`, entry `i` is `keys[i]`, and the returned neighbor
    /// indices are mapped back to keys for easy assertions.
    fn insert(t: &mut Treap, keys: &mut Vec<i32>, key: i32) -> (Option<i32>, Option<i32>) {
        let (pred, succ) = t.insert_and_find_neighbors(|i| key.cmp(&keys[i]));
        keys.push(key);
        (pred.map(|i| keys[i]), succ.map(|i| keys[i]))
    }

    #[test]
    fn single_insert_has_no_neighbors() {
        let mut t = Treap::default();
        let mut keys = Vec::new();
        assert_eq!(insert(&mut t, &mut keys, 5), (None, None));
    }

    #[test]
    fn two_inserts_see_each_other() {
        let mut t = Treap::default();
        let mut keys = Vec::new();
        insert(&mut t, &mut keys, 5);
        assert_eq!(insert(&mut t, &mut keys, 10), (Some(5), None));
        let mut t = Treap::default();
        let mut keys = Vec::new();
        insert(&mut t, &mut keys, 10);
        assert_eq!(insert(&mut t, &mut keys, 5), (None, Some(10)));
    }

    #[test]
    fn middle_insert_gets_both_neighbors() {
        let mut t = Treap::default();
        let mut keys = Vec::new();
        insert(&mut t, &mut keys, 0);
        insert(&mut t, &mut keys, 10);
        assert_eq!(insert(&mut t, &mut keys, 5), (Some(0), Some(10)));
    }

    #[test]
    fn ascending_insertion_order_still_finds_correct_neighbors() {
        // A plain (unbalanced) BST degenerates into a linked list under
        // sorted insertion order; this specifically stresses that the
        // treap's priority-based rotations keep neighbor-tracking correct
        // even though the *tree shape* is being aggressively reshuffled.
        let mut t = Treap::default();
        let mut keys = Vec::new();
        for i in 0..2000 {
            let (pred, succ) = insert(&mut t, &mut keys, i);
            assert_eq!(pred, if i == 0 { None } else { Some(i - 1) });
            assert_eq!(succ, None); // always the largest so far
        }
    }

    #[test]
    fn descending_insertion_order_still_finds_correct_neighbors() {
        let mut t = Treap::default();
        let mut keys = Vec::new();
        for i in (0..2000).rev() {
            let (pred, succ) = insert(&mut t, &mut keys, i);
            assert_eq!(pred, None); // always the smallest so far
            assert_eq!(succ, if i == 1999 { None } else { Some(i + 1) });
        }
    }

    #[test]
    fn scrambled_insertion_matches_brute_force_neighbors() {
        let mut state: u64 = 0xD1CE_BEEF_C0FF_EE00;
        let mut next = || {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
            (state >> 33) as i32
        };
        let mut values: Vec<i32> = (0..3000).collect();
        for i in (1..values.len()).rev() {
            let j = (next() as usize) % (i + 1);
            values.swap(i, j);
        }

        let mut t = Treap::default();
        let mut keys = Vec::new();
        for v in values {
            let (pred, succ) = insert(&mut t, &mut keys, v);

            // `keys` already contains `v` itself (pushed by `insert`); the
            // strict inequalities exclude it from its own neighbor search.
            let brute_pred = keys.iter().copied().filter(|&x| x < v).max();
            let brute_succ = keys.iter().copied().filter(|&x| x > v).min();
            assert_eq!(pred, brute_pred, "predecessor mismatch inserting {v}");
            assert_eq!(succ, brute_succ, "successor mismatch inserting {v}");
        }
    }
}
