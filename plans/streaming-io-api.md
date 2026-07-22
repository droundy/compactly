# Streaming & async IO APIs (encode-to-`Write`, decode-from-`Read`)

Status: design settled on **delay-interleave** (option D). Implementation of
step 1 (the in-memory `Range` format change) starting; async still open.

## Motivation

The current public API is `encode(&T) -> Vec<u8>` / `decode(&[u8]) -> T`. To
encode you hold **both** the original object and its full compressed `Vec` in
memory; to decode you hold the full compressed `&[u8]` and the materialized `T`.
That is fine until the value is a significant fraction of RAM, at which point we
want to stream: encode straight into a `Write` (or async sink) and decode
straight out of a `Read` (or async source), so peak memory is ~one copy of the
data plus a bounded coder buffer, not two.

## The actual near-term decision: keep 1.0 streaming-*compatible*

**We do not need a streaming API for 1.0.** What we need is to not *foreclose*
one — or to consciously decide streaming is not near-future. The whole rest of
this document establishes one load-bearing fact:

> Streaming's **only format prerequisite** is the incompressible-byte layout
> (Decision 2 / delay-interleave, below). Everything else — the
> `RangeEncoder<W>`/`RangeDecoder<R>` coder types, `encode_to`/`decode_from`, and
> the entire async boundary — is **additive API** that changes no bytes and can
> be introduced in any later minor release with zero compatibility risk.

So the 1.0 question is just two format-level choices:

1. **Incompressible-byte layout.** Today: one blob appended at the end
   (un-streamable). Streaming needs the interleaved layout (Decision 2). If 1.0
   ships the at-end layout with **no format version/discriminator**, adding
   streaming later is a **breaking** format change. Three postures:
   - **(a) Adopt the interleaved layout in 1.0 (recommended).** The 1.0 format is
     already the streamable one; a later streaming API is pure additive code.
     Cost now: the format-change work — and with delay-interleave, **zero** bytes
     of steady-state overhead (unlike framing's small per-chunk headers). Bonus:
     the *sync* `encode_to`/`decode_from` become cheap enough to also ship in 1.0.
   - **(b) Defer but add a 1-byte format discriminator.** 1.0 stays at-end; a
     future interleaved variant is self-identifying, so it can coexist. Keeps the
     door open at the cost of one byte per value (matters for tiny values in a
     compression library) and a decoder that dispatches on it. Note: the v2
     format today has **no leading version byte** (verified — raw entropy from
     byte 0, only a trailer magic for the blob), so this posture *adds* a
     permanent per-value byte, whereas (a) needs no discriminator (one format).
   - **(c) Conscious "no streaming near-future."** Ship at-end, no discriminator;
     accept that streaming later means a new major/format. Cheapest now,
     forecloses the most.
2. **Default coder.** Both `Range` and `Ans` can stream — `Range` natively,
   `Ans` (rANS) via per-chunk store-and-reverse (see the rANS section). So the
   coder choice does **not** gate streamability, and TODO #19's flip to `Ans`
   does not forfeit it; the interleaved layout (choice 1) is what matters.

**Recommendation:** posture **(a)**. Delay-interleave makes 1.0 streaming-ready
by construction with *zero* format overhead, lets us optionally ship sync
`encode_to`/`decode_from` in 1.0, and leaves the (still-unsettled) async design
as risk-free later work. If we want the smallest 1.0 change, posture **(b)** is
the minimum that keeps the door open; **(c)** is the explicit "not near-future"
choice. The rest of this doc is the detail behind these.

## Coder streamability: Range natively, rANS in chunks

| coder | encode direction | decode | streamable encode |
|-------|------------------|--------|-------------------|
| `Range` (arithmetic, v2 default) | forward, emits `ready_bytes()` as it goes | forward | **yes, natively** — one continuous coder over the whole stream |
| `Ans` (rANS) | **backward** — buffers ops, encodes LIFO, `reverse()`s | forward | **yes, in chunks** — bounded to one chunk (below) |

Plain rANS can't stream a *single* backward pass (the decoder reads front-to-back,
so the encoder must process back-to-front, needing all symbols first — `ans.rs`
records `Vec<Op>` and reverses in `into_vec`). But **rANS is classically used in
chunks**: cut the symbol stream into fixed-size chunks, record just one chunk's
ops, reverse-encode it, flush, repeat — memory bounded to a chunk. Cross-chunk
**adaptation is preserved**: the forward pass that records ops carries the
context across chunk boundaries (the same forward pass the in-memory coder
already does, merely cut into pieces), so the decoder replays the identical
context evolution. Cost is one rANS state-flush per chunk (~4–8 bytes) —
negligible at a 64 KiB chunk.

**Decision 1 (revised): both coders can stream.** `Range` streams as a
continuous coder; `Ans` streams chunk-wise. So streamability is *not* a
Range-vs-Ans fork and does **not** constrain TODO #19. One asymmetry to remember:
for `Range`, chunk boundaries (if any) are transport-only — the coder flows
across them; for `Ans`, each chunk is a self-contained coding unit whose state is
flushed at the boundary.

## Scope decision (settled): v2 only, and change the v2 format itself

We are **not** touching v1. It is the inferior legacy format that exists only
for hypothetical existing clients; the whole point of this work is to prepare a
**1.0** release whose default is a solid, streamable **v2** we can build on.
Because v2's byte format is **not yet frozen** (pre-1.0), we are free to change
the format itself so that a single v2 layout both streams and is what the
in-memory API produces — no separate "streaming format." (For the record, v1
already streams — production `Writer<W>` writes forward, `Reader<R>` reads
forward, no deferred incompressible buffer — but we deliberately leave it alone.)

## The core design problem: interleaving incompressible bytes

Today both v2 coders keep `incompressible_bytes: Vec<u8>` separate from the
entropy stream and append it (behind a `MAGIC_*` marker) in `into_vec`. That
layout is O(N) and un-streamable: for incompressible-heavy data (random bytes,
the float "raw" tier over a huge `Vec<f64>`) the whole thing buffers until the
end. The separation exists for **speed** — incompressible runs are `memcpy`'d as
a block instead of coded 8 bits/byte (the raw-tier decode is ~120× a memcpy, a
deliberate optimization we must not lose).

The tension: **`memcpy` speed means the bytes bypass the coder; a bypassed byte
is not sequenced by the coder, so the decoder can't know when it appears; and
the decoder reads *ahead* of its logical position (its state holds a fixed
window of future entropy bytes), so raw bytes placed "inline" get swallowed into
the arithmetic state before the decode logic knows they're raw.** Four ways out,
in the order we considered them:

- **A. Inline as uniform bits.** Code each incompressible byte as 8 bits at 50/50
  through the coder (the trait default; what v1 does). Streams, length-free — but
  throws away the `memcpy` fast path. **Rejected on speed.**
- **B. Byte-align flush per run.** Flush the coder to a byte boundary at each run,
  `memcpy`, resume. Keeps `memcpy` but pays ~1 byte padding + re-init per run;
  the raw tier's many tiny runs make that ~10%+. **Rejected as a general
  mechanism** (but reused as the fallback for *large* runs in D).
- **C. Chunked dual-stream framing.** Keep the two streams separate and flush
  interleaved `[E-len][E][I-len][I]` frames every `K` bytes. Keeps `memcpy`,
  streams, ~0.005% overhead at `K = 64 KiB`. Simple and robust — kept as the
  **documented fallback** if D's correctness proves too fiddly.
- **D. Delay-interleave (chosen).** Keep `memcpy`, interleave the raw bytes
  directly into the single stream, and carry **no length headers at all** — the
  decoder already knows every run's length from the decode logic. This works by
  having the encoder pay forward the decoder's *deterministic* read-ahead. It is
  the cleanest format (zero overhead, fully interleaved) at the cost of more
  encoder/decoder bookkeeping; spec below.

**Decision 2 (chosen): option D — delay-interleave — is the v2 format**, with C
as the fallback if implementation/verification of D proves not worth it.

## Delay-interleave spec (option D)

### The core rule

After its initial state fill, the decoder pulls exactly one entropy byte per
renormalization, in lockstep with the encoder's one-emit-per-renorm — so the
decoder's cumulative *pull* count stays a **fixed constant `W` ahead** of the
encoder's cumulative *emit* count, where `W` is the decoder's initial fill (the
coder's state width in bytes). `W` is a known coder constant.

Therefore: **when the encoder reaches an incompressible run, it withholds the raw
bytes until it has emitted `W` more entropy bytes, then splices the run into the
stream.** When the decoder's logic reaches that field, it has pulled exactly
`emit_count + W` entropy bytes — precisely where the encoder placed the run — so
the run sits at the decoder's read cursor. The decoder `memcpy`s the run's known
length directly out of the stream (bypassing the arithmetic state), then coding
resumes; the arithmetic state's next pull lands on the entropy byte after the
run. No length is written because both sides already know it.

Carries are safe: a correct range coder emits only *settled* bytes (it holds
unsettled ones via cache + carry-count), so a later carry can never reach back
into bytes already written before a spliced run. The delay is counted in settled
emitted bytes.

### Edge case 1 — end of stream

The decoder's final `W` bytes of read-ahead have no corresponding emit (it
zero-pads past EOF today). A run within `W` bytes of the end has no `W` real
entropy bytes to hide behind. Handle at `finish`: flush the coder, emit any
still-withheld runs, and define a small deterministic tail convention so the
decoder's trailing read-ahead cannot be mistaken for raw bytes (e.g. the encoder
emits exactly the padding the decoder will consume, and the decoder knows from
its own logic when the value is complete). Bounded, one-time.

### Edge case 2 — a large contiguous run

The `W`-byte delay is fed by entropy from *subsequent* symbols. Short runs
interspersed with coded data (the common case — a `Vec<f64>` raw tier is 8 raw
bytes, a coded selector, 8 more…) keep entropy flowing, so the withheld buffer
never exceeds ~`W` + a couple of runs. But a *single giant* `Incompressible`
field (e.g. a 1 GB blob — precisely the motivating case) has no interspersed
entropy to cover the delay, so withholding it would buffer O(field). For runs
whose length exceeds a fixed threshold `T`, fall back to **B**: flush the coder,
`memcpy` the blob straight through (bounded output buffer, not held in the
encoder), re-init after. This is chosen *deterministically from the run length*,
which the decoder also knows — so still **no signal in the stream**: a run longer
than `T` flushes; a shorter run delays; both sides decide identically.

### Correctness surface (must be property-tested)

D trades format cleanliness for a real correctness burden: a wrong `W`, an
off-by-one at a run boundary, or a botched tail is *silent corruption*. Tests
must cover random interleavings of coded values and raw runs of every length
(0, 1, …, `W`-ish, `T`-ish, `>T`), runs landing within `W` of the end, multiple
runs closer than `W` apart, and empty/one-byte values — round-tripping under
both coders and the in-memory and streaming paths (which must produce identical
bytes).

## Sync API

Two layers.

### Low-level: streaming coder types (v2)

```rust
/// EntropyCoder that flushes ready bytes straight to `W` instead of a Vec.
pub struct RangeEncoder<W: Write> { /* state + small buffer + deferred io::Error */ }
/// EntropyDecoder that pulls bytes from `R` on demand.
pub struct RangeDecoder<R: Read> { /* state + small buffer + deferred io::Error */ }
```

`RangeEncoder<W>` implements `EntropyCoder`; it writes settled entropy bytes to
`W` as they emerge, and applies the delay-interleave rule (Decision 2 / option
D): it withholds each incompressible run until `W_state` more entropy bytes have
been written, then splices the run in — falling back to flush-`memcpy`-reinit for
runs over the threshold. `RangeDecoder<R>` implements `EntropyDecoder`, pulling
entropy bytes from `R` into its arithmetic state and, at each
`decode_incompressible_bytes(n)`, reading the `n` raw bytes sitting at its cursor.
The in-memory `Range` runs the *same* logic against a `Vec` sink instead of a
`Write`, so the two paths produce identical bytes and share one implementation.
(`W_state` here is the coder's state-width constant `W` from the spec; the
withheld-run buffer and the ~`W`-byte look-behind are the only extra state over
today's coder.)

**Infallible-trait wrinkle.** `EntropyCoder`/`EntropyDecoder` methods are
infallible (`encode_bits` returns `()`, `decode_bits` returns `[bool; N]`) — no
place to surface an `io::Error` on the hot path, and we do *not* want to add
`Result` to those methods (they're the innermost loop). Resolution:

- Encode: the encoder stores `Option<io::Error>`. On the first write error it
  latches the error and turns subsequent writes into no-ops; `finish()` returns
  the latched error. Errors surface at `finish`, hot path stays branch-light.
- Decode: reads past EOF already yield "arbitrary but in-range" bits that
  higher-level `Encode::decode` validates. A real read error is latched the same
  way and surfaced by the top-level `decode_from` (which already returns
  `Result<T>`). `decode_incompressible_bytes` already returns `Result`, so bulk
  reads surface errors directly.

### High-level convenience functions

```rust
// v2 (crate::v2::, also re-exported at crate root next to encode/decode)
pub fn encode_to<T: Encode, W: Write>(value: &T, writer: W) -> io::Result<()>;
pub fn decode_from<T: Encode, R: Read>(reader: R) -> io::Result<T>;
```

`encode_to` builds a `RangeEncoder::new(BufWriter::new(writer))`, runs
`value.encode(&mut enc, &mut ctx)`, then `enc.finish()`. `decode_from` mirrors it
with `RangeDecoder`/`BufReader`. Internal `BufWriter`/`BufReader` so callers
needn't wrap; document that a raw unbuffered `W`/`R` is handled but buffering is
automatic. The existing `encode(&T) -> Vec<u8>` becomes a thin wrapper over
`encode_to` with a `Vec` sink (identical bytes, since the format is now the
framed one).

## Async API — OPEN, needs exploration

This is deliberately unsettled. Two independent questions: (1) what is the async
**boundary abstraction**, and (2) how does the sync coder **drive** it.

### (1) Boundary abstraction: prefer a stream of `Bytes` chunks over raw `AsyncRead`/`AsyncWrite`

The delay-interleave format is a single flat byte stream (no format-level
frames), but for transport it can be sliced into `Bytes` chunks at **arbitrary**
boundaries — a chunk is just "the next run of output bytes." That maps almost
exactly onto the ecosystem's "body" abstraction, which is worth matching so we
interoperate with web frameworks and object stores instead of forcing callers to
adapt:

- **`bytes::Bytes`** — a cheaply-cloneable, sliceable, refcounted chunk. This is
  the universal currency for "bytes in flight." Our encoder can hand out each
  transport chunk as a `Bytes` (ideally without copying, if the output buffer is
  a `BytesMut` we `split().freeze()`); our decoder consumes `Bytes` chunks and
  slices coder input out of them. Chunk == `Bytes` item is a clean fit.
- **HTTP bodies** — `http-body`'s `Body` yields `Frame<Bytes>`; hyper/axum/tonic
  all speak this. A compactly stream that produces/consumes `Bytes` chunks drops
  into a response/request body directly.
- **Object stores (S3 etc.)** — the read side of `aws-sdk-s3` `GetObject` is a
  `ByteStream` that is, at bottom, a stream of `Result<Bytes, _>`; other stores
  (`object_store` crate) expose the same shape. The write side for large/unknown
  length is **multipart upload** (upload N parts), which lines up with our
  framing — a part is just a run of frames. So a `Stream<Item = Result<Bytes>>`
  boundary gives near-direct S3 interop in both directions. **To verify by
  prototyping against `aws-sdk-s3` / `object_store`.**
- **`Stream` / `TryStream`** — `futures::Stream<Item = T>` and its fallible alias
  `TryStream` (`Item = Result<T, E>`) are the lingua franca, but the ergonomics
  are genuinely awkward (manual `Pin`/`poll_next`, the split fallible/infallible
  traits you noted, no `async fn` in the trait). An alternative worth weighing:
  define our own tiny boundary trait with `async fn`-in-trait (now stable), e.g.
  `trait ChunkSink { async fn put(&mut self, b: Bytes) -> io::Result<()> }` /
  `trait ChunkSource { async fn next(&mut self) -> Option<io::Result<Bytes>> }`,
  and provide adapters to/from `Stream`, `AsyncWrite`/`AsyncRead`, S3, and hyper.
  That keeps our surface clean and shoves the `Stream` awkwardness into one
  optional adapter module.

### (2) Driving the sync coder: the suspension problem

The `Encode` traversal is sync and recursive; it cannot become a `poll_next`
generator without stable coroutines/generators (Rust has none), so we cannot
run it *on* the async task producing one chunk per poll. Realistic bridges:

- **`spawn_blocking` + bounded channel of `Bytes`.** Run the sync coder on a
  blocking thread; it pushes frames into a bounded channel (backpressure) and
  the async side exposes the channel as a `Stream<Item = Result<Bytes>>` (encode)
  or feeds a channel the sync decoder pulls from (decode). The channel-of-`Bytes`
  is the natural crossing, and this reuses the sync coder unchanged. It *is* the
  idiomatic place for CPU-bound compression (off the executor). Costs: a blocking
  thread per op, `T: Send + 'static`, and a dependency on *some* executor's
  blocking pool.
- **Stackful coroutine** (`corosensei`/`genawaiter`-style) to suspend the
  traversal at chunk boundaries without a full thread — avoids `spawn_blocking`
  but adds an unsafe-ish dep and its own caveats. Worth a look, lower priority.
- **Buffer-then-send** (encode fully in memory, then async-send the frames) —
  trivial, but O(N); only for small values.

Runtime neutrality: `bytes` and `futures::Stream` are **not** tokio-specific, so
the *boundary* can be runtime-neutral; only the `spawn_blocking` bridge needs an
executor, and that can be a thin feature-gated adapter (`tokio` first) rather
than baked into the core.

### Next step for async: prototype, don't commit

Because the ergonomics only reveal themselves in use, the plan is to **prototype**
a `Bytes`-chunk boundary against (a) an in-memory `Stream`, (b) an `axum`/hyper
body, and (c) `aws-sdk-s3` `GetObject`/multipart `PutObject`, and compare a
`Stream`/`TryStream` surface vs a small `async fn`-in-trait `ChunkSink`/
`ChunkSource` surface — before committing any async types to the 1.0 API. The
sync `encode_to`/`decode_from` and the Decision-2 format change do not depend on
this and can land first.

## rANS streaming details (chunked)

Mechanics are in "Coder streamability" above. A few implementation notes for when
we do the `Ans` streaming path (it is *not* needed for the first cut, which is
`Range`):

- **Chunk = coding unit.** Each `Ans` chunk records its ops, reverse-encodes,
  flushes state (~4–8 bytes), and is independently decodable. The context carries
  across chunks via the forward record pass, so no compression loss beyond the
  per-chunk flush bytes.
- **Interaction with delay-interleave.** rANS will likely use the **framed**
  approach *between* chunks (each chunk length-prefixed). *Within* a chunk we
  could still delay-interleave the incompressible bytes among that chunk's ops —
  which may not be bad, since a chunk is already fully buffered for the reverse
  pass, so the splice is just an in-buffer interleave with no streaming
  constraint. rANS has no carries, so the splice is simpler than `Range`'s, and
  the chunk boundary makes the tail edge case trivial. (Thought for later; start
  with `Range`.)
- **Chunk size** is an internal knob (share `K`/`T` thinking with the `Range`
  path), invisible in the byte format beyond the flush bytes.

## Decode-into-sink (out of scope, noted)

`decode_from` still materializes the whole `T`. For values too big to hold even
decoded, a visitor/SAX-style streaming deserialization (callbacks per element)
would be needed — a much larger API. Out of scope here; the immediate win is not
*also* holding the compressed bytes.

## Proposed sequencing (all v2, toward 1.0)

1. **Reframe the v2 `Range` format to delay-interleave (Decision 2 / option D),
   in memory first.** Replace the append-at-end incompressible blob with the
   `W`-delay splice (+ flush-fallback for runs over `T`). Keep it a pure
   in-memory `encode(&T)->Vec<u8>` / `decode(&[u8])` change so it can be A/B'd for
   size and round-tripped exhaustively (the correctness surface above) *without*
   any IO plumbing yet. This is the format commitment that must land before 1.0
   freezes. `Ans` decode must read the same layout. **This is the step we start
   implementing now.** If D's correctness proves not worth the bookkeeping, fall
   back to option C (framing) here.
2. **`RangeEncoder<W>`/`RangeDecoder<R>` + `encode_to`/`decode_from`.** Point the
   same coder logic at a `Write`/`Read` instead of a `Vec`; `encode(&T)->Vec<u8>`
   becomes a thin wrapper. Includes the deferred-`io::Error` handling.
3. **`Ans` streaming (chunked)** — optional, when we want rANS on huge inputs.
4. **Async** — prototype the `Bytes`-chunk boundary (see async section); feature-
   gated, additive, last.

## Open decisions

- **D1 (revised)** both coders can stream — `Range` natively, `Ans` chunk-wise —
  so streamability does not gate the coder choice or TODO #19.
- **D2 (chosen)** the v2 format becomes **delay-interleave** (option D): `memcpy`
  speed, fully interleaved, **zero** format overhead, one format for both APIs.
  Fallback to option C (chunked framing) only if D's correctness proves not worth
  it. First implementation step.
- **D5 (settled)** v2 only; do not touch v1. This is 1.0 prep — land the format
  change (D2) before freezing.
- **D3 (open, explore)** async is unsettled. Leaning: a `Bytes`-chunk boundary
  (matching hyper bodies / S3 `ByteStream` / `object_store`) driven by a
  `spawn_blocking` + bounded-channel bridge, since the sync traversal can't be a
  `poll_next` generator. Prototype a `Stream`/`TryStream` surface vs a small
  `async fn`-in-trait `ChunkSink`/`ChunkSource` against real S3 + hyper before
  committing. Sync API + format land first and don't depend on this.
- **D4 (folded into D3)** ecosystem: keep the boundary runtime-neutral (`bytes`,
  `futures::Stream` aren't tokio-specific); confine executor coupling to a
  feature-gated `spawn_blocking` adapter (`tokio` first).
- **Error model (proposed)** deferred-latched `io::Error` surfaced at
  `finish`/top-level, so the infallible hot-path coder trait is unchanged.
- **`W` (pinned): `W = 8`.** The `Range` decoder window is a `u64` filled from
  the first 8 bytes and pulled one byte per renorm in lockstep with the encoder,
  so the decoder's pull-count is exactly 8 ahead of the encoder's emit-count. The
  delay is a constant 8 entropy bytes. The coder is **carry-free** (Subbotin
  `lo`/`hi`: a byte is emitted only once `lo` and `hi` agree on it, so it never
  changes) — "emit only settled bytes" is automatic, no carry-counting.
- **`T` (to choose)** the large-run flush-fallback threshold; internal, invisible
  in the format except via where runs land. Start with something like a few KiB
  and tune.
