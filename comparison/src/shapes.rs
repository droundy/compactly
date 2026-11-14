//! The types the benches derive `Encode` for.
//!
//! They live here because the derive binds to its crate's own `compactly`, so a
//! bench could only ever derive them against this checkout. Here, `comparison`'s
//! twins derive them against each other compactly, which is what lets a bench put
//! this checkout and `main` in one group on the same data.

use std::collections::BTreeMap;

use crate::meteorites::MeteoriteData;

/// A record shaped the way streamed data usually is: a short string beside a
/// couple of integers — the shape callers actually stream.
#[derive(Debug, Clone, PartialEq, compactly::v2::Encode)]
pub struct Record {
    pub id: u64,
    pub count: u32,
    pub name: String,
    pub active: bool,
}

/// The same record with the integer side widened, which holds the number of
/// string characters fixed while adding non-string work around them.
#[derive(Debug, Clone, PartialEq, compactly::v2::Encode)]
pub struct WideRecord {
    pub id: u64,
    pub count: u32,
    pub a: u64,
    pub b: u64,
    pub c: u32,
    pub d: u16,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, compactly::v2::Encode)]
pub enum ThreeOptions {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, compactly::v2::Encode)]
#[rustfmt::skip]
pub enum SeventeenOptions {
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q,
}

/// A map whose values are mapped with a strategy of their own, which is not
/// something `BTreeMap<String, _>` alone says.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, compactly::v2::Encode)]
pub struct MetNames {
    #[compactly(Mapping<Compressible, Normal>)]
    pub mets: BTreeMap<String, MeteoriteData>,
}
