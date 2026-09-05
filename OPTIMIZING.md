# Optimizing decode (and encode) speed

Working notes on the effort to make decoding faster (primary goal) without
harming the compression rate. Read this together with the git log — several
commits below are the durable result of experiments recorded here.

Our focus for optimization is the `v2` encoder in `src/v2/`  This has two
entropy coders `Range` and `Ans`.  `Range` is currently the default, but `Ans`
is faster at decoding and may become the default in the future.  We want to
optimize both approaches with a slight focus on `Ans`.

"Faster at decoding" is **12–32% across the workload set** and the rate is a
wash (≤0.06%) — but *which decode route* changes the answer, and on
incompressible data through `decode_from` it reverses to **+327%**. See
[`Ans` against `Range`](#ans-against-range-across-the-workload-set-2026-08-28)
below, which is the place to start before any decision that turns on which coder
wins, and `./coder-routes-table.sh` to bring its table up to date.

## How to benchmark on this machine

Every measurement in this project goes through the [`scaling`] crate. It
samples a benchmark until the **standard error** of its mean is under 0.1%,
prints that error beside the number, and marks the line `(limit)` if it ran
out of time first or `(untrusted)` if it took too few samples for the error
bar itself to mean anything. That replaces the whole apparatus this section
used to describe — iteration counts, `perf stat` cycle counts, min-of-N, and
eyeballing whether a difference was real.

[`scaling`]: https://github.com/droundy/scaling

- **Quiesce the machine first — a human task, not Claude's.** The human runs
  ``sudo `which quiet-bench` reserve 2`` to reserve CPU 2 (a P-core): turbo
  off (kills thermal drift), performance governor, SMT sibling cpu3 offlined,
  all other processes and IRQs herded onto the remaining CPUs, ASLR off.
  ``sudo `which quiet-bench` restore`` (or a reboot) undoes it. `quiet-bench`
  ships with `scaling`; Claude: never run it, or anything else, under sudo.
  - **Check the setup is active:** `quiet-bench run true` exits 0 iff a
    reservation exists. (`quiet-bench status` answers a different question —
    whether *this* process is on it.) The reservation lives in
    `/run/quiet-bench.cpus`, on tmpfs, so it cannot be stale after a reboot.
    If the check fails, stop and ask the user to reserve a CPU; measurements
    on an unquiesced machine are not worth taking.
  - **Run every benchmark through `quiet-bench run <cmd…>`.** It pins the
    command to the reserved CPUs and advertises them in `SCALING_BENCH_CPUS`,
    which `scaling` reads to pin itself from inside the process too. Anything
    not run through it lands on the crowded housekeeping CPUs and gains
    nothing. Every benchmark binary here says so on stderr when it notices.
    Build *outside* the wrapper, so compilation is not pinned to one core.
- **Check load first:** `top -b -n1 | grep %Cpu` — want >90% idle.
- **Reading a result.** A line like `15.789ms ± 0.016ms` means the standard
  error of the mean is 16µs. Two results differ meaningfully when the gap
  between them is several times the larger `±`; that is the whole test, and
  it is why the error bar is printed in the same unit as the value. `(limit)`
  means less precise than asked for — look at the `±` to see how much less.
  `(untrusted)` means the `±` is not evidence of anything.
- **What the `±` does not cover.** It is the sampling error *within one
  process*: it cannot see CPU frequency state, code layout, or anything else
  that differs between separate runs of separate builds. So:
  - Comparing two arms of the **same binary** (`coder-routes ans` vs
    `coder-routes range`, `micro-batch seq` vs `batch`) — the `±` is the
    whole story. No alternation needed.
  - **Turbo off is what makes wall-clock trustworthy here.** Before it was,
    a back-to-back "all of A, then all of B" comparison once showed a uniform
    fake −15% (2026-07-04) — including on datasets the change could not
    touch — because the fan spun up mid-run and later runs landed on a slower
    clock. `quiet-bench reserve` pins the frequency, which is why alternation
    is now only needed across builds.
  - Comparing **across commits or builds**, the floor is about **±1% of
    binary-layout noise** on workloads dominated by library/runtime code:
    e.g. the `strings` workload spends >50% in `BTreeMap::insert`+`memcmp`,
    and those identical functions were measured 4–6% apart between two builds
    differing only in compactly code. **Alternate the two builds** and treat
    anything under ~1% as unresolved however tight the `±` looks. This is
    also why 0.1% is the precision target and not something smaller: below
    the layout floor there is nothing left to buy.
- **Precision knobs.** `BENCH_REL_ERROR` (default `0.001`) and
  `BENCH_MAX_SECONDS` (default `10`) override the target and the backstop —
  `BENCH_REL_ERROR=0.01` for a quick sweep over many cells. They are read by
  `compactly::benchmarking::config`, which every binary below uses.
- **Everything under `src/bin/` needs `--features benchmarking`.** That is
  what pulls in `scaling` (a dev-dependency would not reach `src/bin`) as
  well as the forced-walk and forced-decoder hooks. Cargo **silently skips** a
  target whose required-features are missing — you get no error, just no
  binary — so build them as e.g. `cargo build --release --features
  benchmarking --bin coder-routes`, adding `,stream` for the async ones.
- **`coder-routes` is the workload runner**, and most questions about a
  workload are a run of it:

  ```
  [COUNT=n] [CHUNKS=n] coder-routes <workload> [ans|range] \
      [slice|from|stream|encode|encode-to]
  ```

  Workloads: `u64` and `u64-seq` (`COUNT` values, random and consecutive),
  `strings` (a `BTreeSet<String>` of 38k meteorite names — THE per-character
  `char`/`u8` tree-walk workload), `enums` and `enums17` (100k skewed
  3-variant / uniform 17-variant enums, the `AtMost` discriminant path
  through the derive), `floats` (100k non-integer `f64`, the incompressible
  raw-bits path), `compressible` (the meteorite CSV through Lz77), `records`
  and `records-wide` (a short string beside integers, the shape callers
  stream), `ipv6` (from `ipv6.txt` in the cwd), and `atmost<N>` for N in the
  monomorphized ladder (3 4 6 8 12 16 24 32 64 128), 50k uniform values —
  the depth sweep that located the per-coder prefetch crossover. All but the
  `u64`s, `enums`, `floats`, `ipv6` and the ladder read
  `comparison/src/meteorites.csv`, so run from the workspace root.

  It prints the time per call, the time per element, and a `result …` line
  for `./coder-routes-table.sh` to parse. **This bin replaced a dozen
  one-workload bins** (`just-decompress`, `just-compress-strings`,
  `range-decode-collapse`, `async-decode-cost`, …) on 2026-09-05; entries
  below that name those are records of runs made before that, and the
  equivalent is a `coder-routes` invocation with the matching workload and
  route.
- The remaining bins each measure something `coder-routes` structurally
  cannot:
  - `micro-batch seq|batch` — the ANS adaptive bit-decode with nothing else in
    the loop: a stream of independent adaptive bits through `decode_bit`
    (`seq`) vs `decode_bits` (`batch`), via two `Encode` impls that encode
    byte-for-byte identically. Best signal for batch-coder work.
  - `ans-phases encode|decode` — splits either direction into its two phases
    (model work vs entropy coding), the second isolated by subtraction with
    the error bars added in quadrature.
  - `[COUNT=n] ans-chunk-tracking untracked|tracked` — forces `AnsDecoder`'s
    two `CHUNKED` instantiations on the same bytes, isolating the per-op
    `ops_left` check. Single-chunk input only; it asserts that.
  - `[COUNT=n] [CHUNKS=n] [RATE_MBPS=n] async-decode-overlap collect|overlap|both`
    — does decoding actually overlap the wait for the next chunk? A property
    of a delivery schedule, so it needs the schedule; `coder-routes … stream`
    answers the different question of what the machinery costs with every
    byte already in hand.
  - `bench-arc-str` — frozen copies of the superseded `Arc<str>` encodings
    beside the current one, so a landed decision can be re-measured without
    checking out an old commit.
- `cargo bench` runs the survey tables in `benches/`, which use `scaling`'s
  default 1% target rather than the 0.1% above: they compare codecs that
  differ by factors, so a percent is precision to spare. A cell those tables
  could not pin down to 1% is marked `!`. `benches/atmost.rs` is the
  exception — it is an A/B, uses the 0.1% config, and reports each margin
  with the error bar the two measurements imply.
- Instruction counts are no longer part of the method. Decode is
  **latency-bound** (measured IPC ≈ 1.39), so fewer instructions can still be
  slower; `perf stat -e cpu_core/instructions/` remains useful when
  *explaining* a result, but never for deciding one. If you do reach for
  `perf`, the `cpu_core/` prefix and the pinning are both load-bearing on this
  hybrid CPU — a bare `-e cycles` counts on one core type while the process
  migrates between them, and silently reports a fraction of the work.

## Empirical results so far

### `Ans` against `Range` across the workload set (2026-09-05)

"`Ans` is faster at decoding" is asserted at the top of this document; this is
the measurement behind it, and the places it does not hold.

**Regenerate with `./coder-routes-table.sh`** — it prints both tables below on
stdout, ready to paste back, and takes a couple of minutes. `-q` for a quicker
1%-precision pass, `-d`/`-e` for one table, or name workloads
(`./coder-routes-table.sh strings records`) to refresh a few rows. It refuses to
run unless the machine is quiesced.

Every cell runs `src/bin/coder-routes.rs`, which puts the same value through
every route either coder supports: decoding by `slice` (the borrowing decoder),
`from` (`decode_from` over a `&[u8]` used as a `Read`) and `stream`
(`decode_stream` over a 64-chunk source); encoding by `encode` (to a fresh
`Vec`) and `encode-to` (`encode_to` into a fresh `Vec` used as a `Write`).
**There is no async encode for either coder**, which is why the encode table has
two routes where the decode table has three — worth keeping in view, because it
means "async" here is decode only. **Both arms of a comparison are the same
binary** with different arguments, so binary-layout noise cancels and the `±` on
each cell is the whole uncertainty — no alternation, and none of the min-of-N
this table used to need.

Times are per operation, `±` one standard error; the Δ column carries the error
those two imply and marks with `?` any difference smaller than three of them.
Earlier versions of this table were in `perf` cycle counts, so its numbers are
not comparable digit-for-digit with what the git history shows — the
*percentages* are, and they agree to about half a point where both exist.

Measuring only the `slice` route gets two of the conclusions below wrong, so
the route is not a detail — which is why `coder-routes` takes the route as an
argument rather than picking one.

#### Decode

| workload | route | Range | Ans | Δ | size Δ |
|---|---|---|---|---|---|
| `strings` | `slice` | 12.490±0.011ms | 10.041±0.010ms | **-19.6±0.1%** | +0.04% |
| `strings` | `from` | 12.477±0.012ms | 10.428±0.002ms | **-16.4±0.1%** | +0.04% |
| `strings` | `stream` | 12.476±0.012ms | 10.095±0.009ms | **-19.1±0.1%** | +0.04% |
| `enums` | `slice` | 3.190±0.003ms | 2.435±0.002ms | **-23.7±0.1%** | +0.06% |
| `enums` | `from` | 3.235±0.003ms | 2.370±0.002ms | **-26.8±0.1%** | +0.06% |
| `enums` | `stream` | 3.227±0.003ms | 2.448±0.002ms | **-24.2±0.1%** | +0.06% |
| `enums17` | `slice` | 9.041±0.009ms | 7.019±0.007ms | **-22.4±0.1%** | +0.02% |
| `enums17` | `from` | 9.111±0.009ms | 7.113±0.007ms | **-21.9±0.1%** | +0.02% |
| `enums17` | `stream` | 9.173±0.008ms | 7.037±0.007ms | **-23.3±0.1%** | +0.02% |
| `floats` | `slice` | 336.679±0.337µs | 446.920±0.447µs | **+32.7±0.2%** | +0.00% |
| `floats` | `from` | 448.560±0.448µs | 1.914±0.002ms | **+326.7±0.6%** | +0.00% |
| `floats` | `stream` | 856.379±0.856µs | 3.282±0.003ms | **+283.3±0.5%** | +0.00% |
| `compressible` | `slice` | 107.837±0.055ms | 81.856±0.039ms | **-24.1±0.1%** | +0.06% |
| `compressible` | `from` | 109.319±0.055ms | 90.801±0.035ms | **-16.9±0.1%** | +0.06% |
| `compressible` | `stream` | 126.595±0.042ms | 111.783±0.061ms | **-11.7±0.1%** | +0.06% |
| `records` | `slice` | 72.980±0.057ms | 55.855±0.046ms | **-23.5±0.1%** | +0.03% |
| `records` | `from` | 73.856±0.057ms | 57.699±0.052ms | **-21.9±0.1%** | +0.03% |
| `records` | `stream` | 78.477±0.047ms | 64.734±0.055ms | **-17.5±0.1%** | +0.03% |
| `records-wide` | `slice` | 107.450±0.062ms | 81.094±0.076ms | **-24.5±0.1%** | +0.02% |
| `records-wide` | `from` | 110.423±0.092ms | 88.160±0.086ms | **-20.2±0.1%** | +0.02% |
| `records-wide` | `stream` | 114.021±0.092ms | 97.078±0.096ms | **-14.9±0.1%** | +0.02% |
| `atmost3` | `slice` | 1.869±0.002ms | 1.328±0.001ms | **-28.9±0.1%** | +0.03% |
| `atmost3` | `from` | 1.857±0.002ms | 1.309±0.001ms | **-29.5±0.1%** | +0.03% |
| `atmost3` | `stream` | 1.947±0.002ms | 1.329±0.001ms | **-31.8±0.1%** | +0.03% |
| `atmost8` | `slice` | 2.603±0.003ms | 1.844±0.002ms | **-29.1±0.1%** | +0.02% |
| `atmost8` | `from` | 2.647±0.003ms | 1.965±0.002ms | **-25.7±0.1%** | +0.02% |
| `atmost8` | `stream` | 2.614±0.003ms | 1.857±0.002ms | **-29.0±0.1%** | +0.02% |
| `atmost16` | `slice` | 3.475±0.003ms | 2.568±0.003ms | **-26.1±0.1%** | +0.01% |
| `atmost16` | `from` | 3.470±0.003ms | 2.625±0.003ms | **-24.4±0.1%** | +0.01% |
| `atmost16` | `stream` | 3.488±0.003ms | 2.574±0.003ms | **-26.2±0.1%** | +0.01% |
| `atmost32` | `slice` | 4.292±0.004ms | 3.550±0.004ms | **-17.3±0.1%** | +0.01% |
| `atmost32` | `from` | 4.238±0.004ms | 3.672±0.004ms | **-13.4±0.1%** | +0.01% |
| `atmost32` | `stream` | 4.312±0.004ms | 3.543±0.004ms | **-17.8±0.1%** | +0.01% |
| `atmost128` | `slice` | 5.462±0.005ms | 4.740±0.005ms | **-13.2±0.1%** | +0.00% |
| `atmost128` | `from` | 5.451±0.005ms | 4.798±0.005ms | **-12.0±0.1%** | +0.00% |
| `atmost128` | `stream` | 5.457±0.005ms | 4.762±0.005ms | **-12.7±0.1%** | +0.00% |

#### Encode

| workload | route | Range | Ans | Δ | size Δ |
|---|---|---|---|---|---|
| `strings` | `encode` | 11.175±0.011ms | 11.736±0.012ms | **+5.0±0.1%** | +0.04% |
| `strings` | `encode-to` | 11.147±0.011ms | 11.261±0.011ms | **+1.0±0.1%** | +0.04% |
| `enums` | `encode` | 2.048±0.002ms | 2.635±0.003ms | **+28.7±0.2%** | +0.06% |
| `enums` | `encode-to` | 1.971±0.002ms | 2.576±0.003ms | **+30.7±0.2%** | +0.06% |
| `enums17` | `encode` | 3.818±0.004ms | 3.888±0.004ms | **+1.8±0.1%** | +0.02% |
| `enums17` | `encode-to` | 3.800±0.004ms | 3.917±0.004ms | **+3.1±0.1%** | +0.02% |
| `floats` | `encode` | 1.391±0.001ms | 1.010±0.001ms | **-27.4±0.1%** | +0.00% |
| `floats` | `encode-to` | 1.373±0.001ms | 1.008±0.001ms | **-26.6±0.1%** | +0.00% |
| `compressible` | `encode` | 401.293±0.217ms | 394.405±0.161ms | **-1.7±0.1%** | +0.06% |
| `compressible` | `encode-to` | 403.784±0.248ms | 394.624±0.365ms | **-2.3±0.1%** | +0.06% |
| `records` | `encode` | 45.427±0.031ms | 42.401±0.040ms | **-6.7±0.1%** | +0.03% |
| `records` | `encode-to` | 45.893±0.043ms | 42.337±0.033ms | **-7.7±0.1%** | +0.03% |
| `records-wide` | `encode` | 74.404±0.062ms | 66.982±0.054ms | **-10.0±0.1%** | +0.02% |
| `records-wide` | `encode-to` | 75.828±0.069ms | 66.839±0.022ms | **-11.9±0.1%** | +0.02% |
| `atmost3` | `encode` | 1.369±0.001ms | 1.390±0.001ms | **+1.5±0.1%** | +0.03% |
| `atmost3` | `encode-to` | 1.286±0.001ms | 1.429±0.001ms | **+11.1±0.2%** | +0.03% |
| `atmost8` | `encode` | 1.113±0.001ms | 1.134±0.001ms | **+1.9±0.1%** | +0.02% |
| `atmost8` | `encode-to` | 1.067±0.001ms | 1.154±0.001ms | **+8.1±0.2%** | +0.02% |
| `atmost16` | `encode` | 1.283±0.001ms | 1.277±0.001ms | **-0.5±0.1%** | +0.01% |
| `atmost16` | `encode-to` | 1.208±0.001ms | 1.287±0.001ms | **+6.6±0.2%** | +0.01% |
| `atmost32` | `encode` | 1.546±0.001ms | 1.442±0.001ms | **-6.8±0.1%** | +0.01% |
| `atmost32` | `encode-to` | 1.499±0.001ms | 1.459±0.001ms | **-2.6±0.1%** | +0.01% |
| `atmost128` | `encode` | 1.976±0.002ms | 1.802±0.002ms | **-8.8±0.1%** | +0.00% |
| `atmost128` | `encode-to` | 1.915±0.002ms | 1.828±0.002ms | **-4.5±0.1%** | +0.00% |

### Dead ends that should not be retried without new evidence

Kept because each one forecloses a re-run someone would otherwise be tempted
to try; the reasoning is the durable part, the exact numbers are illustrative.

- **Per-run collection sentinel** (`items.chunks(SENTINEL_EVERY)` instead of a
  per-element countdown): looked like +2.4% on a normal build, but that was
  code placement — force-aligned, it's a wash that flips sign with element
  cost (the closure it requires blocks context hoisting). Not worth a second
  marker schedule to keep in lockstep with `Sentinel`'s.
- **Batching independent fixed-width bits via `decode_bits::<N>` at small N**
  (tried on `Ipv6Addr`'s 14 zero-flag bits): **+6.6% slower on `Range`, a wash
  on `Ans`**, even after the fused `decode_bits` override later landed (still
  +1.1% on `Ans`). Both coders are sequential bit-by-bit, so there's no ILP to
  exploit at small N — the win only shows up on **wide** batches (52–64 float
  bits), not 14-bit groups. Don't convert more small callers.
- **Two-stream interleaved rANS** (the fgiesen "rANS in practice" trick): 48–107%
  SLOWER with the one-bit-at-a-time `&mut self` API, because the state swap
  serializes through memory (store→load forwarding). `decode_bits(&mut
  [BitContext; N])` later gave the crate a register-resident multi-bit surface
  that removes that obstacle in principle (interleave two rANS states across
  even/odd bits within one call) — nobody has prototyped it since; the tree
  codes' *dependent* bits (`u8`, `UBits`, `Bits<N>`) would need independent-bit
  decoding added first to benefit from it.
- **Multisymbol tree coding without fusion, and register-resident per-bit tree
  decode, each tried alone**: both lost or washed (multisymbol +5.6–11.6%
  slower — the CDF-construction multiply sits ON the serial bit-decision
  chain; register-residency for the per-bit walk washed, since the `Decoder`
  fields were already store-forwarding-hidden in L1). **The combination later
  won** — see "Fused-context speculative tree walk" below — so treat either
  half alone as a dead end, not the idea itself. Kept from this work: the
  shared `encode_tree`/`decode_tree` trait methods (per-bit, bit-identical)
  and the `just-decompress-strings` benchmark.
- **Routing `MAX = 2` through the bitwise walk**, following up on `MAX = 1`'s
  win: looked good on an isolated shootout (12–15% faster on both
  distributions, both coders) but **reversed on the real 3-variant-enum
  workload** — Ans decode +20%, Ans/Range encode +21–38% worse. Same trap as
  an earlier `CompleteBitwise` lead: an isolated-walk result on tiny trees can
  invert once the real enum-match layer and skewed data are in play. `MAX = 2`
  stays a symbol.

**Bonus finding, still relevant when reading size assertions — the `Range`
coder codes near-certain bits BELOW entropy in narrow intervals.** When the
interval width drops under 256, `split()`'s `(width >> 8) * prob` is 0, so a
`true` bit costs ~0 bits regardless of its modeled probability. 64
fresh-context copies of `u8::MAX` (true entropy 64 bytes) encode to 23 bytes.
Multisymbol, which codes honestly at `width ≥ 2^32`, loses this accident —
that's why it "regresses" some all-extreme-value size assertions.

### Fused-context speculative tree walk — multisymbol now BEATS per-bit (2026-07-03)

Profiling the multisymbol decode of an *unsorted* `Vec<String>` of the 38k
meteorite names (`src/bin/ans-phases.rs`, built via `HashSet` so there
is no shared-prefix coding; ~450 KB encoded) showed the model side (86% of
decode) dominated by the `SymbolRange::from_slot` walk (~43% of the run) and
the `BitContext` `LOOKUP`/`OUTCOMES` table loads (~32%). Every level of the
walk was a serial chain: load `contexts[node]` → load `LOOKUP[state]` →
`width×prob>>8` multiply → compare → bit → next node. Three changes, all
bit-identical (every `assert_bits!` unchanged):

1. **Fused table** (`FUSED` in `src/v2/symbol.rs`): one entry per `BitContext`
   holding `{probability, adapt(false), adapt(true)}`, built by compile-time
   BFS from the default state (`probability`/`adapt` in the generated
   `bit_context.rs` are now `const fn`; the generator emits that too). One
   load per node replaces the separate probability and adapt lookups, and the
   adapt successor is already in hand when the bit resolves.
2. **Speculate both ways in `from_slot`**: fetch *both* children's fused
   entries (loads depend only on `node`, issuing a level ahead) and compute
   *both* children's splits before the bit resolves. The critical path is then
   the multiply chain plus one cmov per level; the compare hangs off the side.
3. `split()` multiply narrowed u64 → u32 (product fits: `2^16 × 255 < 2^32`).

Results (pinned core 2, min of runs):

| benchmark | before | after | Δ |
|---|---|---|---|
| `ans-decode-phases` (Vec\<String\>, full decode ms/iter) | 24.34 | 14.91 | **−39%** |
| `just-decompress-strings ans` 500× (Gcycles) | 22.68 | 20.52 | **−9.5%** |
| `just-decompress-strings range` 500× (Gcycles) | 25.95 | 22.55 | **−13%** |

Scaled to the 2000-iter table above: `Ans` 82.1B vs the 83.07B *per-bit*
baseline (~1% faster), `Range` 90.2B vs 96.71B (**−6.7%**) — multisymbol now
wins outright on strings. (Today's pre-change branch numbers, 90.7B/103.8B,
reproduce that table's multisymbol column, so the comparison is sound.) Caveat:
the per-bit path never got the fused-table treatment; but its chain is
dominated by the rANS `decode_step` state dependency per bit, which a fused
table cannot remove, while multisymbol pays one coder step per symbol *and*
now has the shorter walk chain. After the change the remaining hot lines are
the walk arithmetic itself (`split` multiply ~10%, `contains` compare ~8%,
speculative loads/selects ~24%); the `BitContext` table lines fell from ~32%
to ~1%. Further wins likely need format changes (e.g. deeper fusion via a
`SymbolRange::BITS` bump) — bit-compatibility is not a constraint per David.

### Full comparison-suite A/B: multisymbol's big win is ENCODE (2026-07-03)

`cargo bench -p comparison` on `main` vs the `multisymbol-tree-coding` branch
(multisymbol + fused walk), wall-clock, pinned core. David predicted this:
encode pays *none* of multisymbol's latency penalty — the value is known, so
there is no serial bit-decision chain to lengthen — while reaping all its
benefits: one deferred `Op` per symbol instead of one per bit for `Ans` (8×
less buffer traffic for byte trees), one interval step instead of `log2(N)`
for `Range`, plus the fused table in `for_value`. Encoded sizes are unchanged
(±few bytes, the known +0.01–0.03% shift).

| dataset | Range encode | Ans encode | Range decode | Ans decode |
|---|---|---|---|---|
| suicide data / rates / suicide (×2) | **−39…−52%** | **−36…−42%** | −8…−24% | −13…−28% |
| meteorite names | **−37%** | **−33%** | −7.5% | −0.7% (wash) |
| meteorites / by name | −15…−16% | −15…−17% | −7…−9% | −3…−7% |
| single cards / single meteorites | −10…−14% | −9…−13% | −6…−8% | −0.4…−6% |
| books / mtg / meteorites by small name | −1…−3% | −3% | −2…−7% | −4…+1% |

Reading guide: the bottom row is the `Compressible`/Lz77-dominated group —
mtg encodes in ~823 ms of which tree coding is a sliver, so multisymbol can't
move it. The wall-clock noise floor (zstd/bincode reference rows, identical
code in both builds) was up to ±44% on the µs-scale datasets and ≤ ~12% on the
large ones, so individual decode deltas under ~10% are directional only — but
the sign is consistent across both coders and all datasets, agrees with the
pinned cycle-count A/Bs above, and the encode deltas are far above any noise.

Consequence: the "encode speed is not a current target" stance below predates
this — multisymbol makes tree-heavy encode 15–50% faster as a side effect of
the decode work, and a `SymbolRange::BITS` bump (deeper fusion) should extend
both the encode win and the Ans-decode wash on strings. Raw outputs:
`bench-main.txt` / `bench-branch.txt` in the session scratchpad.

### Escaped-tree fusion: `is_ascii` + ASCII tree in one coder step (2026-07-04)

Branch `deeper-fusion` (PR base: `multisymbol-tree-coding`). Every ASCII
character used to cost two coder steps: a `bool` (`is_ascii`) bit and the
7-bit `Bits<128>` tree symbol. They are now fused into one *escaped-tree*
symbol (`SymbolRange::{for_value,from_slot}_escaped` in `src/v2/symbol.rs`,
`encode_escaped_tree`/`decode_escaped_tree` in the coder traits): the root
bit is the guard, its false branch is a depth-1 escape leaf (non-ASCII, which
then encodes its chunks as before), and its true branch continues into the
7-level ASCII subtree — one interval, one renormalization, for the whole
8-bit-deep symbol.

**No `SymbolRange::BITS` bump was needed**, contrary to the older note above:
the fused depth is 8 levels, the same as the existing `u8` byte trees, so
`M = 2^16` still gives every leaf a slot and `Ans::Op::Symbol` stays two
`u16`s. Size cost is the escape leaf's reserve squeeze, ~2–3 millibits per
ASCII char (the 1720-char `COMPRESSIBLE_TEXT` grew 8980 → 8986 bits, +0.07%);
`Raw` keeps the unfused per-bit format, and Lz77/`Compressible` is untouched
(its literals are plain byte trees, no guard bit — nothing to fuse).

Results (pinned core 2, alternating A/B, min of runs) vs the multisymbol
branch:

| benchmark | multisymbol | fused | Δ |
|---|---|---|---|
| `ans-decode-phases` full decode (unsorted `Vec<String>`, ms/iter) | 16.46 | 15.78 | **−4.4%** |
| `ans-decode-phases` entropy-only phase (ms/iter) | 3.31 | 2.19 | **−34%** |
| `ans-encode-phases` total (build + into_vec, ms/iter) | 4.75 | 4.43 | **−6.7%** |
| `ans-encode-phases` into_vec alone (ms/iter) | 1.13 | 0.87 | **−23%** |
| `just-decompress-strings range` 500× (Gcycles) | 23.40 | 22.76 | **−2.8%** |
| `just-decompress-strings ans` 500× (Gcycles) | 21.07 | 21.35 | +1.3% (†) |
| `just-decompress-net` (untouched path, control) | 131.33 | 131.25 | wash |

(†) Not a coding regression: that workload is >50% `BTreeMap::insert` +
`memcmp` (set construction), the walk's profile share is identical (11.8%)
on both sides, and the identical-code construction functions themselves
measured 4–6% apart — binary-layout noise. The coding-dominated variant of
the same data (unsorted `Vec<String>`, first row) wins −4.4%.

The comparison suite (wall-clock, thermally noisy — see the benchmarking
note above) agrees where it can resolve anything: on the one adjacent
same-conditions pair, `meteorite names` encode Range −5.0% / Ans −8.0%,
`meteorites` Ans −4%, everything else within its ±8% reference noise.

The mechanism, as expected from the multisymbol work: the big Ans win is in
the *entropy/step* phase (one op and one renorm per char instead of two;
−34% replay, −23% into_vec), and `Range` — whose per-bit steps are pricier —
wins outright on decode too. The escaped walk adds its root level to the
decode chain, which eats part of the saved step on `Ans`.

### ULessThan multisymbol coding with seeded contexts (2026-07-08)
`ULessThan<N>` now codes one whole symbol per value (`encode_uless_tree` /
`decode_uless_tree`, walks in `symbol.rs`), like the `Bits`/`u8` trees but
over the uneven binary-search shape (`SymbolRange::split_reserving`: per-child
leaf-count reserves, plain learned probability, no division). With fresh
contexts every value costs the *fractional* `log2(N)` bits, achieved by
seeding each node's initial `BitContext` at its children's leaf proportion
`lo/(lo+hi)` at compile time (`ULessThanContext::SEEDED`); balanced nodes
seed to the ordinary default, so power-of-two `N` (every `usize` length) is
untouched. The old per-bit walk charged integer 3-or-4 bits for `N = 10`; its
apparent sub-integer sizes for last-variant values were an end-of-stream
artifact (the exhausted decoder hallucinates `true` bits, so a trailing
all-`true` run truncates for free), which the symbol path gives up — hence
the `tests/derive.rs` enum size bumps.

Two designs that DON'T work, measured on the dedicated
`just-{de,}compress-enums` workloads (min cycles, 3 alternating pinned runs):
- **Bayes leaf-weighting in the split** (`lo*p : hi*(1-p)`): the adapted
  context already converges to the empirical bit frequency, so a static
  weight on top permanently skews the coded probability — **+3% encoded
  size** on adapted skewed 3-variant enums, and its u64 division on the
  serial decode chain cost **+39%** Ans / **+8.8%** Range decode cycles.
- (The balanced-node fast path recovered none of that on real workloads —
  the division sat exactly on the unbalanced nodes real enums use.)

Final numbers for the seeded, division-free design vs pre-change main
(min cycles, 3 alternating pinned runs on the pure-discriminant workloads):
- **encode: Ans −32.8%, Range −19.9%** — one buffered op instead of
  `~log2(N)` per value.
- **decode: Range −4.4%, Ans +10.4%** — Range's pricier per-bit steps make
  the single symbol step a win; Ans's lean bit steps don't, on this
  ~100%-discriminant microbench. Porting `from_slot`'s speculative child
  prefetch into `from_uless_slot` made N=3 *worse* (Ans +17.4%, Range +4.7%
  vs main; i.e. +6%/+9% over plain) — reverted.

**Why the prefetch loses on shallow trees (profiled 2026-07-08):** on the
N=3 workload the prefetch build executes **+81% instructions** and +69%
branches for the same decodes (perf stat), yet only +6% cycles — IPC rises
2.10 → 3.60 as the wide core absorbs the speculative work. Both versions
fully unroll (zero backward jumps); the cost is the speculation itself
(both children's `half` index arithmetic + double FUSED loads per level,
mostly wasted at depth 1-2) plus **register pressure**: the prefetch's
carried state (`cur`/`lo_cur`/`hi_cur`, both splits/lengths) produces 9
stack-spill stores + 13 reloads in the hot function where the plain walk
has zero, putting store-forwarding latency back *on* the critical path.
**Depth flips the verdict** (`just-decompress-enums seventeen`, N=17,
depth 4-5): prefetch went Ans +1.4% (wash) / **Range −7.6%** — instructions
still +70-83%, but now there is real serial-chain latency to hide, same as
the depth-8 byte tree where speculation won originally.

**The real crossover is per-coder, not per-depth** (swept 2026-07-08 with
`just-decompress-uless`, min cycles of 3 interleaved rounds, prefetch Δ vs
plain; run on battery under load — spreads on decisive cells ≤1.6%):

| N   | Ans    | Range  |     | N   | Ans    | Range  |
|-----|--------|--------|-----|-----|--------|--------|
| 3   | +15.0% | +11.0% |     | 16  | +10.6% | −12.2% |
| 4   | +17.3% | −16.9% |     | 24  | +12.2% | −7.5%  |
| 6   | +22.5% | −13.0% |     | 32  | +10.0% | −6.4%  |
| 8   | +10.1% | −10.6% |     | 64  | +4.4%  | −4.7%  |
| 12  | +13.1% | −10.5% |     | 128 | +10.5% | −3.7%  |

`Ans` never wants the prefetch on this pure-`ULessThan` workload — its lean
symbol step leaves the speculative instructions exposed (the N=17 enum
"wash" is as close as it gets, diluted by the enum-match layer). `Range`
wants it for everything but N=3: its u64 `symbol_slot` division gives the
speculation a latency shadow to hide in. Shipped as a per-coder choice:
`Range::decode_uless_tree` takes `from_uless_slot_prefetching` for
`N > ULESS_PREFETCH_MIN_N = 3`, `Ans` always takes the plain walk.
- **size: parity on adapted data** (17564 bytes both sides), fractional-bit
  wins on fresh contexts.

Broad workloads (`just-decompress`, `just-decompress-strings`, both coders)
stayed within the ±0.5% layout-noise floor throughout — real data dilutes
the discriminant path heavily.

### Bits → ULessThan unification (2026-07-09)
`Bits<N>`/`BitsContext` and the `encode_tree`/`decode_tree` trait methods are
gone: `u8` and `UBits<N>` now delegate to `ULessThan<2^k>`, and `symbol.rs`
holds one cutoff-free implementation per tree layout in its own module —
`complete` (power-of-two `N`: heap-ordered contexts, speculative decode; the
old `Bits` machinery verbatim) and `uneven` (any `N`: split-ordered contexts,
plain + prefetching decode) — with the compile-time dispatchers
(`encode_walk`, `decode_walk`, `decode_walk_speculating`,
`{en,de}code_bitwise`) as the only home of the `N`-based cutoffs. Bitstream
is **byte-identical to main** (verified: zero expect-test churn, encoded
sizes equal on the meteorite workload), because for power-of-two `N` both
trees make identical probability/bit decisions and context indexing is
internal state.

Lessons from the three attempts it took (each measured on
`just-{de,}compress-strings`, min cycles of 3-5 interleaved pinned pairs):

1. **A rolled walk is disastrous on the hot byte path.** The naive swap left
   the `u8` tree as `while possible_values_left > 1` — LLVM cannot prove the
   balanced tree's path-independence, so the walk kept a live `bsr`
   (runtime `half`), loop control, and a backward branch: strings decode
   **+13/+22%** (Ans/Range), encode **+16/+22%**. Fix: bound the loop by
   `const { uless_depth(N) }` (exact longest path, computed at compile time)
   with an early break — the loop fully unrolls, and for power-of-two `N`
   every level's lengths constant-fold. This alone recovered encode to
   *better than main* (Ans −3.6%) but decode still lagged (+11/+8%).
2. **The heap layout itself is the decode win — now cleanly isolated.** The
   unrolled split-indexed walk executes the *same instruction count* as
   main's speculative heap walk (+0.1%) with fewer branch misses, yet +11%
   cycles for Ans: pure serial-FUSED-load latency. The split-order
   prefetching walk does NOT recover it (Ans +14.5%, worse than plain — the
   extra index arithmetic and spills land in a register-starved inlined
   frame), and `#[inline(never)]`-outlining the walk is also worse (+17/+19%:
   the coder state round-trips through memory per symbol). Only the heap
   layout gives speculation for free: child indices `2n+1`/`2n+2` depend on
   nothing but the parent's index. Hence the pow2/other split of
   `complete` vs `uneven` — this is the "sparse heap" idea with the sparse
   part not needed (power-of-two trees are dense in `[BitContext; N]`;
   awkward `N` would need up to ~2N slots, unexpressible with stable const
   generics anyway).
3. **`ULessThan` itself got much faster.** vs main (uniform
   `just-decompress-uless`): pow2 `N` now takes the heap walk — N=8 **Ans
   −38.8% / Range −21.6%**, N=16 **−31.2% / −20.9%**, N=128 **−25.7% /
   −19.5%** — and non-pow2 `N` gains the const-depth unroll — N=6 Ans
   **−15.7%**, N=12 Range **−11.3%**. N=3 enums: wash (±0.2%).

Final numbers vs main (`just-{de,}compress-strings`, the `u8`-heaviest real
workload): decode **Ans +3.2%, Range +3.2%**; encode **Ans −3.8%, Range
+0.3%**; `just-decompress` (u64) ±0.5%. The residual ~3% decode cost is NOT
in the walk (the four `from_slot` monomorphizations are byte-identical
functions in both binaries) but in glue — total instructions +1.6%,
suspects: `ULessThanContext::default()` copying `SEEDED` where
`BitsContext::default()` was a memset, and inlining shifts around the `u8` →
`ULessThan<256>` delegation. Worth a follow-up look if strings decode
matters more than the ladder wins.

### ULessThan<N+1> → AtMost<MAX>: dropping the unused context slot (2026-07-09)
`ULessThan<N>` is now `AtMost<MAX>` (holding `0..=MAX`), and its context
shrank from `[BitContext; N]` (one slot never touched — `N` values need only
`N − 1` internal nodes) to a snug `[BitContext; MAX]`. Everything downstream
reparametrized: the `symbol.rs` walks take `MAX`, the trait methods are
`encode_atmost_tree`/`decode_atmost_tree`, `u8` delegates to `AtMost<255>`,
the derive emits `AtMost<{variants − 1}>` (a fieldless single-variant enum's
discriminant context is now zero-sized), and the generated char tables in
`string/init.rs` dropped their unused 256th entry (255 × 4 contexts). The
used indices and walk order are unchanged, so the bitstream is
**byte-identical** (zero expect-test churn; equal encoded bytes on the
meteorite and uniform-ladder workloads).

Performance is regression-free, but proving that taught a lesson about this
machine's noise floor on the *microbenchmarks* (the `just-decompress-uless`
ladder and `just-decompress-enums` runs are 1–8 B cycles, much shorter than
the strings runs):

- Real wins on the big workload: strings decode **Ans −2.1%, Range −0.9%**
  (recovering most of the unification's ~+3% glue residual — the four
  `CharContext` tables now pack 1020 contiguous bytes instead of 1024),
  uless ladder N=6 Ans −3.0%, N=3 Ans −1.1%; everything else ±0.5%.
- The plain A/B first showed scary-looking scatter: uless-8-range **+3.5%**,
  enums-dec-ans **+3.7%**, but also uless-16-range **−3.1%** — the same
  `complete` walk code at neighboring depths moving in opposite directions.
  Instruction counts were identical to ±0.01% in every case (no bounds
  checks appeared; same work). Rebuilding *both* sides with
  `-C llvm-args=-align-all-functions=6 -C llvm-args=-align-all-nofallthru-blocks=6`
  made every delta collapse (+3.5→−1.0%, +3.7→+0.4%, +1.6→−0.3%) and
  uless-16-range *flip sign* (−3.1→+2.2%): pure code-placement luck.
  **Rule: on the short ladder/enum bins, treat |Δ| ≲ 3.5% with identical
  instruction counts as layout noise, and use the forced-alignment rebuild
  to adjudicate before believing any delta there.**

### v2 abstraction cleanup, Wave 2 (2026-07-11)

Structural refactor, all bitstream-preserving (zero expect churn): `AtMost`
became a first-class coder primitive (`encode_atmost`/`decode_atmost` taking
`AtMostContext<MAX>`/`AtMost<MAX>`), the triplicated symbol/bitwise guards
collapsed into one `walks::{encode,decode}_symbol_or_bitwise` behind the
internal `SymbolCoder`/`SymbolDecoder` traits, `encode_bits` gained the
context array so the coder adapts on both sides (mirror of `decode_bits`),
and `UBits<N>` was deleted in favor of `AtMost<2^N − 1>`. A/B was
wave-1-branch vs wave-2-branch, both `--release`, pinned core, min of 3,
tightly interleaved.

Real, instruction-backed wins (these are the point of the UBits removal —
one fewer wrapper monomorphization and inlined `adapt` in the hot loops):

- strings decode **Ans −5.96%** (−2.38% insns), **Range −4.70%** (−1.24%).
- enums encode **Ans −4.78%** (−1.23% insns), **Range −2.77%** (cycles only).
- strings encode **Ans −0.62%** (−0.58% insns); the hot `Vec<String>`
  Sorted-encode loop lost 61 instructions including **5 calls**.

Two adjudications worth recording:

- **`inline(always)` on the dispatch layer is load-bearing.** With a plain
  `#[inline]`, the compiler outlined `decode_symbol_or_bitwise` for the
  `AtMost<7>` Ans path, costing +13% instructions / ~+8% cycles on that one
  monomorphization (uless-8-ans). Forcing the inline restored fusion into the
  coder's symbol step (instruction counts back to identical, delta −0.01%).
  The uless ladder deltas that remained (uless-3 +2.05%/+0.94%, uless-8-ans
  +1.77%) all had identical instruction counts and **collapsed under the
  forced-alignment rebuild** (→ −0.31%, −0.43%, −0.01%): layout noise per the
  rule above.
- **`just-compress-strings range` shows +5.58% and it is NOT the coder.** It
  is the one delta that did *not* collapse under forced alignment — but it is
  construction noise, not a regression: the Range symbol-encode machine code
  is byte-identical (wave-1 `write_symbol` == wave-2 `SymbolCoder::encode_symbol`,
  399 insns each), the whole binary has **203 fewer** instructions, the hot
  Sorted-encode function is **61 smaller**, and the **Ans twin of the exact
  same workload is a −0.62% win**. A real coder regression would move the Ans
  side too. This is the BTreeMap-insert/`memcmp`/`String` construction floor
  (measured 4–6% between builds differing only in compactly code); on this
  workload it is stable per binary-pair and forced-alignment does not fully
  neutralize it, so instruction counts + the same-workload/other-coder
  contrast are the tie-breakers, not the alignment rebuild.

### `AtMost<MAX>` walk shootout tool (2026-07-12)

The `MAX`-based cutoffs picking `complete`/`uneven` layout and
plain/speculating decode (`SPECULATE_MIN_MAX`, the per-coder speculate flag)
were baked in from earlier A/B sweeps on this machine and had no way to be
re-measured off the beaten path. `benches/atmost.rs` (`cargo bench --bench
atmost`) times every (coder × `MAX` × applicable `Walk`) for decode, and once
per *distinct* encode implementation, via `#[doc(hidden)]`
`Range`/`Ans::{encode,decode}_atmost_batch::<MAX, WHICH_WALK>` methods so each
forced walk stays branch-free — no runtime `Walk` dispatch anywhere, benchmark
included.

A walk counts as beating production's choice only if it is faster by ≥5%
*and* by more than three standard errors — `scaling` measures the error bar
on every cell, so significance is a test rather than something to re-run and
eyeball. (Two earlier versions of this: the first reported any single-sample
≥10% gap directly, which could not tell an effect from noise; the second
re-timed each nominee three times with alternated order, which the measured
error bars have now made unnecessary.)

The first run (uniform-random values only) confirmed a real regression
(`Range`'s `UnevenSpeculating` decode reproducibly slower than plain `Uneven`
at several `MAX`) and one surprising, harder-to-trust lead — `Ans`'s
whole-symbol walk losing to the historical per-bit walk at several
power-of-two `MAX`. See "take 2" below, which added a second data
distribution specifically to stress-test that lead before trusting it, and is
what production is actually tuned against.
### Walk shootout, take 2: the data distribution is a first-class axis (2026-07-12)
The shootout above fed **uniform** random values — the entropy worst case,
and a biased one: contexts never adapt away from 50/50, every walk path is
branch-unpredictable (the best case for the latency-hiding speculating
walks), and every symbol costs full `log2(MAX + 1)` bits. Production
`AtMost` data (string bytes, length buckets, enum discriminants) is heavily
skewed. The bench now sweeps a `Skewed` distribution
(`floor((MAX + 1)·u⁸)`, ~50% of mass on value 0 at `MAX = 255`) alongside
`Uniform`, nominates a challenger that wins significantly on *either*
distribution, and reports each finding as a cross-distribution range of
margins with their error bars (`?` marks a distribution where the error bars
cannot support the margin). `ATMOST_DIST=uniform|skewed` restricts the sweep. New
`MAX` points 33/34/40/48 bracket the uneven tree's worst-case-depth step
from 6 to 7 (`tree_depth(35)` is the first 7).

What the quiesced two-distribution run (CPU 2) settled:

- **Range `UnevenSpeculating` decode really is a loss above the depth step,
  on both distributions**: production `UnevenSpeculating` vs plain `Uneven`
  at `MAX` = 34/64/128/256/512 is 14–22% slower on Uniform and **33–42%
  slower on Skewed** (skew makes the plain walk *faster* — predictable
  path — while speculation stays flat, so realistic data widens the loss).
  `MAX = 33` (depth 6) still favors speculating; the flip lands exactly on
  the `tree_depth` 6→7 step, consistent with the +81%-instructions /
  register-spill profile from the ULessThan-era prefetch work. At
  `MAX >= 700` speculating wins again (the walk no longer fully unrolls —
  different codegen regime). Actionable: bound `Range`'s uneven speculation
  window (planned as its own change).
- **The scary "Ans per-bit beats the symbol walk at every power-of-two
  count" finding is a uniform-distribution artifact.** On Skewed it
  *inverts*: `CompleteBitwise` is 13–27% *slower* than production
  `CompleteSpeculating` at `MAX` = 7/15/31/63/127/255 (both coders show the
  same sign flip). This matches the real-string macro history and means no
  production change is warranted. The per-bit walk's uniform win survives
  only at the extremes: tiny (`MAX = 1, 2`) and huge (`MAX >= 700`, where
  it beats plain `Uneven` by 14–35% on *both* distributions — worth a look
  if anyone ever puts a multi-thousand-value `AtMost` on a hot path).
- **Distribution-robust findings worth follow-up**: (1) `MAX = 1`'s symbol
  machinery is pure overhead — plain bit coding wins 5–36% across coders,
  metrics, and distributions (DONE 2026-07-19 — see "Landed");
  (2) `Ans` decode at power-of-two `MAX` =
  15/31/63 prefers `UnevenSpeculating` over production
  `CompleteSpeculating` on **both** distributions (7–23%), reopening the
  complete-vs-uneven layout question for mid-size trees (contradicts the
  2026-07-09 strings-decode lesson, so validate against
  `just-decompress-strings` before believing it); (3) the `MAX = 48` cell's
  plain `Uneven` is anomalously slow on Skewed for both coders (~30% slower
  than neighboring `MAX = 40`) — smells like the known alignment/codegen
  scatter, treat that cell with suspicion.

### Range's uneven speculation window is now depth-bounded (2026-07-12)
Acting on the above: `Walk::production` picks `UnevenSpeculating` for
`Range` only inside a measured window (`speculation_pays` in
`src/v2/atmost/walks.rs`): `MAX >= 3` **and** (`tree_depth(MAX + 1) <= 6`
**or** `MAX >= 700`, where the walk no longer fully unrolls and speculation
measured faster again on both distributions). Since `Uneven` and
`UnevenSpeculating` are bit-identical decode twins, the encoded format is
unchanged; only `Range` decode speed for non-power-of-two value counts with
35..=513 values is affected (derive enums of that size — the `usize`
buckets are `MAX <= 31` and `u8`/strings are power-of-two counts).

The post-change shootout run confirms the fix: every
"production `UnevenSpeculating` loses to `Uneven`" finding in the 34..512
band is gone, and plain `Uneven` is now marked production there. Residual
exception, deliberately left plain: **`MAX = 48` reproducibly prefers
speculation** (12–18% on both distributions, both coders' uneven walks) —
its plain walk monomorphizes anomalously slowly (`MAX = 48` skewed decode
~88/104 ns vs `MAX = 40`'s ~59/72 ns at the same batch size, consistent
across two different binaries, so it is a codegen property of that
monomorphization, not run-to-run scatter). A depth- or count-based rule
can't capture one bad monomorphization; if `AtMost<48>`-sized enums ever
matter, investigate that codegen instead of widening the window.

### Hierarchical (Elias-delta-style) integer encoding + mirrored `usize` prior (2026-07-17)

The default integer `Encode`/`Small` scheme was rebuilt around the value's
*bit length* `bl` instead of one deep leading-zero tree
(`src/v2/ints.rs`): one `AtMost<blbl_max>` symbol for
`blbl = bit_length_of(bl)` (3-level complete tree for u64), then `bl`'s
offset within its `blbl` bucket as a second per-bucket `AtMost` symbol,
then the value mantissa as before. `usize`'s default `Encode` reuses the
exact same compiled code via a `Default`-override context seeded from the
*mirrored* prior (`SeededDistribution::TinyNumbers` in
`src/v2/atmost/geometric.rs`) — tiny
magnitudes dominant, matching real lengths/counts/indices — while
`u16..u128`'s default keeps the uniform-value prior and `Small` stays
flat-seeded.

Why: the old 6-level `AtMost<63>` tree charged every u64/usize **6
adaptive decisions** regardless of magnitude. Each fully-adapted decision
floors at ~11.3 mb (`BitContext`'s 254/256 probability cap — this and the
numbers below were measured before `MAX_PRODUCT` went to 135, which halves
that floor to ~5.7 mb; see the `MAX_PRODUCT` note further down) and each
fresh seeded node at ~0.26 bits (`seed_context`'s 4-observation cap), so
tiny values — the overwhelmingly common case for `usize` — paid double
what a 3-decision path needs. Measured (Millibits, exact):

- Repeated-constant floor: 68 → **34 mb/element** (u64/usize value 1;
  matches 3 × 11.3 exactly). Guarded by
  `repeated_constant_floor_matches_shallow_path` in `usizes.rs`.
- Fresh `usize` costs: 0 → **1.26 bits** (was 3.0), 1 → 2.26, 2-3 → 4.26,
  monotone through the small range (guarded by
  `mirrored_prior_cost_increases_through_the_common_range`).
- Fresh `u64` (uniform prior): 0 → **6.2 bits** (was 13.2), `u64::MAX`
  64.5 (was 65.6). `Small` fresh 0/1 (u64): 6 → **3 bits** (guarded by
  the `Encoded::<_, Small>::new(0_u64)` probes in `usizes.rs`).

Speed (quiesced `benches/integers.rs` A/B vs main, min of 2, 8192 values):
repeated-tiny-constant u64 decode **−55…−61%**, encode −28…−40%, encoded
size halved (92 → 48 bits); `Ans` flat-to-better on the other
distributions (skewed-small u64 decode −13%); `Range` pays **+7…14%** on
multi-magnitude data (random/sorted) for the second symbol's division —
`Ans`'s lean symbol step absorbs it.

**Per-bit `bl`-mantissa was a measured dead end on encode**: the first
iteration coded `bl`'s mantissa as adaptive bits (per-(bucket, position)
contexts) instead of a per-bucket symbol, and encode measured **+20…40%**
on mid/large values — ~5 extra buffered coder ops per value on `Ans` —
while decode was flat-to-better. Converting the mantissa to one
`AtMost` symbol per bucket (complete power-of-two trees, so they get the
fast `CompleteSpeculating` walk) recovered nearly all of it at identical
size. If encode on multi-magnitude `Range` data ever matters more,
fusing the `blbl` + offset symbols into a single coder step is the next
lever.

**Signed integers (2026-07-17, same branch)**: the default signed
`Encode` (previously a fixed-width per-bit path: sign + up to `bits-9`
unary leading-zero bools + `u8` fallback) became sign +
magnitude-through-the-same-hierarchy, with the prior *capped* at bit
length `bits-1` (`seeded_capped` — an i64 magnitude is a uniform 63-bit
value, so the top bit length gets zero prior weight). Fresh `0_i64`/-1:
64 → **7 bits**; `0_i128`: 128 → 7; `MIN`/`MAX` pay ~+1 bit. Speed
(quiesced `benches/signed.rs`, min of 2): **i64 encode −37% (Range) /
−73% (Ans), decode −57% / −35%**; i32 −13…−57%; sizes equal or slightly
better — the old path's up-to-56 sequential adaptive bools per value
were the dominant cost. **16-bit exception (resolved by keeping the old
code)**: the hierarchy initially made u16/i16 decode +9…20% slower (i16
Ans random +40%) — the old u16 tree was a *complete* power-of-two
`AtMost<15>` with the fast speculating walk in one coder step (and old
i16 additionally shortcut through the optimized u8 byte tree), so the
16-bit types gained least from a shorter path and lost most to the
two-symbol split. Both were **reverted to their legacy implementations**
(plain `U16Compact` + `geometric_seeded` single tree;
`impl_signed_default_legacy!` for i16), re-measured bit-identical in
size and within noise in speed vs main, while u32/u64/u128/usize and all
signed wide types keep the hierarchy. If 16-bit decode ever needs the
shorter tiny-value path too, the candidates remain: pad the `blbl` tree
to a complete 8 leaves, or fuse the two symbols into one coder step.

### Fresh `just-decompress` profile: the allocation story is over; forced walk fusion is a small LOSS (2026-07-19)

Re-profiled `just-decompress` (random `Vec<u64>`, `Ans`) to validate an older
claim that "decode is dominated by `memmove`/`malloc`/`free`", which predates
the hierarchical integer rework. It no longer holds — that profile's allocation
story belonged to the old `decode_incompressible_bytes`-heavy format:

- **84.8%** self cycles in the one fully-inlined `Small::<u64>::decode`
  (the two `AtMost` symbol decodes, the ≤7 partial-top-byte adaptive bits,
  value assembly — i.e. coding work on the serial chain);
- **12.1%** in the *outlined* speculative walk closures for `MAX` = 31/15
  (the `bl` 33..64 / 17..32 offset buckets — random u64's hot buckets);
- **2.1%** `__memmove_avx_unaligned_erms` (the ≤7-byte incompressible
  mantissa copy per value); **`malloc`/`free` absent** above 0.5%; the
  `[0u8; 8]` value buffer compiles to registers.

**Forcing the outlined walks to fuse is a measured DEAD END.** LLVM outlines
the `MAX` = 15/31 `from_slot_speculating` closures inside `Small::decode`
(closures can't carry `inline(always)`; marking the walk fn itself
`inline(always)` only inlines it *into* the still-outlined closure). A
whole-program `--inline-threshold=2500` build that verifiably fused them
(zero outlined walk symbols) decoded **~1.1% SLOWER**, reproducibly
(125.16–125.35 → 126.56–126.80 Gcycles; 4 alternated order-flipped pinned
rounds, within-side spread ≤ 0.15%). The naive reading of the Wave-2
"`inline(always)` on the dispatch layer is load-bearing" lesson does NOT
extend to the walk bodies: the *thin dispatch* must fuse (it did cost +13%
instructions outlined), but the *fat walk body* is better outlined — the
12% is real walk work, and keeping it out of line keeps the `Small::decode`
hot loop compact. `from_slot_speculating`'s doc comment now records this;
don't retry without new evidence. Consequence: the per-value
allocation/zeroing concern this profile was checking no longer applies, and
integer decode is now **coding-bound** — the remaining levers are
format-level (fewer/cheaper symbols), not construction-level.
`f64` decode, 100k floats × 1000 iters, pinned core (cycles):

| data                         | adaptive bits        | incompressible bytes        |
|------------------------------|----------------------|-----------------------------|
| structured (fixed exponent)  | 6.674 B/f @ 107.6B   | 8.000 B/f @ **2.02B**       |
| random (varied exponent)     | 8.191 B/f @ 108.4B   | **8.003 B/f** @ **2.05B**   |

- Incompressible decode is **~53× faster** (memcpy vs 64 adaptive decodes).
- For **random** floats incompressible is *both smaller and faster* — adaptive
  modeling can't compress random bits and slightly *expands* them.
- For **structured** floats adaptive bits win on size (compress the predictable
  sign+exponent) — so pure incompressible would *harm* compression there.

### What the `comparison` benchmark reveals
`cargo bench -p comparison` mixes representative structured data (meteorite/MTG
records and the suicide/meteorite numeric tables) with artificial stress cases.
Caveat on reading it: **"books" is NOT a target workload** — it's an artificial
benchmark built to push the Lz77 code to its scaling limit, and large text files
are *not* what `compactly` is for. Weight the structured records (meteorites,
cards, suicide tables) and short strings (names, keys) when prioritizing. Two
things stand out that the float/IPv6 micro-work above never touched:

- **The decode hot path on string-bearing records is `char`/`String`, not
  `u64`/`f64`.** Every ASCII character decodes as `bool` (`is_ascii`) +
  `Bits::<128>::decode` = **8 dependent adaptive bit-decodes per char**, and the
  tree bits are dependent (each context is `ctx.0[filled_up + accumulated_value]`,
  chosen from the bits already decoded), so `decode_bits` batching cannot touch
  them. String fields (meteorite names/recclass, card names/text) decode through
  this per-character tree walk, yet all decode optimization so far has been on
  floats and IPv6.
- **`Ans` decode is uniformly ~1.3–1.8× faster than `Range` at the same size**
  (suicide 187 vs 328 µs; meteorites 18.4 vs 23.2 ms; single cards 66 vs 77 µs),
  with encoded sizes within ~1 byte. Reinforces the `Ans` focus and a possible
  default flip once decode work consolidates there.
- **Encode is far slower than decode on structured data, but that's a known,
  deprioritized cost.** "mtg tenth edition" encodes in 894 ms / decodes in 15 ms;
  "meteorites by small name" (`Mapping<Compressible, Normal>` keys) encodes in
  707 ms vs **38 ms** with plain `Normal` keys — almost all of it `Compressible`'s
  Lz77 match search. The Lz77 encoder has already been through several optimization
  rounds and `Compressible` is not expected to be widely used, so encode speed is
  **not** a current target. The string focus below is on **decode** of the string
  strategies (`Normal`/`Compressible`/`Sorted`) and on `LowCardinality`.
  (UPDATE 2026-07-03: multisymbol coding cut the *non-Lz77* part of encode by
  15–50% anyway — see "Full comparison-suite A/B" above. The Lz77 match-search
  share, e.g. mtg's ~823 ms, is untouched and remains deprioritized.)

### Arena treap with implicit keys: `StringSet` encode −23% (2026-07-19)

Profiling `bench-arc-str encode new ipv4` (20629 distinct strings = 100%
dictionary misses, the `StringSet` worst case) showed ~60% of encode in the
treap machinery: the two `Treap::insert` walks (~33%), `memcmp` (~16%),
malloc/free of the boxed nodes and the per-insert reversed `Box<[u8]>`
suffix keys (~12%), and node drop glue — the actual entropy coding was only
~15%. Rewrote `src/string_set/treap.rs` around that profile:

- **Arena storage**: nodes in one `Vec`, linked by `u32` index — no
  per-node allocation, one free for the whole tree.
- **Implicit keys**: `StringSet` inserts entries in index order, so node
  *n* IS string *n*; the treap stores no keys or values at all (12-byte
  nodes: `u32` priority + 2 child links). Ordering comes from a comparison
  closure over `StringSet.strings` — the prefix treap compares the strings
  directly, and the suffix treap compares reversed bytes *on the fly*
  (`rev_cmp`), eliminating the materialized reversed copy of every string.
  (`rev_cmp` trick: a little-endian `u64` load of a block taken from the
  string's *end* puts later bytes in more-significant positions, so plain
  integer compares walk the reversal 8 bytes at a time.)

Quiesced A/B (`perf stat` cycles, min of 3 alternating rounds): encode
**−23.1% cycles** (20.68 → 15.91 Gcycles, −7.6% instructions), encoded
bytes verified identical (same total order ⇒ same neighbors ⇒ same
stream), decode and the cache-hit path unchanged. Wall clock for the
summary run: 63.6 → 46.1 ms (old dictionary-only encoding: 22.2 ms).

**Measurement trap worth remembering**: the plain A/B showed decode
"+5% cycles, +2.3% instructions" — *surviving* both the forced-alignment
rebuild and `codegen-units=1`, on byte-identical input through
byte-identical decode symbols. It was glibc **allocator-state luck**: in
`bench-arc-str decode`, the setup encode runs in-process first, and what
its context *frees* (41k boxed nodes + 20k reversed keys vs. two flat
`Vec`s) shapes the tcache/bins that the decode loop's ~20k-per-iteration
`String`/`Arc` mallocs then hit. Re-running with
`GLIBC_TUNABLES=glibc.malloc.tcache_count=0` collapsed the delta to −0.6%
(and `decode old`, whose path shares no changed code, was flat all along).
When an in-process A/B has a *different-allocation-history phase before
the timed loop*, adjudicate apparent deltas with the tcache knob before
believing them — instruction-count changes included.

### Survey of remaining gaps (2026-07-19)

A read-through of every `v2` type implementation, looking for paths that never
got the optimization treatment. Two findings led to shipped fixes: **most of
the flagship string workload (>50%) was `BTreeSet`/`BTreeMap` construction**,
not coding — std's `FromIterator` bulk-builds from sorted input far faster
than a per-element `insert` descent (measured **−79%** on a standalone
construction A/B) — and a suspected bug in the float `is_int` fast path for
negative values turned out, once checked, not to be a bug at all. The other
gaps this survey opened (the hybrid float split, Lz77 decode allocations,
`Sorted<Vec<T>>`'s clone-per-collection) are tracked in the TODO list below
where still open, and in "Landed" where since resolved.
### Ans chunked format: the cost is per-batch boundary tracking, not chunking (2026-07-27)

Chunking `Ans` (self-contained rANS chunks every `CHUNK_OPS = 1<<16` ops, so
encoder/decoder memory is bounded and the format can stream) was A/B'd against
pre-chunking `a9184ef`. Quiesced, alternating, min of 3, reps within 0.3%:

| phase | first cut | after `CHUNKED=false` |
|---|---|---|
| encode, multi-chunk (100k u64) | +2.41% | **+2.27%** |
| decode, multi-chunk (100k u64) | +6.06% | **+5.71%** |
| encode, single-chunk (2k u64) | +0.95% | **−0.87%** |
| decode, single-chunk (2k u64) | **+21.51%** | **−1.07%** |

The +21.5% on a *single-chunk* decode was the surprise — one never-taken branch
should be free. Isolated by stripping just the `ops_left` load/store/decrement
from the two hot decode paths: that alone restored it to −1.55%. So the entire
regression was **per-batch boundary bookkeeping**; the framing and the per-chunk
rANS state flush cost essentially nothing. It hurts most on cache-resident data
(compute-bound); on 100k u64s the memory traffic dilutes it to ~6%.

Fix: `Decoder<'a, const CHUNKED: bool>`. A stream whose *first* frame is final
(op-count 0) is single-chunk — the common case — and needs no tracking at all,
so `Ans::decode` peeks the frame and instantiates `CHUNKED = false`, compiling
the checks and decrements out entirely. Single-chunk decode is now at parity
(−1.07%). Multi-chunk still pays ~5.7% decode / ~2.3% encode, which is the real
price of bounded memory + streamability for huge values.

**The residual multi-chunk decode cost is per-op, and resisted every mitigation.**
Sweeping `CHUNK_OPS` (100k-u64 decode, vs pre-chunking baseline) confirms it is
invariant to chunk size — bigger chunks help only by making a value *single*-chunk:

| `CHUNK_OPS` | encode | decode |
|---|---|---|
| 1<<12 | +2.90% | +5.85% |
| 1<<14 | **+0.97%** | +5.80% |
| 1<<16 (shipped) | +2.29% | +5.71% |
| 1<<18 | +3.75% | +5.92% |
| 1<<24 | +4.44% | **+0.32%** ← workload is now single-chunk |

Decode is flat at ~5.8% for every size that actually chunks. Encode has a sweet
spot at 1<<14 (op-buffer locality in the reverse pass vs per-chunk fixed cost)
and degrades as chunks grow. `1<<16` is kept because *lowering* it pushes more
values into the penalized multi-chunk regime, and single-chunk decode is free;
raising it hurts encode and unbounds memory, which is the whole point.

`perf` on multi-chunk decode: 83% of cycles in the monomorphized
`Small<u64>::decode::<Decoder>`, whose hot loop shows `test %r11,%r11` + `dec
%r11` (the `ops_left` guard/decrement, ~3.7% combined) plus stack-spill reloads
and a 136-byte frame — i.e. the cost is the bookkeeping *plus* the register
pressure from the extra field. Four mitigations, all **measured worse** than the
straightforward version (each quiesced, alternating, min of 3):

- `saturating_sub` -> `wrapping_sub` (drops a cmov): **+6.43%**
- `#[cold] #[inline(never)]` on `load_next_chunk`, which `perf` showed being
  inlined (varint parsing and all) into the hot decode path: **+7.61%**
- `ops_left` as `u32` instead of `usize`, to shrink the struct: **+6.83%**
- Using `state == 0` as the boundary signal, dropping the counter entirely:
  **incorrect.** The rANS state returns to exactly 0 at a chunk's end only for
  pure-*bit* chunks (as `check_ans_coder` asserts); once whole-**symbol** steps
  are interleaved it does not, so the decoder misses the boundary and runs into
  the next chunk's bytes. (It "measured" −30% precisely because it had stopped
  decoding correctly — the same trap as any A/B against broken code.)

Another instance of the codegen-sensitivity theme: every variant doing strictly
less work ran slower. Treat the shipped form as a local optimum unless something
structural changes.

**Build-ops vs entropy-code split** (measured on pre-chunking, where the phases
separate cleanly) — the ratio is workload-dependent and *inverts*:

| workload | build `Vec<Op>` | entropy-code |
|---|---|---|
| 38k meteorite strings | 79.4% | 20.6% |
| 100k `u64` | 37.4% | 62.6% |

Relevant to the idea of entropy-coding each chunk on a background thread while
the main thread keeps recording ops: a perfectly overlapped two-stage pipeline
costs `max(build, entropy)` instead of the sum, i.e. a ceiling of ~1.26x on
strings but ~1.60x on numeric data. Chunks are the natural handoff unit (each is
an independent rANS stream), and the stages share no mutable model — contexts
adapt during recording, so stage 2 only needs the finished op vector.

### Streaming decoder: the single-chunk fast path is worth even more there (2026-08-08)

The `CHUNKED` const generic that saved 21% on the in-memory `Decoder` was
initially only on that decoder; `AnsDecoder<R>` always tracked boundaries.
Giving it the same `const CHUNKED: bool` (`Ans::decode_from` reads the first
frame's tag and picks, since a stream cannot be peeked and re-read) pays off
harder than it does in memory. New A/B bin `just-decompress-stream`, which
forces both instantiations over the same single-chunk bytes in one build
(`Cursor<&[u8]>` source, so no filesystem in the measurement); quiesced,
alternating, min of 3, reps within 0.4%:

| streaming, single-chunk 2k `u64` | cycles | instructions |
|---|---|---|
| `CHUNKED = true` | 10,084,229,270 | 25,838,387,233 |
| `CHUNKED = false` | 7,466,523,825 | 22,981,489,923 |
| | **−25.96%** | **−11.06%** |

**Clamping a cursor that cannot be out of range cost 2.7%.** Both streaming hot
paths re-derived their slice as `entropy[epos.min(entropy.len())..]` and then
recovered `consumed` via a saved `before` length. `epos <= entropy.len()` is an
invariant (`enter_chunk` sets it within the new region; each step advances it to
`len - remaining.len()` for a suffix), so the `min` was provably dead and the
length bookkeeping redundant. Dropping both: streaming decode **−2.65% cycles,
−4.25% instructions**, no change to the slice decoder. Invariant is now
documented on the field so it stays checkable.

**How much does streaming cost over memory?** With both fast paths in, the same
bin decodes the same bytes through `Ans::decode` (borrowing slice decoder) and
`Ans::decode_from` (owning streaming decoder over `Cursor<&[u8]>`):

| workload | slice | stream | stream cost |
|---|---|---|---|
| 2k `u64` (single chunk, cache-resident) | 6,908,825,341 | 7,263,957,888 | **+5.14%** |
| 100k `u64` (multi-chunk, memory-bound) | 10,834,965,673 | 11,458,232,609 | **+5.75%** |

(Before the clamp fix these were +8.1% / +9.2%.) Instructions run ~19% higher
while cycles are only ~5% higher — the extra work is largely absorbed by
memory-level parallelism. Notably the residual is **not** the owned-buffer copy:
`perf` puts `__memmove_avx_unaligned_erms` at just 2.4%, with 70% in the
monomorphized `Small<u64>::decode`. It is the per-batch cursor bookkeeping —
the streaming decoder loads `entropy` (ptr+len) plus `epos` and stores a
recomputed `epos`, where the slice decoder loads and stores a `&[u8]` directly.

So **collapsing to a single decoder implementation would cost ~5% on the
default in-memory path** — the remaining gap is structural (owned buffer +
index cursor vs. borrowed slice), and closing it without `unsafe` would need
the decoder generic over a region-supplier trait so `&[u8]` can stay zero-copy,
which keeps two cursor implementations anyway. Not attempted; recorded so the
tradeoff is a decision rather than a rediscovery.
### Sweeping `MAX_PRODUCT`: one byte is a DEAD END, 135 is a free win (2026-08-08)

`MAX_PRODUCT` (in `generate_bit_context.rs`) caps which `(trues, falses)` count
pairs get their own `BitContext` state — `Bucket::new` keeps a pair only while
`(1 + trues) * (1 + falses) < MAX_PRODUCT`. It therefore sets **both** the state
count and the model's maximum confidence, whose floor is the deepest pure-true
state `True(M−2)False0`. Both directions were swept. Short version: shrinking to
one byte loses, and **135 is a strict improvement over the current 134** (landed
in this PR).

#### Raising to 135 — the free notch (LANDED)

`Probability` is `prob / 256` with `prob: NonZeroU8`, so **1/256 is
representable**, but at `MAX_PRODUCT = 134` the deepest state is `True132False0`
= 2/256: the table stopped one notch short of what the type can express.
`MAX_PRODUCT = 135` adds exactly **4** states (675 → 679), the deepest being
`True133False0` = 1/256, halving the fully-adapted floor from ~11.3 mb per bit
to ~5.7 mb. No cap above 135 can sharpen *this* floor further — `prob` bottoms
out there — so higher caps only buy interior resolution.

**But the model is not symmetric, and the other side still has headroom.** The
mirror-image pure-*false* chain tops out at 254/256 (~11.3 mb), a notch short of
the 255/256 a `NonZeroU8` could hold, because `Distribution::best()` searches
`(1..255)` — an exclusive upper bound, so 255 is never a candidate. A
maximally-predicted `false` bit therefore still costs twice a maximally-predicted
`true` one. That is `best()`'s bound to relax, not `MAX_PRODUCT`'s, and this PR
deliberately leaves it alone — but it looks like a second free notch of the same
kind, and `Probability::new` already emits 255 for large `falses`, so the coders
should handle it (rANS needs only `zeros = 256 - ones >= 1`). Worth measuring
next.

It is free on both axes. **Still 2 bytes**: 679 is in the same `u16` bucket as
675, so every `Encode::Context` keeps its exact size and the tables grow by
8/16/24 bytes total. **Speed-neutral**: force-aligned, instructions came out
marginally *fewer* on all six workloads (−0.001% to −0.005%: strings `Ans`/`Range`,
Lz77, enums, random `u64`, IPv6) with every cycle delta inside the layout
residual. And it cannot touch the frozen format — `v1` has its own
`bit_context.rs`.

Encoded size, real data: meteorite names −0.216% (42630 → 42538 B), Lz77 CSV
−0.048%, IPv6 −0.007%, enums unchanged. Modest overall, but the redundant cases
it targets move hard: `Range::encode(vec![true; 8192])` **17 → 10 bytes**,
`encoded_bits!(BTreeSet…)` 159 → 148, `Encoded<_, Small>` 130 → 119,
`BTreeSet::from_iter(0..1024)` 87 → 82 bits. Two negligible regressions
(8985 → 8989 bits; 284762 → 284908).

#### The ladder above 135 — a real tradeoff, not a win

A full sweep (encoded bytes; strings = meteorite names, lz77 = meteorite CSV):

| M | strings | enums3 | enums17 | lz77 | ipv6 |
|-----|---------|--------|---------|--------|--------|
| 134 | 42630 | 17581 | 52228 | 622455 | 632707 |
| **135** | **42538** | 17581 | 52228 | **622159** | 632662 |
| 136 | 42573 | 17585 | 52235 | 622224 | 632686 |
| 140 | 42575 | 17586 | 52210 | 622004 | 632529 |
| 160 | 42604 | 17554 | 52130 | 621813 | 631996 |
| 200 | 42700 | 17503 | 52084 | 621644 | 631303 |
| 256 | 42830 | 17475 | 51924 | 621318 | 630473 |

135 is the unique point better-or-equal everywhere; 136 already regresses on all
five. Past it the two effects separate cleanly — **strings degrade monotonically
while stationary bulk data keeps improving** — because a bigger state set adapts
more slowly but estimates a stationary source more accurately. Strings sit on the
fast-adaptation side (thousands of per-character contexts, each seeing few
samples); enums/IPv6/Lz77 sit on the other (few contexts, ~100k samples each).
So a cap above 135 is a workload bet, not an upgrade. If it is ever revisited,
note that M=256 costs strings +0.47% to buy IPv6 −0.35%.

#### One-byte `BitContext` (249 states) — DEAD END

Dropping `MAX_PRODUCT` from 134 to 60 (249 states, the largest bucket cap that
fits a byte) halved every `Encode::Context`'s memory as predicted — and
bought no speed at all: contexts were already comfortably L1-resident, so
there was no miss traffic to remove. Meanwhile the probability floor doubles
(the maximally-predicted-bit fixed point moves from 2/256 to 4/256), so more
coded bits means **more instructions on every redundant real workload** (Lz77
+3.68%, meteorite names +1.07%, IPv6 +1.06%) and larger encoded size
(meteorite names +1.29%, `BTreeSet::from_iter(0..1024)` **+49%** — the most
redundant streams pay most, since the 2/256 floor was doing all the work
there). Two float/enum rows moved the other way pre-alignment-rebuild but
collapsed to a wash once force-aligned — another instance of "believe
instructions, not cycles" on this machine.

**Don't retry a plain reduced-state cap.** A smarter non-uniform 256-state
layout (keep the long confidence chains at full depth, quantize only the
interior states) could recover most of the size, but would be chasing a speed
win this experiment shows isn't there: at *zero* size cost, halved contexts
measured a wash on every workload.
### Same collapse, `Range` side: also ~5% / +15% instructions — keep both (2026-08-08)

After the streaming-IO unification (PR #44) gave both coders a uniform
`type Reader`/`new`/`into_result`, `RangeDecoder<R>` can be instantiated on
`R = &[u8]`, so the natural question is whether it can *replace* the bespoke
borrowing slice `Decoder<'a>` and delete a decoder type. It cannot — same verdict
as the `Ans` side above. A/B bin `range-decode-collapse` (`slice` = `Range::decode`,
`stream` = `Range::decode_from::<_, &[u8]>` — a bare `&[u8]` reader, the most
favorable case, no `Cursor`), same bytes/build, each arm monomorphized; quiesced,
min of 3 (reps within 0.2%):

| workload | metric | slice | stream | stream cost |
|---|---|---|---|---|
| 2k `u64` (cache-resident) | instructions | 26,598,852,916 | 30,596,790,435 | **+15.03%** |
| | cycles | 12,759,685,276 | 13,446,695,236 | **+5.38%** |
| 100k `u64` (memory-bound) | instructions | 26,581,744,879 | 30,614,723,151 | **+15.17%** |
| | cycles | 14,021,567,469 | 15,226,268,683 | **+8.59%** |

Instruction count is a steady **+15%**; cycles +5–9% (widening memory-bound,
opposite of `Ans`'s memory-parallelism absorption). Structural cause, visible in
source: `Decoder<'a>` has a hand-fused batch `decode_bits` keeping
`state`/`value`/`bytes` register-resident and indexing via `split_first`, while
`RangeDecoder<R>` pulls **one byte at a time through `Read`** (`read_one_byte`)
with error-latch branches and a non-fused loop — `<&[u8] as Read>::read` does not
optimize down to the fused path. Keep both decoders. (Reproducer, now that `range-decode-collapse` has been folded into
`coder-routes`: `quiet-bench run ./target/release/coder-routes u64 range
slice|from`.)

### Async decode: what the machinery costs, and how it was paid down (2026-08-09 to 2026-08-28)

This arc — from the first async decoder to the current design — is recorded
as one narrative rather than each measurement's original snapshot, since most
of the intermediate numbers were superseded within the same arc. The
load-bearing pieces that came out of it are `Encode::MAX_BYTES` and chunk
alignment (both described in full below); treat the rest as the reasoning
trail that got there.

**The starting cost was steep and highly uneven.** Feeding the whole buffer as
a single chunk (so no await ever actually suspends, making the machinery cost
countable in isolation) showed the async traversal costing **+56–62% cycles,
+123% instructions** on integers but only **+10.5% cycles, +44% instructions**
on strings, against the same decode through the plain sync path.
`decode_bits`/`decode_symbol_step` becoming futures means every await point
loses inlining and register-residency; the spread tracks **await density, not
total work** — `Small<u64>::decode` is a chain of many small awaits (several
`AtMost` symbol steps, an incompressible read, per-bit partial-byte decodes)
where a `char`'s whole 8-level tree walk is fused into one `decode_symbol_step`
call. On this machine that put integer decode at ~56 MB/s of compressed input
through the async path against ~100 MB/s through the slice decoder — not
obviously hidden behind a slow network, so worth reducing rather than
accepting.

A wall-clock overlap measurement (`async-decode-overlap`: a simulated
constant-rate network feeding `decode_stream`, compared against "await
everything, then decode") confirmed the API's overlap *shape* was already
correct — `decode_stream` sits pinned at its own decode time regardless of how
fast chunks arrive, i.e. `max(arrival, decode)` against the baseline's
`arrival + decode` — but that decode time was the *async* one, 1.68× the sync
decoder's on this workload, so break-even against the naive baseline only
arrived around 50 MB/s and `decode_stream` was *slower* above it. Fixing that
ratio, not the overlap logic, was the rest of the work:

1. **Single-chunk fast path.** A source that has already delivered everything
   has no overlap to offer, so `decode_stream` checks for that up front and
   hands the whole buffer to the ordinary sync slice decoder. Free to within
   measurement noise (+0.01–0.03% cycles) on both integer and string
   workloads, and it collapses the entire async penalty for every value small
   enough to arrive in one piece — every value already in memory, and every
   value small enough to arrive whole.

2. **`Encode::MAX_BYTES` — sync-decode anything that fits in the buffer.**
   Each type declares the most bytes one value can occupy; the decoder runs
   the *sync* implementation for any value whose bound already fits in what
   has arrived (`with_sync`), so waiting is confined to moments there is
   genuinely nothing to decode. Default is `usize::MAX` (opt-in, safe by
   construction — no existing impl had to change to introduce it). Getting the
   bounds tight mattered a lot: the first cut charged a *renormalization*
   worst case per bit (up to 8 bytes), which is real but not additive across
   bits — the right split is **information** (additive per operation: ≤1
   byte/bit, ≤3 bytes/symbol, 1 byte/incompressible byte) plus **settling**
   (a fixed margin added once per handoff by `sync_capacity`, never by
   callers, derived from the coder's window-width invariant rather than per
   operation). That distinction took `u64`'s bound from 95 bytes to 20 and was
   worth roughly 3× at small chunk sizes. Every bound is property-tested
   against real decodes under adapted contexts (`v2::max_bytes`), where the
   worst case lives; observed-vs-declared margins are now ~2× rather than
   ~10×, with `with_sync`'s own `debug_assert` (fires if a decode consumes
   every buffered byte without the stream having ended) as the backstop
   against a bound that's wrong rather than merely loose. Net result on 100k
   `u64` (802 KB): **+1.7% instructions at 64 chunks**, where the single-chunk
   fast path alone had left +151%. `Values<Vec<T>>` batches several elements
   per handoff so the sync decoder's state stays register-resident across a
   run rather than round-tripping per element — the shape every other
   collection eventually needed too (item 7 below).

3. **`ChunkSource` coalescing.** `ready_bytes()` originally meant "one stream
   chunk," not everything actually delivered, so a producer emitting small
   chunks made the `MAX_BYTES` gate fail even when the whole input had
   arrived. `ChunkSource` now polls until `Pending`, coalescing everything
   already available (capped at `READY_TARGET` = 256 KiB, so a fast transport
   isn't simply drained into memory — the bounded-buffer property the
   streaming decoder exists for). This flattened a 1.26–9.26% spread across
   chunk sizes down to a nearly flat ~2.4–3%, with real wins at small chunk
   sizes (meteorite strings at 783-byte chunks: −3.9%) and no regression at
   the large end.

4. **`Ans`-specific: the frame drain, then not being too eager about it.**
   `Ans` can only decode a frame once it has arrived whole (its incompressible
   bytes live after the entropy region), but every op inside `enter_chunk`
   delegates to the sync implementation, so the async decoder only needs to
   suspend *between* frames. First cut: **+136% cycles, +200% instructions**
   against the sync slice decoder on 100k `u64`. An O(1) read-ahead check
   (previously O(frames), scanning a `VecDeque` on every bit and every
   symbol) took a fifth of the async path's instructions off on its own.
   Draining every already-arrived frame, not just one frame of look-ahead,
   brought `is_final` forward to the moment the last frame lands rather than
   only ever covering the tail — down to **+4.5% cycles, +10.5%
   instructions**, which is the owned-buffer copy cost (`Ans` currently copies
   every byte four times on its way to the decoder — see the open TODO item
   below) rather than the async machinery itself.

   That drain bought no *overlap*, though: `read_ahead` awaited the whole
   next frame the moment the current one was entered, serializing arrival and
   decode (`arrival + decode` at every rate, confirmed independently by both
   the wall-clock overlap measurement and the instruction count, from
   opposite directions). The fix was to defer that await until `ops_left`
   says the sync decoder is actually about to need the next frame
   (`OPS_MARGIN`), while keeping two cheap habits that turned out to be
   load-bearing: a non-suspending peek that takes a frame only once it has
   *wholly* arrived (deferring alone regressed fast rates by 77%, because the
   old eager drain was what got `is_final` true early), and pumping the
   source on a `PUMP_INTERVAL` throttle so `ready_bytes` doesn't go stale
   while a buffered frame decodes. Net result: `Ans::decode_stream` moved
   from *always* `arrival + decode` to tracking `max(arrival, decode)` — at
   10 MB/s, 81.4 ms decode against 80.3 ms of pure arrival, i.e. the decode is
   essentially entirely hidden — for **+2.0% instructions** in the
   never-suspending case where none of that benefit is visible. Measuring
   headroom against the true `max(arrival, sync_decode)` ideal afterward
   showed it peaks (~40%) exactly where arrival and decode are balanced and
   vanishes at both extremes — the shape that motivated the next, bigger
   change.

5. **Chunk alignment: making the decode-side gate free instead of adding
   `MAX_OPS`.** The remaining headroom needs the sync decoder reachable
   *mid-stream*, which means answering "does this value's decode stay inside
   the buffered frames?" — and `Ans` frames are delimited by **op count**, not
   bytes, so `MAX_BYTES` alone can't answer it (one op can emit zero bytes or
   thousands). A mirrored `MAX_OPS` constant, with its own property-testing
   burden, was the obvious answer and is not what got built. Instead, since
   both sides of the format are ours: make the *encoder* refuse to flush a
   chunk boundary in the middle of a bounded value. **The rule:**

   > An `Encode` impl whose `MAX_BYTES` is `usize::MAX` must call
   > `EntropyCoder::split_point` between the parts it encodes. A **bounded**
   > impl must not.

   `split_point` is a provided no-op that `Range`/`Millibits` fold away;
   `AnsEncoder` overrides it with what used to be its unconditional
   op-count-based flush check, so a non-final chunk is now flushed *only* at
   a declared split point. A bounded value therefore provably lies entirely
   within one chunk — no depth tracking needed (containing a split point
   makes a value unbounded by construction, via `MAX_BYTES`'s
   `saturating_add`/`max` composition, so being at a split point *is* being
   at depth zero), no trait-method rename, and it needed almost no new call
   sites: every length-driven encode loop already calls `Sentinel::encode`
   once per element at exactly the right moment, so the split point
   piggybacks there for `Vec`, `Box<[T]>`, `VecDeque`, `Sorted`, maps, sets,
   and `String`; two loops without a `Sentinel` (the `Incompressible`
   byte-piece loop and low-cardinality's `chars` loop) declare their own.
   Composites (tuples, arrays, derived structs) need nothing — a value is
   unbounded *because* one of its parts is, and every unbounded part in this
   crate already splits. The decode-side gate collapses to
   ```rust
   const { T::MAX_BYTES != usize::MAX } && ops_left > 0
   ```
   one const-folded bool and one nonzero test, both derived from state the
   sync decoder already tracks. The `ops_left > 0` half matters: alignment
   leaves a value either wholly in this chunk or wholly in the next, and the
   second case opens with a fresh `load_next_chunk`.

   An initial version also capped atomic-value size (`CHUNK_ATOMIC_MAX_BYTES`)
   so a huge bounded value (e.g. `[u64; 100_000]`) would still be split —
   dropped once looked at properly: a **non-final** frame's declared entropy
   length has to be capped regardless (real amplification — ten bytes of
   varint could otherwise ask for a `usize::MAX` buffer), and now is,
   *exactly*, derived from the frame's own op count in its header; the
   **final** frame has no declared length and needs none, since making the
   decoder hold N bytes there already requires a peer to have sent N bytes —
   1:1, no amplification, and bounding an untrusted reader is `Read::take`'s
   job, not the format's. So no policy constant remains in the chunk
   machinery; every check is derived from the frame in front of it. Wrong
   input (a boundary genuinely mid-value, e.g. from a peer that didn't run
   this encoder) becomes a decode `Err`, not corruption or a hang — pinned by
   a dedicated test that halves a frame's declared op count to force exactly
   that case.

   Measured on the balanced `RATE_MBPS = 0` regime the headroom curve peaks
   at: **−36.9%** wall clock (30.98 → 19.56 ms), decaying toward a small
   (+1.5–1.9%) real, repeatable per-handoff cost at very fast arrival rates —
   fixed by the next step.

6. **`can_continue`: re-asking the gate from inside a run.** `sync_capacity`
   has to answer before a handoff opens, and mid-stream all it can lean on is
   `ops_left`, which only ever promises **one** value — a limit on what's
   knowable in advance, not on what's decodable. `EntropyDecoder::can_continue`
   re-asks the same question after each element inside the run (for `Ans`,
   `ops_left > 0`); its default is `false`, so `Range` — whose budget is an
   exact byte count — is unaffected (measured −0.10%, inside noise), and no
   const gate was needed to keep it that way. This is what actually removed
   the fast-arrival regression from the step above and deepened the peak: at
   the balanced rate, **−39.16%**; at fast arrival, the regression *inverts*
   to a small win (−4.6% to −8.5%) instead of the earlier +1.5–1.9% cost.
   Handoff counts moved from tens of millions of per-element handoffs back
   down to thousands (roughly one every 2,700 elements on 1M `u64`s).

7. **Batching every collection, not just `Vec`.** An audit found `Vec<T>` was
   the *only* collection handing several elements to the sync decoder per
   handoff — the other fourteen (both `String` loops, the maps, the sets,
   `VecDeque`, `Box<[T]>`, `Sorted<Vec<T>>`, `bytes`, `low_cardinality`) paid
   a full `with_sync` handoff **per element**, which for `Range` means
   copying `state`/`value`/`bytes` into the slice decoder and back out every
   single element. Converting `String` to `Vec`'s batching shape
   (`sentinel::decode_elements`, a shared `pub(crate)` helper needing only the
   two primitives `sync_capacity` + `with_sync`) measured **−20.6% to −24.9%
   instructions** on a `Vec<String>` workload, unchanged on `u64` (where the
   outer `Vec` was already batching) and on `Ans` (all-or-nothing on
   `reached_final`, so this is a `Range` mid-stream win). All twelve
   worthwhile loops now go through the one helper; `bytes.rs`'s Lz77 loop and
   `low_cardinality`'s per-char loop are structural holdouts (no `&mut C`
   sink to hand it — Lz77's per-element work is a back-reference splice where
   `self` is simultaneously the element context and the output sink).
   Taking the sink as a one-method `ExtendOne` trait rather than a closure
   recovered the last ~0.5% a closure cost from blocked context-hoisting.

   A follow-up review pass removed two more pieces that measured zero: a
   hand-written `has_room_for` override that LLVM already emitted from the
   provided body (one caller, one override, no delta once removed), and an
   unnecessary `Bytes` clone in `Range::with_sync` (**−1% instructions, −1.2%
   cycles** on strings), plus a never-taken sentinel-tick branch inside the
   batch loop. Combined: **−3.14% instructions / −1.21% cycles** on the
   `Range` string batching path, no change to `u64`.

**Durable methodology note from this arc: an always-`Ready` test stream
measures the wrong decoder, silently.** `ChunkSource`'s look-ahead drains such
a stream to completion before decode starts, so the single-chunk fast path
takes over and the async decoder is never even constructed — the chunk size
passed to the fixture is irrelevant. `tests/derive.rs`'s async-decode module
did exactly this from the day it was written (a probe inside the async branch
counted **zero** hits across the whole module) until a `Pending` was added
before each chunk. **Any transport meant to exercise the async decoder must
yield `Pending` at least once, and the fixture must not fit in a single
chunk** — both test harnesses now enforce and document this.

One tried-and-reverted idea from along the way: charging a handoff's consumed
ops to the transport-pump's poll countdown, to restore a per-op polling
cadence through a batched handoff. Measured **slower** near the balanced
arrival/decode rate — nothing is actually waiting on the polls that get
skipped, since a frame is buffered whole before any of it decodes and its
successor is fetched once `OPS_MARGIN` ops remain.

**What's shipped today**, for anyone reading only this summary: `Encode`
declares `MAX_BYTES`; `AsyncEntropyDecoder` requires `sync_capacity` and
`with_sync`, with `can_continue` as an opt-in re-check (`false` default); the
encoder-side contract is "unbounded impls call `split_point` between parts,
bounded impls never do"; and every collection's async decode goes through
`sentinel::decode_elements`. See `Encode::MAX_BYTES`, `EntropyCoder::
split_point`, and `AsyncEntropyDecoder` in `src/v2/` for the current trait
shapes, and `plans/async-encode.md` for how this is expected to extend to
encode.

## TODO (in rough priority order)

1. **Let the mid-stream handoff escalate once the source completes** — the
   batch path never reaches `read_ahead`, which is the only thing that
   notices `ChunkSource::is_complete`, so `sync_capacity` is slow to graduate
   to `usize::MAX` for an **unbounded** `T`. That matters because only that
   graduation lets the outer loop batch at all: 96% of a `Vec<Record>` still
   decodes field-by-field through the async state machine even once the
   whole source has arrived. Worth **+12.1% → +0.7%** on `records` and
   **+5.9% → +0.2%** on `strings` (cycles vs `main`, no-delay). The measured
   fix is an `AsyncEntropyDecoder::pump` that `Ans` forwards to its
   already-throttled `read_ahead`, called once per batch — but it costs
   **+2.3% instructions** on `Range`'s async path (a suspension point inside
   `decode_elements`'s state machine that a const-gated `if` cannot remove,
   since the state still has to exist in the generated future). So this
   wants either a coder-specialized `decode_elements` or no
   `AsyncRangeDecoder` at all — the concrete thing dropping async for `Range`
   would buy.

2. **Let a handoff cross into a frame that has already arrived.**
   `can_continue` answers `ops_left > 0`, so a run ends at every chunk
   boundary even when the next frame is wholly buffered. The exact re-check
   is `ops_left > 0 || reader.has_unentered()`, but `has_unentered` lives on
   `FrameBuffer` and `AnsDecoder` is generic over its reader, so reaching it
   means a bound the public `Ans::decode_from<T, R: Read>` cannot carry.
   Worth one handoff per frame rather than per element — small next to what
   `can_continue` already took, listed for completeness.

3. **Batch a derived struct's bounded fields into one `with_sync`** — the
   same insight as `decode_elements`, applied to structs instead of
   collections. A derived `decode_awaiting` calls `decode_async` **per
   field**; when the struct is bounded overall none of that runs, but one
   `String` or `Vec` field saturates the type to `usize::MAX`, and then every
   *other* field pays its own gate and its own handoff — which is most real
   record types. The derive already computes each field's `MAX_BYTES` at
   compile time, so it can partition fields into maximal runs of bounded
   ones and emit one `sync_capacity(sum_of_run) > 0` gate plus one
   `with_sync` per run — no new trait surface, since `sync_capacity` already
   takes a byte count rather than a type for exactly this reason. Unmeasured;
   size the win first on a struct with several scalar fields beside a
   `String`, mid-stream on `Range` (the `Ans` arm can't benefit until item 1
   above lands).

4. **`Ans`'s `Read`/`Write` plumbing copies every byte four times** —
   transport chunk → `ChunkSource`'s coalesced buffer → `buffer_next_frame`'s
   frame `Vec` → `FrameBuffer::bytes` → `AnsDecoder`'s
   `entropy`/`incompressible` `Vec`s. The slice decoder does none of them.
   The design is already worked out: delete `FrameBuffer` and run the slice
   `Decoder<'a, true>` over `ChunkSource`'s buffer, capped at the last
   complete frame — the same `with_sync` handoff `Range` already uses, saving
   `entropy`/`incompressible`/`rest` as offsets. Sized at ~5% cycles / ~19%
   instructions on entropy-coded workloads, but the route table at the top
   of this document says the ceiling is far higher on incompressible data:
   `f64` through `decode_from` is **+326.7%** against `Range` (where `slice`
   is only +32.7%), charged per *incompressible* byte rather than per byte
   overall.

5. **`Ans`'s incompressible-byte path is slower than `Range`'s on decode** —
   and this is *not* the plumbing above, since it shows on `slice`, which
   does none of that buffering: decode `slice` **+32.7%**. Nothing recorded
   predicts it; the first step is a profile (`perf record` on `coder-routes
   floats ans slice` against the `range` arm), not a patch.
   - **The encode half of this item has reversed and is closed.** It read
     "slower in both directions", on a 2026-08-28 measurement of encode
     **+52.1%** cycles / **+37.7%** instructions. The 2026-09-05 table has
     `floats` encode at **−27.4±0.1%** — `Ans` now the faster of the two by a
     wide margin, on both encode routes. The only change to the coder between
     the two tables is the ANS chunk-alignment work merged in
     [#54](https://github.com/droundy/compactly/pull/54); nothing here has
     confirmed that is the cause, and it is a large enough swing to be worth
     understanding if anyone touches this path.

6. **`Ans` encode is time-bound, not work-bound** — `enums` costs **+28.7%
   on 6.9% *fewer* instructions** than `Range` (the instruction counts are
   from the 2026-08-28 `perf` pass; the times are current), and `strings`
   holds the same shape at **+5.0%**. The claim used to extend to `enums17`
   and the whole `AtMost` ladder; on the 2026-09-05 table it no longer does —
   `enums17` has shrunk to +1.8% and the ladder has crossed over, from +1.5%
   at `atmost3` to **−8.8%** at `atmost128`. So this is now about two
   workloads rather than a general property, which narrows where to look.
   `Ans` executing less and taking longer is an IPC problem, pointing
   at the two-pass structure (record ops, then encode backwards) rather than
   the coding itself — the largest result against `Ans` that isn't about
   incompressible bytes, and the one to explain before making it the
   default. A flat profile (2026-08-28, one workload, machine not quiesced)
   puts `AnsEncoder::flush_chunk` (the second, entropy-coding pass) at
   **17.9%** of cycles against `<u8 as Encode>::encode` at 21.7% — so the
   *first* pass (recording ~384 KiB of ops per chunk to memory and back) is
   the larger suspect. Two follow-ups, cheapest first:
   - **Shrink `CHUNK_OPS` so the ops buffer is L2-resident** — a one-line,
     no-API-change experiment (it moves chunk boundaries and the
     `expect-test` size assertions with them, which is fine — `v2` is
     unfrozen).
   - **Encode frames in parallel.** `flush_chunk` is a pure function of
     `(ops, incompressible_bytes)` — it never touches a `BitContext`, since
     `encode_bits` resolves the probability at record time and stores it in
     the op — so frames are embarrassingly parallel across chunks (K workers
     plus a sequence number and reorder buffer), and being a pure function
     this **cannot change the output**, so there's no format question and
     byte-identity is free. Don't read 17.9% as a cap: with full overlap the
     win is `(r + e) / max(r, e)` for record cost `r` and entropy cost `e`,
     and both are independent optimization targets that will tend to even
     each other out as work goes into whichever is limiting — the
     `CHUNK_OPS` experiment is a complement to this, not a substitute for it
     (shrinking `r` only makes parallelizing `e` *more* attractive). The
     cost is threads: a `std::thread` pool behind an optional feature,
     bounded double-buffering to keep peak memory stated, and emphatically
     not a spawn onto the caller's async executor. See `plans/async-encode.md`,
     which is written so as not to foreclose this.

7. **There is no async encode, for either coder.** "Async" currently means
   decode only — worth remembering whenever the route tables above tempt a
   conclusion about dropping `Range`'s async path, since that decision is
   narrower than it looks and not symmetric with anything on the encode
   side. A plan covering both coders lives at `plans/async-encode.md`
   (PR #49): it builds the traversal once over an `AsyncEntropyCoder` trait
   (so "we'd have to build it twice" isn't a cost in the drop-`Range`
   decision), and sequences `Ans` first, because `Ans`'s chunk boundaries are
   *in the bytes* — byte-identity with sync encode is then equivalent to
   "the async traversal reached exactly the sync traversal's split points,"
   which is the traversal's characteristic bug. `Range`'s chunk-boundary
   invariance would hide that same bug completely.

8. **Properly A/B the register-residency win** of `decode_bits::<N>` vs the
   per-bit path — the float per-bit baseline was never cleanly measured on
   its own.

9. **Const-generic incompressible read** for compile-time-known sizes (IP
    octets, single bytes): `decode_incompressible::<const N>() -> [u8; N]`
    avoids the runtime length and inlines the small copy instead of
    `memmove`. (Rejected a slice-returning variant: it pushes a size check
    onto callers.)

10. **Cheaper Lz77 offset/back coding.** Profiling
    (`just-decompress-compressible`, redundant data) showed
    `Small<usize>`-based offset/back decode at **57%** of Lz77 decode time —
    dominant enough that the malloc/copy micro-opts once proposed here (each
    ≤ ~5%, some near-zero on redundant data) aren't worth doing except
    opportunistically. The real lever is the narrower/faster `Small<usize>`
    item below. Two small opportunistic wins still open, low risk: decode a
    chunk's literal bytes straight into `out` instead of round-tripping
    through a temp `Box<[u8]>`, and replace the self-referential match
    copy's per-byte `out.push(out[i])` with `Vec::extend_from_within` (with
    an overlap fallback), as fast deflate decoders do.

11. **Partial-top-byte as one `AtMost` symbol per `lz` bucket** — the ≤7
    sequential adaptive bools in `Small<u64>`'s partial top byte have
    position-fixed contexts (independent given `lz`); one symbol per bucket
    is the same format-level move that won elsewhere, aimed at the
    now-coding-bound integer decode. Caveat: on uniform bits `Ans`'s bitwise
    path can beat the symbol walk (the walk-shootout finding), so A/B on
    both uniform and skewed data. Related: fusing the `blbl` + offset
    symbols into one coder step (the `Range` multi-magnitude lever named
    below).

12. **Runtime-bounded dictionary-index symbols** — `LowCardinality` and
    `DictContext` encode indices both sides know are `< dict.len()` as
    general `Small<usize>` (bucket symbol + offset symbol + fallback); a
    runtime-`max` variant of the `uneven` walk would code them in one symbol
    with no probability mass wasted on impossible indices. Format change;
    size and speed on every cache hit.

13. **Decide the default-coder flip to `Ans`** — decode is uniformly 1.3–1.8×
    faster at equal size, and multisymbol fixed encode; the remaining work is
    a decision plus format-stability bookkeeping, not engineering. See "`Ans`
    against `Range` across the workload set" at the top of this document for
    where it does and doesn't hold.

14. **Micro-nits, worth doing opportunistically**: `String::encode` walks the
    string twice (`chars().count()`, then the encode pass); `[T; N]::decode`
    round-trips through a heap `Vec`; the remaining per-char construction
    cost on string decode (`char::from_u32` + `push(char)`) — residue of a
    measured-dead-end ASCII fast-path attempt (the construction side it
    targeted is only ~0.5% of decode; the coding side is the real cost),
    which grows in relative importance now that `BTreeSet` bulk-build has
    removed most of the construction overhead around it.

15. **A narrower / faster `Small<usize>`, and narrower Lz77 offsets** — two
    related follow-ups to the Lz77 offset switch to plain `usize` (Landed):
    - *Faster `Small<usize>`.* Its bucket-prefix scheme (`AtMost<7>` bucket
      then per-bucket offset) spends an extra symbol on every value ≥ 64
      versus plain `usize`'s direct path — worth a redesign that keeps the
      small-value tightness without the wasted bucket symbol on large
      values.
    - *Narrower backing int* — **measured dead end.** `u32` offsets
      (including a tiny-seeded `U32Compact`) reclaimed nothing against
      `usize`: a ≤64 KiB offset uses the same bit-length depth at either
      width. The only way to get both the size win and the speed win is the
      `Small<usize>` redesign above, not a type swap.

## New strategy ideas (compression rate, often also decode speed)

These are *new strategy types* (new `Encode<S>` impls), not coder-level speed tweaks, so they
live a little outside this doc's primary "make decode faster" scope. They are here
because several also *help* decode: a strategy that turns a full value into a
1-bit-plus-tiny-index hit replaces a whole tree-walk (#2) with a couple of bit
decodes, so a good hit rate is both smaller and faster.

- **`Correlated<const N>`** — a bounded-recency / move-to-front model for fields
  that have local repetition but *not* low overall cardinality. Keep the `N` most
  recently seen values in a small ring buffer; on encode, emit one `is_recent` bit
  and, on a hit, the index into the window (a `Bits<N>` tree, cheapest for the
  most-recent slot if we move-to-front); on a miss, encode the value normally and
  push it into the window. Contrast with `LowCardinality`, which keeps an
  *unbounded* dictionary of every distinct value forever — great for a handful of
  repeated strings, but its index grows and its `HashMap`/cache balloons when
  cardinality is high. `Correlated` instead bets on temporal locality (the next
  value often equals a recent one), like an LZ77 back-reference window but over
  whole values rather than byte runs. Good fit for time-series-ish columns, paths
  with shared recent prefixes, repeated foreign keys, etc.
  - **`const N` vs runtime N — recommend `const N`.** The derive attribute already
    takes generic strategies (`Mapping<K,V>`, `Bits<N>`), and contexts here are
    fixed-size arrays built via `Default` (e.g. `BitsContext<N>`), so
    `#[compactly(Correlated<8>)]` drops straight into the existing machinery with
    the window as `[T; N]` on the stack and a `Bits<N>` index that the #2 tree-decode
    work speeds up. Runtime N would need a heap window and a way to thread a
    parameter through `Context::default()`, which the strategy framework does not
    currently support. "N from the type" doesn't have a natural meaning here. So:
    `Correlated<const N: usize>`, perhaps with a `Correlated = Correlated<8>` alias
    for the common case. Pick a default N by measuring hit-rate vs index-cost on the
    `comparison` records.
  - Open question worth a quick experiment first: on which `comparison` columns does
    a small recency window actually beat `LowCardinality` / `Normal` on size? If the
    repeated values are also globally few, `LowCardinality` already wins; `Correlated`
    only pays off when cardinality is high *but* locality is real.

## Landed so far
- **Lz77 offsets: default `usize` instead of `Small<usize>` (2026-07-21)** —
  `Lz77`'s `count`, `offset`, and `self_offset` switched from
  `Small<usize>` to plain `usize`. A back-reference offset is usually ≥ 64, so
  `Small<usize>` wasted an `AtMost<7>` bucket symbol before recursing into
  `Small<u64>`; the default `usize` (tiny-seeded `U64Compact`) codes it
  directly. Quiesced `just-decompress-compressible` (meteorite CSV, 300×):
  decode **Ans −4.8%** (43.87B→41.77B), **Range −4.2%** (56.84B→54.47B). Size
  is ~neutral (tiny inputs smaller via `count`; big redundant corpus +0.25%).
  Follow-ups in the narrower/faster `Small<usize>` TODO item (faster
  `Small<usize>`; `u32` offsets to reclaim the
  0.25%). Format change (v2). `just-decompress-compressible` bench added.
- **Default float encoding: integer / decimal / raw behind saturating
  selectors (was TODO #6/#7)** — the v2 default `Encode` for `f64`/`f32`
  classifies each value into three tiers behind two selector bits:
  `is_raw = true` → raw bits stored *incompressibly* (a memcpy on decode);
  `is_raw = false, is_int = true` → whole integer via `Small<i64>`;
  `is_raw = false, is_int = false` → short decimal `mantissa·10^power`
  (reusing the merged `to_decimal`/`decimal_value`/`POW10` from the `Decimal`
  strategy). Each selector uses `BitContext::SATURATED_TRUE` (a new
  compile-time-computed associated const, the fixed point of `adapt(true)`):
  once a column has only ever taken one branch up to the adaptation cap, both
  sides skip coding the bit and commit to it — an all-raw column pays no
  per-value selector and decodes as a bare memcpy; an all-integer column pays
  no decimal cost. A value that needs the other branch after saturation falls
  back to raw (still exact); `is_raw` only saturates toward raw, since the
  structured branch has no universal escape. Sizes vs main (100k/category):
  **2-decimal 7.111 → 2.195 B/f (−69%)**, **1.0+u/1e6 6.648 → 3.595 (−46%)**,
  half-integers ~flat, random 8.191 → 8.000; fixed-exp 6.674 → 8.000 (+20%,
  the accepted incompressible-raw-tier cost — the exponent is no longer
  modeled). Random-float decode **657M/1.41B cycles (Ans/Range) vs main's
  79B/167B ≈ 120×** — the saturating memcpy. (Bidirectional `is_int`
  saturation was prototyped and dropped: neutral on realistic mixed-decimal
  columns, not worth the complexity.)
- **`Sorted<Vec<T>>` builds in place (was TODO #15, 2026-07-19)** — the
  generic sorted-`Vec` delta strategy (the per-element strategy inside
  `BTreeSet<Vec<u8>>` and friends, and any `#[compactly(Sorted)]` `Vec`
  field) now decodes into `ctx.previous`
  (truncate to the shared prefix + push the suffix + return one exact-size
  clone) instead of copying the prefix out and cloning the whole result
  back, and encode keeps the buffer via `clone_from`. v2 + v1, decode-side
  construction only — bitstream unchanged, zero snapshot churn. Same shape
  as the `Sorted<String>` fix below (−6…−8% there); no dedicated benchmark
  exercises this path, so it ships on tests + the string precedent rather
  than a fresh A/B. Corrupt-stream nit: a `shared_prefix` beyond the
  previous length now decodes leniently (truncate no-op) where the old
  slice would panic.
- **Bulk-build `BTreeSet`/`BTreeMap` on decode (was TODO #13, 2026-07-19)** —
  decode now stages elements in a `Vec` and `collect`s, letting std's
  `FromIterator` bulk-build packed nodes from the sorted stream instead of
  paying a per-element `insert` descent (with its tree-walk `memcmp`s and
  node splits). Sites: `Values<S>` for `BTreeSet`, `CompactU64Set`, and
  `Mapping` for `BTreeMap`, in both v2 and v1. Decode-side value
  construction only — bitstream unchanged, zero snapshot churn; identical to
  the old insert loop for every valid stream and for any key/element type
  whose `Ord` agrees with its `Eq`. The two diverge only on a corrupt stream
  carrying an Ord-equal run of a coarser-`Ord` type, where `collect` keeps
  every Eq-distinct entry instead of deduping by `Ord` — not UB, which is
  all decode promises for corrupt input (see the code comments and
  `btreeset_bulk_build_keeps_ord_equal_dupes`). Quiesced A/B
  (`just-decompress-strings`, 500×, min of 3 alternated pinned rounds,
  within-side spread ≤ 0.1%): decode **Ans 19.70 → 8.43 Gcycles = −57.2%,
  Range 20.80 → 10.54 = −49.3%** — the >50% construction share the
  2026-07-19 survey profile identified, almost entirely gone. HashSet/
  HashMap keep their insert loops (no sorted-input bulk build to exploit).
- **`Sorted` string decode builds in place (was TODO #12, 2026-07-19)** — the
  decode paid two copies per string: re-encoding the shared prefix
  char-by-char into a fresh `String`, then `clone_from`-ing the result back
  into `ctx.previous` for the next delta. Now the string is built *in place*
  in `ctx.previous`: truncate to the shared prefix's byte offset (a char
  boundary, found by walking `char_indices` — no re-encode), push the decoded
  suffix chars, return one exact-size clone. Also drops the per-call `String`
  allocation (`previous`'s buffer is reused across the collection). Decode-side
  value construction only — bitstream unchanged, zero snapshot churn.
  Quiesced A/B (`just-decompress-strings`, min of 3 alternated pinned rounds,
  spread ≤ 0.35%): decode **Ans 42.17 → 38.93 Gcycles = −7.7%, Range 46.37 →
  43.57 = −6.1%** — large given >50% of the workload is `BTreeSet`
  construction the change can't touch.
- **`MAX = 1` codes as a plain bit, not a symbol (2026-07-19)** — acting on the
  walk shootout's distribution-robust finding above: `Walk::production` now
  resolves `MAX = 1` to `CompleteBitwise`, whose "walk" is a single ordinary
  `encode_bit`/`decode_bit` step — the symbol path's interval build and 16-bit
  renormalization were pure overhead for a two-valued symbol (measured 5–36%
  slower across both coders, both metrics, and both distributions). This is a
  *format change* for `AtMost<1>` users (2-variant derive enums, the 2-value
  offset buckets in the integer hierarchy, `usizes.rs`/`byte.rs` `b1` fields):
  every churned size snapshot moved 1–5 *millibits smaller* (the bit step has
  no reserve squeeze), and the `Ans` doctest's 5-value example grew 5 → 6
  bytes (renormalization granularity on a tiny payload). **Fresh quiesced
  shootout on the change** (in-process alternated rounds): old production →
  `CompleteBitwise` at `MAX = 1` is decode **Ans −5/−6%, Range −22/−23%**,
  encode **Ans −12/−29%, Range −14/−32%** (Skewed/Uniform), and no challenger
  beat the new decode pick on either distribution. The only counter-signal is
  Range *encode* via the `Uneven` symbol walk: 24% faster on Uniform but 37%
  slower on Skewed (realistic data) — production stays bitwise. A macro A/B on
  `benches/integers.rs` (3 alternated cross-binary rounds, min) showed u32
  skewed-small Ans decode −5.4…−6.0% every round, but u64/usize skewed-small
  moved +2…+7% while untouched control rows (u16 legacy, zstd/bincode/bitcode
  references) scattered just as much — cross-binary layout noise; no reliable
  macro signal either way, as expected for a code that is a small slice of
  integer coding time. Same run's fresh follow-up lead: `MAX = 2` decode now
  prefers the bitwise walk on **both** distributions (Ans 12–15%, Range
  12–13%) — the old "tiny extreme" uniform-only finding reproduces on Skewed
  too, making a `MAX = 2` bitwise route the next candidate change (validate on
  `just-{de,}compress-enums`, the 3-variant enum workload). **UPDATE
  2026-07-20: validated and REJECTED** — the enum-workload A/B reversed the
  lead (Ans decode +20%, Ans/Range encode +21…38%) — see the dead-ends list
  above.
- **`Compressible` (Lz77) decode: skip `old_filter` upkeep (was TODO #11)** — the
  8 KiB 4-gram bitset maintained by `push_old` is read *only* by the encode-side
  match scan (`eager`/`eager_chunk`); decode never calls `eager`, so the per-byte
  `old_filter.set` loop was pure waste on decode. Split into `push_old` (encode:
  filter loop + deque) and `push_old_decode` (deque only); `Lz77::decode`
  (`bytes.rs:316`) now calls the latter. Encode is unchanged, decode produces
  identical bytes (all size/round-trip tests unchanged). The remaining
  `out.clone()` per-string copy (Rc-sharing idea) is left as a separate item.
- **`Sorted<u8>`/`<i8>`: always encode the wrapping delta (was TODO #13)** —
  dropped the `fits_in_i8` bool and the whole mid-tree `ByteContext` fallback
  (`skip_bits` + manual state reconstruction). `value.wrapping_sub(previous) as i8`
  always round-trips (`previous.wrapping_add(delta as u8)` inverts it) and wrapping
  always takes the short way around the byte circle, so `|delta| <= 128` for every
  pair — the "doesn't fit" case was dead code. Encode/decode are now a single
  branchless `Small<i8>` + `wrapping_add`. As a follow-up the `full_value`
  `ByteContext` (256-entry adaptive table, only ever used for the first element)
  was dropped too: the first byte now stores raw via `Incompressible`, which has no
  context — smaller `SortedU8Context` (just `previous` + `delta`) and no per-context
  allocation. Net size on `sorted_u8_ascii`: 31 → 29 bits (−1 `fits` bit per
  non-first element; +1 bit because the lone first byte no longer benefits from the
  adaptive tree across repeated encodes). Guarded by the exhaustive
  `sorted_u8_roundtrip` (all 256×256 pairs + every i8), still green. `i8` delegates
  to `u8` so it came along free.
- **`LowCardinality<Arc<str>>` over `LowCardinality<String>` (was TODO #11)** — not
  a coder change; a user-facing steer. `LowCardinality` reconstructs each
  *repeated* value from its dictionary, which for `String` is a fresh allocation
  per cache hit (most rows in low-cardinality data); `Arc<str>` makes a hit a
  refcount bump and shares one backing buffer. A/B on the meteorite `recclass`
  column (38k values, perf cycles, min of 2 pinned runs, identical 20625-byte
  output):

  | coder | `String` | `Arc<str>` | delta      |
  |-------|----------|-----------|-------------|
  | Range | 254.7B   | 211.9B    | **−16.8%**  |
  | Ans   | 198.8B   | 152.4B    | **−23.3%**  |

  Clear, consistent win (wall-clock A/B was too noisy to trust on `Range` — one run
  even showed −4% — so this was settled with `perf` cycle counts). Done: (1) added
  v1 `Arc<str>` `Encode` + `LowCardinality` impl (v2 already had both); (2)
  converted every `LowCardinality` `String` field in `comparison` to `Arc<str>`
  (needs serde's `rc` feature); (3) the `EncodeV2` derive now emits a
  `#[deprecated]`-style compiler warning (via the `proc-macro-warning` crate)
  pointing at any `LowCardinality` `String`-bearing field and suggesting `Arc<str>`;
  (4) documented the antipattern on the `LowCardinality` strategy in `src/lib.rs`.
  NB: the warning fires from the **v2** derive only (a type usually derives both v1
  and v2; warning from both would double it).
- `make EntropyDecoder bit-decode infallible` — `decode_bit*` return `bool`, not
  `Result`; ~0.7% fewer cycles, simpler hot path.
- `add batched const-generic bit encode/decode to the entropy traits` —
  `decode_bits_nonadaptive::<N>` / `decode_bits::<N>` and
  `encode_bits::<N>([(bool,Probability);N])` primitives; the `Ans` decoder
  inlines its math into the batched primitive (~2.4% faster decode).
- (this session, see git log) `decode_bit` routes directly through the single-bit
  primitive (the batch machinery regressed N=1); `decode_bits::<N>` is the
  register-resident split form for N>1; floats decode via `decode_bits::<$bits>`.
- **Fused adaptive `decode_bits` override (`Ans` + `Range`)** — the trait's
  *default* `decode_bits` was never optimized: it builds a `[Probability; N]`
  array, decodes, then walks the contexts a *second* time to `adapt`. Both coders
  now override `decode_bits` to do probability-lookup → decode → adapt in **one
  pass**, keeping coder state (`state`/`bytes`, plus `value` for `Range`) in
  locals and touching each context once (shared `decode_step` helper per coder, so
  no duplicated coder math). Correct because the batched contexts are independent.
  - **`Ans` float `Vec<f64>` decode: 106.5B → 78.0B cycles = −27%**
    (`just-decompress-floats ans`, phased-vs-final, same binary). On the
    `micro-batch` pure-bits A/B the batch went 38.12B → 27.3B.
  - **`Range` is the *default* coder** and its float decode hits the same path;
    the fused override is **neutral there (~0.2%, 187.2B → 186.8B)**, because
    `Range`'s per-bit decode is ~2.4× heavier than `Ans` (187B vs 84B for the same
    floats) so the batch-machinery overhead is a much smaller fraction. Kept for
    symmetry; it doesn't hurt. (NB: float decode bins are code-layout-sensitive;
    trust same-binary deltas, and `micro-batch` for batch work.)
- **`decode_bits(&mut [BitContext; N])` instead of `[&mut BitContext; N]`** — the
  remaining gap (fused batch still ~6% behind per-bit at N=16) was the caller's
  `each_mut()` building an array of `N` pointers on the stack. Passing the context
  array by `&mut` lets the coder index it in place. This **closed and reversed**
  the gap: on `micro-batch`, batch went 29.6B → **27.3B**, now ~7% *faster* than
  the per-bit path (29.6B in the same binary); `Ans` floats 83.7B → **78.0B**.
  Downside the caller pays: the `N` contexts must live in one array — callers that
  don't have them contiguous can't use it (so far only floats/`micro-batch` do).
- **`EntropyDecoder` collapsed to two required methods** — first dropped the
  const-generic `decode_bits_nonadaptive::<N>` (only live use was `N == 1`), then
  dropped `decode_bit_nonadaptive` too. The trait is now: required
  `decode_bits<N>(&mut [BitContext; N])` + required `decode_incompressible_bytes`,
  with `decode_bit` the only default (`decode_bits(array::from_mut(ctx))` — a
  free `&mut T → &mut [T; 1]` reinterpret). So `decode_bits` is *the* bit-decode
  primitive; coders optimize one method and `decode_bit` falls out of it.
  - Verified the `N == 1` hot path did **not** regress (the old "N=1 via the batch
    is slower" finding was specific to the pre-fusion machinery): `just-decompress`
    u64 105.3B vs HEAD 106.7B — slightly *faster*.
  - `Raw` now implements `decode_bits` + `decode_incompressible_bytes` (it used the
    removed primitive via the old defaults). Coder-internal tests that needed an
    arbitrary-probability decode call `decode_step` directly.
  - `encode_bits::<N>` stays — on `Ans` it's a real win (one `Vec::extend` of N vs
    N pushes).
