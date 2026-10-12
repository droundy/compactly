# Moving the benchmarks to `scaling` 0.9

Status: **plan**, no code yet. It replaces the methodology written on
2026-09-05 — "How to benchmark on this machine" in
[OPTIMIZING.md](../OPTIMIZING.md), `benches/common/mod.rs`, and the
positional-argument convention every bench follows — because the published
`scaling` 0.9.0 is not the unpublished 0.9.0 those were built against. The API
was rewritten: measurement is now *declared* where it is written and run from a
`main` of one call, and the things this project hand-rolled (significance
tests, error propagation, a shell script that assembles a table with `awk`) are
the library's.

Nothing survives untouched. All 12 benches and `comparison/benches/bench.rs`
use `bench_gen_env`, `bench_scaling_gen`, `Stats` or `Config`'s public fields,
and none of those exist any more.

## Status (2026-10-11)

Done, on `scaling-0.9-benches`: `bench`, `strategies` and `coders` are on 0.9;
`atmost` is migrated; `scaling_old`, `benches/common/`, `coder-routes-table.sh`
and the scaling-fit benchmarks are gone; OPTIMIZING.md's methodology and
`Ans`-vs-`Range` tables are regenerated from the new suite, and CLAUDE.md says how
to run and extend it. Not done: the published-vs-HEAD comparison (below), which
no longer has a stage of its own to wait for.

Where the plan changed, and why:

- **A whole-hearted cut, not a port.** The plan carried every bench across. After
  looking at what the port produced, the rule became: a bench stays only if it
  answers a question that is still open and cannot be a candidate or input of
  another group. Cut, with their answers in OPTIMIZING.md and the code at
  `0c7bf74`: `ans-phases` (where ANS time goes), `micro-batch` (`decode_bits`
  against `decode_bit`), `ans-chunk-tracking`, and `bench-arc-str` (frozen
  encodings for a landed decision, which also needed a corpus that is not in the
  repository); the 40 scaling-fit benchmarks in `bench` (39 of them ended
  `(limit)` with `R²=0.000`, which is `scaling` saying the cost is not a power
  law); the `u16`/`i16`/`i64` families, two byte distributions and the zstd rows
  in `strategies`; half the `AtMost` ladder in `coders`.
- **Four targets became three, one with two files.** `coders` is `routes.rs`
  (the grid) and `overlap.rs` (needs `stream`).
- **`atmost` is kept** — it is the check that the tuned walk choices stay
  correct — and then moved into the crate as an ignored test
  (`src/v2/atmost/shootout.rs`), so the forced-walk API it needed could go
  private and the `benchmarking` feature could be deleted. It was migrated with `production` as the baseline, a candidate that
  looks up `Walk::production` at run time, so the baseline follows the tuning
  table instead of duplicating it. Each walk is a candidate for exactly the value
  counts it is valid for, and asserts as much on its first call.
- **The precision target is `scaling`'s default 1%**, as decision 4 says. Decision
  5 (arguments become filters, with a shared parser in `benches/support`) was built
  and then removed: the benches have no argument handling at all, and
  `Config::default()` is the whole policy. To look at less, narrow the `Config`
  in a bench's `main`. The shootout, which needed narrowing most, is four ignored
  tests whose names do it.
- **Inputs are `build_once`.** The local `scaling` gained `build_once` inputs, built
  once per group and lent by reference, which replaced the `thread_local` plus `Rc`
  memoization decision 6 describes. `scaling` is pinned by git revision until a
  release has it.
- **Allocator state turned out to be part of the measurement.** Not in the plan:
  the same `Ans` decode read +160% to +320% against `Range` depending only on
  glibc's heap layout, and a cell run alone differed 2× from the same cell in
  the suite. `benches/support` pins glibc's mmap, trim and top-pad thresholds.
  This changed three recorded conclusions (OPTIMIZING.md, TODO items 4 and 5).
- **`comparison` as a dev-dependency has a cost**: it enables `compactly`'s
  default features, so `cargo test --no-default-features` builds the lib and
  derive tests with `v1`/`v2` on. That CI job ran no tests before (both targets
  have none without the features); it only checked they compile feature-less, and
  no longer does. `cfg_attr`-gating `comparison`'s ~60 `#[compactly(..)]`
  attributes would restore it; not done.

Still to do: the published-vs-HEAD groups and placement control ("Published-vs-HEAD"
below); re-running `strategies` after its trim; deciding whether the now-unused
`Ans::decode_from_forced`, `replay_entropy_decode` and `is_single_chunk` should go.

## Facts this rests on

Verified on 2026-10-10 rather than assumed:

- `scaling` and `scaling-macros` 0.9.0 are on crates.io.
- **The dev-dependency cycle works.** A scratch workspace where `lib`
  dev-depends on `helper` and `helper` depends on `lib` builds and runs its
  bench. So compactly may dev-depend on `comparison`, which depends on
  compactly.
- **Depending on our own published copy works**, same name *and* same version
  from two sources: a package named `compactly` at 0.2.0 with
  `published = { package = "compactly", version = "=0.2.0" }` in
  `[dev-dependencies]` compiled both and ran a bench calling each. No version
  bump needed.
- `~/src/tinyset/benches/{versions,controls}.rs` is a working suite of the
  shape proposed here, by the same author, including the against-the-last-
  release comparison and its controls. **Copy its idioms rather than inventing
  ours.**
- The five design questions below were answered by the session working on
  `scaling`; where the answer is non-obvious it is marked *(upstream)*.

## What the new API gives us

`#[scaling::bench]`, `#[scaling::input]`, `#[scaling::metrics]` and
`#[scaling::bench_scaling]` register through `inventory` at link time;
`Config::default().run_and_print()` discovers everything in the binary,
measures it **interleaved**, and prints it. `Config::run()` hands back a
`Report` instead, reachable by name (`timing`, `comparison`, `metrics`,
`groups`).

Four of those matter more than the rest here:

- **Comparisons are first-class.** A group has candidates and inputs that
  never name each other: every candidate is measured on every input of its
  group with its type, in one interleaved round on the *same* generated
  values, and the difference carries its own error bar — paired, not two
  independent numbers subtracted.
- **Significance is computed**, by a two-tailed Bonferroni-t limit at a 5%
  family-wise error rate over the comparisons the run holds. `atmost.rs`'s
  `SIGMAS = 3.0`, `DistMargin::significant` and the nominate-then-confirm
  dance were a crude stand-in for exactly this.
- **`#[scaling::metrics]`** computes extra columns from what a candidate
  returned — encoded size, compression ratio — printed as `800B (+100%)`
  beside the time. With `scaling::Allocator` installed it also reports peak,
  net, total allocation and allocation count.
- **Filters.** `filter_groups`, `filter_candidates`, `filter_inputs` take a
  name predicate, so which registrations run is the one thing still decided at
  run time.

`quiet-bench` is unchanged: same `reserve`/`restore`/`run`/`status`, same
`SCALING_BENCH_CPUS` self-pinning, same rule that the human runs the `sudo`
halves. Everything in OPTIMIZING.md about quiescing the machine stays as
written.

## The decisions

### 1. Everything is declared

*(upstream)* The imperative `Suite::add` / `InputGroup::add_input` methods are
`pub` on types that are not exported — a leftover of the API tightening in
scaling's PR 19, not a door. Declarative is the only path, and a `macro_rules!`
that emits `#[scaling::bench]` is how a matrix gets declared; `name =` wants a
string literal, so a label travels as `$label:literal`, not `stringify!`.

### 2. `atmost` keeps its matrix, through `types(...)`

*(upstream, built to check)* `types(..)` parses Rust *types*, so
`types(AtMost<3>, AtMost<16>, AtMost<64>)` on a generic candidate registers one
cell per `MAX`, and the same list on a generic `#[scaling::input]` supplies
them. A candidate is paired only with inputs of its own type, so "which walks
apply to this `MAX`" is just the list each walk carries. A const parameter on
the function itself cannot be listed — but `AtMost<MAX>` already wraps it.

So the shootout is a handful of generic functions, one per `Walk`, not the
hundreds of macro-generated ones I first assumed. Encode and decode are
separate groups, as are the two coders; the distributions are the inputs; the
walk `Walk::production` selects is the `baseline`, which is exactly the
comparison the file exists to make.

**What `types(..)` does not give is a name to select by.** *(upstream, listed
from a build)* Every instantiation has the same candidate name and the same
input name — the filters were asked just `"input"` for `AtMost<3>`,
`AtMost<16>` and `AtMost<64>` — and the type shows only in the report, as
`walk_a@input (AtMost<16>)`. So `-- input=max64` could not select one `MAX`.
The candidates stay generic with `types(..)`; the **inputs** are registered by
a small `macro_rules!` per `MAX` with a distinct `name = "max64"`, which both
keeps the type pairing and gives argv something to match. (Or fold the `MAX`
into the group name; the macro is the less invasive.)

### 3. Phase subtraction: the whole is the baseline, the part is a candidate

*(upstream)* There is no sub-phase notion, but there is something better than
the quadrature subtraction `common::difference` does today. Candidates of a
group are measured in the same round on the same input, so
`report.comparison("…")?.against_baseline()` yields timings whose
`Timing::difference()` is a **paired** `Difference` — tighter than quadrature,
which cannot see the correlation. Make the whole the baseline and the phase a
candidate, and the remainder is `-d.ns ± d.std_error`.

Not `uninteresting`: that only measures a candidate roughly.

### 4. The default 1% target *is* this project's precision policy

This reverses the 0.1% decision of 2026-09-05, which the new semantics make
wrong rather than merely expensive.

*(upstream)* In a comparison the relative goal is a **sensitivity**, not a
standard error: sampling stops when the smallest difference that would be
called a change — at the multiple-comparison-corrected level — is at or below
the goal. The default `0.01` therefore already means "stop once a 1%
difference would be significant", which is this project's rule that changes
under 1% are not trusted. `with_relative_error(0.001)` asks for 0.1%
differences, roughly 100× the samples, and under a 10 s cap many groups would
end `(limit)`.

For a standalone benchmark the goal is still a relative standard error, as
before. So:

- comparisons and A/Bs: the default, written out as
  `with_relative_error(0.01)` so it stays put, as tinyset does;
- `with_rough_error(0.1)` (also the default) for `uninteresting` candidates;
- `BENCH_REL_ERROR` / `BENCH_MAX_SECONDS` go away, replaced by flags (below).

Two consequences to keep in mind: `with_max_time` is **per candidate**, so a
group of *k* candidates can take *k* × 10 s; and the correction counts the
comparisons the run holds, so a filtered run is judged more leniently than a
full one. A full run is the reference.

### 5. Arguments become filters

The benches keep taking arguments — the thing worth preserving from the
current design — but they select *which registrations run* rather than which
workload is built:

```
cargo bench --bench coders -- strings          # groups whose name contains "strings"
cargo bench --bench coders -- slice strings    # both words must match the group name
cargo bench --bench coders -- input=max64      # inputs named "max64"
cargo bench --bench coders -- cand=ans         # candidates named "ans"
cargo bench --bench coders -- --quick          # relative error 5%
cargo bench --bench coders -- --time=30        # seconds per candidate
```

**A free argument matches group names only.** *(upstream, built to check)* The
three filters are ANDed across kinds and never see each other's names, so a
free argument matched against group, candidate and input names runs nothing
unless all three contain the text — `n.contains("walk")` on all three ran
nothing because no group or input was called "walk". Two ways out; this plan
takes both:

- **Encode what you select by in the group name.** One group per route ×
  workload (`slice_strings`, `stream_records`, …), `Range` its baseline and
  `Ans` its candidate. That costs the per-route grid across workloads the
  library would have printed, and buys `-- slice strings` meaning what it says.
  Several free arguments narrow further (asking `filter_groups` twice ANDs),
  so a word per axis works.
- **Prefixed arguments feed one filter each**: `group=…`, `cand=…`, `input=…`.
  Rarely wanted, which is the point of keeping them explicit.

Two behaviours to design around, not discover:

- A standalone benchmark (one with no `group`) is asked all three filters,
  with the group and candidate filters given its module-qualified name
  (`bench::alone`) and the input filter given `""`. So **any `input=` text
  drops every standalone benchmark**; a free argument, which touches only
  groups, does not.
- A candidate filter cannot remove a group's baseline — it is kept whenever
  another candidate of the group survives — so `cand=ans` leaves `Range` in
  place, which is what you want from an A/B.

`-`-prefixed arguments are options, which also skips the `--bench` cargo
passes to a `harness = false` target. That parser is ~40 lines in a shared
module and is the only argument handling left.

`COUNT` becomes `sizes(…)` on an input; `CHUNKS` and `RATE_MBPS` become one
input per value (with a distinct `name =`, so `input=` can find it). Where
size-dependence is the question, `#[bench_scaling]` answers it properly.

### 6. Corpora are expensive; make the input a cheap handle

`scaling` builds an input per iteration and clones it per candidate. A
`BTreeSet` of 38k meteorite names built per iteration would swamp the decode
it feeds, so the input *type* must be cheap to produce and to clone.

*(upstream)* The mechanism: the input is a handle — `Rc<Corpus>` or a struct
holding one — and the generator returns a clone of a corpus **memoised in a
`thread_local!` / `OnceLock`**. The generator runs many times a round, so the
build must be cached, not merely cheap. A candidate taking `&I` is then pooled
automatically with no `reuse_input`, and a group that mixes in a `&mut I`
candidate from a non-reuse input (which makes *every* candidate take fresh
inputs) costs almost nothing, since each fresh input is a refcount bump.
tinyset's idiom, with its `Rc<RefCell<…>>` pool, is the heavier version of
this and is for inputs that carry per-iteration state (its random probe); ours
mostly do not.

And the pool may not be needed at all: compactly's `encode` takes `&T` and
allocates its own output `Vec<u8>`, and `decode` takes `&[u8]`. Neither
consumes its input, so
every encode candidate takes `&Rc<Corpus>` (or `&Corpus` through the handle)
and every decode candidate takes the encoded bytes behind an `Rc<[u8]>`. Only
a candidate that really consumes or mutates its input — none that I know of
here — would need `reuse_input` or fresh copies. **Every workload still has to
say which it is**: this is the one place where a careless port would quietly
measure corpus construction.

### 7. Four targets, not one

One binary per *purpose*, because a binary is both the unit of interleaving
("these numbers were measured in the same machine") and the unit of the
family-wise correction (every counted comparison makes the others harder to
call a change). One suite of everything would make the focused A/Bs pay for
the survey grids.

| target | holds | features |
|---|---|---|
| `benches/bench.rs` | compactly `Range`/`Ans`/`v1` against bincode, bincode1, zstd, serde_json, bitcode (all `uninteresting`) on the `comparison` corpora, with size and allocation metrics; the published-vs-HEAD groups, on the `comparison-published` twin; the placement control | default |
| `benches/strategies.rs` | the survey grids: `integers`, `signed`, `bytes` — strategy × type, now printed by the library | `v2` |
| `benches/coders.rs` | the A/B workbench: `Ans` vs `Range` over routes × workloads, the ANS phase splits, `micro-batch`, chunk tracking, async overlap, `Arc<str>` | `v2`, with `benchmarking`/`stream` parts behind `#[cfg(feature)]` |
| `benches/atmost.rs` | the walk shootout, a family of its own | `v2`, `benchmarking` |

`atmost` declares `required-features = ["v2", "benchmarking"]` because it is
nothing without them. `coders` declares only `["v2"]` and gates the forced
walks and the stream routes with `#[cfg(feature = …)]`, so a plain
`cargo bench` still measures the route table instead of cargo silently
skipping the target — the trap CLAUDE.md already warns about.

`coders.rs`'s `main` uses `Config::run()`, prints the `Report`, and then prints
the derived phase remainders; the others are `run_and_print()`. *(upstream)*
`run()` returns a **partial report on Ctrl-C** rather than exiting, so `main`
must check `report.was_interrupted()` before deriving anything from it — a
remainder subtracted from half-measured numbers is not a remainder — and exit
with status 130, as `run_and_print` does, so `cargo bench && next-step` does not
carry on after a partial run.

## Where each current bench goes

| now | becomes |
|---|---|
| `coder-routes.rs` (16k, 20 workloads × 2 coders × 5 routes) | groups in `coders.rs`, one **per route × workload** (`slice_strings`, …) so a free argument can select them: `Range` the baseline and `Ans` the candidate, encoded size a metric. **This is the `coder-routes-table.sh` table, generated by the library.** |
| `coder-routes-table.sh` (128 lines, `awk` Δ propagation) | deleted |
| `atmost.rs` (`DistMargin`, `SIGMAS`, `NOMINATE_FRACTION`, findings) | `atmost.rs`, generic over `AtMost<MAX>`; the statistics deleted |
| `ans-phases.rs` | `coders.rs`, two groups (encode, decode) with the whole as baseline |
| `micro-batch.rs` | `coders.rs`, one group, `seq` baseline vs `batch` |
| `ans-chunk-tracking.rs` | `coders.rs`, one group, `untracked` baseline vs `tracked` |
| `async-decode-overlap.rs` | `coders.rs`, `collect` baseline vs `overlap`, one input per delivery rate |
| `bench-arc-str.rs` (30k) | `coders.rs`, `old`/`btree`/`new` candidates; the missing-corpus skip becomes a `filter_groups` when `ipv4.txt` is absent |
| `integers.rs`, `signed.rs`, `bytes.rs` (`format_time`, `!` marks, hand-built grids) | `strategies.rs`; the formatting deleted |
| `bench.rs` (`stats_alloc`, `mem_allocated`, `FLAT_CELL`/`SCALING_CELL`) | `bench.rs` on `scaling::Allocator` and metrics; `stats_alloc` dev-dependency dropped |
| `comparison/benches/bench.rs` | folded into `bench.rs`; `comparison` keeps its corpora and `src/bin/profile.rs`, loses its `[[bench]]` and its `scaling` dev-dependency |
| `benches/common/mod.rs` (`config`, `report`, `report_env`, `difference`, `per_unit`, `print`, `warn_unless_quiet`) | a much smaller shared module: the argument parser, the `Config` policy, corpus loading, and a quiesce warning over `scaling::quiet::status()` |
| `diff-bench.sh` | keeps its purpose (working tree vs stash) but loses its reason to be the default answer |

## Published-vs-HEAD, and the placement floor

`published = { package = "compactly", version = "=0.2.0" }` in
`[dev-dependencies]`, then per workload a group with the published type as
`baseline` and the current one as candidate — tinyset's `versions.rs` shape.
This is what `diff-bench.sh`'s stash-and-compare was approximating, done
properly: one interleaved run, paired differences, a real significance test.

**Derived types need a twin of the corpus crate, not a renamed dependency.**
`compactly-derive` emits `extern crate compactly;` and unqualified
`compactly::v2::…` paths, so a derive only works in a crate whose own
dependency is named `compactly`. A rename on the bench crate's edge cannot
help: the published crate's derive would resolve back to the path crate.

The fix is to rename one level up. `comparison-published` is a second package
with its own `compactly = "=0.2.0"` (registry) dependency whose `lib.rs`
pulls in `comparison`'s modules by `#[path]`. Cargo resolves each package's
dependency graph separately, so one is built against the path crate and the
other against the published one, and the bench crate dev-depends on both
(`comparison_published = { package = "comparison-published", path = … }`).
Every corpus type then exists twice, each derived against its own compactly,
and a group's baseline is the published encode of the published type.

Verified in a scratch workspace on 2026-10-10: one shared source file with a
`#[derive(compactly::v2::Encode)]` struct, built by a path-compactly crate and
a registry-compactly twin, both encoded from one bench (218 bytes each).
`Ans` exists in 0.2.0, so the `Ans`/`Range` pairing is available on both
sides.

Costs to carry:

- **The shared source must compile against both versions.** It is limited to
  what 0.2.0's API offers. The derive is `compactly::v2::Encode` in both;
  `EncodeV2` is only the proc-macro's own name inside `compactly-derive`, and
  `compactly::v2::EncodeV2` does not resolve (which is how the first scratch
  attempt failed). A corpus type that wants something newer stays out of the
  twin and out of the comparison.
- **Paths inside `comparison`.** `include_str!` is relative to the file it is
  written in, so it keeps working through `#[path]`; anything resolving via
  `CARGO_MANIFEST_DIR` would silently read the twin's directory instead.
- The twin is `publish = false`, like `comparison`, and has no code of its own
  beyond the `#[path]` lines and a dependency.
- **Both corpus crates must register nothing.** *(upstream)* Neither carries
  `#[scaling::…]` attributes, and both lose their `scaling` dev-dependency
  (`comparison`'s `[[bench]]` goes in stage 2 anyway). That is why two copies
  of one source file cause no registry collision. The collision to avoid is the
  *bench* side: a source file containing `#[scaling::…]` included twice into
  one binary gives same-named candidates in a group, which is a
  `RegistrationError`, and same name with the same version from two sources
  cannot be told apart. The macro that emits a published-vs-current pair gives
  each copy a distinct `name =` (a `published:` prefix). Two copies feeding one
  group under the same group name is fine — that is how a group is built.
- When 0.2.0 stops being the last release, bump one version string.

On placement, `scaling`'s own measurements say a function compiled into two
places can read tens of percent apart, which is most of what OPTIMIZING.md
calls the "~1% binary-layout floor" and then some. Two things follow:

- Add `rustflags = ["-C", "llvm-args=-align-all-functions=6"]` to
  `.cargo/config.toml` (which exists, with everything commented out).
  *(upstream)* Recommended for a benchmarking workspace: a `.cargo/config.toml`
  is neither published nor inherited by dependents, so it affects only our own
  builds. Costs: slightly larger code, and an LLVM option whose name could
  change. It took one test from 105 of 432 false "changed" verdicts to 9.
- Carry controls, as tinyset's `controls.rs` does. The cheap one is free and
  should land with the suite: two candidates calling the *same* current
  function, whose gap is pure placement. The old-against-a-copy-of-itself
  control needs a separately built, renamed copy of the released source, which
  cannot be a committed dependency (tinyset's is a local `git archive` copy,
  uncommitted) — document the procedure in OPTIMIZING.md rather than wiring it
  in.

About 8% of placement effect survives alignment between separately built
copies, so a published-vs-HEAD gap smaller than that is not evidence.

## Sequencing

Each stage ends with `cargo bench --all-features` running green, plus the
usual `cargo +nightly fmt --all --check`, both clippy passes and
`cargo check --no-default-features`.

1. **Dependencies and policy.** `scaling = "0.9"` from crates.io (drop the
   `git` key here and in `comparison/Cargo.toml`), the `comparison`
   dev-dependency, the alignment rustflag, and the new shared module: argument
   parser, `Config` policy, quiesce warning. Nothing measured yet.
2. **`bench.rs`.** Fold `comparison/benches/bench.rs` in, move to
   `scaling::Allocator` and metrics, drop `stats_alloc`. First target that
   prints a 0.9 table; the one to check the whole shape against.
3. **`strategies.rs`.** `integers` + `signed` + `bytes`, deleting their
   formatting. Mechanical after stage 2, and it retires the most code per line
   written.
4. **`coders.rs`.** The big one. Route table first (it is also
   `coder-routes-table.sh`'s replacement — delete the script in this stage),
   then phases, then `micro-batch` and chunk tracking, then async overlap and
   `Arc<str>`. Pooled inputs get decided here.
5. **`atmost.rs`.** The `types(..)` rewrite, and the deletion of the
   significance machinery.
6. **Published-vs-HEAD**: the `comparison-published` twin, its groups, and the
   placement control.
7. **Docs.** Rewrite "How to benchmark on this machine" (shorter: quiesce, run
   a target, read the grid, filter by name), the CLAUDE.md benchmarking
   paragraphs, and the "add a workload, not a target" rule — which becomes
   "add a candidate or an input". Re-measure the `Ans`-vs-`Range` tables so
   the numbers in OPTIMIZING.md come from the new suite, and note what the
   1% sensitivity target means for reading them.

Stages 2–5 are independent of each other; 6 depends on 2.

## Risks and wrinkles

- **Run time.** A full `cargo bench --all-features` is now minutes to hours:
  `max_time` is per candidate, and there are a few hundred. The filters are
  the day-to-day interface, and CI should build the benches (`--no-run`) or
  run them with `--time` set very small, never a full run.
- **Registrations in a shared module register in every binary that includes
  it.** *(upstream)* `benches/common/mod.rs` is `mod common;` in each target, so
  anything it declares with `#[scaling::…]` exists in all of them. A metrics
  function marked `allocation` there would make every binary without the
  `Allocator` refuse to run, because that check is before anything runs. The
  shared module registers nothing; allocation metrics live in `bench.rs`.
- **A metrics function per group and return type.** A second metrics function
  for the same return type in one group is a contradiction. Encode candidates
  returning `Vec<u8>` share one `size` metric; where a macro stamps groups out,
  it must emit exactly one per group. Decode candidates return the decoded
  value and need none.
- **One global allocator per binary.** `scaling::Allocator` counts whenever it
  is installed and slows allocation-heavy code a little, the baseline
  included. It belongs in `bench.rs`, where sizes and memory are the point,
  and should stay out of `coders.rs`.
- **Metrics run the candidate once more** after the timing, outside it. Fine
  for everything here, but it means an encode candidate runs one extra time
  and a random input must be seeded or the metric moves between runs.
- **Local corpora.** `ipv4.txt` and `ipv6.txt` are not in the repository, and
  declarations are static, so a group that needs one must be filtered out at
  startup rather than skipped where it is declared.
- **MSRV.** `scaling` needs Rust 1.81. Dev-dependency only, so it bounds who
  can run the benchmarks, not who can use the crate.
- **`cargo bench` and `--all-features` are different families**, since the
  correction counts what the run holds. Worth one sentence in OPTIMIZING.md so
  a tighter verdict in one is not read as a contradiction of the other.
