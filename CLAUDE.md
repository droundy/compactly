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

`rustfmt.toml` sets `imports_granularity = "Module"`, which is an unstable
rustfmt option. Stable rustfmt doesn't reject it — it prints a
can't-set-this-option warning and exits 0 — so a stable `cargo fmt --check`
(what the pre-commit hook runs) passes even on unmerged, nested imports and
enforces nothing about them. CI's `rustfmt` job therefore runs on nightly,
where the option is actually applied — a similar gap to the clippy
stable/nightly one above, but here it means the CI job can fail on imports
the pre-commit hook saw as clean. Reproduce locally with `cargo +nightly fmt
--all --check` (needs `rustup component add rustfmt --toolchain nightly`).

Features `v1` and `v2` are both on by default. The optional `generate_bit_context` feature enables tools for regenerating the pre-computed `bit_context.rs` files.

The optional `benchmarking` feature exposes the benchmark-support API — forced
tree walks (`Walk`, `WALKS`, `encode_atmost_batch`/`decode_atmost_batch`),
forced decoder instantiations (`Ans::decode_from_forced`), and entropy-phase
replay (`replay_entropy_decode`, `is_single_chunk`). These bypass the choices
the library makes for itself and some silently produce wrong answers on the
wrong input, so they are off by default and not covered by semver. Anything
that calls them (`benches/atmost.rs`, `src/bin/ans-decode-phases.rs`,
`src/bin/just-decompress-stream.rs`) needs `benchmarking` in its
`required-features`, **and** cargo silently *skips* targets whose
required-features are off — so a lint or build break in them hides unless the
feature is named. That is why CI clippies twice and runs `cargo test
--all-features`. Items the lib's own unit tests also use are gated
`#[cfg(any(test, feature = "benchmarking"))]` so plain `cargo test` keeps its
coverage.

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

The library has two binary formats, each living in its own module:

| Module | Coder | Stability | Notes |
|--------|-------|-----------|-------|
| `compactly::v1` | Arithmetic/range coding | **Frozen** — guarded by `tests/v1-encoding` | Written to any `std::io::Write` |
| `compactly::v2` | ANS (Asymmetric Numeral Systems) | **Not stable; still free to change** | Default re-exported as `compactly::{encode, decode, Encode}` |

Both versions share the same overall design — only the entropy coder differs.

**`v2` is not a frozen format.** There is no `v2-encoding` stability test and none
is wanted yet: changes that move the bitstream (context seeds, tree shapes, chunk
boundaries) are still fair game, and are made on their merits. Only `v1` is
committed to. Do not add a v2 stability test, and do not reject a v2 change on
compatibility grounds, without an explicit decision to freeze it.

### Core traits

**In `v2`** ([src/v2/mod.rs](src/v2/mod.rs)):
- `Encode<S = Normal>` — types that can be encoded; has an associated `Context` (the adaptive probability model) and `encode`/`decode` **associated functions** (they take `value: &Self`, not `&self`, so a type with several strategies has no ambiguous method call). `S` is the encoding strategy: `Encode<Small> for u64` is the same type coded a different way, selected per field via `#[compactly(Small)]`. There is no separate `EncodingStrategy` trait — the strategy is a parameter, so wrapper types like `Option<T>` and `Box<T>` lift *every* strategy generically in one impl.
- There is **no** `.encode(...)` method — `Encode::encode` takes `value: &Self`, so every call goes through a strategy: `Normal::encode(&v, coder, ctx)` for the default, `Small::encode(&v, coder, ctx)` for a named one. A `&self` method could only ever sugar the default strategy (and could not sugar `decode` at all, which has no receiver), so it would leave two spellings for one operation; it was tried and removed. Size estimation is the crate-private `v2::millibits(&value)`, used by the size tests.
- `Strategy` — opt-in marker on the strategy types giving `Small::encode(&v, coder, ctx)` / `Small::decode(r, ctx)` syntax. Not blanket-implemented (that would make `u8::encode` ambiguous) and not inherent methods (that would collide with v1's identically-spelled calls on the same shared marker types).
- `EntropyCoder` — something that can accept bits with probabilities (`Range`, `Ans`, `Millibits` all implement this)
- `EntropyDecoder` — the read side

**In `v1`** ([src/v1/mod.rs](src/v1/mod.rs)):
- Same `Encode` trait shape but uses `Writer<W: std::io::Write>` / `Reader<R: std::io::Read>` instead of the trait objects above

### Strategy types

Defined in [src/lib.rs](src/lib.rs) and implemented per-type across the `v1/` and `v2/` subdirectories:
`Normal`, `Small`, `Compressible`, `Incompressible`, `Sorted`, `LowCardinality`, `Decimal`, `Mapping<K,V>`, `Values<V>`

### Probability model

`BitContext` ([src/v2/bit_context.rs](src/v2/bit_context.rs)) is a **generated** file — a state machine tracking the running count of true/false bits to maintain an adaptive probability estimate. Do not edit it by hand; it is regenerated via the `generate_bit_context` feature and the tools in `src/v2/generate_bit_context.rs`.

### Entropy coders in v2

Three `EntropyCoder` implementations, each useful for different purposes:

| Type | Purpose |
|------|---------|
| `Range` | Default encoder; arithmetic/range coding; what `v2::encode` uses |
| `Ans` | ANS encoder; same interface, potentially faster |
| `Millibits` | Size estimation only; does not produce bytes |

### Derive macros

[compactly-derive/src/lib.rs](compactly-derive/src/lib.rs) exposes three derives via `synstructure`:
- `Encode` (alias for `EncodeV1`) — generates `v1::Encode` impl
- `EncodeV1` — explicit v1 derive
- `EncodeV2` — generates `v2::Encode` impl

The derive generates a `Context` struct with one field per struct/enum field (each field's own `Context` type), enabling per-field adaptive learning.
