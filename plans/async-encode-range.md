# Async streaming encode for `Range` (mirror of the async decode PR)

Status: **design**, to be built on top of the async **decode** work (PR #46,
branch `async-decode`). Reuses that branch's `stream` feature, its `bytes` /
`futures-core` dependencies, its `MAX_BYTES` machinery, and the
`ChunkSource`/`AsyncRangeDecoder` shape as the template to mirror. No executor,
no tokio; `futures-core` only.

See [`streaming-io-api.md`](streaming-io-api.md) for the format-level background
the whole streaming effort rests on (delay-interleave, `W = 8`, the deferred-error
model). This document is only the **encode** half of the async API, and it
supersedes that doc's async-encode pessimism ("the traversal cannot be a
`poll_next` generator without stable coroutines") — the decode PR showed the way
out, below.

## The goal

`v2::encode(&T) -> Vec<u8>` holds both the value and its whole compressed output
in memory. The sync `Range::encode_to<W: Write>` already streams to a `Write`.
This adds the **async** streaming encoder: produce the compressed bytes *as a
stream of [`Bytes`] chunks*, so a large value can be uploaded to the network
(S3 / object_store / an HTTP body) with peak memory of ~one copy of the value
plus a bounded coder buffer — never the whole compressed blob.

The headline surface is a chunk **stream we hand out**, since that is what the
upload ecosystem pulls from (`ByteStream::from_stream`, `reqwest::Body::wrap_stream`,
`http-body`); and a chunk **sink we push into**, for `object_store`'s
`WriteMultipart` and `futures`/`tokio` `AsyncWrite`. One mechanism underlies
both (below).

## Why the encoder is *smaller* than the decoder

The decode PR had to make **every** type async, because the input arrives in
chunks whose boundaries are the transport's choice: any value can straddle a
chunk, so every read point must be able to suspend. It then bolted a *sync
fast-path* back on (`sync_capacity` / `sync_decode_if_there_is_room` /
`with_sync`, gated by each type's `MAX_BYTES`) to stop paying the async tax once
enough was buffered, and paid for a whole coder-state handoff (`Decoder`
positioned at the async decoder's cursor).

Encode inverts both asymmetries, and they compound in our favour:

1. **We choose the chunk boundaries.** The compressed form is one flat byte
   stream (delay-interleave: no format frames), sliceable at *any* offset. So
   chunking is purely a matter of **draining the output buffer** when convenient
   — it is completely independent of the value structure, and **no value is ever
   split across a chunk**. A value therefore never has to suspend *mid-encode*.
2. **One continuous coder the whole time.** The async encoder holds a single
   `RangeEncoder<BytesMut>` and both sync and async code call methods on that
   same object. There is **no state handoff** — nothing like the decoder's
   `with_sync` / `Decoder`-repositioning exists here, because there is never a
   second coder to hand state to.

Consequently the async encode surface is the **inverse** of the decode surface:

| | decode (PR #46) | encode (this doc) |
|---|---|---|
| default per type | **async**, suspends at every read | **sync**, appends to the buffer |
| special-cased | sync fast path (`with_sync`), for *bounded* types | async override, for *large / unbounded* types |
| coder-state handoff | yes (`with_sync`, `Sync<'a>`) | **none** (one `RangeEncoder`) |
| gate | `MAX_BYTES` finiteness → may go sync | `MAX_BYTES` large/∞ → must go async |

## The core idea: sync by default, containers yield

`EncodeAsync::encode_async` has a **default implementation that is fully sync**:
run the existing sync `encode` against the shared coder, appending to the buffer,
and return. No `.await`, no split. This is correct for every type — the only
thing the default forgoes is *handing control back*, so a value encoded through
the default adds all of its bytes to the buffer before the caller can drain.

That is exactly right for a **small** value (its `MAX_BYTES` is a few bytes; the
enclosing container drains right after). It is *wrong* for a **large or
unbounded** value — a big collection, a long string, a 1 MB `Incompressible`
blob, a fixed `[u8; 1_000_000]` — which would buffer `O(field)` before control
ever returns. So:

> **A type should implement `encode_async` (override the default) iff it can
> produce a large amount of output in one value — i.e. it is unbounded (a
> collection) *or* bounded but large.** Everything else inherits the sync
> default and needs no code at all.

The override's job is only to **hand control back periodically** so the buffer
can drain and the sink can apply backpressure:

```rust
/// The async twin of `EncodingStrategy`'s encode half. Mirror of `DecodeAsync`.
///
/// The default is the *sync* encoder: correct for every bounded-small type,
/// which is most of them, so most types need no async code. A type overrides
/// this only when one value can produce a large amount of output — any
/// collection, or a bounded-but-large type (`[u8; N]` with big `N`, a large
/// `Incompressible` run) — so that it drains the buffer as it goes rather than
/// buffering the whole value.
pub trait EncodeAsync<T>: EncodingStrategy<T> {
    async fn encode_async<S: ChunkSink>(
        value: &T,
        enc: &mut AsyncRangeEncoder<S>,
        ctx: &mut Self::Context,
    ) -> std::io::Result<()> {
        // Default: the existing sync path, straight into the buffer.
        Self::encode(value, &mut enc.coder, ctx);
        Ok(())
    }
}

// Containers override to recurse + drain between elements:
impl<T> EncodeAsync<Vec<T>> for Normal
where
    Normal: EncodeAsync<T> + EncodingStrategy<T, Context = <T as Encode>::Context>,
{
    async fn encode_async<S: ChunkSink>(
        v: &Vec<T>, enc: &mut AsyncRangeEncoder<S>, ctx: &mut VecContext<T>,
    ) -> std::io::Result<()> {
        Small::encode(&v.len(), &mut enc.coder, &mut ctx.len);
        for elem in v {
            <Normal as EncodeAsync<T>>::encode_async(elem, enc, &mut ctx.element).await?;
            enc.drain_if_full().await?; // put a chunk iff buffered >= chunk_target
        }
        Ok(())
    }
}
```

The async encoder is just the continuous coder plus the sink:

```rust
pub struct AsyncRangeEncoder<S> {
    coder: RangeEncoder<BytesMut>, // one continuous coder, buffering into BytesMut
    sink: S,
    chunk_target: usize,           // e.g. 64 KiB
    error: Option<std::io::Error>, // latched, surfaced by finish (as the sync coder does)
}

impl<S: ChunkSink> AsyncRangeEncoder<S> {
    /// Drain the ready front of the buffer as one chunk, if it has reached the
    /// target. The only `.await` a container override adds per element.
    async fn drain_if_full(&mut self) -> std::io::Result<()> {
        if self.coder.buffered() >= self.chunk_target {
            let chunk = self.coder.take_ready(); // BytesMut::split().freeze() — zero copy
            self.sink.put(chunk).await?;
        }
        Ok(())
    }

    /// Flush the coder's tail + any remaining buffered bytes as final chunk(s).
    async fn finish(mut self) -> std::io::Result<()> {
        self.coder.finish_into_buffer();          // last byte + delay-interleave tail runs
        let rest = self.coder.take_ready();
        if !rest.is_empty() { self.sink.put(rest).await?; }
        self.sink.finish().await
    }
}
```

Notes:

- **`drain_if_full` between elements is the whole per-element tax:** one length
  compare, awaiting only ~once per `chunk_target`. Cheaper than the decoder's
  per-element drain, and far cheaper than the decoder's per-*byte* suspension
  points.
- **No `MAX_BYTES` gate at runtime.** `MAX_BYTES` is used only at *derive time*
  to decide which types get an override (below); the runtime path never consults
  it.
- **The tail** (delay-interleave withheld runs + the coder's final settled byte)
  is produced by the same `RangeEncoder::finish` logic the sync path already has;
  it just lands in the buffer and drains as ordinary chunks. `Range` is
  carry-free (`W = 8`), so nothing about chunk boundaries interacts with carries.

## The `RangeEncoder<W>` API the coder must expose

Today's `RangeEncoder<W: Write>` (async-decode branch, `src/v2/arith.rs`) writes
settled bytes straight to `W` via `push_entropy` / `write_out`, holds
`withheld: [Vec<u8>; W_DELAY]` for the delay-interleave splice, and latches an
`io::Error`. For the async encoder we want the same coder buffering into an
in-memory `BytesMut` and letting the async layer decide when to drain. Two
shapes, pick during implementation:

- **(a) `W = BytesMut` directly.** `BytesMut: !Write`, so either add a thin
  `Write`-for-`BytesMut` wrapper, or generalise `RangeEncoder`'s sink from
  `io::Write` to a tiny internal `PushBytes` trait implemented for both
  `Vec<u8>`/`W: Write` and `BytesMut`. Then add:
  - `fn buffered(&self) -> usize` — bytes sitting in the sink not yet drained
    (excludes still-`withheld` runs, which are not yet splice-eligible).
  - `fn take_ready(&mut self) -> Bytes` — `split().freeze()` of the drained
    front. Must not cut inside a not-yet-spliced withheld run; the `W_DELAY`
    splice already guarantees a run is written only once its target byte is,
    so "everything written so far" is always a safe cut point.
  - `fn finish_into_buffer(&mut self)` — the body of today's `finish` (append
    `last_byte`, flush remaining `withheld` runs) but leaving bytes in the
    buffer rather than returning `W`.
- **(b) keep `W: Write`, sink = a `BytesMut`-backed `Write`.** Least change to
  `RangeEncoder`; `buffered`/`take_ready` read the wrapper's `BytesMut`. Likely
  the smaller diff. Decide by which keeps `push_entropy` untouched.

Either way the arithmetic, `withheld` handling, `W_DELAY` splice and tail are
**unchanged and shared** with the sync/in-memory coder — only where the bytes
rest differs, exactly as `Range` vs `RangeEncoder<W>` already share one impl.

## Which types implement `encode_async`

Reuse the async-decode branch's per-type `MAX_BYTES` (finite ⇒ bounded).

- **Derive:** a struct/enum inherits the sync default **iff every field is
  bounded** (finite `MAX_BYTES`) *and* the total is below a "large" threshold
  `L`; otherwise the derive emits a field-recursive `encode_async` that calls
  each field's `encode_async` (so an unbounded/large field reaches its own
  override and yields). This is the mirror of the decoder derive's `MAX_BYTES`
  computation and `decode_variants_async`, and reuses the same generic-bound
  plumbing (`Normal: EncodeAsync<#t>` predicates, the recursion base case).
- **Hand-written overrides** — the container/large strategies, ~a dozen:
  `Vec<T>`/slices, `String` + byte blobs (`Vec<u8>`), `BTreeMap`/`BTreeSet`
  (+ hash variants), `Compressible` (Lz77), `Arc<str>` dictionary encoding,
  `Sorted`, `LowCardinality`, and the **large bounded** cases: big fixed arrays
  `[T; N]` and the `Incompressible` strategy (drain mid-run, mirroring the
  delay-interleave `T`-threshold flush-fallback so a 1 GB blob streams).
- **Everything else** — scalars, floats, `bool`, `Option`, tuples of bounded
  types, `AtMost`, `NonZero`, enums of scalars, small structs — **inherits the
  sync default untouched.** This is the bulk of the type surface and it costs
  nothing.

Threshold `L` for "bounded but large": start at a few KiB (same order as the
delay-interleave `T`), tune later; it only affects *when* a bounded value bothers
to drain, never correctness.

## The async boundary: one sink trait, two front ends

Mirror the decoder's `ChunkSource`. The sink is the minimal dual:

```rust
/// The push dual of the decoder's `ChunkSource`. `async fn` desugared to
/// `-> impl Future` for the same reason `AsyncEntropyDecoder`'s methods are:
/// keep the public trait warning-free and avoid forcing a `Send` bound onto T.
pub trait ChunkSink {
    fn put(&mut self, chunk: Bytes) -> impl Future<Output = std::io::Result<()>>;
    fn finish(&mut self) -> impl Future<Output = std::io::Result<()>>;
}
```

### Front end 1 — a `Stream<Bytes>` we hand out (no tokio)

This is the ecosystem-native upload shape (S3 SDK `ByteStream::from_stream`,
`reqwest::Body::wrap_stream`, `http-body`). The plan doc feared it needed a
coroutine or a spawned task; it does **not**, once the traversal is async. It is
the `async-stream` / `genawaiter` self-driven-generator trick, inlined over
`futures-core`:

```rust
/// A ChunkSink that turns the encode future into a Stream: park a chunk, then
/// suspend exactly once. The *consumer's* poll_next is the resume signal, so
/// there is no executor, no thread, no channel — and perfect backpressure
/// (exactly one chunk produced per poll).
struct YieldSink(Rc<RefCell<Option<Bytes>>>);
impl ChunkSink for YieldSink {
    async fn put(&mut self, chunk: Bytes) -> std::io::Result<()> {
        *self.0.borrow_mut() = Some(chunk);
        YieldOnce::default().await;  // Pending on first poll, Ready on the next
        Ok(())
    }
    async fn finish(&mut self) -> std::io::Result<()> { Ok(()) }
}

pub fn encode_to_byte_stream<T>(value: T) -> impl Stream<Item = std::io::Result<Bytes>>
where /* Normal: EncodeAsync<T>, T owned */ {
    // struct holding Pin<Box<encode future>> + the Rc<RefCell<Option<Bytes>>>.
    // poll_next: poll the future.
    //   Pending          => a chunk was parked -> Ready(Some(Ok(take()))).
    //   Ready(Ok(()))    => final flush already parked -> then Ready(None).
    //   Ready(Err(e))    => Ready(Some(Err(e))), then None.
}
```

Correctness of "every `Pending` is a parked chunk": in this front end the sink is
`YieldSink` and the encode traversal awaits **nothing else** (it is pure CPU plus
`put`), so a `Pending` bubbling out of the future is always a `put`. Keep it
robust anyway by checking the cell (`Pending` + empty cell ⇒ propagate `Pending`)
so the same `EncodeStream` still works if a genuinely-awaiting sink is ever
composed.

- **No executor, no thread, no channel, no `Send` forced by us** (`Rc`/`RefCell`,
  single task). The only `Send + 'static` bound is the one **the consumer**
  imposes at its call site (`ByteStream::from_stream`, `wrap_stream` may move the
  body cross-thread) — satisfied when `value: Send + 'static`, not by any
  runtime of ours.
- **Chunk granularity is ours** via `chunk_target`; `split().freeze()` is
  zero-copy.

### Front end 2 — push into a caller's sink

`impl ChunkSink for` `futures::io::AsyncWrite` (write-all + flush) and for
`object_store::WriteMultipart` (its `write`/`put` are *sync* and non-blocking;
backpressure via an occasional `wait_for_capacity().await`, `finish().await`).
This covers S3 / GCS / Azure through `object_store` with no spawn. Then:

```rust
pub async fn encode_stream<T, S: ChunkSink>(value: &T, sink: S) -> std::io::Result<()>;
```

drives `encode_async` into that sink and calls `AsyncRangeEncoder::finish`.

Both front ends are the **same** `encode_async` + `AsyncRangeEncoder`; only the
`ChunkSink` differs. Feature-gate everything under the existing `stream` feature.
Optional thin adapters (a `tokio::io::AsyncWrite` `ChunkSink`, examples wiring
`aws-sdk-s3` multipart / `reqwest`) can live behind an extra `tokio` feature so
the core stays runtime-neutral.

## Error model

Same as the sync coder and the async decoder: latch the first `io::Error`, turn
later writes into no-ops, surface it at `finish` / from the top-level
`encode_stream` (or as a `Stream` item in the `Bytes`-stream front end). The hot
path stays branch-light.

## Correctness surface (must be tested)

- **Byte-identical to sync.** For every corpus and both front ends, the async
  encoder must produce **exactly** the bytes `Range::encode` / `encode_to`
  produce — property-tested across random values, and round-tripped through the
  sync **and** async decoders (all four encode×decode combinations agree).
- **Chunk-boundary invariance.** The output is independent of `chunk_target`
  and of where `drain_if_full` fires (drive the same value at chunk targets
  1, 2, 7, 64 KiB, `usize::MAX`; all identical). This is the encode analog of
  the decoder's chunk-splitting tests.
- **Backpressure / bounded memory.** With a slow sink, peak buffered bytes stays
  ~`chunk_target` + one value's `MAX_BYTES` (or, for a large-bounded/unbounded
  field, ~`chunk_target` + `L`), *not* `O(value)`. Assert against a counting
  sink over a large `Vec` and a large `Incompressible` blob.
- **Edge cases:** empty value, single tiny value (no drain ever fires — one
  chunk at `finish`), a value whose entire output is < `chunk_target`, the
  delay-interleave tail landing exactly at a boundary, and a `put` that errors
  (latched, surfaced, no further writes).

## Sequencing

1. **Expose the buffer API on `RangeEncoder`** (`buffered` / `take_ready` /
   `finish_into_buffer`, and the `BytesMut` sink), unit-tested against the
   existing sync `finish` for byte-identity. No async yet.
2. **`AsyncRangeEncoder<S>` + `ChunkSink` + `encode_stream(value, sink)`**, with
   the `EncodeAsync` trait's **sync default only** and hand-written overrides for
   `Vec` and `String`. Vertical slice: `Vec<u64>` and `Vec<String>` stream
   byte-identically to sync, tested. (Mirror of the decode PR's first vertical
   slice.)
2b. **Prototype the `Bytes`-stream front end** (`YieldSink` / `encode_to_byte_stream`)
    as a `src/bin/async-encode-*.rs`, driven by `futures_executor::block_on`
    into a fake S3 sink, to confirm the tokio-free `Stream` end-to-end and
    **measure the async tax** vs sync `encode_to` (as PR #46 measured decode).
3. **Fill in the remaining container overrides** (maps, sets, `Compressible`,
   `Arc<str>`, `Sorted`, `LowCardinality`) and the large-bounded cases (big
   arrays, `Incompressible` mid-run drain).
4. **Derive `EncodeAsync`**, gating on `MAX_BYTES` finiteness + `L` exactly as
   the decode derive gates on `MAX_BYTES`; port its generic-bound plumbing.
5. **Ship both front ends** under `stream`; optional `tokio` adapters + S3 /
   reqwest examples behind a `tokio` feature.

## Open questions

- **`chunk_target` and `L` defaults.** Internal knobs, invisible in the format.
  Start `chunk_target` ~64 KiB, `L` ~ a few KiB; tune with the step-2b bench.
- **Reuse `async-stream`/`genawaiter` vs inline `YieldOnce`.** Inlining ~40
  lines keeps deps to `futures-core` (matches the decode PR). Decide when writing
  the front end.
- **`RangeEncoder` sink shape** (a) vs (b) above — smallest diff wins; settle in
  step 1.
- **`Ans` async encode** is out of scope here (chunked, store-and-reverse per
  the streaming-io doc); this document is `Range` only, as the decode PR was.
