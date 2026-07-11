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
cargo clippy --all-targets --workspace  # lint; expected to pass cleanly
cargo check --no-default-features  # CI checks this; run before pushing
```

`cargo clippy --all-targets --workspace` is expected to pass with no warnings. In
test code, prefer `#[allow(...)]` for noisy lints over restructuring the test.
CI runs clippy on the *newest* stable, which may know lints the locally
installed clippy doesn't — if CI's clippy job fails while local clippy passes,
read the CI log and apply its suggested fix (or `rustup update stable`).

Features `v1` and `v2` are both on by default. The optional `generate_bit_context` feature enables tools for regenerating the pre-computed `bit_context.rs` files.

CI also builds with `--no-default-features` (including a wasm target), so it
compiles every target without `v1`/`v2`. The usual failure is a new `src/bin/`
binary that uses `compactly::v2` without a matching
`required-features = ["v2"]` entry under `[[bin]]` in Cargo.toml — add that
entry whenever you add a binary. The pre-commit hook runs
`cargo check --no-default-features` to catch this before CI does.

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
