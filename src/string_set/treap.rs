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

#[derive(Clone)]
struct Node<K, V> {
    key: K,
    value: V,
    priority: u64,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

/// An ordered map (by `K: Ord`) supporting
/// [`Treap::insert_and_find_neighbors`]: insert a new key and report its
/// immediate predecessor and successor (by `V`) in one tree walk.
#[derive(Clone)]
pub(super) struct Treap<K, V> {
    root: Option<Box<Node<K, V>>>,
    rng: SplitMix64,
}

impl<K, V> Default for Treap<K, V> {
    fn default() -> Self {
        Self {
            root: None,
            rng: SplitMix64::new(),
        }
    }
}

impl<K: Ord, V: Copy> Treap<K, V> {
    /// Insert `key` (assumed absent -- see [`super::StringSet::insert_new`])
    /// with `value`, returning the values of its immediate predecessor and
    /// successor by key order, if any.
    pub(super) fn insert_and_find_neighbors(&mut self, key: K, value: V) -> (Option<V>, Option<V>) {
        let priority = self.rng.next();
        let mut pred = None;
        let mut succ = None;
        self.root = Some(Self::insert(
            self.root.take(),
            key,
            value,
            priority,
            &mut pred,
            &mut succ,
        ));
        (pred, succ)
    }

    fn insert(
        node: Option<Box<Node<K, V>>>,
        key: K,
        value: V,
        priority: u64,
        pred: &mut Option<V>,
        succ: &mut Option<V>,
    ) -> Box<Node<K, V>> {
        let Some(mut node) = node else {
            return Box::new(Node {
                key,
                value,
                priority,
                left: None,
                right: None,
            });
        };
        match key.cmp(&node.key) {
            Ordering::Less => {
                *succ = Some(node.value);
                node.left = Some(Self::insert(
                    node.left.take(),
                    key,
                    value,
                    priority,
                    pred,
                    succ,
                ));
                if node.left.as_ref().unwrap().priority > node.priority {
                    node = rotate_right(node);
                }
            }
            Ordering::Greater => {
                *pred = Some(node.value);
                node.right = Some(Self::insert(
                    node.right.take(),
                    key,
                    value,
                    priority,
                    pred,
                    succ,
                ));
                if node.right.as_ref().unwrap().priority > node.priority {
                    node = rotate_left(node);
                }
            }
            Ordering::Equal => {
                unreachable!("Treap::insert_and_find_neighbors requires an absent key")
            }
        }
        node
    }
}

/// Standard BST right rotation: promotes `node`'s left child to the root of
/// this subtree, demoting `node` to be that child's right child.
fn rotate_right<K, V>(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut left = node.left.take().expect("rotate_right needs a left child");
    node.left = left.right.take();
    left.right = Some(node);
    left
}

/// Mirror of [`rotate_right`].
fn rotate_left<K, V>(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut right = node.right.take().expect("rotate_left needs a right child");
    node.right = right.left.take();
    right.left = Some(node);
    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_insert_has_no_neighbors() {
        let mut t: Treap<i32, i32> = Treap::default();
        assert_eq!(t.insert_and_find_neighbors(5, 5), (None, None));
    }

    #[test]
    fn two_inserts_see_each_other() {
        let mut t: Treap<i32, i32> = Treap::default();
        t.insert_and_find_neighbors(5, 5);
        assert_eq!(t.insert_and_find_neighbors(10, 10), (Some(5), None));
        let mut t: Treap<i32, i32> = Treap::default();
        t.insert_and_find_neighbors(10, 10);
        assert_eq!(t.insert_and_find_neighbors(5, 5), (None, Some(10)));
    }

    #[test]
    fn middle_insert_gets_both_neighbors() {
        let mut t: Treap<i32, i32> = Treap::default();
        t.insert_and_find_neighbors(0, 0);
        t.insert_and_find_neighbors(10, 10);
        assert_eq!(t.insert_and_find_neighbors(5, 5), (Some(0), Some(10)));
    }

    #[test]
    fn ascending_insertion_order_still_finds_correct_neighbors() {
        // A plain (unbalanced) BST degenerates into a linked list under
        // sorted insertion order; this specifically stresses that the
        // treap's priority-based rotations keep neighbor-tracking correct
        // even though the *tree shape* is being aggressively reshuffled.
        let mut t: Treap<i32, i32> = Treap::default();
        for i in 0..2000 {
            let (pred, succ) = t.insert_and_find_neighbors(i, i);
            assert_eq!(pred, if i == 0 { None } else { Some(i - 1) });
            assert_eq!(succ, None); // always the largest so far
        }
    }

    #[test]
    fn descending_insertion_order_still_finds_correct_neighbors() {
        let mut t: Treap<i32, i32> = Treap::default();
        for i in (0..2000).rev() {
            let (pred, succ) = t.insert_and_find_neighbors(i, i);
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

        let mut t: Treap<i32, i32> = Treap::default();
        let mut inserted: Vec<i32> = Vec::new();
        for v in values {
            let (pred, succ) = t.insert_and_find_neighbors(v, v);

            let brute_pred = inserted.iter().copied().filter(|&x| x < v).max();
            let brute_succ = inserted.iter().copied().filter(|&x| x > v).min();
            assert_eq!(pred, brute_pred, "predecessor mismatch inserting {v}");
            assert_eq!(succ, brute_succ, "successor mismatch inserting {v}");

            inserted.push(v);
        }
    }
}
