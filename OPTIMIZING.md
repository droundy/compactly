# Optimizing decode (and encode) speed

Working notes on the effort to make decoding faster (primary goal) without
harming the compression rate. Read this together with the git log — several
commits below are the durable result of experiments recorded here.

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

### Two-stream interleaved rANS — DEAD END (reverted)
Implemented the fgiesen "rANS in practice" two-interleaved-states trick
(correct, all tests passed). Decode got **48–107% SLOWER** across
integers/signed/bytes. Cause: with the one-bit-at-a-time `&mut self` API the
state lives in memory; the swap doubles per-bit memory traffic and serializes
via store→load forwarding, so the hoped-for ILP never materializes. Do not retry
without a 2-wide API that keeps both states in registers.

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

1. **Convert more independent-fixed-width callers to `decode_bits::<N>`** so the
   `Ans` register-resident batch (`decode_bits_nonadaptive`) runs across many
   bits per call. Good candidate: `Ipv6Addr` zero-flags (14 independent bits,
   already comment-marked "batched" in `src/v2/net.rs`). NOTE: the tree codes
   (`u8`, `UBits`, `Bits<N>`) select each bit's context from previously-decoded
   bits, so their bits are NOT independent and cannot batch.

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
