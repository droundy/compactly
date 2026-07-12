# Optimizing decode (and encode) speed

Working notes on the effort to make decoding faster (primary goal) without
harming the compression rate. Read this together with the git log — several
commits below are the durable result of experiments recorded here.

Our focus for optimization is the `v2` encoder in `src/v2/`  This has two
entropy coders `Range` and `Ans`.  `Range` is currently the default, but `Ans`
is faster at decoding and may become the default in the future.  We want to
optimize both approaches with a slight focus on `Ans`.

## How to benchmark on this machine

The benchmark harness in `benches/` is convenient but the laptop is noisy
(browsers, Netflix, etc.). For reliable A/B work:

- **Quiesce the machine first — a human task, not Claude's.** The human runs
  `sudo ./bench-quiet.sh 2` to reserve CPU 2 (a P-core): turbo off (kills
  thermal drift), performance governor, SMT sibling cpu3 offlined, all other
  processes and IRQs herded onto the remaining CPUs, ASLR off, unprivileged
  `perf` enabled. `sudo ./bench-quiet.sh restore` (or a reboot) undoes it.
  Claude: never run this script or anything else under sudo.
  - **Before benchmarking, check that the setup is active:** `bench true`
    exits 0 iff the machine is quiesced. (The setup installs a `bench`
    wrapper in `/usr/local/bin` that reads the reserved CPU list from
    `/run/bench-quiet.cpus` and refuses to run without it; `/run` is tmpfs,
    cleared by both `restore` and reboot, so the check can't be stale.) If
    it fails, stop and ask the user to run `sudo ./bench-quiet.sh 2` —
    measurements on an unquiesced machine are not worth taking.
  - **Run every benchmark through the wrapper:** `bench <cmd…>` expands to
    `taskset -c <reserved cpus> <cmd…>` — e.g.
    `bench perf stat -e cpu_core/cycles/ <bin>` for cycle counts, or
    `bench cargo bench --bench bench` for criterion. Anything not run
    through it lands on the crowded housekeeping CPUs and gains nothing
    from the setup. Build first (`cargo build --release` or
    `cargo bench --no-run`) *outside* the wrapper so compilation isn't
    pinned to a single core.
- **Check load first:** `top -b -n1 | grep %Cpu` — want >90% idle.
- **Prefer cycle counts over wall time:** `perf` counts cycles per-process, so
  it is far less noisy than wall-clock under contention:
  `taskset -c 2 perf stat -e cpu_core/cycles/ <bin>` and take the **min** of a
  few runs.
- Focused decode/encode workloads live in `src/bin/`:
  - `just-decompress` — decode `Vec<u64>` (random) 5000×.
  - `just-decompress-floats` — decode `Vec<f64>` 1000× (prints compressed size).
  - `just-compress` — encode `Vec<u64>` (heavy, ~1.3T cycles/run; slow to A/B).
  - `just-decompress-net` — decode `Vec<Ipv6Addr>` (ANS coder) from `ipv6.txt`
    2000× (~138B cycles/run); needs `ipv6.txt` in the cwd.
  - `just-decompress-strings [ans|range] [iters]` — decode a
    `BTreeSet<String>` of 38k meteorite names (default 2000×, ~83B cycles on
    `Ans`); THE per-character `char`/`u8` tree-walk workload. Reads
    `comparison/src/meteorites.csv`, so run from the workspace root.
  - `just-compress-strings [ans|range] [iters]` — the encode-side twin of
    `just-decompress-strings` (default 2000×; ~40B cycles per 1000 on `Ans`).
  - `micro-batch seq|batch` — isolates the ANS adaptive bit-decode: decode a
    stream of independent adaptive bits via `decode_bit` (`seq`) vs `decode_bits`
    (`batch`), nothing else in the loop. Best signal for batch-coder work.
  - `just-decompress-enums [ans|range] [seventeen] [iters]` /
    `just-compress-enums …` — decode/encode a `Vec` of 100k skewed 3-variant
    (or uniform 17-variant) enums (default 2000×); isolates the
    `AtMost` discriminant path through the derive.
  - `just-decompress-uless <N> [ans|range] [iters]` — decode 50k uniform
    `AtMost<N-1>` values for value counts N in a monomorphized ladder
    (3…128); the depth-sweep tool that located the per-coder prefetch
    crossover.
- Instruction count is NOT a good proxy here: decode is **latency-bound**
  (measured IPC ≈ 1.39), so fewer instructions can still be slower and vice
  versa. Trust cycles.
- **Thermal throttling makes sequential A/B runs lie** (observed 2026-07-04:
  the fan spins up during a long benchmark and later runs land on a slower
  clock). A back-to-back "all of A, then all of B" wall-clock comparison once
  showed a uniform fake −15% — including on datasets the change couldn't
  touch — while the zstd/bincode reference rows (identical code both sides)
  moved 12–33% *in the same direction*. Always **alternate A and B runs**
  and check the reference rows before believing any wall-clock delta; cycle
  counts are less sensitive but still benefit from alternation.
- Expect ~±1% of **binary-layout noise** on workloads dominated by
  library/runtime code: e.g. `just-decompress-strings` spends >50% in
  `BTreeMap::insert`+`memcmp`, and those identical functions were measured
  4–6% apart between two builds differing only in compactly code.

## Empirical results so far

### Profiling `just-decompress` (random u64)
- IPC ≈ 1.39 (latency-bound), branch-miss ≈ 15%, L1-dcache miss ≈ 0.16%.
- By cycles, the hot spots are `memmove`/`rep stos` and `malloc`/`free` — i.e.
  the output `Vec` alloc/zero and `decode_incompressible_bytes` copying each
  value's "full bytes". The rANS arithmetic (`imul`) is only ~2% of cycles.
- The decoder state round-trips through memory every bit even in the baseline:
  `Decoder` is threaded by `&mut` through deeply nested generic `decode` calls,
  so its fields never get promoted to registers across a loop.

### Batching Ipv6 zero-flags via `decode_bits::<14>` — DEAD END (reverted)
Replaced the 14 sequential `bool::decode` zero-flag decodes in `Ipv6Addr` with a
single `reader.decode_bits(ctx.zero.each_mut())`. Correct, all tests pass.
A/B'd on `just-decompress-net` (min cycles of 4 pinned runs, tightly clustered):

| coder          | per-bit baseline | batched `decode_bits::<14>` | delta            |
|----------------|------------------|-----------------------------|------------------|
| `Range` (dflt) | 194.55B          | 207.48B                     | **+6.6% slower** |
| `Ans`          | 138.26B          | 138.78B                     | +0.4% (wash)     |

Batching **does not help even with `Ans`** — the coder whose batch primitive the
TODO's "register-resident" premise was built around. It's a clear regression on
`Range` and a wash-to-slightly-worse on `Ans`.

Cause: both coders are sequential — decoding bit *i+1* updates state from bit *i*,
so there is **no ILP to exploit** at this call site. The adaptive `decode_bits`
default also makes 4 passes over the batch materializing three 14-element stack
arrays (`each_mut` → `each_ref().map(probability)` → `decode_bits_nonadaptive` map
→ `zip`+`adapt`), pure overhead versus the fused per-bit loop.

Takeaway: the float `decode_bits` win does **not** generalize to small batches on
either coder. At N=14 the array machinery dominates any register-residency gain.
Don't convert more small callers; if revisiting, only large-N independent groups
are plausibly worth measuring. (Aside: `Ans` decode is ~30% fewer cycles than
`Range` on this IPv6 workload — 138B vs 195B.)

**Re-measured after the fused `Ans` `decode_bits` override landed** (which cut the
batch overhead a lot — see "Landed"): `Ans` IPv6 batched is now **139.83B vs
138.26B per-bit = still +1.1% slower**. So the fused override narrows but does not
close the small-N gap, and `Ipv6Addr` stays per-bit. The override's win shows up
only on **wide** batches (the 52–64 float bits), not 14-bit groups.

### Two-stream interleaved rANS — DEAD END (reverted)
Implemented the fgiesen "rANS in practice" two-interleaved-states trick
(correct, all tests passed). Decode got **48–107% SLOWER** across
integers/signed/bytes. Cause: with the one-bit-at-a-time `&mut self` API the
state lives in memory; the swap doubles per-bit memory traffic and serializes
via store→load forwarding, so the hoped-for ILP never materializes. Do not retry
without a 2-wide API that keeps both states in registers.

**UPDATE — that prerequisite now exists.** `decode_bits(&mut [BitContext; N])`
(see "Landed") decodes `N` *independent* bits in one call with coder state held
in locals. That is exactly the register-resident, multi-bit surface the retry
needs: within one `decode_bits` call you could run 2+ interleaved rANS states
(even bits ← state A, odd bits ← state B) so the independent update chains run in
parallel for real ILP. This is the actual point of the `decode_bits` line of work
— the float numbers are just a clean, isolated testbed (`f64` is *not* a hot path;
`usize`/signed ints/strings matter far more). NEXT: prototype 2-state interleaved
`Ans` encode+decode inside `micro-batch` to measure the ILP ceiling on independent
bits before touching real types. Caveat for real types: the tree codes
(`u8`/`UBits`/`Bits<N>`) decode *dependent* bits (each context chosen from prior
bits), so they can't feed `decode_bits` as-is — capturing the integer/string win
needs independent-bit decoding there too.

### Tree-symbol decode: multisymbol coding AND register-residency — both DEAD ENDS (measured)

> **UPDATE 2026-07-03: the multisymbol verdict is overturned** — with the
> fused-context speculative walk (next section) multisymbol decode now *beats*
> the per-bit baseline on the string workload. The numbers below remain valid
> as history for the *unoptimized* walk.

Two related plans for the `u8`/`Bits<N>`/`UBits<N>` dependent tree walk (the
per-character string hot path) were fully implemented and A/B'd. Both lose or
wash; neither should be retried at ≤8-bit tree depth without new evidence.
Benchmarked with `just-decompress-strings` (decode a `BTreeSet<String>` of 38k
meteorite names 2000×), `just-decompress-net`, and `just-decompress`; min of 4
pinned runs (`taskset -c 2 perf stat -e cpu_core/cycles/`), >94% idle, on AC.

| decode workload        | per-bit baseline | multisymbol (1 coder step/symbol) | plan #2 (fused per-bit `decode_tree`) |
|------------------------|------------------|-----------------------------------|---------------------------------------|
| meteorite names, `Ans`  | 83.07B          | 90.44B (**+8.9%**)                | 83.23B (+0.2%, wash)                  |
| meteorite names, `Range`| 96.71B          | 102.17B (**+5.6%**)               | 97.81B (+1.1%)                        |
| IPv6, `Ans`             | 137.33B         | 153.22B (**+11.6%**)              | 140.97B (+2.7%)                       |
| random u64, `Ans`       | 105.26B         | 105.37B (wash; barely uses trees) | —                                     |

**Multisymbol (whole-tree) coding** (`plans/multisymbol-tree-coding.md`; full
implementation in the follow-up PR to the one landing this note): walk the tree
once to build a single 16-bit cumulative interval (`SymbolRange`) and pay ONE
coder step (one renormalization) per symbol instead of `log2(N)`. It works —
lossless by construction via a per-level reserve, rANS and range-coder symbol
steps share the existing renorm invariants, size is ~neutral (+0.01–0.03%;
meteorite names 42588 → 42602 bytes) — but decode is consistently SLOWER.
Counters show why: instructions +2.6%, branch misses −6%, yet IPC drops ~7%.
Decode is latency-bound, and the CDF construction (a `width×prob>>8` multiply
per level) sits ON the serial bit-decision dependency chain, while the
renormalizations it removes were cheap, well-predicted branches OFF the critical
path. Replacing the reserve clamp with a branch-free squeeze
(`split = ((width − 2·reserve)·prob >> 8) + reserve`) clawed back ~2.5%; the
rest is inherent. (Also note: the rANS *encode* buffer grows from 2 to 6
bytes/op, and `Range` needs a Subbotin-style carry-less clamp renormalization to
guarantee `width ≥ 2^32` before a symbol step — validated correct, adds rare
≤1-bit clamp waste.)

**Plan #2, register-resident per-bit tree decode**
(`plans/decode-tree-register-resident.md`, was TODO #2): same walk, same
format (bit-identical), but coder state held in locals across the `log2(N)`
dependent steps via fused `decode_tree` overrides. Measured a wash on the very
workload it targeted (strings `Ans` +0.2%) and slightly negative elsewhere
(+1–3%). The per-bit `decode_bit` path through the fused `decode_bits::<1>`
override was already effectively optimal — the `Decoder` fields stay hot in L1
and store-forwarding hides the round-trip, exactly as the `decode_bits::<14>`
IPv6 dead end found. The unrolled-per-call-site walks may also cost icache.

**Kept from this work:** the `encode_tree`/`decode_tree` trait methods (per-bit
defaults, bit-identical) — one shared, documented walk instead of three
hand-rolled copies in `byte.rs`/`bits.rs`, and the seam a future coder-level
experiment needs; the `just-decompress-strings` string-decode benchmark; and
this note. Deeper fusion (>8-bit trees, e.g. fusing `is_ascii`+`Bits<128>` into
one 8-bit symbol) would amortize the symbol-step cost over more bits and
remains unmeasured — bump `SymbolRange::BITS` in the follow-up PR if trying.
(UPDATE 2026-07-04: tried and it WINS — no `BITS` bump needed; see
"Escaped-tree fusion" below.)

**Bonus finding — the `Range` coder codes near-certain bits BELOW entropy in
narrow intervals.** When the interval width drops under 256 (a straddled
top-byte boundary), `split()`'s `(width >> 8) * prob` is 0, so `split == lo`:
a `true` bit then costs ~0 bits regardless of its modeled probability (and a
`false` bit collapses the interval into an 8-byte flush). E.g. 64 fresh-context
copies of `u8::MAX` (true entropy 64 bytes) encode to 23 bytes. Multisymbol,
which codes honestly at `width ≥ 2^32`, "regressed" several all-ones size
assertions purely by losing this accident. Worth knowing when reading
`assert_bits!` numbers for repeated extreme values.

### Fused-context speculative tree walk — multisymbol now BEATS per-bit (2026-07-03)

Profiling the multisymbol decode of an *unsorted* `Vec<String>` of the 38k
meteorite names (`src/bin/ans-decode-phases.rs`, built via `HashSet` so there
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
re-measured off the beaten path — the old dispatch only ever called the walk
it currently picks. Replaced the two-value `WalkStyle` enum and the scattered
`is_power_of_two`/`SPECULATE_MIN_MAX` branches (`encode_walk`, `decode_walk`,
`decode_walk_speculating`) with one `Walk` enum (`Complete`,
`CompleteSpeculating`, `Uneven`, `UnevenSpeculating`, `CompleteBitwise`,
`UnevenBitwise`) and a single `Walk::production::<MAX>(speculate)` resolver;
production and the new shootout bench both go through the same
`encode_atmost_walk`/`decode_atmost_walk` dispatch, called with a
compile-time-constant `Walk` so it still folds to one branch per
monomorphization (verified: all `walks.rs` bit-identity tests pass, full
suite green, `cargo bench --bench bench` unmoved). Added a plain
(non-speculating) `complete::from_slot` — previously `complete`'s decode was
*always* speculative, so there was no baseline to compare it against; adding
it surfaced a real latent bug (unconditional `contexts[0]` load panicking at
`MAX == 0`, masked in production by the `MAX == 0` short-circuit upstream),
now fixed with the same early-return `uneven::from_slot_speculating` already
had.

`benches/atmost.rs` (`cargo bench --bench atmost`) times every
(coder × `MAX` × applicable `Walk`) for decode, and once per *distinct*
encode implementation (`Walk::encode_with` maps a speculating walk to its
plain twin, since they share one encode body — timing both would just be two
noisy samples of the same code), via new `#[doc(hidden)]`
`Range`/`Ans::{encode,decode}_atmost_batch::<MAX, WHICH_WALK>` methods
(`WHICH_WALK` is a `const` generic indexing the `WALKS` array, so each forced
walk is still branch-free — no runtime `Walk` dispatch anywhere, benchmark
included), and marks the walk `Walk::production` currently picks. A walk
that beats production's choice by ≥5% on the initial sweep is only
*nominated*; it's re-timed against production 3 more times, alternating
measurement order each round (cancels monotonic drift/thermal bias), and
only reported as a confirmed finding if it wins every round with a ≥5%
median margin — replacing an earlier version of this tool that reported any
single-sample ≥10% gap directly, which couldn't tell a real effect from
run-to-run noise.

One full run's confirmed findings (single process, 3 in-process alternated
rounds each — not yet cross-checked with `bench-quiet.sh` across separate
invocations): `Range`'s `UnevenSpeculating` decode reproducibly *slower*
than plain `Uneven` at `MAX` = 64, 128, 256, and 512 (18–23% slower) — the
specific case the original version of this tool flagged at 64/128 on a
single sample, now confirmed and widened. More surprising: at several `MAX`
(3000, 4095, and the small power-of-two counts 7/15/31/63/127/255) `Ans`
decode via the historical per-bit `*Bitwise` walk reproducibly *beat* the
whole-symbol walk it's meant to replace by 20–36% — e.g. `MAX=4095`:
`CompleteSpeculating` 186.8ns vs `CompleteBitwise` 123.5ns. That's enough
walks and enough margin to not be a fluke of this run, but it contradicts
the whole-symbol design's premise (one entropy-coder renormalization per
symbol vs. one per bit), so it needs cross-process/quiesced confirmation and
some investigation into *why* before anyone considers changing
`Walk::production` on the strength of it.

### Float bits: adaptive bits vs incompressible bytes (BIG finding)
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

## TODO (in rough priority order)

1. ~~**Convert more independent-fixed-width callers to `decode_bits::<N>`**~~ —
   TRIED on `Ipv6Addr` zero-flags (14 independent bits). A/B'd on **both** coders:
   **+6.6% slower on `Range`**, **+0.4% (wash) on `Ans`** (see dead-end note
   above). The register-residency premise does not pay off at small N on either
   coder, so do not convert more small callers. NOTE for the record: the tree
   codes (`u8`, `UBits`, `Bits<N>`) select each bit's context from
   previously-decoded bits, so their bits are NOT independent and cannot batch
   anyway.

2. ~~**Register-resident tree-node decode for `Bits<N>` / `char`**~~ — TRIED,
   measured a **wash** (strings `Ans` +0.2%, `Range` +1.1%, IPv6 +2.7%), and the
   more aggressive whole-tree *multisymbol* variant is **+6–12% slower** — see
   the "Tree-symbol decode … both DEAD ENDS" note above. The `decode_tree` API
   (per-bit default) landed; the coder-level overrides did not. The remaining
   hope for tree-decode speed is NOT coder plumbing but decoding fewer
   bits/symbols (e.g. #3's ASCII fast-path, or the recent-values cache idea
   below).

3. **ASCII fast-path for `String` decode** — text is almost all ASCII, yet each
   char still pays the `is_ascii` bit, a `char::from_u32` validity check, and an
   `out.push(char)` UTF-8 re-encode. Consider a per-string "all ASCII" flag (format
   change, 1 bit/string) so an all-ASCII string decodes as a run of 7-bit
   `Bits<128>` straight into the byte buffer, skipping the per-char branch and
   `from_u32`. Measure the size cost vs the decode win; pairs naturally with #2.
   (UPDATE 2026-07-04: the escaped-tree fusion above removed the `is_ascii`
   *coder step*; the remaining upside here is the value-construction side —
   bytes straight into the buffer instead of `char` round-trips.)

4. **Const-generic incompressible read** for compile-time-known sizes
   (IP octets, single bytes): `decode_incompressible::<const N>() -> [u8; N]`
   avoids the runtime length and inlines the small copy instead of `memmove`.
   (We rejected a slice-returning variant because it pushes a size check onto
   callers.)

5. **`decode_until_true` entropy-decoder method** — a method for the
   leading-zero search: decode bits with successive contexts until one comes up
   `true`, returning the index, e.g.
   `fn decode_until_true(&mut self, contexts: &mut [BitContext]) -> usize`.
   This is the *dominant per-value loop* in integer decode and it is
   data-dependent (you don't know the count up front), so the fixed-`N`
   `decode_bits` batch can't cover it; a dedicated method lets the `Ans` decoder
   keep coder state register-resident across the loop. Likely the biggest
   integer-decode lever still available in the coder itself.

6. **Explore float entropy** — Try out different categories of floating point
   numbers and identify where the entropy is within the float.  e.g. for
   integers, decimal numbers like 0.1 power of two fractions like 0.0125,
   irrational numbers, etc.  We'd like to know if some of the bytes/bits are
   usually random and whether there is a way to compress the compressible and
   make the incompressible fast.

7. **Hybrid float encoding** — the likely best-of-both: adaptive-code the
   structured high bits (sign + exponent) and store the ~random low mantissa as
   incompressible bytes. Byte-aligned proposal for `f64`: adaptive top 16 bits
   (sign+exp+top-4-mantissa), incompressible low 48 bits (6 bytes); analogous
   for `f32` (top 16 adaptive, low 16 incompressible). Expectation: ~same
   compression as today in both structured and random cases, but only ~16
   adaptive bits + a memcpy to decode (≈4× faster, no compression harm). Decide
   the exact split, then implement + measure both size and cycles.
   - Alternatively, if the project is willing to accept the structured-data
     compression cost, pure incompressible floats are a trivial ~53× decode win.

8. **Properly A/B the register-residency win** of `decode_bits::<N>` vs the
   per-bit path (float per-bit baseline was never cleanly measured).

9. **Cut per-value allocation/zeroing** in decode — the largest cycle sink per
   profiling (output `Vec` alloc, the `[0u8; 8]` value buffer zeroed per
   integer). This is in `vecs.rs`/`ints.rs`/`mod.rs`, not the coder itself.

10. **Consider Elias Delta encoding** for Small integers — This might be a nice
    alternative for `usize` and maybe even for `u32` and friends.

11. **`Compressible` (Lz77) decode** — decode is the wanted target here (encode is
    deprioritized, see findings). Per `Lz77::decode`:
    - **Literal bytes dominate.** Each chunk's literal is decoded as
      `Values<Normal>` — i.e. every literal byte is a `u8` *tree* decode (8 dependent
      bits), the same per-byte tree walk as #2. For low-redundancy strings, decode
      time is mostly literals, so #2 (register-resident tree decode) is the main
      lever for `Compressible` decode too.
    - **`push_old(out.clone())` runs on decode** (`bytes.rs:316`): it clones the
      whole decoded output into the `old` deque *and* loops setting `old_filter`
      bits — but the filter is only read by encode-side matching, so on decode that
      whole `old_filter` maintenance loop was pure waste. ~~Skip filter updates when
      decoding.~~ **DONE (see Landed):** decode now calls `push_old_decode`, which
      maintains the `old` deque without the filter loop. The `out.clone()` itself is
      still a per-string alloc+copy; consider sharing (`Rc`) the buffer between the
      returned value and the `old` entry.
    - length/back/offset are `Small` decodes — `decode_until_true`-shaped (#5).

12. **`Sorted` strings decode** (`string.rs` `SortedContext::decode`) — three costs:
    the `char` decode loop (helped by #2); `ctx.previous.clone_from(&out)` copies the
    whole string into `previous` every call (needed for the next delta, but it's a
    full copy per string); and `out.extend(ctx.previous.chars().take(shared_prefix))`
    re-encodes the shared prefix char-by-char even though `previous` is already valid
    UTF-8 — since `shared_prefix` is a char count on a char boundary, the prefix
    *bytes* could be copied directly (find the byte offset of the `shared_prefix`-th
    char, then `extend_from_slice`). Measure whether the byte-copy is worth it.

## New strategy ideas (compression rate, often also decode speed)

These are *new `EncodingStrategy` types*, not coder-level speed tweaks, so they
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
