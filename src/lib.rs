mod encoded;
pub mod v0;
pub mod v1;

/// A wrapper around a value causing it to be encoded with a particular strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Encoded<T, S> {
    value: T,
    _phantom: std::marker::PhantomData<S>,
}

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

/// A strategy for encoding values that are often repeated.
///
/// This can be shockingly efficient when there are just a few values for e.g. a
/// string field.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LowCardinality;

/// A strategy for encoding floating point values that have round decimal values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal;

/// Apply the strategy `K` to keys.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Keys<K: Default + Clone> {
    _phantom: std::marker::PhantomData<K>,
}

/// Apply the strategy `V` to values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Values<V: Default + Clone> {
    _phantom: std::marker::PhantomData<V>,
}

/// Apply the respective strategies to keys and values.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeysValues<K: Default + Clone, V: Default + Clone> {
    _key: std::marker::PhantomData<K>,
    _value: std::marker::PhantomData<V>,
}
