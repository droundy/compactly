#![warn(missing_docs)]
//! Serialize your data compactly!
//!
//! This crate provides a serialization framework fundamentally similar to
//! [serde](https://docs.rs/serde) or [bincode](https://docs.rs/bincode), which
//! enables you to derive a trait [`Encode`] and then use this trait to
//! [`encode`] and to ['decode`] your data, but much more compactly than bincode
//! or other formats.
//!
//! # How to use
//!
//! ```
//! #[derive(compactly::Encode, bincode::Encode)]
//! struct Point {
//!     x: f64,
//!     y: f64,
//! }
//!
//! #[derive(compactly::Encode, bincode::Encode)]
//! struct Shape {
//!     corners: Vec<Point>,
//! }
//!
//! let square = Shape { corners: vec![
//!     Point { x: 1.0, y: 1.0 },
//!     Point { x: 2.0, y: 1.0 },
//!     Point { x: 2.0, y: 0.0 },
//!     Point { x: 1.0, y: 0.0 },
//! ]};
//!
//! let encoded: Vec<u8> = compactly::encode(&square);
//! let encoded_bincode: Vec<u8> = bincode::encode_to_vec(&square, bincode::config::standard()).unwrap();
//! assert_eq!(encoded.len(), encoded_bincode.len() / 10); // compaclty encoded is less than 10% of bincode
//! ```
//!
//! # Using a stable format
//!
//! If you are encoding your data for temmporary use (e.g. a cache or network
//! transit with the same version of `compactly`), the above works great.
//! However, if you are looking to encode your data persistently across
//! versions, you will want to use `compactly::v1` which will result in a
//! binary-stable format accessible across all future versions of `compactly`.
//! (Or in the future, perhaps you'll want a newer and more compact format.)
//!
//! ## Example
//! ```
//! #[derive(Default, compactly::v1::Encode)]
//! struct Human {
//!     first_name: String,
//!     last_name: String,
//!     ssn: Option<u64>,
//!     year_of_birth: u64,
//! }
//! let encoded: Vec<u8> = compactly::v1::encode(&Human::default());
//! ```
//!
//! # Enabling improved encoding strategies
//!
//! In order for `compactly` to optimally compress your data, you can provide
//! hints (an [`EncodingStrategy`]) as to what kind of distribution of values
//! you expect.  This will change the format, so you'll want to get this right
//! *before* saving your encoded data into long-term storage.
//!
//! ## Example
//! ```
//! #[derive(Default, compactly::v1::Encode)]
//! struct Human {
//!     #[compactly(LowCardinality)]
//!     first_name: String,
//!     #[compactly(LowCardinality)]
//!     last_name: String,
//!     ssn: Option<u64>,
//!     #[compactly(Small)]
//!     year_of_birth: u64,
//! }
//! let encoded: Vec<u8> = compactly::v1::encode(&Human::default());
//! ```
//!
//! ## Encoding strategies
//!
//! |Strategy  | Meaning | Effect |
//! |----------|---------|--------|
//! | [Normal] | Default strategy | Encode based on data type alone. |
//! | [Small]  | Values are small | Use a var-int encoding, or whatever might be appropriate for "small" data of this type. |
//! | [Decimal]| Numbers may be decimals | Optimize for floating point numbers encoded with limited decimal precision.  Any data may be stored compactly, but this will take etra time to check if values could be *more* compactly stored as decimals. |
//! | [LowCardinality] | Low cardinality | There are few values which are frequently repeated, so store each value only once.  Be aware that this could double memory use, as it will store a mapping between values and `usize`. |
//! | [Sorted] | Values probably sorted | Assume that the values are likely to arrive in sorted order.  Typically this will lead to storing differences between successive values. |
//! | [Compressible] | Expensive compression may be used | Take whatever time is needed to compress this data.  For `String` and `Vec<u8>` this enables [LZ77-style compression](https://en.wikipedia.org/wiki/LZ77_and_LZ78) which can be very slow, but also can provide very good compression for natural language data. |
//! | [Values<S>] | Apply strategy to values of a collection | e.g. `Values<Small>` assumes all values in a `Vec` or `HashSet` are small |
//! | [Mapping<K,V>] | Apply strategies to keys and values of a collection | e.g. `Mapping<Sorted,Decimal>` is the `Normal` strategy for a `BTreeMap`, but you might prefer a `Mapping<LowCardinality,Small>` if you will be storing a large collection of these maps with a limited number of keys, and the values are small. |
//!
//! # How does compactly work?
//!
//! This crate encodes data using
//! [adaptive](https://en.wikipedia.org/wiki/Adaptive_coding) [range
//! coding](https://en.wikipedia.org/wiki/Range_coding).  Each type that can be
//! encoded (and really each strategy for each type) has a
//! [Context][Encode::Context]. which is a type that holds the model for the
//! distribution of values. As the data is necoded, this model is updated (this
//! is the essence of [adaptive
//! coding](https://en.wikipedia.org/wiki/Adaptive_coding)),
//!
//! At its core, the encoding is done on a bit-by-bit manner, i.e. each type has
//! a fundamental bitwise encoding, and the `Context` stores the probability of
//! each bit being 1 or 0.  Most types have a relatively "clever" encoding such
//! that even without adaptive coding (i.e. learning the patterns from your
//! actual data), common values should be encoded in fewer bits.
//!
//! When you derive [`Encode`] for a struct (or enum), compactly will create a
//! new [`Encode::Context`] which stores distinct `Context` values for each
//! field of your struct (or enum), which means that as your data is encoded,
//! compactly will adaptivly learn the distinct patterns of values for each
//! field.

pub mod ans;
mod encoded;
pub mod v1;

pub use v1::{decode, encode, Encode};

/// A wrapper around a value causing it to be encoded with a particular strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoded<T, S> {
    value: T,
    _phantom: std::marker::PhantomData<S>,
}

/// The default strategy for encoding data.
///
/// This exists so that code may be written only once that needs to be able to
/// handle any strategy.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Normal;

/// A strategy for encoding values that are small.
///
/// e.g. if there are integers then they should be small integers.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Small;

/// A strategy for encoding values that are particularly compressible.
///
/// For instance, this will attempt to apply Lz77-like encoding to strings.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compressible;

/// A strategy for encoding values that cannot be compressed.
///
/// Examples would be encrypted or random bytes.  In this case, `compactly`
/// abandons any attempt at compression (e.g. variable bitlength) and also
/// adopts a faster algorithm where possible.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Incompressible;

/// A strategy for encoding values that have been sorted.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sorted;

/// A strategy for encoding values that are often repeated.
///
/// This can be shockingly efficient when there are just a few values for e.g. a
/// string field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LowCardinality;

/// A strategy for encoding floating point values that have round decimal values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal;

/// Apply the respective strategies to keys and values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mapping<K, V> {
    _key: std::marker::PhantomData<K>,
    _value: std::marker::PhantomData<V>,
}

/// Apply this strategy to values held inside.
///
/// This applies to any sort of collection such as a `Vec` or a `HashSet`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Values<V> {
    _value: std::marker::PhantomData<V>,
}
