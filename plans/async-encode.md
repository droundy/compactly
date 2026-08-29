# Async streaming encode

Status: **design**, for both coders. No code yet.

Builds on two things: the async **decode** work (PR #46, merged as `54d2d66`),
whose trait shapes this mirrors and whose `ChunkSource` is the template for the
sink; and `EntropyCoder::split_point` (PR #54), which already places every point
at which an encode may be interrupted. See
[`streaming-io-api.md`](streaming-io-api.md) for the format-level background the
whole streaming effort rests on — delay-interleave, `W_DELAY = 8`, the
deferred-error model.

The plan covers both coders in one piece because they share almost all of it:
one traversal, one sink trait, two front ends, and a per-coder adapter of a few
dozen lines. `Ans` is built first, for reasons given under
[Sequencing](#sequencing).

## The goal

`v2::encode(&T) -> Vec<u8>` holds both the value and its whole compressed output
in memory. The sync `encode_to<W: Write>` already streams to a `Write`. This adds
the **async** streaming encoder: produce the compressed bytes *as a stream of
[`Bytes`] chunks*, so a large value can be uploaded to the network (S3 /
object_store / an HTTP body) without ever holding the whole compressed blob.

Be precise about what that buys, because the obvious claim oversells it. The API
takes `&T`, so the value is *already* wholly in memory; what streaming removes is
the compressed output, which is by construction smaller — usually much smaller —
than the value it came from. Dropping a fraction of an already-committed
footprint is real but modest. The wins that actually justify the API are:

- **Time to first byte.** The upload starts after the first chunk, not after the
  whole encode.
- **Backpressure.** A slow sink stops the encode, rather than the encode racing
  ahead into a growing `Vec`.
- **Ecosystem fit.** `ByteStream::from_stream` / `Body::wrap_stream` /
  `http-body` want a `Stream<Bytes>` and there is no allocation-free way to hand
  them one today.

Note what is *not* on that list: overlapping the encode with the transmission.
For the usual kernel-buffered sink that happens anyway, and for the case where it
does not, it needs a deliberate addition — see
[Does encoding overlap sending?](#does-encoding-overlap-sending).

(Encoding from a *stream of items* rather than a `&T` is the design that would
genuinely bound peak memory by the value; it is a different API and out of scope
here — see [Open questions](#open-questions).)

The headline surface is a chunk **stream we hand out**, since that is what the
upload ecosystem pulls from; and a chunk **sink we push into**, for
`object_store`'s `WriteMultipart` and `futures`/`tokio` `AsyncWrite`. One
mechanism underlies both.

## Why the encoder is smaller than the decoder

The decode side had to make **every** type async, because the input arrives in
chunks whose boundaries are the transport's choice: any value can straddle a
chunk, so every read point must be able to suspend. It then bolted a *sync fast
path* back on (`sync_capacity` / `with_sync`, gated by each type's `MAX_BYTES`)
to stop paying the async tax once enough was buffered, and paid for a whole
coder-state handoff — a `Decoder` constructed and positioned at the async
decoder's cursor.

Encode inverts both asymmetries, and they compound in our favour:

1. **We choose the boundaries.** A boundary is never *forced* on us mid-value the
   way an arriving chunk's is forced on the decoder. Every suspension point is
   one we placed — so a **bounded value never suspends mid-encode**, which is
   most of the type surface and something the decoder can promise for no type at
   all. Containers do suspend inside their own value, at the split points they
   already declare; their bytes span chunks by design.
2. **There is no handoff, and no state to hand.** The async encoder holds one
   continuous coder for the whole encode, and both the sync and async paths call
   methods on that same object. Encoding a bounded sub-value synchronously is
   *literally* `Self::encode(value, enc.sync(), ctx)` — one accessor, no
   construction, no positioning, no closure. Nothing like `with_sync` exists here
   because there is never a second coder.

So the async encode surface is the **inverse** of the decode surface:

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
names it. `encode_awaiting` is required with no default, mirroring
`decode_awaiting`, so an omitted async body is a compile error rather than a
silent `O(value)` buffer.

**The gate is `== usize::MAX`, with no size threshold.** A threshold `L`, so that
a *bounded but large* type (`[u8; 1_000_000]`) could drain mid-value, is not
merely unnecessary but **forbidden**, by the second half of `split_point`'s rule
(below): a bounded impl must not declare a split point, because a bounded value
that cannot straddle a chunk boundary is exactly what lets `Ans`'s async
*decoder* hand a whole value to the sync decoder mid-stream. A big fixed array is
deliberately atomic. It buffers `O(N)`, the async encoder can do nothing about
it, and this is not an asymmetry: the decoder has the same limitation on the same
types, since `sync_capacity(1_000_000)` answers 0 until the whole million bytes
have arrived.

Two things fall out of the gate for free:

- **Transparent wrappers need no rule.** `Option<T>`'s bound is
  `bool::MAX_BYTES.saturating_add(T::MAX_BYTES)` (`option.rs:57`), and the
  saturation means `Option<Vec<Item>>::MAX_BYTES == usize::MAX`. It takes the
  async path because it *is* unbounded, not because someone remembered to write a
  forwarding impl. Same for `Box`, `Result`, tuples, and every derived struct
  with an unbounded field — the derive already sums with `saturating_add`
  (`compactly-derive/src/v2.rs:440`). There is nothing to forward and therefore
  nothing to get wrong.
- **Nothing needs tuning**, because there is no knob on this side at all.

## The drain schedule: `split_point`

`EntropyCoder::split_point` is a sync, no-argument hook meaning *"a chunking
coder may end a chunk here, because no bounded value is partly encoded"*, under
one rule:

> An impl whose `MAX_BYTES` is `usize::MAX` must call this between the parts it
> encodes. A **bounded** impl must not.

Three call sites cover the entire type surface:

| site | covers |
|---|---|
| `Sentinel::encode` (`sentinel.rs:154`) | every length-driven loop — `Vec`, `BTreeMap`/`BTreeSet` and the hash variants, `Sorted`, `LowCardinality`, `String`, `Compressible`, `Arc<str>` |
| `low_cardinality.rs:297` | the `encode_miss` char loop, which carries no `Sentinel` |
| `vecs.rs:414` | `Vec<u8>` under `Incompressible`, one call per `INCOMPRESSIBLE_PIECE` |

and `every_unbounded_type_offers_split_points` asserts the coverage is complete
rather than leaving it to inspection. (Those line numbers, and every
`split_point` reference below, are from PR #54's branch — the hook does not exist
on `main` yet.)

**These are exactly the async encoder's drain points, and it must not invent
others.** Two schedules for one thing would drift, and the rule already says what
a correct one is. Concretely, wherever a codec's sync body calls
`writer.split_point()`, its `encode_awaiting` body awaits `enc.split().await?` in
the same place.

That does not make the async bodies free — a `Vec<T>`'s `encode_awaiting` is
still a separate loop from its `encode` — but it closes the design question
"where does it drain?", and the list of impls needing an async body is precisely
the list that calls `split_point`, transitively. On the decode side that list is
centralized in one shared helper, `sentinel::decode_elements`, used by 13 call
sites; the encode mirror is an `encode_elements` helper with the same reach.

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

## The coders

Both point their existing streaming encoder at an in-memory `BytesMut` and drain
it at split points. `bytes::buf::Writer<BytesMut>` is `bytes`' own `io::Write`
adapter (`bytes-1.12.1/src/buf/writer.rs:77`, `get_mut` at `:52`), so neither
`RangeEncoder<W>` nor `AnsEncoder<W>` needs any generalisation — and, being an
in-memory buffer, it cannot fail, which is why neither adapter carries a latched
error.

### `Ans`

Three facts about `src/v2/ans.rs` do all the work:

1. **`flush_chunk` emits one complete, self-contained frame** (`ans.rs:266`):
   header varints, then the entropy body and the raw incompressible region, in an
   order the frame tag distinguishes. Nothing about a frame depends on what
   follows it. That is the chunk the sink wants, already formed.
2. **`split_point` is the only place a non-final chunk is flushed.** So "when do
   we cut?" is not an open question: it is `CHUNK_OPS` ops, decided inside the
   coder, and the async layer neither chooses nor influences it.
3. **`write_out` is the single choke point** for every byte the encoder emits
   (`ans.rs:232`), and the only thing that touches `W`.

```rust
pub struct AsyncAnsEncoder<'a, S> {
    coder: AnsEncoder<bytes::buf::Writer<bytes::BytesMut>>,
    sink: &'a mut S,
}

impl<S: ChunkSink> AsyncEntropyCoder for AsyncAnsEncoder<'_, S> {
    type Coder = AnsEncoder<bytes::buf::Writer<bytes::BytesMut>>;

    fn sync(&mut self) -> &mut Self::Coder { &mut self.coder }

    /// The coder decides whether a frame is due; we only carry away whatever it
    /// has finished. A `while let` rather than an `if let` on purpose — see
    /// "Keeping parallel frame encoding open".
    async fn split(&mut self) -> std::io::Result<()> {
        self.coder.split_point();
        while let Some(frame) = self.coder.take_flushed() {
            self.sink.put(frame).await?;
        }
        Ok(())
    }

    async fn finish(mut self) -> std::io::Result<()> {
        self.coder.finish_into_buffer();      // flush_chunk(true) — the final frame
        while let Some(tail) = self.coder.take_flushed() {
            self.sink.put(tail).await?;
        }
        Ok(())
    }
}
```

`take_flushed` is the one thing to add to `AnsEncoder`, and it is three lines:
`get_mut().split().freeze()`, returned as `Some` when non-empty. `AnsEncoder` is
already `pub(crate)` and already generic over `W: Write`, so nothing else moves —
**no changes to `ans.rs`'s coding at all**.

### `Range`

`Range`'s compressed form is one flat byte stream (delay-interleave; no format
frames), sliceable at *any* offset. So chunking is a matter of draining the
output buffer when it has grown enough, which is the one knob on this side:

```rust
pub struct AsyncRangeEncoder<'a, S> {
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
            self.sink.put(chunk).await?;
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
only once `lo` and `hi` agree in it, so a settled byte is never revised. This is
a property of the arithmetic, *not* of `W_DELAY = 8`, which is the decoder's
u64-window delay — the two are unrelated. Second, `push_entropy`
(`arith.rs:465`) writes each settled byte and splices `withheld[slot]`
immediately after it, so the splice never reaches backwards into bytes already
handed out. "Everything written so far" is therefore always a safe cut.

Today's `RangeEncoder<W: Write>` (`arith.rs:423`) needs three `pub(crate)`
additions over `Writer<BytesMut>`, and no change to the arithmetic, the splice,
or the tail:

- `fn buffered(&self) -> usize` — bytes in the sink not yet drained. Excludes
  still-`withheld` runs, which are not splice-eligible and so are invisible here.
- `fn take_ready(&mut self) -> Bytes` — `get_mut().split().freeze()`.
- `fn finish_into_buffer(&mut self)` — the body of today's `finish`
  (`arith.rs:501`: append `last_byte`, flush remaining `withheld` runs) leaving
  bytes in the buffer rather than returning `W`.

### What `Range` needs that `Ans` does not

| `Range` | `Ans` |
|---|---|
| `buffered()` / `take_ready()` / `finish_into_buffer()` on the encoder, plus their byte-identity unit tests | one `take_flushed()`; `finish_into_buffer` is the existing `flush_chunk(true)` |
| the **cut-anywhere invariant** — the carry-free argument plus the splice-ordering argument | nothing to prove. The only cut is a frame boundary, and frames are self-contained by construction |
| a `chunk_target` knob, and a default for it to be tuned | none. Chunk size is `CHUNK_OPS`, already chosen, already in the sync encoder |
| **chunk-boundary invariance** tested across `chunk_target` in 1, 2, 7, 64 KiB, `usize::MAX` | vacuous. There is no knob to vary, so byte-identity with sync encode is not a property to establish but a consequence: `split()` calls the same `split_point` the sync encoder calls, at the same points, and `flush_chunk` is untouched |

Everything else — the traversal, the sink, both front ends, the error model,
cancellation — is shared and mentions neither coder.

## Limitations

Three, all of them structural rather than provisional. State each on the public
entry points, and pin each with a test, so that a future change to any of them is
deliberate.

**Bounded but large is deliberately atomic.** A `[u8; N]` with large `N`, or any
other bounded impl, must not declare a split point — see [the gate](#the-gate-max_bytes-at-run-time)
— so it buffers `O(N)`. The decoder has the same limitation on the same types.

**`Range`: a large `Incompressible` run.** `encode_incompressible_bytes`
(`arith.rs:544`) copies the run into `withheld[slot]`, where it waits for the
`W_DELAY` splice. Those bytes are not in the sink, `buffered()` does not see
them, and `split()` never emits them. Chopping the run does not help:
`Vec<u8>` under `Incompressible` **does** split into `INCOMPRESSIBLE_PIECE`
(64 KiB) pieces and **does** declare a split point at each one (`vecs.rs:414`),
and as `incompressible_pieces` documents at `vecs.rs:389`, "`Range` appends each
piece to the same withheld slot (no entropy is written between pieces, so the
slot cannot advance)". The pieces exist for the decoder's allocation cap and for
`Ans`; only `push_entropy` releases a withheld run, and between two pieces
nothing calls it. The escape sketched in `streaming-io-api.md:174-181` — flush
the coder, `memcpy` the run through, re-init — is **rejected**: it is a format
change touching both sides, so it cannot ride inside an additive async API, and
flushing the entropy coder mid-stream costs compactness.

So for `Range` the honest bound is:

> peak buffered ≈ `chunk_target` + the largest single `Incompressible` run +
> the largest *bounded* value in flight.

For the common shapes — a `Vec<f64>`'s 8-byte raw tiers, string bytes, Lz77
literals — runs are small and interspersed with entropy, so this is
indistinguishable from `chunk_target`.

**`Ans`: the same symptom, a different cause, and a fix it can have.** Each
64 KiB piece is one op, and `split_point` flushes at `CHUNK_OPS` (65536) ops, so
it takes a **4 GiB** value before a flush is due; peak buffering is
`min(run, 4 GiB)`. Unlike `Range`'s, this is fixable *in the coder*: give
`AnsEncoder::split_point` a second trigger on `incompressible_bytes.len()`
alongside its op count. That changes where frames land and so changes the bytes,
which is allowed — per CLAUDE.md, v2 is not frozen and chunk boundaries are named
as fair game, and there is no `v2-encoding` stability test to update. Two
cautions: it changes **sync** encode output too (correct — byte-identity is
between async and sync of the same build — but the `expect-test` size assertions
move for any corpus with runs over the threshold), and it is a compression
question as well as a memory one, since more frames means more per-frame
`STATE_BYTES` flushes. A follow-up with its own measurement, not part of this
work; the async encoder is correct either way and only its bound improves.

## Keeping parallel frame encoding open

`Ans`'s `flush_chunk` reads only `self.ops` and `self.incompressible_bytes`,
builds a fresh `Encoder::new()` per chunk, and **never touches a `BitContext`** —
`encode_bits` resolves each probability at record time and stores it *in* the op
(`Op::Bit(b, probability)`). The entropy pass is therefore a pure function of
`(ops, incompressible_bytes)`, and non-final frames carry their own
`entropy_len`/`raw_len`, so a frame is self-describing. Frames are consequently
**embarrassingly parallel across chunks** — not merely pipelineable against the
traversal — and because a pure function cannot change its output, parallelizing
them is byte-identical for free.

That is an `Ans` *encoder* optimization, not an async one: it applies equally to
the sync `Ans::encode_to`, it is measured and sequenced in OPTIMIZING.md, and
**nothing here depends on it**. Its value is roughly `(r + e) / max(r, e)` for
record and entropy phase costs `r` and `e`, so it grows as the two phases even
out — and both are data-dependent and both are optimization targets, so today's
ratio is a snapshot, not a bound. What this plan owes it is only that the async
design not rule it out. Four things do that, and they cost nothing if it never
happens:

1. **`take_flushed()` returns the next *completed* frame in order, if any** — not
   "the frame this split point produced". So `split()` may get `None` at a split
   point that did flush, and may get several later. Hence the `while let` above
   rather than an `if let`; with one worker or none, the loop runs at most once
   and the shapes are identical.
2. **`finish()` drains the pipeline** before emitting the final frame, which the
   `while let` also covers, plus a join inside `finish_into_buffer`.
3. **The memory bound is stated as `K × (ops buffer + frame)`**, not one frame.
   The ops buffer is `CHUNK_OPS × size_of::<Op>()` ≈ 384 KiB, so the pool must be
   bounded — double-buffering at minimum — or a fast traversal with a slow sink
   races ahead and the bounded-memory claim is lost. State `K = 1` today.
4. **"Frames reach the sink as they are produced" weakens to "within `K` frames
   of being produced"**, which is what the correctness surface below asserts. The
   stronger "one `put` per frame, at the same offsets as sync `encode_to`" is
   unaffected, since identity is preserved.

One thing to *not* accommodate: spawning frame work onto the caller's async
executor. That would reintroduce the runtime dependency this whole design exists
without, and it would make the `Stream` front end incoherent — its backpressure
is defined by the consumer's `poll_next` driving a single task. If parallelism
happens it is a `std::thread` pool behind an optional feature, invisible here.

## The async boundary: one sink trait, two front ends

Mirroring the decoder's `ChunkSource`, the sink is the minimal dual:

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
that, and `&mut self` additionally leaves "put after finish" representable in the
type system. So we do not own the sink's lifecycle: the encoder borrows it,
pushes every chunk including the tail, and returns.

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

Named **`encode_stream`**, mirroring `decode_stream`: the `*_stream` suffix means
"the `Stream`-shaped end of the async API" on both sides. The push form is
`encode_to_sink`, mirroring the sync `encode_to`.

This is the ecosystem-native upload shape (`aws_sdk_s3`'s
`ByteStream::from_stream`, `reqwest::Body::wrap_stream`, `http-body`). It needs
no coroutine and no spawned task, once the traversal is async — it is the
`async-stream` / `genawaiter` self-driven generator, inlined over `futures-core`:

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
        // encode future !Send.
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

**The `Send`/`Sync` requirement is load-bearing and easy to lose.** `Rc` in the
parking cell would make the returned stream unconditionally `!Send + !Sync`
**regardless of `T`**, and both APIs this front end exists to serve —
`ByteStream::from_stream` and `Body::wrap_stream` — require
`Stream + Send + Sync + 'static`. It would not compile at either motivating call
site. (This is why `async-stream` parks its value in a thread-local rather than
an `Rc`.)

`Arc<Mutex<_>>` fixes the cell. But `Send`-ness of the *stream* also needs the
encode future to be `Send`, hence `T: Send`, every `Context` `Send` (they are
plain data), and no non-`Send` temporary alive across an await — none of which is
visible in a signature, and all of which a later edit to any `encode_awaiting`
can break silently. So this plan requires a **compile-time assertion in the same
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

A named `pub struct EncodeStream<T>`, which the assertion could take as a type
parameter and callers could spell in a struct field, would need the future boxed
as `dyn Future + Send`, making `Send` unconditional rather than leaked through
from `T`. Keep `impl Stream` until a caller needs the name.

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

### Does encoding overlap sending?

As written above, `split()` does `sink.put(chunk).await`, so the traversal is
suspended for as long as `put` is. That is worth being precise about, because the
answer is different in three regimes and only one of them is a real gap.

**Our own CPU never overlaps anything, and cannot.** Encoding a chunk is a long
non-yielding stretch of compute; nothing else in the task is polled during it.
That is inherent to single-task async, not a property of this design, and the
only fixes are a thread or a separate task — both of which the runtime-neutrality
constraint rules out here. It is also the *smaller* effect: see the
[parallel-frames](#keeping-parallel-frame-encoding-open) note for where CPU
overlap would actually come from.

**For a kernel-buffered sink, the I/O overlaps already.** A socket `put` calls
`poll_write`, the kernel accepts the bytes into the socket buffer, and `put`
returns `Ready` without ever suspending; transmission then proceeds in the kernel
while we encode the next chunk. `put` suspends only when the socket buffer is
full — which is exactly when backpressure is *supposed* to stop us. So for
`AsyncWrite`-shaped sinks the serial structure loses nothing.
`object_store::WriteMultipart` is deliberately the same shape: its `write` is
sync and non-blocking, with an occasional `wait_for_capacity().await`.

**For a sink whose `put` awaits a round trip, the traversal stalls, and this is
the gap.** A naive one-request-per-chunk uploader would give
encode → stall → encode → stall, with the encoder idle for the whole request.
Fixing it does *not* need an executor: put a bounded queue between the traversal
and the sink, and poll two futures in one task —

```rust
// `split()` pushes into the queue, awaiting only when it is full — so the
// traversal runs ahead by up to K chunks instead of stopping at the first.
// The sender future owns the sink and drains the queue. A hand-rolled join of
// exactly two futures is ~40 lines over `core::future`, no new dependency.
join(traversal(value, &queue), sender(sink, &queue)).await
```

The traversal's `Pending` (queue full) is what gives the sender a chance to be
polled, and during a round trip the traversal is free to fill the queue — so this
converts stall time into encode time, up to `K` chunks' worth. The costs are a
`K`-chunk rise in the memory bound, which must be folded into
[Limitations](#limitations), and a slightly wider cancellation surface, since
dropping the join must drop both halves.

**`encode_stream` delegates the question to its consumer.** `YieldSink` hands
over exactly one chunk per `poll_next` and suspends, so whether chunk `N` is on
the wire while chunk `N+1` is being encoded is the consumer's structure, not
ours. `hyper`/`reqwest` poll the body again after handing the previous frame to
the socket, which is the kernel-buffered case above.

Recommendation: build the serial form first — it is correct, it is what the
sink-shape survey above says most real sinks want, and the queue changes no
public API. Then measure a round-trip sink in step 6 and add the queue if it
shows. Keep `ChunkSink` as it is either way: the queue sits *behind* it, and the
sender is the only caller of `put`.

## Error model

**Propagate; do not latch.** The sync coders latch because `push_entropy` is
reached from `encode_bits`, which returns `()` and cannot propagate. That
constraint does not apply here on either half:

- The only fallible operation is `sink.put`, already awaited inside a function
  returning `io::Result<()>`, so `?` works at the one site that needs it.
- The coder itself is **infallible** here. Its sink is `Writer<BytesMut>`, an
  in-memory buffer whose `write_all` cannot fail, so the latched error is
  permanently `None` on this path.

Latching would mean encoding a multi-gigabyte value into a buffer whose upload
has already failed and reporting it at the end: maximum CPU for zero benefit. A
failed `put` returns immediately, unwinding the traversal, and reaches the caller
from `encode_to_sink` or as the final `Stream` item. The hot path keeps no error
branch, which is also cheaper than a latch.

## Correctness surface (must be tested)

- **Byte-identical to sync.** For every corpus and both front ends, the async
  encoder must produce **exactly** the bytes `encode` / `encode_to` produce —
  property-tested across random values, and round-tripped through the sync
  **and** async decoders (all four encode×decode combinations agree).
- **Byte-identity is the traversal's test, not just the coder's — on `Ans`.**
  Because omitting a drain moves a frame boundary, assert byte-identity against
  sync `Ans::encode_to` on a corpus that crosses `CHUNK_OPS` for *every*
  container type. That is the property doing the work described under
  [Sequencing](#sequencing), and it is worthless on a fixture small enough to fit
  one frame.
- **`encode_awaiting` reaches a split point wherever `encode` does — on
  `Range`.** The encode-side twin of `every_unbounded_type_offers_split_points`:
  run each unbounded fixture through a sink that counts `put`s and assert it is
  asked more than once. `Range` needs this because chunk-boundary invariance
  means nothing else can catch a missing drain.
- **Chunk-boundary invariance, `Range` only.** Output is independent of
  `chunk_target` and of where `split` fires: drive the same value at 1, 2, 7,
  64 KiB, `usize::MAX`; all identical.
- **One `put` per frame, `Ans` only.** Assert against a recording sink that the
  chunks delivered are exactly the frames sync `Ans::encode_to` writes, split at
  the same offsets — not merely that concatenating them matches. A `split()` that
  drained twice per frame, or coalesced two, would still be byte-identical.
- **Backpressure / bounded memory.** With a slow sink, peak buffered bytes stays
  at the bound stated under [Limitations](#limitations), not `O(value)`. Assert
  against a counting sink over a large `Vec`. Include `Vec<Option<Vec<Item>>>`
  and `Vec<(u8, Vec<Item>)>` — where a wrong `MAX_BYTES` saturation would show.
- **Frames reach the sink within `K` frames of being produced**, not at `finish`:
  encode a value spanning several chunks and assert `put` was called before the
  traversal ended. `K = 1` unless frames are parallelized; write the assertion so
  raising `K` does not invalidate it.
- **The limitations are asserted, not aspirational.** A value with one large
  `Incompressible` run *is* expected to buffer `O(run)` under `Range` and
  `min(run, 4 GiB)` under `Ans`, and a `[u8; N]` with large `N` `O(N)`; write
  each as a test that pins the actual peak.
- **`Send + Sync + 'static` on the `Stream` front end**, as the `const _`
  assertion and as a test passing the stream to a function bounded that way.
- **Cancellation.** Drop the stream mid-encode at several poll counts; assert no
  leak, mirroring `tests/cancel.rs`.
- **Edge cases:** empty value; a value whose whole output is one chunk (nothing
  drains until `finish`); `Range`'s delay-interleave tail landing exactly on a
  boundary; a `put` that errors (propagates immediately, no further `put`).

## Sequencing

The traversal is shared, so it has to be built against *some* coder. It should be
`Ans`, and the reason is testing rather than the size of the adapter.

The traversal's characteristic bug is **an `encode_awaiting` body that forgets
its drain** — byte-correct, silently `O(value)`, and easy to miss in one of
thirteen impls.

- On `Ans` that bug **changes the bytes**. Frame boundaries are in the output,
  `split()` is a passthrough to the same `split_point` the sync encoder calls,
  and `flush_chunk` is untouched — so *"the async traversal reaches exactly the
  sync traversal's split points"* is **equivalent to** byte-identity with sync
  encode. One property, applied to every corpus already in the tree, catches both
  omitted drains and spurious ones.
- On `Range` that same bug is **invisible**. Chunk-boundary invariance — the
  property that lets `take_ready` cut anywhere — means the output does not depend
  on where you drain, so a body that never drains at all passes byte-identity.
  Catching it needs a bespoke assertion per fixture.

The flexibility that makes `Range`'s coder side easy is the same flexibility that
hides its traversal bugs. That outweighs `Ans`'s adapter being smaller, though it
points the same way. One caveat on fixtures: the value must cross `CHUNK_OPS`
(65536 ops) or no non-final frame is flushed and byte-identity is vacuous;
`every_unbounded_type_offers_split_points` already had to build such fixtures.

Two things argue the other way, and neither is worth much. `Range` is the default
coder, so `v2::encode_stream` would be `Range` and going that way ships the
user-visible thing first — weak, since nothing is released and v2 is unfrozen.
More concretely, **`Range`-first is the only order that can start before PR #54
merges**, since `Ans`'s `split_point` flush lives on that branch while `Range`
could place its own drains without it. That is a reason to merge #54, not a
reason to maintain a second drain schedule.

So:

1. *Optional insurance:* the `RangeEncoder` buffer API — `buffered()` /
   `take_ready()` / `finish_into_buffer()`, unit-tested against the existing sync
   `finish` for byte-identity. Pure sync, gated on nothing, and having a second
   coder in hand while designing `AsyncEntropyCoder` is what stops a
   one-implementation trait from baking in `Ans`'s assumptions.
2. `AnsEncoder::take_flushed` (and `flush_chunk(true)` reachable as
   `finish_into_buffer`), then `ChunkSink` + `AsyncEntropyCoder` +
   `AsyncAnsEncoder` + `encode_to_sink`, with `Encode::encode_async`'s provided
   default and hand-written `encode_awaiting` for `Vec<T>` and `String` only.
   Vertical slice, checked by byte-identity against sync `Ans::encode_to` on an
   over-`CHUNK_OPS` corpus. A recording sink suffices; no `Stream` front end yet.
   Settle the `ChunkSink` shape here, before impls exist to churn.
3. `encode_elements` mirroring `sentinel::decode_elements`, then the remaining
   container bodies — maps, sets, `Compressible`, `Arc<str>`, `Sorted`,
   `LowCardinality`, and `low_cardinality::encode_miss`'s char loop. The list is
   exactly the impls that reach a `split_point`. Byte-identity checked at each.
4. Derive `encode_awaiting`, mirroring `decode_awaiting` field for field and
   variant for variant (`compactly-derive/src/v2.rs:443`), with the same
   bounds-on-type-parameters plumbing. No `MAX_BYTES` gate in the derive — the
   gate is in `encode_async`'s default, at run time.
5. **Then `Range`:** `AsyncRangeEncoder`, `chunk_target`, and the invariance and
   peak-buffered tests that `Range` needs and `Ans` gets for free.
6. The `Bytes`-stream front end (`YieldSink` / `encode_stream`), prototyped as
   `src/bin/async-encode-*.rs` driven by `futures_executor::block_on` into a fake
   S3 sink, to confirm the tokio-free `Stream` end to end and **measure the async
   tax** against sync `encode_to`. Land the `Send + Sync + 'static` assertion in
   this commit. The binary needs `required-features = ["stream"]` under its
   `[[bin]]` entry or CI silently skips it. Coder-independent, so it happens once
   rather than per coder.
7. Ship both front ends under `stream`; optional `tokio` adapters and
   S3 / reqwest examples behind a `tokio` feature.
8. Measure both coders against their sync `encode_to`, on
   `src/bin/coder-routes.rs`'s corpus, and add the rows to OPTIMIZING.md's
   coder-routes tables — which currently say "**There is no async encode for
   either coder**" and will need that sentence retired.
9. *Optional follow-ups, separately measured:* the byte-size flush trigger for
   `Ans`'s incompressible bound, and parallel frame encoding.

## Bearing on "should `Range` keep async at all?"

A standing question is whether `Range`'s async support could be dropped if `Ans`
outperforms it, to simplify the code. Two things this plan can say about it.

**The code cost is not where it looked.** With the traversal generic, `Range`
costs one buffer API and one impl, and `Ans` costs one impl. "We would have to
build async encode twice" is not a cost anyone has to weigh.

**The encode measurements point the other way from the decode ones.** PR #57's
table has `Range` **encoding faster than `Ans` on most of the corpus** — `enums`
+29.6% cycles for `Ans`, `floats` +52.1%, `strings` +5.0%, against `Ans` wins of
−6.5% to −9.9% on `records` / `records-wide` / `atmost128`. `Ans` executes *fewer*
instructions on almost every encode row and still loses cycles, which is an IPC
problem rather than a work problem. The `f64` collapse that dominates the decode
argument (+309.6% on `from`) is present on encode too but far milder, because
encode does not pay the gather-the-whole-frame cost.

So the two directions do not agree, and dropping `Range`'s async support is a
decode-side question that an encode-side plan should not answer.

## Open questions

- **`chunk_target` default, for `Range`.** An internal knob, invisible in the
  format. Start ~64 KiB; tune with the step-6 bench. (`READY_TARGET`, the
  decoder's analogue, is 256 KiB and chosen against an `Ans` frame, so it is not
  a precedent.)
- **Reuse `async-stream`/`genawaiter` vs inline `YieldOnce`.** Inlining ~40 lines
  keeps deps to `futures-core`, matching the decode side. `async-stream` would
  also solve the `Send` problem for us (thread-local, not `Rc`) — worth weighing.
- **Encoding from a stream of items.** This API takes `&T`, so it bounds the
  *output*, not the input. A `Stream<Item>`-in / `Stream<Bytes>`-out encoder is
  the shape that would bound peak memory by the value itself. Different API,
  different `Context` lifetime story (the adaptive model must persist across
  items), out of scope — but it is the honest answer to "can I encode something
  bigger than RAM", and worth deciding before 1.0.

  Worth knowing while deciding: for `Ans` that shape needs **no async machinery
  in the library at all**, because the caller owns the loop and so the suspension
  points are the caller's:

  ```rust
  let mut enc = Ans::chunk_encoder();            // owns the ops buffer + contexts
  for item in items {
      enc.push(&item)?;                          // sync: records ops, may flush a frame
      while let Some(frame) = enc.take_flushed() {
          sink.put(frame).await?;                // the caller's await, not ours
      }
  }
  for frame in enc.finish() { sink.put(frame).await?; }
  ```

  Full backpressure, bounded memory, no `encode_awaiting`, no `ChunkSink`, no
  derive change, no generator. It works because an `Ans` frame is self-contained
  and the contexts adapt across frames without the frames depending on each
  other — and it would work for `Range` too with `take_ready` in place of
  `take_flushed`, so it is not an argument between the coders. The catch is the
  one above: it does not encode a `Vec<T>` *as* a `Vec<T>`, since the length
  prefix and the `Sentinel` markers a collection codes around its elements belong
  to the container, so this is a different encoding needing a decoder that agrees
  with it.
