pub use super::ans::Probability;
use super::symbol::SymbolRange;
use super::{EntropyCoder, EntropyDecoder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithState {
    lo: u64,
    hi: u64,
}

pub const SHIFT: u8 = 8;

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

    #[inline]
    pub fn last_byte(self) -> u8 {
        let hi = (self.hi >> 56) as u8;
        let lo = (self.lo >> 56) as u8;
        // when convenient, we'd like to avoid ending with a magic byte.
        if hi == MAGIC_HAS_INCOMPRESSIBLE[1] || hi == MAGIC_LACKS_INCOMPRESSIBLE[1] {
            if lo < hi && lo + 1 < hi {
                // being cautious unless lo == hi == 255
                lo + 1
            } else {
                hi
            }
        } else {
            hi
        }
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

    /// Minimum interval width required before coding a whole 16-bit tree
    /// symbol in one step. Guarantees every one of the `2^16` slots spans at
    /// least `2^16` values, so slot boundaries are exact and the top-slot
    /// rounding waste is at most a `2^-16` fraction of the interval. The
    /// per-bit path tolerates arbitrarily narrow intervals, so this is only
    /// enforced (via [`ArithState::clamp_for_symbol`]) on the symbol path.
    const MIN_SYMBOL_WIDTH: u64 = 1 << 32;

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

/// Use range coding to encode bits.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Range::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 23);
/// assert_eq!(compactly::v2::Range::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    bytes: Vec<u8>,
    incompressible_bytes: Vec<u8>,
    state: ArithState,
}

impl EntropyCoder for Range {
    #[inline]
    fn encode_bits<const N: usize>(&mut self, bits_with_probabilities: [(bool, Probability); N]) {
        for (value, probability_of_false) in bits_with_probabilities {
            self.bytes
                .extend_from_slice(&self.state.encode(probability_of_false, value));
        }
    }

    /// Whole-tree symbol encode: one interval narrowing + renormalization for
    /// the `log2(N)`-bit symbol instead of one per bit.
    #[inline]
    fn encode_tree<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        value: usize,
    ) {
        if N < 2 {
            return;
        }
        while self.state.clamp_for_symbol() {
            self.bytes.extend_from_slice(&self.state.ready_bytes());
        }
        let range = SymbolRange::for_value(contexts, value);
        self.state.narrow_symbol(range);
        self.bytes.extend_from_slice(&self.state.ready_bytes());
    }

    #[inline]
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        self.incompressible_bytes.extend_from_slice(bytes);
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
    /// Convert the encoded value in to a `Vec` of bytes.
    #[inline]
    pub fn into_vec(mut self) -> Vec<u8> {
        self.bytes.push(self.state.last_byte());
        if self.incompressible_bytes.is_empty() {
            if self.bytes.last_chunk().copied() == Some(MAGIC_HAS_INCOMPRESSIBLE)
                || self.bytes.last_chunk().copied() == Some(MAGIC_LACKS_INCOMPRESSIBLE)
            {
                self.bytes.extend_from_slice(&MAGIC_LACKS_INCOMPRESSIBLE);
            }
            self.bytes
        } else {
            let mut len = self.incompressible_bytes.len();
            self.incompressible_bytes.extend_from_slice(&self.bytes);
            // This is a funny tweak on LEB128.  We encode the length as 7-bit
            // bytes that are encoded little-endian, but then we decode it in
            // reversed so it is decoded big-endian.  The "final" byte is
            // indicated by the most significant bit being set.
            self.incompressible_bytes.push((len & 127) as u8 | 128);
            len >>= 7;
            while len > 0 {
                self.incompressible_bytes.push((len & 127) as u8);
                len >>= 7;
            }
            self.incompressible_bytes
                .extend_from_slice(&MAGIC_HAS_INCOMPRESSIBLE);
            self.incompressible_bytes
        }
    }
}
impl From<Range> for Vec<u8> {
    fn from(value: Range) -> Self {
        value.into_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    state: ArithState,
    value: u64,
    /// The incompressible set of bytes.
    incompressible: &'a [u8],
}

const MAGIC_HAS_INCOMPRESSIBLE: [u8; 2] = [b'Y', b'a'];
const MAGIC_LACKS_INCOMPRESSIBLE: [u8; 2] = [b'N', b'o'];

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let last = bytes.last_chunk().copied();
        let (bytes, incompressible) = if last == Some(MAGIC_LACKS_INCOMPRESSIBLE) {
            (&bytes[..bytes.len() - 2], [].as_slice())
        } else if last == Some(MAGIC_HAS_INCOMPRESSIBLE) {
            let mut bytes = &bytes[..bytes.len() - 2];
            let mut incompressible_len = 0;
            while let Some((&b, rest)) = bytes.split_last() {
                bytes = rest;
                incompressible_len = (incompressible_len << 7) | (b & 127) as usize;
                if b & 127 != b {
                    break;
                }
            }
            let (incompressible, compressed) = bytes.split_at(incompressible_len);
            (compressed, incompressible)
        } else {
            (bytes, [].as_slice())
        };
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
            incompressible,
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

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Whole-tree symbol decode, the inverse of `Range::encode_tree`: recover
    /// the slot with one division, walk the tree to the value, then do a
    /// single narrowing + renormalization. State is kept in locals across the
    /// whole symbol (register-resident), as in `decode_bits`.
    #[inline]
    fn decode_tree<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> usize {
        if N < 2 {
            return 0;
        }
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
        let (range, decoded) = SymbolRange::from_slot(contexts, slot);
        state.narrow_symbol(range);
        pull(&mut state, &mut value, &mut bytes);
        self.state = state;
        self.value = value;
        self.bytes = bytes;
        decoded
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
        if self.incompressible.len() < bytes.len() {
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                self.incompressible.len(),
                bytes.len()
            )));
        }
        let (b, rest) = self.incompressible.split_at(bytes.len());
        bytes.copy_from_slice(b);
        self.incompressible = rest;
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
        // Full-stream mixed test through the real encoder/decoder: interleaved
        // bits and whole-tree byte symbols with shared adapting contexts.
        for trial in 0..2000 {
            let n_ops = rand::random::<usize>() % 200;
            #[derive(Debug, Clone, Copy)]
            enum Planned {
                Bit(bool, Probability),
                Byte(u8),
            }
            let mut plan = Vec::new();
            for _ in 0..n_ops {
                if rand::random::<bool>() {
                    plan.push(Planned::Bit(rand::random(), rand::random()));
                } else {
                    plan.push(Planned::Byte(rand::random()));
                }
            }
            let mut encode_contexts = [super::super::bit_context::BitContext::default(); 256];
            let mut encoder = Range::default();
            for op in &plan {
                match *op {
                    Planned::Bit(b, probability) => encoder.encode_bit(probability, b),
                    Planned::Byte(b) => encoder.encode_tree(&mut encode_contexts, b as usize),
                }
            }
            let bytes = encoder.into_vec();
            let mut decoder = Decoder::new(&bytes);
            let mut decode_contexts = [super::super::bit_context::BitContext::default(); 256];
            for (i, op) in plan.iter().enumerate() {
                match *op {
                    Planned::Bit(b, probability) => {
                        let decoded = decode_step(
                            &mut decoder.state,
                            &mut decoder.value,
                            &mut decoder.bytes,
                            probability,
                        );
                        assert_eq!(decoded, b, "bit {i} of trial {trial}");
                    }
                    Planned::Byte(b) => {
                        let decoded = decoder.decode_tree(&mut decode_contexts);
                        assert_eq!(decoded, b as usize, "byte {i} of trial {trial}");
                    }
                }
            }
            assert_eq!(encode_contexts, decode_contexts);
        }
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
                encoder.encode_bit(p, bit);
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
}
