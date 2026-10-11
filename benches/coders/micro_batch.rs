//! The ANS adaptive bit-decode batch (`decode_bits`) against the sequential
//! per-bit path (`decode_bit`), with nothing else in the loop — no octet decode,
//! no incompressible bytes, minimal `Vec` shape. The best signal for work on the
//! batch coder.
//!
//! `Seq` and `Batch` encode byte-for-byte identically (the same independent
//! adaptive bool contexts in the same order); only their `decode` differs. So
//! decoding the same byte stream both ways is a clean A/B of the batch
//! machinery, and the input asserts that the two streams are the same.

use std::rc::Rc;

use compactly::v2::{
    Ans, AsyncEntropyDecoder, Encode, EntropyCoder, EntropyDecoder, Strategy as _,
};
use compactly::Normal;

const N: usize = 16; // bits per group (compile-time batch width)
const GROUPS: usize = 100_000;

type Ctx = [<bool as Encode>::Context; N];

#[derive(PartialEq)]
struct Seq([bool; N]);
impl Encode for Seq {
    type Context = Ctx;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, w: &mut E, c: &mut Self::Context) {
        for (b, ctx) in value.0.iter().zip(c.iter_mut()) {
            Normal::encode(b, w, ctx);
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(r: &mut D, c: &mut Self::Context) -> Result<Self, std::io::Error> {
        let mut bits = [false; N];
        for (b, ctx) in bits.iter_mut().zip(c.iter_mut()) {
            *b = r.decode_bit(ctx);
        }
        Ok(Seq(bits))
    }

    const MAX_BYTES: usize = N * <bool as Encode>::MAX_BYTES;
    #[inline]
    async fn decode_awaiting<D: AsyncEntropyDecoder>(
        r: &mut D,
        c: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        let mut bits = [false; N];
        for (b, ctx) in bits.iter_mut().zip(c.iter_mut()) {
            *b = r.decode_bit(ctx).await;
        }
        Ok(Seq(bits))
    }
}

#[derive(PartialEq)]
struct Batch([bool; N]);
impl Encode for Batch {
    type Context = Ctx;
    #[inline]
    fn encode<E: EntropyCoder>(value: &Self, w: &mut E, c: &mut Self::Context) {
        for (b, ctx) in value.0.iter().zip(c.iter_mut()) {
            Normal::encode(b, w, ctx);
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(r: &mut D, c: &mut Self::Context) -> Result<Self, std::io::Error> {
        Ok(Batch(r.decode_bits(c)))
    }

    const MAX_BYTES: usize = N * <bool as Encode>::MAX_BYTES;
    #[inline]
    async fn decode_awaiting<D: AsyncEntropyDecoder>(
        r: &mut D,
        c: &mut Self::Context,
    ) -> Result<Self, std::io::Error> {
        Ok(Batch(r.decode_bits(c).await))
    }
}

/// Deterministic biased bits: ~20% true, like real-world flag streams.
fn groups() -> Vec<[bool; N]> {
    let mut s: u64 = 0x9e3779b97f4a7c15;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    (0..GROUPS)
        .map(|_| std::array::from_fn(|_| (next() % 5) == 0))
        .collect()
}

fn encoded() -> Rc<Vec<u8>> {
    let groups = groups();
    let seq: Vec<Seq> = groups.iter().map(|&g| Seq(g)).collect();
    let batch: Vec<Batch> = groups.iter().map(|&g| Batch(g)).collect();
    let (seq, batch) = (Ans::encode(&seq), Ans::encode(&batch));
    assert_eq!(seq, batch, "Seq and Batch must encode identically");
    eprintln!(
        "micro-batch: {GROUPS} groups × {N} bits, encoded {} bytes ({:.3} bits/bit)",
        seq.len(),
        seq.len() as f64 * 8.0 / (GROUPS * N) as f64,
    );
    Rc::new(seq)
}

thread_local! {
    static BYTES: Rc<Vec<u8>> = encoded();
}

#[scaling::input(group = "micro-batch", name = "flag stream")]
fn input() -> Rc<Vec<u8>> {
    BYTES.with(Rc::clone)
}

#[scaling::bench(group = "micro-batch", name = "decode_bit (seq)", baseline)]
fn seq(bytes: &Rc<Vec<u8>>) -> bool {
    Ans::decode::<Vec<Seq>>(bytes).expect("decode")[GROUPS / 2].0[0]
}

#[scaling::bench(group = "micro-batch", name = "decode_bits (batch)")]
fn batch(bytes: &Rc<Vec<u8>>) -> bool {
    Ans::decode::<Vec<Batch>>(bytes).expect("decode")[GROUPS / 2].0[0]
}
