// Microbenchmark isolating the ANS adaptive bit-decode batch (`decode_bits`)
// against the sequential per-bit path (`decode_bit`), with nothing else in the
// loop — no octet decode, no incompressible bytes, minimal Vec shape.
//
// `Seq` and `Batch` encode byte-for-byte identically (same independent adaptive
// bool contexts in the same order); only their `decode` differs. So decoding the
// same byte stream both ways is a clean A/B of the batch machinery.
//
// Usage: `micro-batch seq|batch`   (decode ITERS× under `perf stat`)
use compactly::v2::{Ans, Encode, EntropyCoder, EntropyDecoder};

const N: usize = 16; // bits per group (compile-time batch width)
const GROUPS: usize = 100_000;
const ITERS: usize = 1500;

type Ctx = [<bool as Encode>::Context; N];

#[derive(PartialEq)]
struct Seq([bool; N]);
impl Encode for Seq {
    type Context = Ctx;
    #[inline]
    fn encode<E: EntropyCoder>(&self, w: &mut E, c: &mut Self::Context) {
        for (b, ctx) in self.0.iter().zip(c.iter_mut()) {
            b.encode(w, ctx);
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
}

#[derive(PartialEq)]
struct Batch([bool; N]);
impl Encode for Batch {
    type Context = Ctx;
    #[inline]
    fn encode<E: EntropyCoder>(&self, w: &mut E, c: &mut Self::Context) {
        for (b, ctx) in self.0.iter().zip(c.iter_mut()) {
            b.encode(w, ctx);
        }
    }
    #[inline]
    fn decode<D: EntropyDecoder>(r: &mut D, c: &mut Self::Context) -> Result<Self, std::io::Error> {
        Ok(Batch(r.decode_bits(c)))
    }
}

// Deterministic biased bits: ~20% true, like real-world flag streams.
fn gen_groups() -> Vec<[bool; N]> {
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

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "batch".to_string());
    let groups = gen_groups();

    let seq: Vec<Seq> = groups.iter().map(|&g| Seq(g)).collect();
    let batch: Vec<Batch> = groups.iter().map(|&g| Batch(g)).collect();
    let enc_seq = Ans::encode(&seq);
    let enc_batch = Ans::encode(&batch);
    assert_eq!(enc_seq, enc_batch, "Seq and Batch must encode identically");
    eprintln!(
        "{} groups × {N} bits, encoded {} bytes ({:.3} bits/bit)",
        GROUPS,
        enc_seq.len(),
        enc_seq.len() as f64 * 8.0 / (GROUPS * N) as f64,
    );

    let mut checksum = 0u64;
    match mode.as_str() {
        "seq" => {
            for _ in 0..ITERS {
                let d = Ans::decode::<Vec<Seq>>(&enc_seq).expect("decode");
                checksum = checksum.wrapping_add(d[GROUPS / 2].0[0] as u64);
            }
        }
        "batch" => {
            for _ in 0..ITERS {
                let d = Ans::decode::<Vec<Batch>>(&enc_batch).expect("decode");
                checksum = checksum.wrapping_add(d[GROUPS / 2].0[0] as u64);
            }
        }
        other => panic!("unknown mode {other:?}; use seq|batch"),
    }
    eprintln!("mode={mode} checksum={checksum}");
}
