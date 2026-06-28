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

## TODO (in rough priority order)

1. ~~**Convert more independent-fixed-width callers to `decode_bits::<N>`**~~ —
   TRIED on `Ipv6Addr` zero-flags (14 independent bits). A/B'd on **both** coders:
   **+6.6% slower on `Range`**, **+0.4% (wash) on `Ans`** (see dead-end note
   above). The register-residency premise does not pay off at small N on either
   coder, so do not convert more small callers. NOTE for the record: the tree
   codes (`u8`, `UBits`, `Bits<N>`) select each bit's context from
   previously-decoded bits, so their bits are NOT independent and cannot batch
   anyway.

2. **Const-generic incompressible read** for compile-time-known sizes
   (IP octets, single bytes): `decode_incompressible::<const N>() -> [u8; N]`
   avoids the runtime length and inlines the small copy instead of `memmove`.
   (We rejected a slice-returning variant because it pushes a size check onto
   callers.)

3. **`decode_until_true` entropy-decoder method** — a method for the
   leading-zero search: decode bits with successive contexts until one comes up
   `true`, returning the index, e.g.
   `fn decode_until_true(&mut self, contexts: &mut [BitContext]) -> usize`.
   This is the *dominant per-value loop* in integer decode and it is
   data-dependent (you don't know the count up front), so the fixed-`N`
   `decode_bits` batch can't cover it; a dedicated method lets the `Ans` decoder
   keep coder state register-resident across the loop. Likely the biggest
   integer-decode lever still available in the coder itself.

4. **Explore float entropy** — Try out different categories of floating point
   numbers and identify where the entropy is within the float.  e.g. for
   integers, decimal numbers like 0.1 power of two fractions like 0.0125,
   irrational numbers, etc.  We'd like to know if some of the bytes/bits are
   usually random and whether there is a way to compress the compressible and
   make the incompressible fast.

5. **Hybrid float encoding** — the likely best-of-both: adaptive-code the
   structured high bits (sign + exponent) and store the ~random low mantissa as
   incompressible bytes. Byte-aligned proposal for `f64`: adaptive top 16 bits
   (sign+exp+top-4-mantissa), incompressible low 48 bits (6 bytes); analogous
   for `f32` (top 16 adaptive, low 16 incompressible). Expectation: ~same
   compression as today in both structured and random cases, but only ~16
   adaptive bits + a memcpy to decode (≈4× faster, no compression harm). Decide
   the exact split, then implement + measure both size and cycles.
   - Alternatively, if the project is willing to accept the structured-data
     compression cost, pure incompressible floats are a trivial ~53× decode win.

6. **Properly A/B the register-residency win** of `decode_bits::<N>` vs the
   per-bit path (float per-bit baseline was never cleanly measured).

7. **Cut per-value allocation/zeroing** in decode — the largest cycle sink per
   profiling (output `Vec` alloc, the `[0u8; 8]` value buffer zeroed per
   integer). This is in `vecs.rs`/`ints.rs`/`mod.rs`, not the coder itself.

## Landed so far
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
