use super::atmost::{walks, AtMost, AtMostContext};
use super::model::{Probability, SymbolCoder, SymbolDecoder, SymbolRange, SHIFT};
use super::{EntropyCoder, EntropyDecoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithState {
    lo: u64,
    hi: u64,
}

impl Default for ArithState {
    #[inline]
    fn default() -> Self {
        ArithState {
            lo: 0,
            hi: u64::MAX,
        }
    }
}

impl ArithState {
    #[inline]
    fn ready_bytes(&mut self) -> Bytes {
        let mut bytes = Bytes::default();
        if self.lo == self.hi {
            for b in self.lo.to_be_bytes() {
                bytes.push(b);
            }
            self.lo = 0;
            self.hi = u64::MAX;
        } else {
            for _ in 0..8 {
                let lo_byte = (self.lo >> 56) as u8;
                let hi_byte = (self.hi >> 56) as u8;
                // #[cfg(test)]
                // {
                //     let width = self.hi - self.lo;
                //     println!("width = {width:016x}");
                //     println!("  min = {:016x}", u64::MAX >> 8);
                //     println!("lo_byte {lo_byte:02x}");
                //     println!("hi_byte {hi_byte:02x}");
                // }
                if lo_byte == hi_byte {
                    self.lo <<= 8;
                    self.hi <<= 8;
                    // #[cfg(test)]
                    // {
                    //     println!("next_byte resetting to {self:x?}");
                    // }
                    bytes.push(lo_byte);
                } else {
                    return bytes;
                }
            }
        }
        bytes
    }

    /// The single byte that finalizes the stream: the top byte of the interval,
    /// which the decoder pulls to disambiguate the last coded value.
    #[inline]
    pub fn last_byte(self) -> u8 {
        (self.hi >> 56) as u8
    }

    /// Returns a set of bytes to be written out.
    #[must_use]
    #[inline]
    pub fn encode(&mut self, prob: Probability, value: bool) -> Bytes {
        if self.hi == self.lo + 1 {
            // special case that we need to handle differently.
            let bytes = if value {
                self.hi.to_be_bytes()
            } else {
                self.lo.to_be_bytes()
            };
            self.lo = 0;
            self.hi = u64::MAX;
            return Bytes { bytes, count: 8 };
        }
        let split = self.split(prob);
        debug_assert!(split < self.hi, "{self:x?} {prob:?}");
        debug_assert!(split >= self.lo);
        debug_assert!(self.hi > self.lo);
        if value {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        self.ready_bytes()
        // println!("encoding {prob} {shift} {value:?}   with split {split:016x} gives {self:x?}");
    }

    /// Returns bit and the number of bytes that need to be read.
    #[inline]
    pub fn decode(&mut self, prob: Probability, value: u64) -> (bool, usize) {
        if self.hi == self.lo + 1 {
            let bit = value == self.hi;
            self.hi = u64::MAX;
            self.lo = 0;
            return (bit, 8);
        }
        let split = self.split(prob);
        let b = value > split;
        // Branchless: compute both lo/hi updates and select via CMOV.
        self.lo = if b { split + 1 } else { self.lo };
        self.hi = if b { self.hi } else { split };
        (b, self.consume_decoded_bytes())
    }

    /// Normalize state after decode and return number of compressed bytes consumed.
    /// Uses leading_zeros to avoid a branch-heavy loop, eliminating ~12.5% mispredictions.
    #[inline]
    fn consume_decoded_bytes(&mut self) -> usize {
        let diff = self.lo ^ self.hi;
        if diff == 0 {
            self.lo = 0;
            self.hi = u64::MAX;
            return 8;
        }
        let n = (diff.leading_zeros() / 8) as usize;
        self.lo <<= n * 8;
        self.hi <<= n * 8;
        n
    }

    #[inline]
    fn split(self, Probability { prob }: Probability) -> u64 {
        // debug_assert!(prob < 1 << SHIFT);
        debug_assert!(self.hi > self.lo);
        let width = self.hi - self.lo;
        debug_assert!(self.lo >> 56 != self.hi >> 56);
        self.lo + (width >> SHIFT) * prob.get() as u64
    }

    /// Minimum interval width required before coding a whole tree symbol in
    /// one step. Guarantees every one of the `M` slots spans at least `M`
    /// values, so slot boundaries are exact and the top-slot rounding waste is
    /// at most a `1/M` fraction of the interval. The per-bit path tolerates
    /// arbitrarily narrow intervals, so this is only enforced (via
    /// [`ArithState::clamp_for_symbol`]) on the symbol path. (Must stay below
    /// `2^56` for `clamp_for_symbol`'s single-boundary argument to hold.)
    const MIN_SYMBOL_WIDTH: u64 = (SymbolRange::M as u64) * (SymbolRange::M as u64);

    /// Carry-less clamp renormalization (Subbotin-style): if the interval is
    /// too narrow for a symbol step, it must straddle exactly one top-byte
    /// boundary (byte-wise renormalization would otherwise have shifted it
    /// out). Discard the smaller side of that boundary so renormalization can
    /// proceed; the encoder simply never codes into the discarded part, at a
    /// cost of at most one bit per (rare) clamp. The choice depends only on
    /// `(lo, hi)`, which encoder and decoder share, so they always agree.
    ///
    /// Returns whether it clamped; the caller must then renormalize
    /// (`ready_bytes` / `consume_decoded_bytes`) and call this again.
    #[inline]
    fn clamp_for_symbol(&mut self) -> bool {
        if self.hi - self.lo >= Self::MIN_SYMBOL_WIDTH {
            return false;
        }
        // width < 2^56 with unequal top bytes ⟹ exactly one multiple of 2^56
        // lies in (lo, hi]: hi rounded down to its top byte.
        let boundary = self.hi & (0xFF << 56);
        debug_assert!(self.lo < boundary && boundary <= self.hi);
        if boundary - self.lo > self.hi - boundary + 1 {
            self.hi = boundary - 1;
        } else {
            self.lo = boundary;
        }
        true
    }

    /// Narrow the interval to `range`'s slots. Requires
    /// `width >= MIN_SYMBOL_WIDTH` (see [`ArithState::clamp_for_symbol`]).
    /// The top slot absorbs the sub-slot rounding remainder, mirroring how the
    /// per-bit `encode` gives the true branch everything above `split`.
    #[inline]
    fn narrow_symbol(&mut self, range: SymbolRange) {
        let step = (self.hi - self.lo) >> SymbolRange::BITS;
        let end = range.start() + range.width();
        self.hi = if end == SymbolRange::M {
            self.hi
        } else {
            self.lo + step * end as u64 - 1
        };
        self.lo += step * range.start() as u64;
        debug_assert!(self.hi > self.lo);
    }

    /// Which slot the decoder's window `value` falls in. Values in the
    /// top-slot remainder (and garbage past the end of the stream) clamp to
    /// the top slot.
    #[inline]
    fn symbol_slot(&self, value: u64) -> u32 {
        let step = (self.hi - self.lo) >> SymbolRange::BITS;
        (value.wrapping_sub(self.lo) / step).min((SymbolRange::M - 1) as u64) as u32
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Bytes {
    bytes: [u8; 8],
    count: usize,
}

impl Bytes {
    #[inline]
    fn push(&mut self, byte: u8) {
        self.bytes[self.count] = byte;
        self.count += 1;
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.bytes[..self.count]
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = std::iter::Take<std::array::IntoIter<u8, 8>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().take(self.count)
    }
}

/// Delay-interleave constant: the decoder's window `value` is a `u64` filled
/// from the first 8 stream bytes and pulled one byte per renorm in lockstep
/// with the encoder, so the decoder's cumulative pull-count stays exactly this
/// many bytes ahead of the encoder's emit-count. An incompressible run is
/// therefore spliced into the stream `W_DELAY` entropy bytes after the point it
/// was produced, so it lands exactly at the decoder's read cursor when the
/// decode logic reaches that field. See plans/streaming-io-api.md.
const W_DELAY: usize = 8;

/// Use range coding to encode bits.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Range::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 4);
/// assert_eq!(compactly::v2::Range::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    /// The interleaved output built up as we go: entropy bytes with each
    /// incompressible run already spliced in at its delayed offset. `into_vec`
    /// only has to flush the coder's last byte and any tail runs.
    bytes: Vec<u8>,
    /// Count of *entropy* bytes in `bytes` so far (excludes spliced runs), used
    /// to schedule the `W_DELAY` splice.
    entropy_written: usize,
    /// Incompressible runs not yet spliced into `bytes`. A run recorded at
    /// entropy count `E` must follow entropy byte `E + W_DELAY`; since the delay
    /// is exactly `W_DELAY`, at most one delay-cycle of runs is outstanding, so
    /// this is a ring of `W_DELAY` slots indexed by `E % W_DELAY`. Runs recorded
    /// at the same `E` (consecutive, no entropy between) accumulate in one slot.
    withheld: [Vec<u8>; W_DELAY],
    state: ArithState,
}

impl Range {
    /// Append settled entropy `bytes`, splicing each withheld run in as its
    /// target entropy byte (`recorded_at + W_DELAY`) is written. Slot
    /// `entropy_written % W_DELAY` holds the run due `W_DELAY` bytes after it was
    /// recorded, so flushing that slot right after writing byte `entropy_written`
    /// places the run exactly after its target byte.
    #[inline]
    fn push_entropy(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.bytes.push(b);
            self.entropy_written += 1;
            let slot = self.entropy_written % W_DELAY;
            if !self.withheld[slot].is_empty() {
                let run = std::mem::take(&mut self.withheld[slot]);
                self.bytes.extend_from_slice(&run);
            }
        }
    }
}

impl EntropyCoder for Range {
    #[inline]
    fn encode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        bits: [bool; N],
    ) {
        for (value, ctx) in bits.into_iter().zip(contexts.iter_mut()) {
            let ready = self.state.encode(ctx.probability(), value);
            self.push_entropy(&ready);
            *ctx = ctx.adapt(value);
        }
    }

    #[inline]
    fn encode_atmost<const MAX: usize>(
        &mut self,
        ctx: &mut AtMostContext<MAX>,
        value: AtMost<MAX>,
    ) {
        walks::encode_symbol_or_bitwise(self, ctx, value)
    }

    #[inline]
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        // Withhold the run for W_DELAY entropy bytes (its slot is revisited then).
        let slot = self.entropy_written % W_DELAY;
        self.withheld[slot].extend_from_slice(bytes);
    }
}

impl SymbolCoder for Range {
    /// Code one whole-symbol interval: clamp the range state so a symbol
    /// fits, then a single narrowing + renormalization.
    #[inline]
    fn encode_symbol(&mut self, range: SymbolRange) {
        while self.state.clamp_for_symbol() {
            let ready = self.state.ready_bytes();
            self.push_entropy(&ready);
        }
        self.state.narrow_symbol(range);
        let ready = self.state.ready_bytes();
        self.push_entropy(&ready);
    }
}

impl Range {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        let mut reader = super::arith::Decoder::new(bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
    /// Whether `Range`'s decoder asks [`Walk::production`](super::Walk::production)
    /// to speculate on a non-power-of-two value count (see
    /// [`SymbolDecoder::SPECULATES`]). Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub const SPECULATES: bool = <Decoder<'static> as SymbolDecoder>::SPECULATES;
    /// Encode `values` using an explicitly forced tree walk, bypassing
    /// [`Walk::production`](super::Walk::production)'s usual choice for
    /// `MAX`. `WHICH_WALK` indexes [`WALKS`](super::WALKS). Benchmark support
    /// for `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub fn encode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        values: &[super::AtMost<MAX>],
    ) -> Vec<u8> {
        walks::encode_atmost_batch::<Self, MAX, WHICH_WALK>(Self::default(), values).into_vec()
    }
    /// The decode side of [`Self::encode_atmost_batch`]: decode `n` values
    /// with the same forced walk. Benchmark support for
    /// `benches/atmost.rs`, not part of the stable API.
    #[doc(hidden)]
    pub fn decode_atmost_batch<const MAX: usize, const WHICH_WALK: usize>(
        bytes: &[u8],
        n: usize,
    ) -> Vec<super::AtMost<MAX>> {
        walks::decode_atmost_batch::<Decoder, MAX, WHICH_WALK>(Decoder::new(bytes), n)
    }
    /// Finish encoding: append the coder's final byte, then flush any tail runs
    /// still withheld (recorded within `W_DELAY` of the end). Each tail run is
    /// placed after zero-padding the entropy up to its target, so the decoder's
    /// trailing read-ahead window fills with padding rather than raw bytes. The
    /// result is a single flat stream — no trailer, no magic.
    #[inline]
    pub fn into_vec(mut self) -> Vec<u8> {
        let last = self.state.last_byte();
        self.push_entropy(&[last]);
        let mut remaining = self.withheld.iter().filter(|r| !r.is_empty()).count();
        while remaining > 0 {
            self.bytes.push(0);
            self.entropy_written += 1;
            let slot = self.entropy_written % W_DELAY;
            if !self.withheld[slot].is_empty() {
                let run = std::mem::take(&mut self.withheld[slot]);
                self.bytes.extend_from_slice(&run);
                remaining -= 1;
            }
        }
        self.bytes
    }
}
impl From<Range> for Vec<u8> {
    fn from(value: Range) -> Self {
        value.into_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder<'a> {
    /// The single flat delay-interleave stream: entropy bytes with each
    /// incompressible run spliced in. Entropy `pull`s and
    /// `decode_incompressible_bytes` both advance this one cursor; the W_DELAY
    /// splice guarantees a run sits exactly at the cursor when its field is
    /// reached.
    bytes: &'a [u8],
    state: ArithState,
    value: u64,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let (value, bytes) = if let Some((&first, rest)) = bytes.split_first_chunk() {
            (u64::from_be_bytes(first), rest)
        } else {
            let mut b = [0; 8];
            b[..bytes.len()].copy_from_slice(bytes);
            (u64::from_be_bytes(b), [].as_slice())
        };
        Self {
            bytes,
            state: ArithState::default(),
            value,
        }
    }
}

/// One range-decode bit step, operating on locals so the caller can keep `state`,
/// the decode window `value`, and the input cursor `bytes` register-resident
/// across a whole batch instead of round-tripping them through the `Decoder`.
#[inline(always)]
fn decode_step(
    state: &mut ArithState,
    value: &mut u64,
    bytes: &mut &[u8],
    probability: Probability,
) -> bool {
    let (out, sz) = state.decode(probability, *value);
    for _ in 0..sz {
        let byte = if let Some((&b, r)) = bytes.split_first() {
            *bytes = r;
            b
        } else {
            0
        };
        *value = (*value << 8) + byte as u64;
    }
    out
}

impl<'a> SymbolDecoder for Decoder<'a> {
    /// Unlike `Ans`, `Range` asks for the speculating walk — its u64-division
    /// symbol step provides the latency shadow that absorbs the speculation's
    /// extra instructions (measured −4…−17% at value counts ≥ 4); see the
    /// walk inventory in `atmost::walks`.
    const SPECULATES: bool = true;

    /// Whole-symbol decode step, the inverse of `Range::encode_symbol`:
    /// recover the slot with one division, let `walk` recover the value and
    /// interval (adapting its contexts), then do a single narrowing +
    /// renormalization. State is kept in locals across the whole symbol
    /// (register-resident), as in `decode_bits`.
    #[inline]
    fn decode_symbol_step(&mut self, walk: impl FnOnce(u32) -> (SymbolRange, usize)) -> usize {
        let mut state = self.state;
        let mut value = self.value;
        let mut bytes = self.bytes;
        let pull = |state: &mut ArithState, value: &mut u64, bytes: &mut &[u8]| {
            let n = state.consume_decoded_bytes();
            for _ in 0..n {
                let byte = if let Some((&b, r)) = bytes.split_first() {
                    *bytes = r;
                    b
                } else {
                    0
                };
                *value = (*value << 8) + byte as u64;
            }
        };
        while state.clamp_for_symbol() {
            pull(&mut state, &mut value, &mut bytes);
        }
        let slot = state.symbol_slot(value);
        let (range, decoded) = walk(slot);
        state.narrow_symbol(range);
        pull(&mut state, &mut value, &mut bytes);
        self.state = state;
        self.value = value;
        self.bytes = bytes;
        decoded
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Whole `AtMost` symbol decode; see [`SymbolDecoder::decode_symbol_step`].
    #[inline]
    fn decode_atmost<const MAX: usize>(&mut self, ctx: &mut AtMostContext<MAX>) -> AtMost<MAX> {
        walks::decode_symbol_or_bitwise(self, ctx)
    }

    /// Adaptive batch decode, fused into a single pass (mirrors the `Ans`
    /// override). We keep `state`/`value`/`bytes` in locals and do lookup → decode
    /// → adapt in one pass, touching each independent context once, rather than
    /// re-reading the decoder fields every bit.
    #[inline]
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        let mut state = self.state;
        let mut value = self.value;
        let mut bytes = self.bytes;
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut value, &mut bytes, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        self.state = state;
        self.value = value;
        self.bytes = bytes;
        bits
    }

    #[inline]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        // By the W_DELAY splice, the run sits at the cursor right now: the
        // entropy `pull`s that fill the window read exactly W_DELAY bytes ahead,
        // so the cursor lands on this run's first byte. Read it straight off.
        if self.bytes.len() < bytes.len() {
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                self.bytes.len(),
                bytes.len()
            )));
        }
        let (b, rest) = self.bytes.split_at(bytes.len());
        bytes.copy_from_slice(b);
        self.bytes = rest;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU8;

    use rand::Rng;

    use super::*;

    fn rand_prob() -> (Probability, bool) {
        let value_bool = rand::random::<bool>();
        (rand::random::<Probability>(), value_bool)
    }

    #[test]
    fn encode_decode_last_byte() {
        fn test_state(original_s: ArithState) {
            assert_eq!(
                original_s.clone().ready_bytes().count,
                0,
                "state should already be regularized!"
            );
            assert!(original_s.hi > original_s.lo);
            // println!("\noriginal_s is {original_s:x?}");
            // println!("================================");
            for value_bool in [false, true] {
                let (p, _) = rand_prob();

                let mut s = original_s;
                let encoded_bytes = s.encode(p, value_bool);
                // println!("state after encoding {value_bool:?} is {s:x?}");

                let split = original_s.split(p);

                let values = if value_bool {
                    let rand_value = || rand::thread_rng().gen_range(split + 1..=original_s.hi);
                    vec![split + 1, original_s.hi, rand_value(), rand_value()]
                } else {
                    let rand_value = || rand::thread_rng().gen_range(original_s.lo..=split);
                    vec![original_s.lo, split, rand_value(), rand_value()]
                };
                // println!("\nsplit is {split:x} and choice is {value_bool:?}");
                for value in values {
                    // println!("\n  value={value:x} for {original_s:x?} and {value_bool:?}");
                    let mut decoding_s = original_s;
                    let (decoded, sz) = decoding_s.decode(p, value);
                    // println!("  after decoding {decoded:?} from {value:x} is {decoding_s:x?}");
                    assert_eq!(sz, encoded_bytes.count);
                    assert_eq!(decoded, value_bool);
                    assert_eq!(s, decoding_s);
                }
            }
        }

        test_state(ArithState {
            lo: u64::MAX / 2,
            hi: u64::MAX / 2 + 1,
        });

        let mut s = ArithState::default();
        for _ in 0..10_000 {
            // create a valid state
            s.lo = rand::random();
            if s.lo == u64::MAX {
                s.lo = 0;
            }
            s.hi = s.lo + 1 + (rand::random::<u64>() % (u64::MAX - s.lo));
            println!("initially s is {s:x?}");
            assert!(s.hi > s.lo);
            s.ready_bytes();
            println!("after regularization s is {s:x?}");
            test_state(s);
        }
    }

    #[test]
    fn zero_byte() {
        let mut s = ArithState::default();
        for _ in 0..7 {
            assert_eq!(
                s.encode(
                    Probability {
                        prob: NonZeroU8::new(127).unwrap()
                    },
                    false,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: NonZeroU8::new(127).unwrap(),
            },
            false,
        );
        assert_eq!(bytes.count, 1);
        assert_eq!(bytes.bytes, [0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn one_byte() {
        let mut s = ArithState::default();
        assert_eq!(
            s.split(Probability {
                prob: NonZeroU8::new(128).unwrap()
            }) >> 8,
            (u64::MAX / 2) >> 8
        );
        for _ in 0..8 {
            assert_eq!(
                s.encode(
                    Probability {
                        prob: NonZeroU8::new(127).unwrap()
                    },
                    true,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: NonZeroU8::new(127).unwrap(),
            },
            true,
        );
        assert_eq!(bytes.count, 1);
        assert_eq!(bytes.bytes, [255, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn symbol_state_roundtrip() {
        // Symbol narrowing must round-trip from every reachable state,
        // including adversarially narrow straddled intervals that force the
        // clamp renormalization.
        fn test_state(s: ArithState) {
            // encoder side: clamp until wide enough (bytes are emitted by the
            // caller in the real coder; here we only track state agreement)
            let mut enc = s;
            let mut enc_clamp_bytes = Vec::new();
            while enc.clamp_for_symbol() {
                enc_clamp_bytes.extend_from_slice(&enc.ready_bytes());
            }
            // decoder side must clamp identically, consuming the same count
            let mut dec = s;
            let mut dec_consumed = 0;
            while dec.clamp_for_symbol() {
                dec_consumed += dec.consume_decoded_bytes();
            }
            assert_eq!(enc, dec, "clamp must be deterministic in (lo, hi)");
            assert_eq!(enc_clamp_bytes.len(), dec_consumed);
            assert!(enc.hi - enc.lo >= ArithState::MIN_SYMBOL_WIDTH);

            // a random symbol interval
            let start = rand::random::<u32>() % SymbolRange::M;
            let width = 1 + rand::random::<u32>() % (SymbolRange::M - start);
            let range = SymbolRange::test_new(start, width);
            let mut narrowed = enc;
            narrowed.narrow_symbol(range);
            assert!(narrowed.lo >= enc.lo && narrowed.hi <= enc.hi);
            // every value in the narrowed interval must recover a slot inside
            // the coded range
            for value in [
                narrowed.lo,
                narrowed.hi,
                narrowed.lo + (narrowed.hi - narrowed.lo) / 2,
            ] {
                let slot = enc.symbol_slot(value);
                assert!(
                    slot >= start && slot < start + width,
                    "slot {slot} outside [{start}, {}) for {enc:x?} value {value:x}",
                    start + width
                );
            }
        }

        // the canonical narrowest straddle
        test_state(ArithState {
            lo: u64::MAX / 2,
            hi: u64::MAX / 2 + 1,
        });
        for _ in 0..10_000 {
            let mut s = ArithState::default();
            if rand::random::<bool>() {
                // adversarial: tiny width straddling a top-byte boundary
                let boundary = ((rand::random::<u64>() % 255) + 1) << 56;
                let below = rand::random::<u64>() % (1 << (rand::random::<u32>() % 40));
                let above = rand::random::<u64>() % (1 << (rand::random::<u32>() % 40));
                s.lo = boundary - 1 - below;
                s.hi = boundary + above;
            } else {
                s.lo = rand::random();
                if s.lo == u64::MAX {
                    s.lo = 0;
                }
                s.hi = s.lo + 1 + (rand::random::<u64>() % (u64::MAX - s.lo));
            }
            s.ready_bytes();
            if s.hi > s.lo {
                test_state(s);
            }
        }
    }

    #[test]
    fn encode_decode_symbols_and_bits() {
        super::super::check_mixed_bits_and_symbols!(Range, Decoder::new);
    }

    #[test]
    fn encode_decode() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 32 * 8;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_prob());
            }
            println!("\n\ntesting {probs:?}");
            let mut encoder = Range::default();
            for &(p, bit) in &probs {
                // `ArithState::encode` is the coder's bit primitive at an
                // arbitrary probability (the trait only offers context-driven
                // encoding).
                let ready = encoder.state.encode(p, bit);
                encoder.bytes.extend_from_slice(&ready);
            }
            let bytes = encoder.into_vec();
            println!("\n\nEncoded random as: {bytes:02x?}\n");
            let mut decoder = Decoder::new(&bytes);
            for &(p, bit) in &probs {
                println!("Decoding {p:?} {bit:?}");
                // `decode_step` is the coder's bit primitive at an arbitrary
                // probability (the trait only exposes context-driven decoding).
                let decoded = decode_step(
                    &mut decoder.state,
                    &mut decoder.value,
                    &mut decoder.bytes,
                    p,
                );
                assert_eq!(decoded, bit);
            }
        }
    }

    /// Adversarial round-trip for the delay-interleave splice (`W_DELAY`):
    /// coded values interleaved with incompressible runs of every length
    /// straddling `W_DELAY`, consecutive short runs, and runs at the very end
    /// (the tail edge case). A wrong delay, off-by-one at a splice, or botched
    /// tail is silent corruption, so this hammers the boundaries.
    #[test]
    fn delay_interleave_roundtrip_adversarial() {
        use crate::{Encoded, Incompressible};
        type Item = (u8, Encoded<Vec<u8>, Incompressible>);
        let mut x = 0x1234_5678_9abc_def0u64;
        let mut rng = || {
            x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
            x
        };
        for trial in 0..300u64 {
            let n = (rng() % 60) as usize;
            let mut items: Vec<Item> = Vec::new();
            for i in 0..n {
                // Cover 0..=20 deterministically on some trials (straddles
                // W_DELAY = 8), random on others.
                let len = if trial % 3 == 0 {
                    i % 21
                } else {
                    (rng() % 21) as usize
                };
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }
            // Force a non-empty raw run as the final element on half the trials
            // (the tail case, within W_DELAY of the end).
            if trial % 2 == 0 {
                let len = (trial as usize % 12) + 1;
                let run: Vec<u8> = (0..len).map(|_| rng() as u8).collect();
                items.push((rng() as u8, Encoded::new(run)));
            }
            let encoded = super::super::encode(&items);
            let decoded: Vec<Item> = super::super::decode(&encoded).unwrap();
            assert_eq!(decoded, items, "trial {trial}");
        }
    }
}
