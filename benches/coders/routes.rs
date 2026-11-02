//! `Ans` against `Range` on every route either coder actually supports, across
//! a set of workloads.
//!
//! One route — the borrowing slice decoder — is the fastest path and not the
//! one a streaming caller takes, so measuring only it leaves the interesting
//! question unanswered: a coder that wins on slices can lose once the bytes
//! arrive through a `Read` or a `Stream`, because the routes differ in what
//! they can keep in registers and in how much they must buffer before decoding
//! anything. They do — see OPTIMIZING.md, where `f64` reverses from +32% to
//! +310% between two of these.
//!
//! Each route is a group, and each workload an input to every group, so the
//! library prints the table this crate used to assemble with a shell script:
//! a row per coder, a column per workload. Each coder runs twice, on this checkout
//! and as `main …` on the tip of `main` (see `benches/versions.rs` for which `main`
//! that is), and `main range` is the baseline, so a branch sees what it changed.
//! `ans` against `Range` on one copy is then the gap between its two rows.
//!
//! Decoding:
//!
//! - `slice` — `Ans::decode` / `v2::decode`, borrowing the buffer.
//! - `from` — `Ans::decode_from` / `Range::decode_from` over a `&[u8]` used as a
//!   `Read`. One byte per `read`, no filesystem in the loop.
//! - `stream` — `decode_stream` over the buffer delivered in 64 pieces (needs
//!   the `stream` feature).
//! - `stream 2` — the same in 2 pieces. Delivered whole, `Range`'s look-ahead
//!   routes the stream straight to the slice decoder; two pieces defeat that
//!   and force the async path.
//!
//! Encoding:
//!
//! - `encode` — `Ans::encode` / `v2::encode`, straight to a fresh `Vec`.
//! - `encode-to` — `Ans::encode_to` / `Range::encode_to` into a fresh `Vec`
//!   used as a `Write`. Fresh rather than reused so it allocates like `encode`
//!   does and the two differ only in the route.
//!
//! **There is no async encode**, for either coder — no `encode_stream` exists to
//! measure. That asymmetry is the point of listing the encode routes here: the
//! decode matrix on its own invites the conclusion that `Range`'s async path
//! could go, and the encode side is a reminder that "async" currently means
//! decode only, so the comparison is narrower than it looks.
//!
//! Every workload runs identical harness code — a comparison across routes or
//! coders is only worth anything if nothing but the route or coder differs. A
//! time is for one whole call, which is one whole value: use the workload's
//! element count (in OPTIMIZING.md, or `Case::head_elements`) to get a per-element
//! cost.
//!
//! The workloads: `u64` and `u64 seq` (random and consecutive, 100k), `u64
//! small` (2000, which is cache-resident and a single chunk — where 100k spans
//! several and is memory-bound), `strings` (a `BTreeSet<String>` of the
//! meteorite names: THE per-character tree-walk workload), `enums` and
//! `enums17` (a skewed 3-variant and a uniform 17-variant enum: the `AtMost`
//! discriminant path through the derive), `floats` (non-integer `f64`, the
//! incompressible raw-bits path), `compressible` (the meteorite CSV through
//! Lz77), `records` and `records wide` (a short string beside integers, the
//! shape callers stream), `ipv6` (needs `ipv6.txt`), and `atmost3`, `8`, `16`, `32` and `128`
//! (50k uniform values of an `AtMost<N-1>`, the depth sweep that located the
//! per-coder prefetch crossover).

use std::collections::BTreeSet;
use std::net::Ipv6Addr;

#[cfg(feature = "stream")]
use compactly::v2::decode_stream;
use compactly::v2::{Ans, Range};
use comparison_main::compactly as main_compactly;
use scaling::Metrics;

/// A value, and what each coder of one copy of compactly wrote for it.
#[derive(Clone)]
pub struct Written<V> {
    value: V,
    range: Vec<u8>,
    ans: Vec<u8>,
}

/// One copy of compactly: its `Encode`, and the values that have to be built from
/// its types. A macro, not a generic, because each copy's `Encode` is a different
/// trait and each copy's derived types are different types.
macro_rules! side {
    ($side:ident, $compactly:ident, $corpora:ident) => {
        pub mod $side {
            // The renamed compactly and corpora come from here, and the generators
            // from the data below.
            #[allow(unused_imports)]
            use super::*;
            pub use $compactly::v2::{Ans, AtMost, Encode, Range};
            pub use $compactly::{Compressible, Encoded};
            pub use $corpora::shapes::{Record, SeventeenOptions, ThreeOptions, WideRecord};

            pub fn write<V: Encode>(value: V) -> Written<V> {
                let range = Range::encode(&value);
                let ans = Ans::encode(&value);
                Written { value, range, ans }
            }

            pub fn round_trips<V: Encode + PartialEq>(w: &Written<V>) -> bool {
                Range::decode::<V>(&w.range).unwrap() == w.value
                    && Ans::decode::<V>(&w.ans).unwrap() == w.value
            }

            pub fn records() -> (Vec<Record>, Vec<WideRecord>) {
                let mut rng = seeded_lcg(0x243f_6a88_85a3_08d3);
                let mut narrow = Vec::new();
                let mut wide = Vec::new();
                for name in meteorite_names() {
                    let (id, r) = (rng(), rng());
                    narrow.push(Record {
                        id,
                        count: r as u32,
                        name: name.clone(),
                        active: r & 1 == 0,
                    });
                    wide.push(WideRecord {
                        id,
                        count: r as u32,
                        a: rng(),
                        b: rng(),
                        c: rng() as u32,
                        d: rng() as u16,
                        name,
                        active: r & 1 == 0,
                    });
                }
                (narrow, wide)
            }

            /// 20% A, 20% B, 60% C — the skew the `btreeset<vec![ThreeOptions; 15]>`
            /// case in `benches/bench.rs` uses.
            pub fn three() -> Vec<ThreeOptions> {
                let mut rng = lcg();
                (0..100_000)
                    .map(|_| match (rng() >> 33) % 10 {
                        0 | 1 => ThreeOptions::A,
                        2 | 3 => ThreeOptions::B,
                        _ => ThreeOptions::C,
                    })
                    .collect()
            }

            pub fn seventeen() -> Vec<SeventeenOptions> {
                use SeventeenOptions::*;
                const VARIANTS: [SeventeenOptions; 17] =
                    [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q];
                let mut rng = lcg();
                (0..100_000)
                    .map(|_| VARIANTS[((rng() >> 33) % 17) as usize])
                    .collect()
            }

            pub fn atmost<const MAX: usize>() -> Vec<AtMost<MAX>> {
                let mut rng = lcg();
                (0..50_000)
                    .map(|_| AtMost::<MAX>::new(((rng() >> 33) as usize) % (MAX + 1)))
                    .collect()
            }

            /// The meteorite CSV, as bytes for `Compressible`'s Lz77 to chew on.
            pub fn csv() -> Encoded<Vec<u8>, Compressible> {
                Encoded::new(
                    include_str!("../../comparison/src/meteorites.csv")
                        .as_bytes()
                        .to_vec(),
                )
            }
        }
    };
}

side!(head, compactly, comparison);
side!(on_main, main_compactly, comparison_main);

/// A workload, on both copies of compactly.
pub trait Case: 'static {
    type Head: head::Encode + PartialEq;
    type Main: on_main::Encode + PartialEq;
    fn head(&self) -> &Written<Self::Head>;
    fn main(&self) -> &Written<Self::Main>;
    /// How many elements a decode produces. Every route folds it over the
    /// decoded value, so a decode cannot be optimized away.
    fn head_elements(value: &Self::Head) -> usize;
    fn main_elements(value: &Self::Main) -> usize;
}

/// Declare a workload: the type that names it, the module that holds its input,
/// the name it is reported under, its value type and how to build one. `$ty` and
/// `$make` are read twice, in each copy of compactly's scope, and both copies get
/// the same data.
///
/// `scaling` builds the input anew for every call, and clones it so that both
/// coders of a round see the same data, so `$make` should draw new data each time:
/// one fixed value is something a branch predictor learns (see "Fresh data by
/// default" in OPTIMIZING.md). A workload that has only one value to offer, a real
/// corpus or a deterministic sequence, says `build_once` after `$make`, and is built
/// once per route. The first value built is checked to round-trip on both coders.
macro_rules! case {
    ($case:ident, $module:ident, $name:literal, $ty:ty, $make:expr $(, $mode:ident)?) => {
        mod $module {
            use super::*;

            pub mod this_checkout {
                #[allow(unused_imports)]
                use crate::routes::{head::*, *};

                pub type V = $ty;
                pub fn make() -> V {
                    $make
                }
            }

            pub mod tip_of_main {
                #[allow(unused_imports)]
                use crate::routes::{on_main::*, *};

                pub type V = $ty;
                pub fn make() -> V {
                    $make
                }
            }

            fn build() -> $case {
                static CHECKED: std::sync::Once = std::sync::Once::new();
                let (head_value, main_value) =
                    crate::support::same_seeds(this_checkout::make, tip_of_main::make);
                let case = $case {
                    head: head::write(head_value),
                    main: on_main::write(main_value),
                };
                CHECKED.call_once(|| {
                    assert!(
                        head::round_trips(&case.head) && on_main::round_trips(&case.main),
                        "{} did not round-trip",
                        $name
                    );
                });
                case
            }

            #[cfg(feature = "stream")]
            #[scaling::input(
                                group("slice", "from", "stream", "stream 2", "encode", "encode-to"),
                                name = $name
                                $(, $mode)?
                            )]
            fn input() -> $case {
                build()
            }

            #[cfg(not(feature = "stream"))]
            #[scaling::input(
                                group("slice", "from", "encode", "encode-to"),
                                name = $name
                                $(, $mode)?
                            )]
            fn input() -> $case {
                build()
            }
        }

        #[derive(Clone)]
        pub struct $case {
            head: Written<$module::this_checkout::V>,
            main: Written<$module::tip_of_main::V>,
        }

        impl Case for $case {
            type Head = $module::this_checkout::V;
            type Main = $module::tip_of_main::V;
            fn head(&self) -> &Written<Self::Head> {
                &self.head
            }
            fn main(&self) -> &Written<Self::Main> {
                &self.main
            }
            fn head_elements(value: &Self::Head) -> usize {
                value.len()
            }
            fn main_elements(value: &Self::Main) -> usize {
                value.len()
            }
        }
    };
}

// ---------- data ----------

/// A generator from a seed nothing else has used, so that each value built is new.
fn lcg() -> impl FnMut() -> u64 {
    seeded_lcg(crate::support::new_seed())
}

fn seeded_lcg(seed: u64) -> impl FnMut() -> u64 {
    let mut x = seed;
    move || {
        x = x
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        x
    }
}

/// `count` pseudo-random `u64`s.
fn u64s(count: usize) -> Vec<u64> {
    let mut rng = lcg();
    (0..count).map(|_| rng()).collect()
}

/// `count` *consecutive* integers: highly compressible, and the cheapest thing
/// to encode of anything here.
fn u64_seq(count: u64) -> Vec<u64> {
    (0..count).collect()
}

fn ipv6s() -> Vec<Ipv6Addr> {
    std::fs::read_to_string("ipv6.txt")
        .expect("the `ipv6` workload reads ipv6.txt from the current directory")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse().expect("valid IPv6"))
        .collect()
}

fn meteorite_names() -> BTreeSet<String> {
    comparison::meteorites::meteorite_names()
        .into_keys()
        .collect()
}

/// Non-integer `f64`s, so decode takes the raw-bits path rather than the
/// integer fast path.
fn floats() -> Vec<f64> {
    let mut rng = lcg();
    (0..100_000)
        .map(|_| {
            let x = rng();
            let bits = if (x >> 52) & 0x7FF == 0x7FF {
                x & !(1u64 << 52)
            } else {
                x
            };
            f64::from_bits(bits)
        })
        .collect()
}

// ---------- the workloads ----------

case!(U64, u64_input, "u64", Vec<u64>, u64s(100_000));
case!(
    U64Small,
    u64_small_input,
    "u64 small",
    Vec<u64>,
    u64s(2_000)
);
case!(
    U64Seq,
    u64_seq_input,
    "u64 seq",
    Vec<u64>,
    u64_seq(100_000),
    build_once
);
case!(
    Strings,
    strings_input,
    "strings",
    BTreeSet<String>,
    meteorite_names(),
    build_once
);
case!(Enums, enums_input, "enums", Vec<ThreeOptions>, three());
case!(
    Enums17,
    enums17_input,
    "enums17",
    Vec<SeventeenOptions>,
    seventeen()
);
case!(Floats, floats_input, "floats", Vec<f64>, floats());
case!(
    CompressibleCsv,
    compressible_input,
    "compressible",
    Encoded<Vec<u8>, Compressible>,
    csv(),
    build_once
);
case!(
    Records,
    records_input,
    "records",
    Vec<Record>,
    records().0,
    build_once
);
case!(
    RecordsWide,
    records_wide_input,
    "records wide",
    Vec<WideRecord>,
    records().1,
    build_once
);
case!(Ipv6, ipv6_input, "ipv6", Vec<Ipv6Addr>, ipv6s(), build_once);

// The ladder is monomorphized, so `MAX` is a literal here. Five rungs are enough
// to see the prefetch crossover (`Ans` loses at the bottom and wins at the top);
// the walk shootout (`src/v2/atmost/shootout.rs`) sweeps every value count the
// production walk is chosen for.
case!(
    AtMost3,
    atmost3_input,
    "atmost3",
    Vec<AtMost<2>>,
    atmost::<2>()
);
case!(
    AtMost8,
    atmost8_input,
    "atmost8",
    Vec<AtMost<7>>,
    atmost::<7>()
);
case!(
    AtMost16,
    atmost16_input,
    "atmost16",
    Vec<AtMost<15>>,
    atmost::<15>()
);
case!(
    AtMost32,
    atmost32_input,
    "atmost32",
    Vec<AtMost<31>>,
    atmost::<31>()
);
case!(
    AtMost128,
    atmost128_input,
    "atmost128",
    Vec<AtMost<127>>,
    atmost::<127>()
);

// ---------- the stream source ----------

#[cfg(feature = "stream")]
mod source {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    use bytes::Bytes;
    use futures_core::Stream;

    /// Deliver the buffer in `chunks` equal pieces, never `Pending` — this
    /// measures the decode route, not suspension, and `block_on` asserts that by
    /// panicking if anything suspends. Building it copies the buffer, and that
    /// is inside the timing, as it always was here.
    pub struct Chunks(std::vec::IntoIter<Bytes>);

    impl Chunks {
        pub fn new(bytes: &[u8], chunks: usize) -> Self {
            let all = Bytes::copy_from_slice(bytes);
            let size = all.len().div_ceil(chunks.max(1)).max(1);
            let mut parts = Vec::new();
            let mut start = 0;
            while start < all.len() {
                let end = (start + size).min(all.len());
                parts.push(all.slice(start..end));
                start = end;
            }
            if parts.is_empty() {
                parts.push(all);
            }
            Chunks(parts.into_iter())
        }
    }

    impl Stream for Chunks {
        type Item = Result<Bytes, std::io::Error>;
        fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.0.next().map(Ok))
        }
    }

    /// Drive a future known never to suspend. Hand-rolled because a real
    /// executor would put its own scheduling in the measurement.
    pub fn block_on<F: Future>(future: F) -> F::Output {
        let mut future = std::pin::pin!(future);
        let mut cx = Context::from_waker(std::task::Waker::noop());
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => {
                panic!("future suspended: the source was supposed to be fully buffered")
            }
        }
    }
}

// ---------- the routes ----------
//
// One generic function per route and coder, registered once for each workload
// type. A candidate is paired only with inputs of its own type, so each cell of
// the grid is a function of exactly one workload.

macro_rules! routes {
    ($($case:ident),+ $(,)?) => {
        #[scaling::bench(group = "slice", name = "main range", baseline, types($($case),+))]
        fn slice_main_range<C: Case>(c: &C) -> usize {
            C::main_elements(&main_compactly::v2::decode::<C::Main>(&c.main().range).unwrap())
        }
        #[scaling::bench(group = "slice", name = "main ans", types($($case),+))]
        fn slice_main_ans<C: Case>(c: &C) -> usize {
            C::main_elements(&on_main::Ans::decode::<C::Main>(&c.main().ans).unwrap())
        }
        #[scaling::bench(group = "slice", name = "range", types($($case),+))]
        fn slice_range<C: Case>(c: &C) -> usize {
            C::head_elements(&compactly::v2::decode::<C::Head>(&c.head().range).unwrap())
        }
        #[scaling::bench(group = "slice", name = "ans", types($($case),+))]
        fn slice_ans<C: Case>(c: &C) -> usize {
            C::head_elements(&Ans::decode::<C::Head>(&c.head().ans).unwrap())
        }

        #[scaling::bench(group = "from", name = "main range", baseline, types($($case),+))]
        fn from_main_range<C: Case>(c: &C) -> usize {
            C::main_elements(
                &on_main::Range::decode_from::<C::Main, _>(c.main().range.as_slice()).unwrap(),
            )
        }
        #[scaling::bench(group = "from", name = "main ans", types($($case),+))]
        fn from_main_ans<C: Case>(c: &C) -> usize {
            C::main_elements(
                &on_main::Ans::decode_from::<C::Main, _>(c.main().ans.as_slice()).unwrap(),
            )
        }
        #[scaling::bench(group = "from", name = "range", types($($case),+))]
        fn from_range<C: Case>(c: &C) -> usize {
            C::head_elements(&Range::decode_from::<C::Head, _>(c.head().range.as_slice()).unwrap())
        }
        #[scaling::bench(group = "from", name = "ans", types($($case),+))]
        fn from_ans<C: Case>(c: &C) -> usize {
            C::head_elements(&Ans::decode_from::<C::Head, _>(c.head().ans.as_slice()).unwrap())
        }

        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream", name = "main range", baseline, types($($case),+))]
        fn stream_main_range<C: Case>(c: &C) -> usize {
            C::main_elements(
                &source::block_on(main_compactly::v2::decode_stream::<C::Main, _, _>(
                    source::Chunks::new(&c.main().range, 64),
                ))
                .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream", name = "main ans", types($($case),+))]
        fn stream_main_ans<C: Case>(c: &C) -> usize {
            C::main_elements(
                &source::block_on(on_main::Ans::decode_stream::<C::Main, _, _>(
                    source::Chunks::new(&c.main().ans, 64),
                ))
                .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream", name = "range", types($($case),+))]
        fn stream_range<C: Case>(c: &C) -> usize {
            C::head_elements(
                &source::block_on(decode_stream::<C::Head, _, _>(source::Chunks::new(&c.head().range, 64)))
                    .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream", name = "ans", types($($case),+))]
        fn stream_ans<C: Case>(c: &C) -> usize {
            C::head_elements(
                &source::block_on(Ans::decode_stream::<C::Head, _, _>(source::Chunks::new(&c.head().ans, 64)))
                    .unwrap(),
            )
        }

        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream 2", name = "main range", baseline, types($($case),+))]
        fn stream_2_main_range<C: Case>(c: &C) -> usize {
            C::main_elements(
                &source::block_on(main_compactly::v2::decode_stream::<C::Main, _, _>(
                    source::Chunks::new(&c.main().range, 2),
                ))
                .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream 2", name = "main ans", types($($case),+))]
        fn stream_2_main_ans<C: Case>(c: &C) -> usize {
            C::main_elements(
                &source::block_on(on_main::Ans::decode_stream::<C::Main, _, _>(
                    source::Chunks::new(&c.main().ans, 2),
                ))
                .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream 2", name = "range", types($($case),+))]
        fn stream_2_range<C: Case>(c: &C) -> usize {
            C::head_elements(
                &source::block_on(decode_stream::<C::Head, _, _>(source::Chunks::new(&c.head().range, 2)))
                    .unwrap(),
            )
        }
        #[cfg(feature = "stream")]
        #[scaling::bench(group = "stream 2", name = "ans", types($($case),+))]
        fn stream_2_ans<C: Case>(c: &C) -> usize {
            C::head_elements(
                &source::block_on(Ans::decode_stream::<C::Head, _, _>(source::Chunks::new(&c.head().ans, 2)))
                    .unwrap(),
            )
        }

        #[scaling::bench(group = "encode", name = "main range", baseline, types($($case),+))]
        fn encode_main_range<C: Case>(c: &C) -> Vec<u8> {
            main_compactly::v2::encode(&c.main().value)
        }
        #[scaling::bench(group = "encode", name = "main ans", types($($case),+))]
        fn encode_main_ans<C: Case>(c: &C) -> Vec<u8> {
            on_main::Ans::encode(&c.main().value)
        }
        #[scaling::bench(group = "encode", name = "range", types($($case),+))]
        fn encode_range<C: Case>(c: &C) -> Vec<u8> {
            compactly::v2::encode(&c.head().value)
        }
        #[scaling::bench(group = "encode", name = "ans", types($($case),+))]
        fn encode_ans<C: Case>(c: &C) -> Vec<u8> {
            Ans::encode(&c.head().value)
        }

        #[scaling::bench(group = "encode-to", name = "main range", baseline, types($($case),+))]
        fn encode_to_main_range<C: Case>(c: &C) -> Vec<u8> {
            let mut sink = Vec::new();
            main_compactly::v2::encode_to(&c.main().value, &mut sink).unwrap();
            sink
        }
        #[scaling::bench(group = "encode-to", name = "main ans", types($($case),+))]
        fn encode_to_main_ans<C: Case>(c: &C) -> Vec<u8> {
            let mut sink = Vec::new();
            on_main::Ans::encode_to(&c.main().value, &mut sink).unwrap();
            sink
        }
        #[scaling::bench(group = "encode-to", name = "range", types($($case),+))]
        fn encode_to_range<C: Case>(c: &C) -> Vec<u8> {
            let mut sink = Vec::new();
            compactly::v2::encode_to(&c.head().value, &mut sink).unwrap();
            sink
        }
        #[scaling::bench(group = "encode-to", name = "ans", types($($case),+))]
        fn encode_to_ans<C: Case>(c: &C) -> Vec<u8> {
            let mut sink = Vec::new();
            Ans::encode_to(&c.head().value, &mut sink).unwrap();
            sink
        }
    };
}

routes!(
    U64,
    U64Small,
    U64Seq,
    Strings,
    Enums,
    Enums17,
    Floats,
    CompressibleCsv,
    Records,
    RecordsWide,
    Ipv6,
    AtMost3,
    AtMost8,
    AtMost16,
    AtMost32,
    AtMost128,
);

/// What the encode routes wrote, so that a faster route that wrote something
/// different cannot pass for a win.
#[scaling::metrics(group("encode", "encode-to"))]
fn size(out: Vec<u8>) -> Metrics {
    Metrics::new().bytes("size", out.len())
}
