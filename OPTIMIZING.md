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
    `Ans`); THE per-character `char`/`Bits<128>` tree-walk workload. Reads
    `comparison/src/meteorites.csv`, so run from the workspace root.
  - `micro-batch seq|batch` — isolates the ANS adaptive bit-decode: decode a
    stream of independent adaptive bits via `decode_bit` (`seq`) vs `decode_bits`
    (`batch`), nothing else in the loop. Best signal for batch-coder work.
- Instruction count is NOT a good proxy here: decode is **latency-bound**
  (measured IPC ≈ 1.39), so fewer instructions can still be slower and vice
  versa. Trust cycles.

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

13. **Eliminate `Op::Bit` by coding a bit as a one-level symbol** (PR #5 review
    idea) — a plain bit is conceptually `Bits<1>`: a `SymbolRange` that splits
    `M` at the bit's probability (width `≈ p·M` or `≈ (1−p)·M`). If the symbol
    path can code that with no measurable cost vs the bit path, the `Op` enum
    in `ans.rs` collapses to just `Op::Symbol` and the coder hot loops lose a
    branch. Cleanup, not a clear win: the symbol step does more arithmetic per
    op than the bit step (and `Range`'s symbol path requires the
    `clamp_for_symbol` width guarantee the bit path doesn't), so it only lands
    if an A/B shows it's at worst a wash.

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
