mod encoded;
pub mod v0;
pub mod v1;

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Values<V> {
    _value: std::marker::PhantomData<V>,
}
