# Async streaming encode for `Range`

Status: **design (revision 4)**. The async **decode** work this mirrors has
**landed** (PR #46, merged as `54d2d66`), so this is no longer written against a
branch — it is written against `main`, plus `EntropyCoder::split_point` from
PR #54, which it depends on and which is called out where it matters.

There is a companion plan, [`async-encode-ans.md`](async-encode-ans.md), for the
`Ans` half. Read this one first: it settles the traversal, which both coders
share. The `Ans` document is short because, once this is built, `Ans` is a drain
policy rather than a second implementation.

## What revision 4 changes, and why

The first three revisions were written before async decode landed. Three of
their central design decisions were guesses about how the decode side would come
out, and all three guessed wrong in the same direction — they invented a second
mechanism where the shipped code has one.

1. **There is no `EncodeAsync` trait.** Revision 3 proposed one, mirroring an
   expected `DecodeAsync`. No such trait exists: `MAX_BYTES`, `decode_awaiting`
   and `decode_async` are members of [`Encode`](../src/v2/mod.rs) itself. So the
   encode twins are `encode_awaiting` and `encode_async`, members of `Encode`,
   with the same relationship (`encode_awaiting` is what you implement,
   `encode_async` is what you call).
2. **The "which types override" question is answered by `MAX_BYTES` at run
   time**, exactly as `decode_async` answers its own. Revision 3 declared this
   impossible ("a proc-macro sees syntax") — true of a *compile-time* gate in the
   derive, irrelevant to a **runtime** `const` comparison in a provided default,
   which is what the decode side actually does. This deletes revision 3's
   per-impl override table, its transparent-wrapper forwarding rule, its
   threshold `L`, and the bounded-memory test that existed to catch a wrapper
   that stopped forwarding. See [The gate](#the-gate-max_bytes-at-run-time).
3. **The drain schedule already exists**, as
   [`EntropyCoder::split_point`](../src/v2/mod.rs) from PR #54. Revision 3
   proposed to place drains by hand in "~a dozen" container impls; `split_point`
   places exactly those points, from **three** call sites, under a stated rule,
   with a test that proves the coverage is complete. The async encoder must
   drain there and nowhere else. See [The drain schedule](#the-drain-schedule-is-already-written-split_point).

One further reversal, from the `Ans` plan rather than from the decode side:

4. **The coder is generic after all.** Revision 3 kept the coder type concrete
   "on purpose", reasoning that "`Ans` async encode will not share this shape at
   all". With `split_point` in the codecs that is false — the two coders' async
   encoders differ only in what happens *at* a split point. So the traversal is
   written once, over an `AsyncEntropyCoder` trait.

What revisions 2 and 3 got right and this one keeps: the `Send`/`Sync` trap in
the `Stream` front end, the borrowed-sink `ChunkSink` with no `finish`, the
propagate-don't-latch error model, and the `Incompressible` limitation — which
[is now sharper](#limitation-a-large-incompressible-run-is-still-not-streamable),
because the code says exactly why it cannot be fixed for `Range`.

## The goal

`v2::encode(&T) -> Vec<u8>` holds both the value and its whole compressed output
in memory. The sync `Range::encode_to<W: Write>` already streams to a `Write`.
This adds the **async** streaming encoder: produce the compressed bytes *as a
stream of [`Bytes`] chunks*, so a large value can be uploaded to the network
(S3 / object_store / an HTTP body) without ever holding the whole compressed
blob.

Be precise about what that buys, because the obvious claim oversells it. The API
takes `&T`, so the value is *already* wholly in memory; what streaming removes is
the compressed output, which is by construction smaller — usually much smaller —
than the value it came from. Dropping a fraction of an already-committed
footprint is real but modest. The wins that actually justify the API are:

- **Time to first byte.** The upload starts after the first `chunk_target`
  bytes, not after the whole encode.
- **Backpressure.** A slow sink stops the encode, rather than the encode racing
  ahead into a growing `Vec`.
- **Ecosystem fit.** `ByteStream::from_stream` / `Body::wrap_stream` /
  `http-body` want a `Stream<Bytes>` and there is no allocation-free way to hand
  them one today.

(Encoding from a *stream of items* rather than a `&T` is the design that would
genuinely bound peak memory by the value; it is a different API and out of scope
here — see Open questions.)

The headline surface is a chunk **stream we hand out**, since that is what the
upload ecosystem pulls from; and a chunk **sink we push into**, for
`object_store`'s `WriteMultipart` and `futures`/`tokio` `AsyncWrite`. One
mechanism underlies both.

## Why the encoder is smaller than the decoder

The decode PR had to make **every** type async, because the input arrives in
chunks whose boundaries are the transport's choice: any value can straddle a
chunk, so every read point must be able to suspend. It then bolted a *sync
fast path* back on (`sync_capacity` / `with_sync`, gated by each type's
`MAX_BYTES`) to stop paying the async tax once enough was buffered, and paid for
a whole coder-state handoff — a `Decoder` constructed and positioned at the async
decoder's cursor.

Encode inverts both asymmetries, and they compound in our favour:

1. **We choose the boundaries.** For `Range` the compressed form is one flat
   byte stream (delay-interleave; no format frames), sliceable at *any* offset,
   so chunking is purely a matter of draining the output buffer when convenient.
   A boundary is never *forced* on us mid-value the way an arriving chunk's is
   forced on the decoder. Every suspension point is one we placed — so a
   **bounded value never suspends mid-encode**, which is most of the type surface
   and something the decoder can promise for no type at all. Containers do
   suspend inside their own value, at the split points they already declare;
   their bytes span chunks by design, which is harmless because `take_ready` may
   cut anywhere already written (below).
2. **There is no handoff, and no state to hand.** The async encoder holds one
   continuous `RangeEncoder<Writer<BytesMut>>` for the whole encode, and both the
   sync and async paths call methods on that same object. Encoding a bounded
   sub-value synchronously is *literally* `Self::encode(value, enc.sync(), ctx)`
   — one accessor, no construction, no positioning, no closure. Nothing like
   `with_sync` exists here because there is never a second coder.

Consequently the async encode surface is the **inverse** of the decode surface:

| | decode (shipped) | encode (this doc) |
|---|---|---|
| default per type | **async**, suspends at every read | **sync**, appends to the buffer |
| special-cased | sync fast path, for *bounded* types | async, for *unbounded* types |
| coder-state handoff | yes (`with_sync`, `Sync<'a>`) | **none** — one coder, one accessor |
| gate | `sync_capacity(MAX_BYTES) > 0`, at run time | `MAX_BYTES == usize::MAX`, at run time |

## The gate: `MAX_BYTES` at run time

`Encode::encode_async` is a **provided default** that consults `MAX_BYTES` and,
for a bounded type, runs the existing sync encoder against the shared coder
without awaiting anything:

```rust
/// Encode a value with this strategy — the method callers should use.
///
/// Runs the whole value through the *sync* encoder whenever it is bounded,
/// and only recurses into `encode_awaiting` when it is not. Being the default
/// rather than something each call site opens by hand is the point: a site
/// that forgot would still be correct, just permanently slow.
#[inline]
fn encode_async<E: AsyncEntropyCoder>(
    value: &Self,
    enc: &mut E,
    ctx: &mut Self::Context,
) -> impl std::future::Future<Output = std::io::Result<()>> {
    async {
        if Self::MAX_BYTES != usize::MAX {
            Self::encode(value, enc.sync(), ctx);
            Ok(())
        } else {
            Self::encode_awaiting(value, enc, ctx).await
        }
    }
}
```

`Self::MAX_BYTES` is an associated const, so the comparison folds away per
monomorphization and one arm of the `if` is dead code in every instantiation.
`enc.sync()` appears exactly once in the library — here — so no codec body ever
names it.

**The gate is `== usize::MAX`, with no size threshold.** Revision 3 had a
threshold `L` so that a *bounded but large* type (`[u8; 1_000_000]`) would drain
mid-value. That is now not merely unnecessary but **forbidden**, and the reason
is PR #54's rule for `split_point`, whose second half is as load-bearing as its
first: a bounded impl **must not** declare a split point, because a bounded value
that cannot straddle a chunk boundary is exactly what lets `Ans`'s async
*decoder* hand a whole value to the sync decoder mid-stream. A big fixed array is
deliberately atomic. So it buffers `O(N)`, the async encoder can do nothing about
it, and this is not an asymmetry: the decoder has the same limitation on the same
types, since `sync_capacity(1_000_000)` answers 0 until the whole million bytes
have arrived.

Three things revision 3 spent prose on fall out of this for free:

- **Transparent wrappers need no rule.** `Option<T>`'s bound is
  `bool::MAX_BYTES.saturating_add(T::MAX_BYTES)` (`option.rs:57`), and the
  saturation means `Option<Vec<Item>>::MAX_BYTES == usize::MAX`. It takes the
  async path because it *is* unbounded, not because someone remembered to write
  a forwarding impl. Same for `Box`, `Result`, tuples, and every derived struct
  with an unbounded field — the derive already sums with `saturating_add`
  (`compactly-derive/src/v2.rs:440`).
- **There is no bounded-memory test for wrapper forwarding**, because there is
  no forwarding to get wrong.
- **`L` does not need tuning**, because it does not exist.

`encode_awaiting` is required with no default, mirroring `decode_awaiting`, so an
omitted async body is a compile error rather than a silent `O(value)` buffer.

## The drain schedule is already written: `split_point`

PR #54 added [`EntropyCoder::split_point`](../src/v2/mod.rs) — a sync, no-argument
hook meaning *"a chunking coder may end a chunk here, because no bounded value is
partly encoded"* — under one rule:

> An impl whose `MAX_BYTES` is `usize::MAX` must call this between the parts it
> encodes. A **bounded** impl must not.

Three call sites cover the entire type surface:

| site | covers |
|---|---|
| `Sentinel::encode` (`sentinel.rs:154`) | every length-driven loop — `Vec`, `BTreeMap`/`BTreeSet` and the hash variants, `Sorted`, `LowCardinality`, `String`, `Compressible`, `Arc<str>` |
| `low_cardinality.rs:297` | the `encode_miss` char loop, which carries no `Sentinel` |
| `vecs.rs:414` | `Vec<u8>` under `Incompressible`, one call per `INCOMPRESSIBLE_PIECE` |

and `every_unbounded_type_offers_split_points` asserts the coverage is complete
rather than leaving it to inspection. (Those three line numbers, and every
`split_point` reference below, are from PR #54's branch — the hook does not exist
on `main` yet.)

**These are exactly the async encoder's drain points, and it must not invent
others.** Two schedules for one thing would drift, and the rule already says
what a correct one is. Concretely, wherever a codec's sync body calls
`writer.split_point()`, its `encode_awaiting` body awaits `enc.split().await?`
in the same place.

That does not make the async bodies free — a `Vec<T>`'s `encode_awaiting` is
still a separate loop from its `encode` — but it means the *design* question
"where does it drain?" is closed, and the list of impls that need an async body
is precisely the list of impls that call `split_point`, transitively. On the
decode side that list is centralized in one shared helper,
`sentinel::decode_elements`, used by 13 call sites; the encode mirror is an
`encode_elements` helper with the same reach.

## The traits

```rust
/// The async twin of `EntropyCoder`, for producing bytes into a sink that can
/// apply backpressure rather than into a buffer that grows.
///
/// Deliberately **not** a subtrait of `EntropyCoder`: `finish` and `new` would
/// come along, and neither means anything here (the tail must reach the sink
/// asynchronously, and construction needs the sink). The accessor below is the
/// whole of the relationship, and it is enough — encoding, unlike decoding,
/// never has to wait for anything, so a bounded value needs no handoff to run
/// synchronously, only a `&mut`.
///
/// Written desugared, as `fn … -> impl Future`, for the same reason
/// `AsyncEntropyDecoder`'s methods are: `async fn` in a public trait trips
/// `async_fn_in_trait`, and this crate builds warning-free. No `+ Send` — that
/// would propagate to `T` forever; auto traits leak through the opaque return
/// type anyway.
pub trait AsyncEntropyCoder {
    /// The continuous coder underneath. One object for the whole encode.
    type Coder: EntropyCoder;

    /// The coder, for encoding a bounded value synchronously. Called from
    /// exactly one place — `Encode::encode_async`'s provided default.
    fn sync(&mut self) -> &mut Self::Coder;

    /// The async twin of `EntropyCoder::split_point`: declare that a chunk may
    /// end here, and let whatever became emittable reach the sink.
    ///
    /// Call it wherever the sync body calls `split_point`, and nowhere else.
    fn split(&mut self) -> impl std::future::Future<Output = std::io::Result<()>>;

    /// Flush the coder's tail to the sink. Does *not* finish the sink — see
    /// `ChunkSink`.
    fn finish(self) -> impl std::future::Future<Output = std::io::Result<()>>;
}
```

and on `Encode`, beside `decode_awaiting`/`decode_async`:

```rust
    /// Encode a value with this strategy into a sink that may need to be
    /// awaited; the async twin of `Self::encode`.
    ///
    /// **Implement this, but call `Self::encode_async`**, which wraps it with
    /// the bounded fast path.
    ///
    /// Must reach `E::split` wherever the sync `Self::encode` reaches
    /// `EntropyCoder::split_point`; a body that does not is correct but
    /// buffers without bound.
    fn encode_awaiting<E: AsyncEntropyCoder>(
        value: &Self,
        enc: &mut E,
        ctx: &mut Self::Context,
    ) -> impl std::future::Future<Output = std::io::Result<()>>;
```

`Range`'s implementation is the coder plus the sink plus one knob:

```rust
pub struct AsyncRangeEncoder<'a, S> {
    /// `Writer<BytesMut>` is `bytes`' own `io::Write` adapter
    /// (`bytes-1.12.1/src/buf/writer.rs:77`, `get_mut` at :52), so
    /// `RangeEncoder` needs no generalisation at all — and, being an in-memory
    /// buffer, it cannot fail, which is why there is no latched error here.
    coder: RangeEncoder<bytes::buf::Writer<bytes::BytesMut>>,
    sink: &'a mut S,
    chunk_target: usize,           // e.g. 64 KiB
}

impl<S: ChunkSink> AsyncEntropyCoder for AsyncRangeEncoder<'_, S> {
    type Coder = RangeEncoder<bytes::buf::Writer<bytes::BytesMut>>;

    fn sync(&mut self) -> &mut Self::Coder { &mut self.coder }

    /// One length compare per split point; awaits only about once per
    /// `chunk_target`.
    async fn split(&mut self) -> std::io::Result<()> {
        if self.coder.buffered() >= self.chunk_target {
            let chunk = self.coder.take_ready();  // split().freeze() — zero copy
            self.sink.put(chunk).await?;          // propagates; nothing latched
        }
        Ok(())
    }

    async fn finish(mut self) -> std::io::Result<()> {
        self.coder.finish_into_buffer();          // last byte + withheld tail runs
        let rest = self.coder.take_ready();
        if !rest.is_empty() { self.sink.put(rest).await?; }
        Ok(())
    }
}
```

**`take_ready` can cut anywhere already written**, which is the invariant
`Range`'s half rests on, and it holds for two independent reasons. First, the
coder is **carry-free**: `ArithState::ready_bytes` (`arith.rs:29`) emits a byte
only once `lo` and `hi` agree in it, so a settled byte is never revised. (This is
a property of the arithmetic, *not* of `W_DELAY = 8`, which is the decoder's
u64-window delay — the two are unrelated and an early draft conflated them.)
Second, `push_entropy` (`arith.rs:465`) writes each settled byte and splices
`withheld[slot]` immediately after it, so the splice never reaches backwards into
bytes already handed out. "Everything written so far" is therefore always a safe
cut. `Ans` needs no equivalent argument; see its plan.

## The `RangeEncoder<W>` API to add

Today's `RangeEncoder<W: Write>` (`src/v2/arith.rs:423`) writes settled bytes
straight to `W`, holds `withheld: [Vec<u8>; W_DELAY]` for the splice, and latches
an `io::Error`. Over `Writer<BytesMut>` it needs three `pub(crate)` additions,
and no change to the arithmetic, the splice, or the tail:

- `fn buffered(&self) -> usize` — bytes in the sink not yet drained. Excludes
  still-`withheld` runs, which are not splice-eligible and so are invisible here
  — see the limitation below.
- `fn take_ready(&mut self) -> Bytes` — `get_mut().split().freeze()`.
- `fn finish_into_buffer(&mut self)` — the body of today's `finish`
  (`arith.rs:501`: append `last_byte`, flush remaining `withheld` runs) leaving
  bytes in the buffer rather than returning `W`.

### Limitation: a large `Incompressible` run is still not streamable

`encode_incompressible_bytes` (`arith.rs:544`) copies the run into
`withheld[slot]`, where it waits for the `W_DELAY` splice. Those bytes are not in
the sink, `buffered()` does not see them, and `split()` never emits them.

Revision 3 said this and was right; what is new is that the code now says
*precisely* why chopping the run does not help. `Vec<u8>` under `Incompressible`
**does** split its run into `INCOMPRESSIBLE_PIECE` (64 KiB) pieces and **does**
declare a split point at each one (`vecs.rs:414`) — and it still does not help
`Range`, because, as `incompressible_pieces` documents at `vecs.rs:389`, "`Range`
appends each piece to the same withheld slot (no entropy is written between
pieces, so the slot cannot advance)". The pieces exist for the decoder's
allocation cap and for `Ans`; only `push_entropy` releases a withheld run, and
between two pieces nothing calls it.

The escape sketched in `streaming-io-api.md:174-181` — flush the coder,
`memcpy` the run through, re-init — stays **rejected**: it is a format change
touching both sides, so it cannot ride inside an additive async API, and
flushing the coder mid-stream costs compactness.

So the honest bound, and the one the tests assert, is:

> peak buffered ≈ `chunk_target` + the largest single `Incompressible` run +
> the largest *bounded* value in flight.

For the common shapes — a `Vec<f64>`'s 8-byte raw tiers, string bytes, Lz77
literals — runs are small and interspersed with entropy, so this is
indistinguishable from `chunk_target`. It degrades for one value carrying one
enormous incompressible blob, and for a bounded-but-large type, which the gate
section explains is deliberate. Document all of it on the public entry points.

## The async boundary: one sink trait, two front ends

Mirror the decoder's `ChunkSource`. The sink is the minimal dual:

```rust
/// The push dual of the decoder's `ChunkSource`. One method. Finishing the
/// sink is the *caller's* business, not ours.
pub trait ChunkSink {
    fn put(&mut self, chunk: Bytes) -> impl Future<Output = std::io::Result<()>>;
}
```

**Why there is no `finish`.** Real sinks return a completion value:
`object_store::WriteMultipart::finish()` yields a `PutResult`, an S3 multipart
completion an ETag. A `finish` returning `io::Result<()>` has nowhere to put
that, and `&mut self` additionally leaves "put after finish" representable. So we
do not own the sink's lifecycle: the encoder borrows it, pushes every chunk
including the tail, and returns.

```rust
let mut w = WriteMultipart::new(upload);
Range::encode_to_sink(&value, &mut w).await?;   // all chunks pushed, tail included
let result: PutResult = w.finish().await?;      // caller's type, caller's call
```

The alternative — `fn finish(self) -> impl Future<Output = io::Result<Self::Output>>`
threaded out through `encode_to_sink` — reaches the same place with an extra
associated type and a consuming trait method. Not worth it when borrowing gives
the caller more access, not less.

### Front end 1 — a `Stream<Bytes>` we hand out (no tokio)

Named **`Range::encode_stream`**, mirroring `Range::decode_stream`: the
`*_stream` suffix means "the `Stream`-shaped end of the async API" on both
sides. The push form is `Range::encode_to_sink`, mirroring the sync `encode_to`.

This is the ecosystem-native upload shape (`aws_sdk_s3`'s
`ByteStream::from_stream`, `reqwest::Body::wrap_stream`, `http-body`).
`streaming-io-api.md` feared it needed a coroutine or a spawned task; it does
**not**, once the traversal is async — it is the `async-stream` / `genawaiter`
self-driven generator, inlined over `futures-core`:

```rust
/// A ChunkSink that turns the encode future into a Stream: park a chunk, then
/// suspend exactly once. The *consumer's* poll_next is the resume signal, so
/// there is no executor, no thread, no channel — and perfect backpressure
/// (exactly one chunk produced per poll).
///
/// `Arc<Mutex<..>>`, not `Rc<RefCell<..>>`: see below. The mutex is uncontended
/// by construction (one task, guard never held across a suspension point) — it
/// carries `Send + Sync`, it does not synchronize.
struct YieldSink(Arc<Mutex<Option<Bytes>>>);

impl ChunkSink for YieldSink {
    async fn put(&mut self, chunk: Bytes) -> std::io::Result<()> {
        // Scoped so the guard drops *before* the await. A MutexGuard held
        // across a suspension point is not Send, which would make the whole
        // encode future !Send again — the bug this Arc exists to fix.
        { *self.0.lock().unwrap() = Some(chunk); }
        YieldOnce::default().await;  // Pending on first poll, Ready on the next
        Ok(())
    }
}

pub fn encode_stream<T: Encode>(value: T) -> impl Stream<Item = std::io::Result<Bytes>> {
    // struct holding Pin<Box<encode future>> + the Arc<Mutex<Option<Bytes>>>.
    // poll_next: poll the future.
    //   Pending          => a chunk was parked -> Ready(Some(Ok(take()))).
    //   Ready(Ok(()))    => final flush already parked -> then Ready(None).
    //   Ready(Err(e))    => Ready(Some(Err(e))), then None.
}
```

**The `Send`/`Sync` trap, which the first draft walked into.** That draft used
`Rc<RefCell<Option<Bytes>>>` and claimed the only `Send + 'static` bound was the
consumer's. Both halves were wrong: `Rc` makes the returned stream
unconditionally `!Send + !Sync` **regardless of `T`**, and both APIs this front
end exists to serve — `ByteStream::from_stream` and `Body::wrap_stream` — require
`Stream + Send + Sync + 'static`. It would not have compiled at either motivating
call site. (This is why `async-stream` parks its value in a thread-local rather
than an `Rc`.)

`Arc<Mutex<_>>` fixes it. But `Send`-ness of the *stream* also needs the encode
future to be `Send`, hence `T: Send`, every `Context` `Send` (they are plain
data), and no non-`Send` temporary alive across an await — none of which is
visible in a signature, and all of which a later edit to any `encode_awaiting`
can break silently. So the plan requires a **compile-time assertion in the same
commit as the front end**:

```rust
// Takes a *value*, because `encode_stream` returns an opaque `impl Stream`:
// there is no type name to pass as a parameter. Never called; naming it in a
// `const _` block is enough to typecheck it.
const _: () = {
    fn assert_send_sync<S: Stream<Item = std::io::Result<Bytes>> + Send + Sync + 'static>(_: S) {}
    fn check() { assert_send_sync(encode_stream(Vec::<String>::new())); }
};
```

plus a test that actually hands the stream to a `Send + Sync + 'static` bound.

The alternative — a named `pub struct EncodeStream<T>` the assertion could take
as a type parameter — needs the future boxed as `dyn Future + Send`, making
`Send` unconditional rather than leaked through from `T`. Keep `impl Stream`
until a caller needs the name.

Correctness of "every `Pending` is a parked chunk": here the sink is `YieldSink`
and the traversal awaits nothing else (pure CPU plus `put`), so a `Pending` out
of the future is always a `put`. Check the cell anyway (`Pending` + empty cell ⇒
propagate `Pending`), so the same front end still works if a genuinely-awaiting
sink is ever composed.

**Cancellation** is a real path, not a corner: the consumer may drop the stream
at any poll, dropping a `Pin<Box<Future>>` holding the value, every live
`Context`, and the coder mid-traversal. `tests/cancel.rs` already has the
`stats_alloc` harness for the decode side; encode gets the mirror.

### Front end 2 — push into a caller's sink

`impl ChunkSink for` `futures::io::AsyncWrite` (write-all + flush) and for
`object_store::WriteMultipart` (its `write`/`put` are *sync* and non-blocking;
backpressure via an occasional `wait_for_capacity().await`). That covers
S3 / GCS / Azure through `object_store` with no spawn. Then:

```rust
pub async fn encode_to_sink<T: Encode, S: ChunkSink>(
    value: &T,
    sink: &mut S,
) -> std::io::Result<()>;
```

drives `encode_async` into that sink and runs `AsyncEntropyCoder::finish`.

Both front ends are the **same** traversal and the same coder; only the
`ChunkSink` differs. Feature-gate under the existing `stream` feature. Optional
thin adapters (a `tokio::io::AsyncWrite` sink, examples wiring `aws-sdk-s3`
multipart / `reqwest`) can live behind an extra `tokio` feature so the core stays
runtime-neutral.

## Error model

**Propagate; do not latch.** An early draft copied the sync coder's
latch-and-no-op model. That is carryover, and here it is wrong on both halves:

- The sync coder latches because `push_entropy` is reached from `encode_bits`,
  which returns `()` and cannot propagate. That constraint is gone: the only
  fallible operation is `sink.put`, already awaited inside a function returning
  `io::Result<()>`, so `?` works at the one site that needs it.
- The coder itself is **infallible** here. Its sink is `Writer<BytesMut>`, an
  in-memory buffer whose `write_all` cannot fail, so `RangeEncoder`'s latched
  error is permanently `None` on this path.

Latching would mean encoding a multi-gigabyte value into a buffer whose upload
has already failed and reporting it at the end: maximum CPU for zero benefit. A
failed `put` returns immediately, unwinding the traversal, and reaches the caller
from `encode_to_sink` or as the final `Stream` item. The hot path keeps no error
branch, which is also cheaper than the latch it replaces.

## Correctness surface (must be tested)

- **Byte-identical to sync.** For every corpus and both front ends, the async
  encoder must produce **exactly** the bytes `Range::encode` / `encode_to`
  produce — property-tested across random values, and round-tripped through the
  sync **and** async decoders (all four encode×decode combinations agree).
- **Chunk-boundary invariance.** Output is independent of `chunk_target` and of
  where `split` fires: drive the same value at 1, 2, 7, 64 KiB, `usize::MAX`; all
  identical. The encode analog of the decoder's chunk-splitting tests.
- **Backpressure / bounded memory.** With a slow sink, peak buffered bytes stays
  ~`chunk_target`, not `O(value)`. Assert against a counting sink over a large
  `Vec`. Include `Vec<Option<Vec<Item>>>` and `Vec<(u8, Vec<Item>)>` — not
  because a wrapper might stop forwarding (nothing forwards any more) but
  because they are where a wrong `MAX_BYTES` saturation would show.
- **The limitations are asserted, not aspirational.** A value with one large
  `Incompressible` run *is* expected to buffer `O(run)`, and a `[u8; N]` with
  large `N` `O(N)`; write both as tests that pin the actual peak, so the bounds
  stay documented and any future change to them is deliberate.
- **`encode_awaiting` reaches a split point wherever `encode` does.** The
  encode-side twin of #54's `every_unbounded_type_offers_split_points`: run each
  unbounded fixture through a sink that counts `put`s and assert it is asked more
  than once. Without it, an async body that silently omits its drain is
  byte-correct and passes every other test here.
- **`Send + Sync + 'static` on the `Stream` front end**, as the `const _`
  assertion and as a test passing the stream to a function bounded that way.
- **Cancellation.** Drop the stream mid-encode at several poll counts; assert no
  leak, mirroring `tests/cancel.rs`.
- **Edge cases:** empty value; a value whose whole output is < `chunk_target`
  (one chunk, at `finish`); the delay-interleave tail landing exactly on a
  boundary; a `put` that errors (propagates immediately, no further `put`).

## Sequencing

1. **Expose the buffer API on `RangeEncoder`** (`buffered` / `take_ready` /
   `finish_into_buffer` over `Writer<BytesMut>`), unit-tested against the
   existing sync `finish` for byte-identity. No async yet.
2. **`AsyncEntropyCoder` + `ChunkSink` + `AsyncRangeEncoder` +
   `encode_to_sink(value, &mut sink)`**, with `Encode::encode_async`'s provided
   default and hand-written `encode_awaiting` for `Vec` and `String` only.
   Vertical slice: `Vec<u64>` and `Vec<String>` stream byte-identically to sync.
   Settle the `ChunkSink` shape here, before impls exist to churn.
3. **Prototype the `Bytes`-stream front end** (`YieldSink` /
   `Range::encode_stream`) as `src/bin/async-encode-*.rs`, driven by
   `futures_executor::block_on` into a fake S3 sink, to confirm the tokio-free
   `Stream` end to end and **measure the async tax** against sync `encode_to`.
   Land the `Send + Sync + 'static` assertion in this commit. The binary needs
   `required-features = ["stream"]` under its `[[bin]]` entry or CI silently
   skips it.
4. **Fill in the remaining `encode_awaiting` bodies** — an `encode_elements`
   helper mirroring `sentinel::decode_elements`, then maps, sets,
   `Compressible`, `Arc<str>`, `Sorted`, `LowCardinality`, and
   `low_cardinality::encode_miss`'s char loop. The list is exactly the impls
   that reach a `split_point`; nothing else needs a body.
5. **Derive `encode_awaiting`**, mirroring `decode_awaiting` field for field and
   variant for variant (`compactly-derive/src/v2.rs:443`), with the same
   bounds-on-type-parameters plumbing. No `MAX_BYTES` gate in the derive — the
   gate is in `encode_async`'s default, at run time.
6. **Ship both front ends** under `stream`; optional `tokio` adapters and
   S3 / reqwest examples behind a `tokio` feature.

Steps 1 and 3 are `Range`-specific. Steps 2, 4 and 5 are shared with `Ans`, which
is the point of making the traversal generic — see the companion plan.

## Bearing on "should `Range` keep async at all?"

A standing question is whether `Range`'s async support could be dropped if `Ans`
outperforms it, to simplify the code. PR #57's measurements say **no, not on
encode**, and the encode direction is what this plan is about:

- `Range` **encodes faster than `Ans` on most of the corpus**: `enums` +29.6%
  cycles for `Ans`, `floats` +52.1%, `strings` +5.0%, against `Ans` wins of
  −6.5% to −9.9% on `records` / `records-wide` / `atmost128`. `Ans` executes
  *fewer* instructions on almost every encode row and still loses cycles — an IPC
  problem in the two-pass structure, not a work problem.
- The `f64` collapse that dominates the decode argument (+309.6% on `from`) is
  present on encode too but far milder (+52.1%), because encode does not pay the
  gather-the-whole-frame cost.

So the two directions do not point the same way, and "drop `Range` async" is a
decode-side question that should not be answered by an encode-side plan. What
this plan can offer is that the cost of keeping both is now small: with the
traversal generic, `Range` costs steps 1 and 3, and everything else is shared.

## Open questions

- **`chunk_target` default.** An internal knob, invisible in the format. Start
  ~64 KiB; tune with the step-3 bench. (`READY_TARGET`, the decoder's analogue,
  is 256 KiB and chosen against an `Ans` frame, so it is not a precedent.)
- **Reuse `async-stream`/`genawaiter` vs inline `YieldOnce`.** Inlining ~40 lines
  keeps deps to `futures-core`, matching the decode side. `async-stream` would
  also solve the `Send` problem for us (thread-local, not `Rc`) — worth weighing.
- **Encoding from a stream of items.** This API takes `&T`, so it bounds the
  *output*, not the input. A `Stream<Item>`-in / `Stream<Bytes>`-out encoder is
  the shape that would bound peak memory by the value itself. Different API,
  different `Context` lifetime story (the adaptive model must persist across
  items), out of scope — but it is the honest answer to "can I encode something
  bigger than RAM", and worth deciding before 1.0. The `Ans` plan notes that for
  that coder a *synchronous* push/take API would deliver it with no async
  machinery at all.
