# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
cargo build                        # build all crates
cargo test                         # run all tests
cargo test test_name               # run a single test by name
cargo test --test derive           # run integration tests in tests/derive.rs
cargo test --test v1-encoding      # run v1 stability tests
cargo bench                        # run all benchmarks
cargo bench --bench bench          # run main benchmark suite
```

Features `v1` and `v2` are both on by default. The optional `generate_bit_context` feature enables tools for regenerating the pre-computed `bit_context.rs` files.

## Performance work

[OPTIMIZING.md](OPTIMIZING.md) tracks the ongoing effort to make decoding faster
without harming the compression rate: how to benchmark reliably on this (noisy)
machine, empirical results, dead ends to avoid, and a prioritized TODO list.
Update it as that work progresses.

## Architecture

This is a Rust serialization library that encodes data using **adaptive entropy coding** — significantly more compact than formats like bincode. There are three crates in the workspace:

- **`compactly`** — the main library ([src/](src/))
- **`compactly-derive`** — proc-macro crate providing `#[derive(Encode)]` ([compactly-derive/](compactly-derive/))
- **`comparison`** — benchmarks comparing compactly against other crates ([comparison/](comparison/))

### Format versions

The library has two independently stable binary formats, each living in its own module:

| Module | Coder | Notes |
|--------|-------|-------|
| `compactly::v1` | Arithmetic/range coding | Written to any `std::io::Write` |
| `compactly::v2` | ANS (Asymmetric Numeral Systems) | Default re-exported as `compactly::{encode, decode, Encode}` |

Both versions share the same overall design — only the entropy coder differs.

### Core traits

**In `v2`** ([src/v2/mod.rs](src/v2/mod.rs)):
- `Encode` — types that can be encoded; has an associated `Context` (the adaptive probability model) and `encode`/`decode` methods
- `EncodingStrategy<T>` — alternate encodings for a type (e.g. `Small`, `LowCardinality`); plug-in strategies used via `#[compactly(Small)]` derive attributes
- `EntropyCoder` — something that can accept bits with probabilities (`Range`, `Ans`, `Millibits`, `Raw` all implement this)
- `EntropyDecoder` — the read side

**In `v1`** ([src/v1/mod.rs](src/v1/mod.rs)):
- Same `Encode` trait shape but uses `Writer<W: std::io::Write>` / `Reader<R: std::io::Read>` instead of the trait objects above

### Strategy types

Defined in [src/lib.rs](src/lib.rs) and implemented per-type across the `v1/` and `v2/` subdirectories:
`Normal`, `Small`, `Compressible`, `Incompressible`, `Sorted`, `LowCardinality`, `Decimal`, `Mapping<K,V>`, `Values<V>`

### Probability model

`BitContext` ([src/v2/bit_context.rs](src/v2/bit_context.rs)) is a **generated** file — a state machine tracking the running count of true/false bits to maintain an adaptive probability estimate. Do not edit it by hand; it is regenerated via the `generate_bit_context` feature and the tools in `src/v2/generate_bit_context.rs`.

### Entropy coders in v2

Four `EntropyCoder` implementations, each useful for different purposes:

| Type | Purpose |
|------|---------|
| `Range` | Default encoder; arithmetic/range coding; what `v2::encode` uses |
| `Ans` | ANS encoder; same interface, potentially faster |
| `Millibits` | Size estimation only; does not produce bytes |
| `Raw` | Bit-packing without probability weighting; useful for testing encoding shape |

### Derive macros

[compactly-derive/src/lib.rs](compactly-derive/src/lib.rs) exposes three derives via `synstructure`:
- `Encode` (alias for `EncodeV1`) — generates `v1::Encode` impl
- `EncodeV1` — explicit v1 derive
- `EncodeV2` — generates `v2::Encode` impl

The derive generates a `Context` struct with one field per struct/enum field (each field's own `Context` type), enabling per-field adaptive learning.

### Testing patterns

Tests inside module files use internal macros:
- `assert_bits!(value, n_bits)` — encode 64 copies, check round-trip and bit count
- `assert_size!(value, n_bytes)` — encode once, check round-trip and byte count
- `assert_ans_bits!` / `raw_bits!` — same for specific coders

The bit/size assertions are **not correctness tests** — they exist so that any change to the encoding logic produces a visible diff in the expected counts, making it easy to observe the compression impact of a change. If you improve (or accidentally worsen) compression, these tests will fail and the new numbers will show exactly how much the encoding size shifted.

Integration tests in [tests/](tests/) require both features (`#![cfg(all(feature = "v1", feature = "v2"))]`) and compare v1 vs v2 encoding sizes.
