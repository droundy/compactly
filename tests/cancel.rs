//! Dropping a `decode_stream` future part-way through must free everything.
//!
//! Cancellation is ordinary for an async API — a `select!` arm losing a race, a
//! timeout, a client hanging up — so a partially-built value has to be
//! reclaimed. Nothing else in the suite drops a decode future at all, and an
//! async resource leak is miserable to find later, so this file exists to pin
//! it.
//!
//! It lives in its own integration test because it installs a
//! `#[global_allocator]`: instrumenting the lib's unit-test binary would put
//! this accounting under all 199 of those tests for no benefit.
#![cfg(feature = "stream")]

use std::alloc::System;
use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;
use std::task::{Context, Poll};

use bytes::Bytes;
use compactly::v2::{Ans, Range};
use futures_core::Stream;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

/// `stats_alloc`'s counters are process-global, and `cargo test` runs a
/// binary's tests on several threads — so an unsynchronized measurement picks
/// up whatever the *other* tests are allocating. Left out, this file reports
/// impossible things: a **negative** byte count, or 4000 more deallocations
/// than allocations (another test dropping its fixture mid-region).
///
/// Each test holds this for its *whole* body, not just the measured region —
/// guarding only the region still leaves the other tests building and dropping
/// their fixtures inside it.
static SERIAL: Mutex<()> = Mutex::new(());

fn serial() -> std::sync::MutexGuard<'static, ()> {
    SERIAL.lock().unwrap_or_else(|e| e.into_inner())
}

/// A stream that hands over at most `budget` chunks and then parks forever.
///
/// A budget rather than the more obvious "`Pending` before every chunk": that
/// alternation is *not* a reliable brake, because `ChunkSource::drain_ready`
/// polls speculatively without suspending, which flips the alternation and lets
/// one outer poll consume the whole input. This shape cannot be outrun — once
/// the budget is spent the decode has nowhere to go but park, which is exactly
/// the state worth cancelling from.
struct Chunks {
    chunks: std::collections::VecDeque<Bytes>,
    budget: usize,
}

impl Chunks {
    fn new(bytes: &[u8], chunk_size: usize, budget: usize) -> Self {
        let all = Bytes::copy_from_slice(bytes);
        let mut chunks = std::collections::VecDeque::new();
        let mut start = 0;
        while start < all.len() {
            let end = (start + chunk_size).min(all.len());
            chunks.push_back(all.slice(start..end));
            start = end;
        }
        Chunks { chunks, budget }
    }
}

impl Stream for Chunks {
    type Item = Result<Bytes, std::io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.budget == 0 {
            // Deliberately no `wake_by_ref`: this stream never makes progress
            // again, and the test polls by hand.
            return Poll::Pending;
        }
        self.budget -= 1;
        match self.chunks.pop_front() {
            Some(chunk) => Poll::Ready(Some(Ok(chunk))),
            None => Poll::Ready(None),
        }
    }
}

/// Drive `future` until it parks or finishes, then drop it. `Some` means it
/// finished — a run that never cancels would make the test vacuous.
fn poll_then_drop<F: Future>(future: F, max_polls: usize) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    let mut cx = Context::from_waker(std::task::Waker::noop());
    for _ in 0..max_polls {
        if let Poll::Ready(v) = future.as_mut().poll(&mut cx) {
            return Some(v);
        }
    }
    None
}

/// What `f` left behind: unmatched allocations, and net bytes still held. Both
/// zero means everything it allocated was freed — the property cancellation has
/// to preserve.
///
/// **Only the count is asserted on.** It is the sound check — every `alloc`
/// matched by a `dealloc` — and it is exact. `stats_alloc`'s byte figures are
/// not reconcilable across a `realloc` however they are combined (a growing
/// `Vec` shows up in both `bytes_reallocated` and the alloc/dealloc totals), so
/// they are reported for diagnosis and nothing more. The 1-byte-chunk case
/// below "holds" 8 MB by that arithmetic while balancing every allocation
/// exactly: it is the coalescing buffer regrowing, which is the known cost of
/// tiny chunks rather than a leak.
fn leaked(f: impl FnOnce()) -> (isize, isize) {
    let region = Region::new(GLOBAL);
    f();
    let stats = region.change();
    let unfreed = stats.allocations as isize - stats.deallocations as isize;
    let bytes =
        stats.bytes_allocated as isize + stats.bytes_reallocated - stats.bytes_deallocated as isize;
    (unfreed, bytes)
}

/// A value big enough that a decode cancelled part-way is holding a partly-built
/// `Vec` of heap strings, so a leak would show.
fn fixture() -> Vec<String> {
    (0..4000u32)
        .map(|i| format!("value number {i} with some padding to make it worth allocating"))
        .collect()
}

/// Run one cancel-and-measure cycle for every budget, asserting nothing is left
/// behind and that the runs really did cancel.
fn check_cancellation(encoded: &[u8], decode: impl Fn(Chunks) -> Option<std::io::Result<usize>>) {
    // Warm up outside the measured regions: first use of a decoder path can
    // touch one-time initialization, which is not a leak but does show up as
    // one in the very first region.
    let _ = decode(Chunks::new(encoded, 64, 4));

    let mut cancelled = 0;
    for budget in [1usize, 2, 3, 5, 8, 13, 40, 100] {
        let mut finished = false;
        let (unfreed, bytes) = leaked(|| {
            finished = decode(Chunks::new(encoded, 64, budget)).is_some();
        });
        if !finished {
            cancelled += 1;
        }
        assert_eq!(
            unfreed, 0,
            "budget={budget}: {unfreed} allocations never freed after dropping \
             the future ({bytes} bytes by stats_alloc's arithmetic)"
        );
    }
    assert!(
        cancelled >= 6,
        "only {cancelled} runs actually cancelled; the test would be vacuous"
    );
}

#[test]
fn dropping_a_range_decode_frees_everything() {
    let _serial = serial();
    let encoded = compactly::v2::encode(&fixture());
    check_cancellation(&encoded, |chunks| {
        poll_then_drop(Range::decode_stream::<Vec<String>, _, _>(chunks), 64)
            .map(|r| r.map(|v| v.len()))
    });
}

#[test]
fn dropping_an_ans_decode_frees_everything() {
    let _serial = serial();
    let encoded = Ans::encode(&fixture());
    check_cancellation(&encoded, |chunks| {
        poll_then_drop(Ans::decode_stream::<Vec<String>, _, _>(chunks), 64)
            .map(|r| r.map(|v| v.len()))
    });
}

/// Cancelling must release chunks the source is still *holding* — the coalesced
/// buffer, not just the partly-built value. One byte per chunk with a budget
/// well short of the input guarantees the decode parks deep inside a value.
#[test]
fn dropping_mid_value_releases_the_source_buffer() {
    let _serial = serial();
    let encoded = compactly::v2::encode(&fixture());
    let mut done = None;
    let (unfreed, bytes) = leaked(|| {
        done = poll_then_drop(
            Range::decode_stream::<Vec<String>, _, _>(Chunks::new(&encoded, 1, 200)),
            64,
        )
        .map(|r| r.map(|v: Vec<String>| v.len()));
    });
    assert!(
        done.is_none(),
        "200 one-byte chunks should not finish a {}-byte input; got {done:?}",
        encoded.len()
    );
    assert_eq!(
        unfreed, 0,
        "{unfreed} allocations never freed after dropping mid-value \
         ({bytes} bytes by stats_alloc's arithmetic)"
    );
}
