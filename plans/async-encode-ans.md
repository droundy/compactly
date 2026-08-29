# Async streaming encode for `Ans`

Status: **design (revision 1)**. Companion to
[`async-encode-range.md`](async-encode-range.md), which settles the traversal
that both coders share; read that first. Depends on the same two things it does:
async decode (PR #46, merged as `54d2d66`) for the trait shapes it mirrors, and
`EntropyCoder::split_point` (PR #54) for the drain schedule.

**The headline: `Ans` is not a second implementation.** Once the Range plan's
steps 2, 4 and 5 exist, `Ans` async encode is one `AsyncEntropyCoder` impl of
about thirty lines, and it needs **no changes to `ans.rs`** beyond making the
existing streaming encoder reachable. The reason is that everything the Range
plan has to build — a place to cut, a policy for when to cut, a proof that
cutting there is safe — `Ans` already has, because its output *is* a sequence of
self-contained frames and PR #54 already made `split_point` the only place a
non-final frame is emitted.

## What `Ans` already has

Three facts about `src/v2/ans.rs`, all of them load-bearing:

1. **`flush_chunk` emits one complete, self-contained frame**
   (`ans.rs:266`): header varints, then the entropy body and the raw
   incompressible region, in an order the frame tag distinguishes. Nothing about
   a frame depends on what follows it. That is the chunk the sink wants, already
   formed.
2. **`split_point` is the only place a non-final chunk is flushed** — that is
   PR #54's change to `AnsEncoder`, and its doc says so in those words. So the
   question the Range plan answers with a `chunk_target` knob ("when do we cut?")
   is not open here: it is `CHUNK_OPS` ops, decided inside the coder, and the
   async layer neither chooses nor influences it.
3. **`write_out` is the single choke point** for every byte the encoder emits
   (`ans.rs:232`), and it is the *only* thing that touches `W`.

Put together: point the existing `AnsEncoder<W>` at an in-memory buffer, call
`split_point()` where the traversal reaches one, and if the buffer is non-empty
afterwards, a whole frame is sitting in it. Take it and `put` it.

## The whole implementation

```rust
pub struct AsyncAnsEncoder<'a, S> {
    /// The existing streaming encoder, buffering into memory rather than
    /// writing through. `Writer<BytesMut>` is `bytes`' own `io::Write` adapter,
    /// so `AnsEncoder` needs no generalisation — exactly as for `Range`.
    coder: AnsEncoder<bytes::buf::Writer<bytes::BytesMut>>,
    sink: &'a mut S,
}

impl<S: ChunkSink> AsyncEntropyCoder for AsyncAnsEncoder<'_, S> {
    type Coder = AnsEncoder<bytes::buf::Writer<bytes::BytesMut>>;

    fn sync(&mut self) -> &mut Self::Coder { &mut self.coder }

    /// The coder decides whether a frame is due; we only carry away what it
    /// produced. At most one frame per call, because `split_point` flushes at
    /// most one.
    async fn split(&mut self) -> std::io::Result<()> {
        self.coder.split_point();
        if let Some(frame) = self.coder.take_flushed() {
            self.sink.put(frame).await?;
        }
        Ok(())
    }

    async fn finish(mut self) -> std::io::Result<()> {
        self.coder.finish_into_buffer();      // flush_chunk(true) — the final frame
        if let Some(tail) = self.coder.take_flushed() {
            self.sink.put(tail).await?;
        }
        Ok(())
    }
}
```

`take_flushed` is the one thing to add to `AnsEncoder`, and it is three lines:
`get_mut().split().freeze()`, returned as `Some` when non-empty. `AnsEncoder` is
already `pub(crate)` and already generic over `W: Write`, so nothing else moves.

## What this does *not* need, which the Range plan does

This is the substance of "easier", stated as the specific work that disappears:

| Range needs | `Ans` |
|---|---|
| `buffered()` / `take_ready()` / `finish_into_buffer()` on the encoder, plus their byte-identity unit tests (step 1) | one `take_flushed()`; `finish_into_buffer` is the existing `flush_chunk(true)` |
| the **cut-anywhere invariant** — the carry-free argument about `ArithState::ready_bytes`, plus the `push_entropy` splice-ordering argument | nothing to prove. The only cut is a frame boundary, and frames are self-contained by construction |
| a `chunk_target` knob, and a default for it to be tuned | none. Chunk size is `CHUNK_OPS`, already chosen, already in the sync encoder |
| **chunk-boundary invariance** tested across `chunk_target` in 1, 2, 7, 64 KiB, `usize::MAX` | vacuous. There is no knob to vary, so byte-identity with sync encode is not a property to establish but a consequence: `split()` calls the same `split_point` the sync encoder calls, at the same points, and `flush_chunk` is untouched |
| a latched-error decision (resolved: propagate) | same resolution, same reason — the buffer is infallible and `put` is the only fallible call |

The `Send`/`Sync` analysis, the `YieldSink` generator, the borrowed-sink
`ChunkSink`, cancellation, and the two front ends are all coder-independent and
are inherited unchanged from the Range plan. Nothing in them mentions `Range`.

## What it shares, and why that is the real cost

The async **traversal** — `Encode::encode_awaiting` on every unbounded impl, the
`encode_elements` helper, and the derive — is the bulk of the async encode work,
and it is identical for both coders. `Ans` cannot avoid it: the traversal is what
lets control return to the sink between elements, and a sync recursive traversal
cannot suspend no matter how cheaply its coder emits chunks. "`Ans` is already
chunked" makes the *coder* side nearly free; it does nothing for the traversal.

That is exactly why the Range plan's revision 4 made the traversal generic over
`AsyncEntropyCoder`. Revision 3 had kept the coder concrete on the reasoning that
"`Ans` async encode will not share this shape at all (it chunks natively — build
a `Vec<Op>` plus the chunk's incompressible bytes, and only *then* is there
anything to do asynchronously, with no traversal of `T` involved)". The first
half of that is right and is the table above; the conclusion drawn from it is
wrong, because the traversal is needed for backpressure and not for chunking.
Without it, `Ans` would buffer every frame it produced until the encode finished
— which is `Ans::encode` into a `Vec` with extra steps.

So the sequencing is: build the Range plan's steps 2, 4 and 5, then add this
impl. There is no `Ans`-specific step before or between.

## The one thing `Ans` can fix that `Range` cannot

The Range plan's standing limitation is that a single large `Incompressible` run
buffers `O(run)`, because the run sits in `withheld[slot]` where only
`push_entropy` can release it, and nothing writes entropy between the
`INCOMPRESSIBLE_PIECE` pieces (`vecs.rs:389`). That is a property of the
delay-interleave and cannot be changed without a format change to both sides.

`Ans` has the same symptom today and a different cause. `Vec<u8>` under
`Incompressible` declares a split point every 64 KiB (`vecs.rs:414`), and at each
one `Ans` *could* flush a frame — but `split_point` only flushes at `CHUNK_OPS`
ops, and each piece is one op, so it takes 65536 pieces, i.e. a **4 GiB** value,
before a flush is due. (#54's own test notes this, which is why that fixture is
excluded there.) Peak buffering is therefore `min(run, 4 GiB)`.

Unlike `Range`'s, this is fixable *in the coder*: give `AnsEncoder::split_point`
a second trigger on `incompressible_bytes.len()` alongside its op count. It
changes where frames land, so it changes the bytes — and that is allowed. Per
CLAUDE.md, v2 is not frozen and chunk boundaries are explicitly named as fair
game; there is no `v2-encoding` stability test to update.

Two cautions before doing it:

- It changes **sync** encode output too, which is correct (byte-identity is
  between async and sync of the same build, not across builds) but means the
  `expect-test` size assertions move for any corpus with runs over the new
  threshold.
- It is a compression question as well as a memory one: more frames means more
  per-frame `STATE_BYTES` flushes. The threshold wants to be large enough that
  this is noise — the same reasoning that set `CHUNK_OPS`.

Treat it as a follow-up with its own measurement, not as part of the async work.
The async encoder is correct either way; only its memory bound improves.

## Does `Ans` async encode change the "drop `Range` async" question?

Only mildly, and not in the direction the coder-side simplicity suggests.

The argument for dropping `Range`'s async support is that `Ans` decodes 13–30%
faster on everything that entropy-codes, so keeping two async coders may not be
worth the code. This plan weakens the code-cost half of that argument in both
directions: with the traversal generic, `Range`'s async encode costs one buffer
API and one impl, and `Ans`'s costs one impl. Neither is where the complexity is.

Meanwhile PR #57's encode table argues the other way on the merits. `Ans`
**encodes slower than `Range`** on most of the corpus — `enums` +29.6% cycles,
`floats` +52.1%, `strings` +5.0% — and on several of those rows it is executing
*fewer* instructions while taking longer, an IPC problem in the record-then-
reverse-encode structure. A world with only `Ans` async support would be one
where the fast decode path and the slow encode path are the same coder.

The honest reading: the decode-side case for dropping `Range` stands or falls on
its own measurements, and this plan neither helps nor hurts it. What it does
settle is that "we would have to build async encode twice" is not a cost anyone
has to weigh.

## An `Ans`-only shortcut worth knowing about

The Range plan lists "encoding from a stream of items" as an open question — the
API shape that would bound peak memory by *the value* rather than by its output.
For `Ans` that shape needs **no async machinery in the library at all**, and it
is worth recording because it is the one place "already chunked" pays off
completely:

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
derive change, no generator — because the caller owns the loop and so the
suspension points are the caller's. It works only because an `Ans` frame is
self-contained and the contexts adapt across frames without the frames
depending on each other.

Two honest caveats, which are why this is a note and not the plan:

- It does **not** encode a `Vec<T>` as a `Vec<T>`. The length prefix and the
  `Sentinel` markers that a collection codes around its elements are the
  container's, not the element's, so a stream of items is a different encoding
  and needs a decoder that agrees with it. That is an API and format design
  question of its own.
- The same trick would work for `Range` with `take_ready` in place of
  `take_flushed`, so it is not really an argument between the coders.

## Correctness surface

Inherit the Range plan's list, minus chunk-boundary invariance (no knob), plus:

- **One `put` per frame.** Assert against a recording sink that the chunks
  delivered are exactly the frames the sync `Ans::encode_to` writes, split at the
  same offsets — not merely that concatenating them matches. A `split()` that
  drained twice per frame, or coalesced two, would still be byte-identical and is
  otherwise invisible.
- **Frames reach the sink as they are produced**, not at `finish`: encode a value
  spanning several `CHUNK_OPS` and assert `put` was called before the traversal
  ended. This is the `Ans` form of the Range plan's split-point-coverage test,
  and it fails the same way — a `encode_awaiting` body missing its drain is
  byte-correct.
- **The `4 GiB` incompressible bound is asserted or documented, not both
  quietly.** Until the second flush trigger exists, state the bound on the public
  entry point; do not imply the frame structure makes it go away.

## Sequencing

1. Land the Range plan's steps 2, 4, 5 (traits, traversal, derive). Nothing here
   is possible before that, and nothing here influences it.
2. Add `AnsEncoder::take_flushed`, and `AsyncAnsEncoder` — the impl above.
3. `Ans::encode_to_sink` and `Ans::encode_stream`, reusing the Range plan's front
   ends verbatim; they are generic over `ChunkSink`, not over the coder.
4. Measure against sync `Ans::encode_to` and against `Range`'s async encode, on
   `src/bin/coder-routes.rs`'s corpus, and add the rows to OPTIMIZING.md's
   coder-routes tables — which currently say "**There is no async encode for
   either coder**" and will need that sentence retired.
5. *Optional follow-up, separately measured:* the byte-size flush trigger.
