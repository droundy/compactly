# Async streaming encode for `Range` (mirror of the async decode PR)

Status: **design (revision 3)**, to be built on top of the async **decode** work
(PR #46, branch `async-decode`), which this branch is based on and **assumes
lands first**. Reuses that branch's `stream` feature, its `bytes` /
`futures-core` dependencies, and the `ChunkSource`/`AsyncRangeDecoder` shape as
the template to mirror. No executor, no tokio; `futures-core` only.

Revision 3 fixes three more review findings: the "no value is ever split across
a chunk" claim was overstated (containers do suspend inside their own value);
the derive cannot gate on `MAX_BYTES` at macro-expansion time, so it emits its
body unconditionally; and the `Send + Sync` assertion could not name an
`impl Stream` return type.

Revision 2 corrects four things review found in the first draft, each noted
inline where it applies: the `Stream` front end's `Rc` made it `!Send` and
unusable at the very call sites it was designed for; the promised mid-run
`Incompressible` drain is not implementable without a format change this library
does not want; transparent wrappers (`Option`, `Box`, tuples) must forward rather
than inherit the sync default; and `ChunkSink::finish` discarded the caller's
completion value. It also drops the copied error-latch, settles the coder-sink
question, and stops overstating the memory win.

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
   chunking is purely a matter of **draining the output buffer** when convenient:
   a boundary is never *forced* on us mid-value the way an arriving chunk's
   boundary is forced on the decoder. Every suspension point is one we placed —
   so a **bounded-small value never suspends mid-encode**, which is most of the
   type surface and something the decoder can promise for no type at all.
   Containers do suspend inside their own value, at the `drain_if_full` they
   place *between* elements; their bytes span chunks by design, which is
   harmless because `take_ready` may cut anywhere already written (below).
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
| gate | `MAX_BYTES` finiteness, checked at run time | none — each impl picks default or override |

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
> collection) *or* bounded but large — *or* it is a transparent wrapper around
> a type that can.** Everything else inherits the sync default and needs no code
> at all.

That last clause is not a footnote; it is where the rule is easy to get wrong.
`Option<T>`, `Box<T>`, `Result<T, E>` and tuples are *structurally* tiny — their
own `MAX_BYTES` contribution is a discriminant or nothing — but their payload is
whatever `T` is. Under the sync default, `Option<Vec<Item>>` buffers the entire
`Vec` before returning, and the bounded-memory test below fails on it.

The fix is cheap because it needs no condition at all: **transparent wrappers
always forward to the inner `encode_async`.** They do not need to inspect
`MAX_BYTES` to decide, because the inner type's *own* default is already the sync
path — forwarding to a bounded-small `T` compiles down to exactly the sync call
it would have made, through an `async fn` that never awaits. The const gate is
therefore free and implicit: the inner type has already made the decision, and
the wrapper just declines to pre-empt it. An explicit
`if T::MAX_BYTES == usize::MAX` branch in the wrapper would reach the same
answer, but it would duplicate a decision that already lives in one place, and
it would be wrong for the *bounded-but-large* case (finite `MAX_BYTES`, above
`L`), which forwarding handles for free.

The cost of forwarding is a generic bound (`Normal: EncodeAsync<T>` on the
wrapper impl) — the same predicate plumbing the derive already needs, and
bounded on the type *parameters* rather than the field types, as the decode
derive settled in `dee3bbf`.

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
        enc: &mut AsyncRangeEncoder<'_, S>,
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
        v: &Vec<T>, enc: &mut AsyncRangeEncoder<'_, S>, ctx: &mut VecContext<T>,
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
pub struct AsyncRangeEncoder<'a, S> {
    /// One continuous coder. `Writer<BytesMut>` is `bytes`' own `io::Write`
    /// adapter, so `RangeEncoder` needs no generalisation — and, being an
    /// in-memory buffer, it cannot fail, which is why there is no latched
    /// error field here (see Error model).
    coder: RangeEncoder<bytes::buf::Writer<BytesMut>>,
    sink: &'a mut S,
    chunk_target: usize,           // e.g. 64 KiB
}

impl<S: ChunkSink> AsyncRangeEncoder<'_, S> {
    /// Drain the ready front of the buffer as one chunk, if it has reached the
    /// target. The only `.await` a container override adds per element.
    async fn drain_if_full(&mut self) -> std::io::Result<()> {
        if self.coder.buffered() >= self.chunk_target {
            let chunk = self.coder.take_ready(); // split().freeze() — zero copy
            self.sink.put(chunk).await?;         // propagates; nothing latched
        }
        Ok(())
    }

    /// Flush the coder's tail + any remaining buffered bytes as final chunk(s).
    /// Does *not* finish the sink — that is the caller's, so they keep whatever
    /// completion value their sink returns.
    async fn finish(mut self) -> std::io::Result<()> {
        self.coder.finish_into_buffer();          // last byte + delay-interleave tail runs
        let rest = self.coder.take_ready();
        if !rest.is_empty() { self.sink.put(rest).await?; }
        Ok(())
    }
}
```

Notes:

- **`drain_if_full` between elements is the whole per-element tax:** one length
  compare, awaiting only ~once per `chunk_target`. How that compares to the
  decoder's async tax is a question for the step-3 measurement, not an
  assumption: the `Range` async *decoder* has no per-element drain (`drain_ready`
  is called only from `fill`, `take_if_single_chunk`, and the `Ans` frame loop)
  and its per-byte suspension points are bypassed by
  `sync_decode_if_there_is_room` in the common case. Measure it; do not assert
  it.
- **No `MAX_BYTES` gate anywhere.** Unlike decode, neither the runtime path nor
  the derive consults it: which types override is a per-impl decision fixed in
  the source (below).
- **The tail** (delay-interleave withheld runs + the coder's final settled byte)
  is produced by the same `RangeEncoder::finish` logic the sync path already has;
  it just lands in the buffer and drains as ordinary chunks.
- **`take_ready` can cut anywhere already written**, which is the invariant the
  whole design rests on, and it holds for two independent reasons. First, the
  coder is **carry-free**: `ArithState::ready_bytes` emits a byte only once
  `lo` and `hi` agree in that byte, so a settled byte is never revised
  afterwards. (This is a property of the arithmetic, *not* of `W_DELAY = 8`,
  which is the decoder's u64-window delay — the two are unrelated and an earlier
  draft conflated them.) Second, `push_entropy` writes each settled byte and
  splices `withheld[slot]` immediately after it, so the splice never reaches
  backwards into bytes already handed out. "Everything written so far" is
  therefore always a safe cut.
- **The coder type is concrete on purpose.** `encode_async` names
  `AsyncRangeEncoder<S>` rather than an `AsyncEntropyCoder` trait, which is *not*
  the mirror of `decode_async<D: AsyncEntropyDecoder>`. That is a deliberate
  deferral: `Ans` async encode will not share this shape at all (it chunks
  natively — build a `Vec<Op>` plus the chunk's incompressible bytes, and only
  *then* is there anything to do asynchronously, with no traversal of `T`
  involved), so generalising now would abstract over one implementation and
  guess wrong about the second. Nothing here is stabilised before `Ans` async
  coding exists; at that point the two get compared and possibly one is dropped
  from v2 entirely.

## The `RangeEncoder<W>` API the coder must expose

Today's `RangeEncoder<W: Write>` (async-decode branch, `src/v2/arith.rs`) writes
settled bytes straight to `W` via `push_entropy` / `write_out`, holds
`withheld: [Vec<u8>; W_DELAY]` for the delay-interleave splice, and latches an
`io::Error`. For the async encoder we want the same coder buffering into an
in-memory `BytesMut` and letting the async layer decide when to drain.

An earlier draft left this as an open choice between generalising the sink to an
internal `PushBytes` trait and keeping `W: Write` over a `BytesMut` wrapper.
**It is not open: `bytes` already ships the wrapper.** `BufMut::writer()` returns
`bytes::buf::Writer<BytesMut>`, which implements `io::Write`
(`bytes-1.12/src/buf/writer.rs:77`) and exposes `get_mut() -> &mut BytesMut`. So
the sink type is `Writer<BytesMut>`, `push_entropy` is untouched, and the coder
generalisation is *zero new code*. What remains to add on `RangeEncoder`:

- `fn buffered(&self) -> usize` — bytes sitting in the sink not yet drained
  (excludes still-`withheld` runs, which are not yet splice-eligible, and so are
  invisible here — see the `Incompressible` limitation below).
- `fn take_ready(&mut self) -> Bytes` — `get_mut().split().freeze()` of the
  drained front. Safe at any offset, per the carry-free / splice-ordering
  argument above.
- `fn finish_into_buffer(&mut self)` — the body of today's `finish` (append
  `last_byte`, flush remaining `withheld` runs) but leaving bytes in the
  buffer rather than returning `W`.

These are `pub(crate)` on the existing `RangeEncoder<W: Write>` where `W` happens
to be `Writer<BytesMut>`; the arithmetic, `withheld` handling, `W_DELAY` splice
and tail are **unchanged and shared** with the sync/in-memory coder — only where
the bytes rest differs, exactly as `Range` vs `RangeEncoder<W>` already share one
impl.

## Which types implement `encode_async`

A per-impl decision, made once when the impl is written — not computed by the
derive or checked at run time. The async-decode branch's per-type `MAX_BYTES`
(finite ⇒ bounded) is the guide for making it, and a concrete const a
hand-written bounded-but-large impl can test against `L`.

- **Derive:** emit the field-recursive `encode_async` — each field's own
  `encode_async` in field order — **unconditionally**, one codegen path for
  every derived type. Do *not* try to skip it for all-bounded-small structs:
  `MAX_BYTES` is an associated const, resolved for a generic field type only at
  monomorphization, and a proc-macro sees syntax. Nothing is lost, for the same
  reason transparent wrappers forward unconditionally (above) — a field that
  inherits the sync default is reached through an `async fn` that never awaits,
  i.e. exactly the sync call an inlined version would have made. The const gate
  stays where it already lives, in each field type's own default-vs-override
  choice. (This is *not* a mirror of the decode derive's `MAX_BYTES` gate, which
  is a **runtime** check inside `decode_async`'s provided default,
  `sync_decode_if_there_is_room`; that derive likewise always emits
  `decode_awaiting`. What carries over is only `decode_variants_async`'s shape
  and the generic-bound plumbing — `Normal: EncodeAsync<#t>` predicates, the
  recursion base case.)
- **Hand-written overrides** — the container/large strategies, ~a dozen:
  `Vec<T>`/slices, `String` + byte blobs (`Vec<u8>`), `BTreeMap`/`BTreeSet`
  (+ hash variants), `Compressible` (Lz77), `Arc<str>` dictionary encoding,
  `Sorted`, `LowCardinality`, and the **large bounded** case of big fixed arrays
  `[T; N]`. (`Incompressible` is *not* in this list — see the limitation below.)
- **Transparent wrappers** — `Option<T>`, `Box<T>`, `Result<T, E>`, tuples,
  `NonZero`-style newtypes: **always forward** to the inner `encode_async`, per
  the rule above. Mechanical, one line each, no condition.
- **Everything else** — scalars, floats, `bool`, `AtMost`, enums of scalars,
  small structs whose fields are all bounded-small — **inherits the sync default
  untouched.** This is the bulk of the type surface and it costs nothing.

Threshold `L` for "bounded but large": start at a few KiB, tune later; it only
affects *when* a bounded value bothers to drain, never correctness.

### Limitation: a single large `Incompressible` run is not streamable

An earlier draft listed `Incompressible` among the overrides, promising it would
"drain mid-run so a 1 GB blob streams". **That is not implementable, and the plan
no longer claims it.**

`encode_incompressible_bytes` (`src/v2/arith.rs:535`) copies the whole run into
`withheld[slot]`, where it waits for the `W_DELAY` splice. Those bytes are not in
the sink, so `buffered()` does not see them and `drain_if_full` never fires for
them. Emitting them earlier is not a matter of trying harder: the splice point is
where the *decoder* expects the run, so moving it changes the byte order, which
the byte-identity requirement below forbids outright.

The only escape is the `T`-threshold flush-fallback sketched in
`streaming-io-api.md:174-181` — flush the coder, `memcpy` the run straight
through, re-init after — and that is **rejected here**, on two grounds. It is a
format change touching both encoder and decoder, so it cannot ride along inside
an additive async API; and flushing the entropy coder mid-stream costs
compactness, which is the wrong trade for this library even where it is possible.

So the honest memory bound, and the one the tests below assert, is:

> peak buffered ≈ `chunk_target` + `L` + **the largest single `Incompressible`
> run in the value**.

For the common shapes — a `Vec<f64>`'s 8-byte raw tiers, string bytes, Lz77
literals — runs are small and interspersed with entropy, so this is
indistinguishable from `chunk_target + L`. It degrades only for one value
carrying one enormous incompressible blob, which is exactly the case that stays
`O(run)`. Document it on the public entry points; do not paper over it.

## The async boundary: one sink trait, two front ends

Mirror the decoder's `ChunkSource`. The sink is the minimal dual:

```rust
/// The push dual of the decoder's `ChunkSource`. `async fn` desugared to
/// `-> impl Future` for the same reason `AsyncEntropyDecoder`'s methods are:
/// keep the public trait warning-free and avoid forcing a `Send` bound onto T.
///
/// One method. Finishing the sink is the *caller's* business, not ours.
pub trait ChunkSink {
    fn put(&mut self, chunk: Bytes) -> impl Future<Output = std::io::Result<()>>;
}
```

**Why there is no `finish` on the trait.** An earlier draft had
`fn finish(&mut self) -> impl Future<Output = io::Result<()>>`, and it threw away
the thing the caller came for. Real sinks return a completion value:
`object_store::WriteMultipart::finish()` yields a `PutResult`, an S3 multipart
completion yields an ETag. A `finish` returning `io::Result<()>` has nowhere to
put that, and `&mut self` additionally leaves "put after finish" representable in
the type system.

The fix that keeps the trait at one method is to **not own the sink's lifecycle**.
The encoder borrows the sink, pushes every chunk including the tail, and returns;
the caller then finishes their own sink and gets their own result type back:

```rust
let mut w = WriteMultipart::new(upload);
Range::encode_to_sink(&value, &mut w).await?;   // all chunks pushed, tail included
let result: PutResult = w.finish().await?;      // caller's type, caller's call
```

The alternative — `fn finish(self) -> impl Future<Output = io::Result<Self::Output>>`
with an associated `Output` threaded out through `encode_to_sink` — reaches the
same place with an extra associated type and a consuming method on the trait.
Not worth it when borrowing gives the caller *more* access, not less. Settle this
in step 2, before any impls exist.

### Front end 1 — a `Stream<Bytes>` we hand out (no tokio)

Named **`Range::encode_stream`**, to mirror the existing
`Range::decode_stream(stream)` on the async-decode branch: the `*_stream` name
means "the `Stream`-shaped end of the async API" on both sides. (An earlier draft
called this `encode_to_byte_stream` and gave `encode_stream` to the push sink,
which made the same suffix mean opposite things in the two directions.) The push
form is `Range::encode_to_sink`, mirroring the sync `encode_to`.

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
///
/// `Arc<Mutex<..>>`, not `Rc<RefCell<..>>`: see below. The mutex is
/// uncontended by construction (one task, and the guard is never held across a
/// suspension point) — it is here to carry `Send + Sync`, not to synchronize.
struct YieldSink(Arc<Mutex<Option<Bytes>>>);
impl ChunkSink for YieldSink {
    async fn put(&mut self, chunk: Bytes) -> std::io::Result<()> {
        // Scoped so the guard is dropped *before* the await. A MutexGuard held
        // across a suspension point is not Send, and would make the whole
        // encode future !Send again — reintroducing the bug this Arc fixes.
        {
            *self.0.lock().unwrap() = Some(chunk);
        }
        YieldOnce::default().await;  // Pending on first poll, Ready on the next
        Ok(())
    }
}

pub fn encode_stream<T>(value: T) -> impl Stream<Item = std::io::Result<Bytes>>
where /* Normal: EncodeAsync<T>, T owned */ {
    // struct holding Pin<Box<encode future>> + the Arc<Mutex<Option<Bytes>>>.
    // poll_next: poll the future.
    //   Pending          => a chunk was parked -> Ready(Some(Ok(take()))).
    //   Ready(Ok(()))    => final flush already parked -> then Ready(None).
    //   Ready(Err(e))    => Ready(Some(Err(e))), then None.
}
```

**The `Send`/`Sync` trap, which the first draft walked straight into.** That
draft used `Rc<RefCell<Option<Bytes>>>` and then claimed "no `Send` forced by us
… the only `Send + 'static` bound is the one the consumer imposes, satisfied when
`value: Send + 'static`". Both halves are wrong. `Rc` makes the returned stream
unconditionally `!Send + !Sync` **regardless of `T`**, and the two APIs this
front end exists to serve —
`aws_sdk_s3::primitives::ByteStream::from_stream` and
`reqwest::Body::wrap_stream` — both require `Stream + Send + Sync + 'static`. The
headline front end would not have compiled at either of its motivating call
sites. (This is exactly why `async-stream` parks its value in a thread-local
rather than an `Rc`.)

`Arc<Mutex<_>>` fixes it, and a thread-local would too; take the `Arc`, it is
plainer. But `Send`-ness of the *stream* also requires the encode future to be
`Send`, which requires `T: Send`, every `Context` to be `Send` (they are plain
data), and no guard or non-`Send` temporary alive across an await. None of that
is visible in a signature, and all of it can be broken silently by a later edit
to any strategy's `encode_async`. So the plan requires a **compile-time
assertion**, in the same commit as the front end:

```rust
// Takes a *value*, because `encode_stream` returns an opaque `impl Stream`:
// there is no type name to pass as a parameter. The fn is never called; naming
// it in a `const _` block is enough to typecheck it.
const _: () = {
    fn assert_send_sync<S: Stream<Item = std::io::Result<Bytes>> + Send + Sync + 'static>(_: S) {}
    fn check() { assert_send_sync(encode_stream(Vec::<String>::new())); }
};
```

plus a test that actually hands the stream to a `Send + Sync + 'static` bound, so
the guarantee is checked rather than hoped for.

The alternative — a named `pub struct EncodeStream<T>`, which callers could
spell in a struct field and the assertion could take as a type parameter —
needs the future boxed as `dyn Future + Send`, making `Send` an unconditional
requirement rather than one that leaks through from `T`. Keep `impl Stream`
unless a caller actually needs the name.

Correctness of "every `Pending` is a parked chunk": in this front end the sink is
`YieldSink` and the encode traversal awaits **nothing else** (it is pure CPU plus
`put`), so a `Pending` bubbling out of the future is always a `put`. Keep it
robust anyway by checking the cell (`Pending` + empty cell ⇒ propagate `Pending`)
so the same `EncodeStream` still works if a genuinely-awaiting sink is ever
composed.

- **No executor, no thread, no channel.** Single task, driven entirely by the
  consumer's `poll_next`. The `Send + Sync + 'static` the consumer needs is
  delivered by the `Arc` above plus `T: Send + 'static`, and asserted at compile
  time — not assumed.
- **Chunk granularity is ours** via `chunk_target`; `split().freeze()` is
  zero-copy.
- **Cancellation** is a real path here, not a corner: the consumer can drop the
  stream at any poll, dropping a `Pin<Box<Future>>` that holds the value, every
  live `Context`, and the coder mid-traversal. The decode branch tests this
  (`tests/cancel.rs`, `fd0009f` "Test that cancelling a decode frees
  everything"); encode must too — see the correctness surface.

### Front end 2 — push into a caller's sink

`impl ChunkSink for` `futures::io::AsyncWrite` (write-all + flush) and for
`object_store::WriteMultipart` (its `write`/`put` are *sync* and non-blocking;
backpressure via an occasional `wait_for_capacity().await`). This covers
S3 / GCS / Azure through `object_store` with no spawn. Then:

```rust
pub async fn encode_to_sink<T, S: ChunkSink>(
    value: &T,
    sink: &mut S,
) -> std::io::Result<()>;
```

drives `encode_async` into that sink and runs `AsyncRangeEncoder::finish`, which
pushes the coder tail as ordinary chunks. The sink is **borrowed**: when this
returns, every byte has been `put`, and completing the upload (and collecting
its `PutResult` / ETag) is the caller's next line. See the `ChunkSink` discussion
above for why `finish` is not our method to call.

Both front ends are the **same** `encode_async` + `AsyncRangeEncoder`; only the
`ChunkSink` differs. Feature-gate everything under the existing `stream` feature.
Optional thin adapters (a `tokio::io::AsyncWrite` `ChunkSink`, examples wiring
`aws-sdk-s3` multipart / `reqwest`) can live behind an extra `tokio` feature so
the core stays runtime-neutral.

## Error model

**Propagate; do not latch.** An earlier draft copied the sync coder's
latch-and-no-op model and gave `AsyncRangeEncoder` an
`error: Option<io::Error>` field. That is carryover, and here it is actively
wrong on both halves:

- The sync coder latches because `push_entropy` is called from
  `encode_bits`, which returns `()` and cannot propagate. That constraint is
  gone: the only fallible operation in the async encoder is `sink.put`, which is
  already `await`ed inside an `async fn` returning `io::Result<()>`, so `?`
  works at the one site that needs it.
- The coder itself is now **infallible**. Its sink is `Writer<BytesMut>` — an
  in-memory buffer whose `write_all` cannot fail — so `RangeEncoder`'s latched
  error is permanently `None` on this path and there is nothing to surface at
  `finish`.

Latching would mean continuing to encode a multi-gigabyte value into a buffer
whose upload has already failed, then reporting it at the end: maximum CPU for
zero benefit. So `AsyncRangeEncoder` carries no error field; a failed `put`
returns immediately, unwinding the traversal, and reaches the caller from
`encode_to_sink` or as the final `Stream` item. The hot path keeps no error
branch at all, which is also *cheaper* than the latch it replaces.

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
  ~`chunk_target` + `L`, *not* `O(value)`. Assert against a counting sink over a
  large `Vec`. Include `Vec<Option<Vec<Item>>>` and `Vec<(u8, Vec<Item>)>`
  specifically — the transparent-wrapper cases, which are the ones that silently
  regress to `O(value)` if a wrapper stops forwarding.
- **The `Incompressible` bound is asserted, not aspirational.** A value with one
  large `Incompressible` run *is* expected to buffer `O(run)`; write that as a
  test that pins the actual peak, so the limitation stays documented and any
  future change to it is deliberate.
- **`Send + Sync + 'static` on the `Stream` front end**, both as the `const _`
  assertion and as a test that passes the stream to a function bounded that way.
- **Cancellation.** Drop the stream mid-encode at several poll counts; assert no
  leak (mirror `tests/cancel.rs` from the decode branch, which already has the
  `stats_alloc` harness).
- **Edge cases:** empty value, single tiny value (no drain ever fires — one
  chunk at `finish`), a value whose entire output is < `chunk_target`, the
  delay-interleave tail landing exactly at a boundary, and a `put` that errors
  (propagates immediately; no further `put` is issued).

## Sequencing

1. **Expose the buffer API on `RangeEncoder`** (`buffered` / `take_ready` /
   `finish_into_buffer` over the `Writer<BytesMut>` sink), unit-tested against
   the existing sync `finish` for byte-identity. No async yet.
2. **`AsyncRangeEncoder<S>` + `ChunkSink` + `encode_to_sink(value, &mut sink)`**,
   with the `EncodeAsync` trait's **sync default only** and hand-written
   overrides for `Vec` and `String`. Vertical slice: `Vec<u64>` and `Vec<String>`
   stream byte-identically to sync, tested. (Mirror of the decode PR's first
   vertical slice.) Settle the `ChunkSink` shape here — one method, borrowed
   sink — before any impl exists to churn.
3. **Prototype the `Bytes`-stream front end** (`YieldSink` /
   `Range::encode_stream`) as a `src/bin/async-encode-*.rs`, driven by
   `futures_executor::block_on` into a fake S3 sink, to confirm the tokio-free
   `Stream` end-to-end and **measure the async tax** vs sync `encode_to` (as
   PR #46 measured decode). Land the `Send + Sync + 'static` assertion in this
   commit, not later. The new binary needs
   `required-features = ["stream"]` under its `[[bin]]` entry, or CI silently
   skips it — `dee3bbf` was just bitten by exactly this.
4. **Fill in the remaining container overrides** (maps, sets, `Compressible`,
   `Arc<str>`, `Sorted`, `LowCardinality`), the large-bounded case of big fixed
   arrays, and the **transparent wrappers** (`Option`, `Box`, `Result`, tuples).
   `Incompressible` is *not* in this step; see the limitation above.
5. **Derive `EncodeAsync`**, emitting the field-recursive body unconditionally
   (no `MAX_BYTES` gate — see above); port the decode derive's generic-bound
   plumbing (bounds on type *parameters*, per `dee3bbf`).
6. **Ship both front ends** under `stream`; optional `tokio` adapters + S3 /
   reqwest examples behind a `tokio` feature.

## Open questions

- **`chunk_target` and `L` defaults.** Internal knobs, invisible in the format.
  Start `chunk_target` ~64 KiB, `L` ~ a few KiB; tune with the step-3 bench.
- **Reuse `async-stream`/`genawaiter` vs inline `YieldOnce`.** Inlining ~40
  lines keeps deps to `futures-core` (matches the decode PR). Decide when writing
  the front end. Note that `async-stream` would also solve the `Send` problem for
  us (thread-local, not `Rc`) — a point in its favour worth weighing.
- **Encoding from a stream of items.** The API here takes `&T`, so it bounds the
  *output*, not the input. A `Stream<Item>`-in / `Stream<Bytes>`-out encoder is
  the shape that would bound peak memory by the value itself. Different API,
  different `Context` lifetime story (the adaptive model must persist across
  items), out of scope — but it is the honest answer to "can I encode something
  bigger than RAM", and worth a decision before 1.0.
- **`Ans` async encode** is out of scope here, and is expected to look
  *structurally different* rather than parallel: `Ans` already chunks its output,
  so the first step is building a `Vec<Op>` plus one chunk's incompressible bytes
  entirely synchronously, and only the emission of finished chunks is async —
  with no traversal of `T` in the async part, hence no `EncodeAsync`, no
  `ChunkSink` recursion, and plausibly no contact with this code at all. Nothing
  in this document is stabilised before that exists; once both are built, the
  comparison may retire one of them from v2.
