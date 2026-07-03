mod probability;

use super::bit_context::BitContext;
use super::symbol::SymbolRange;
use super::{EntropyCoder, EntropyDecoder};
pub use probability::Probability;
mod bytes;
use bytes::Bytes;

type State = u32;
const STATE_BYTES: usize = std::mem::size_of::<State>();

const MAGIC_HAS_INCOMPRESSIBLE: u8 = 137;
const MAGIC_LACKS_INCOMPRESSIBLE: u8 = 173;

/// ANS entropy encoding.
///
/// Can be used to encode data.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::v2::Ans::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 24);
/// assert_eq!(compactly::v2::Ans::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ans {
    ops: Vec<Op>,
    incompressible_bytes: Vec<u8>,
}

/// One deferred coding operation. rANS runs the coder backwards over the whole
/// buffer in [`Ans::into_vec`], so symbols are recorded here next to bits to
/// preserve their interleaving. The symbol interval is stored packed
/// (`width` is in `1..=M`, so `width - 1` fits a `u16`) to keep the buffer
/// entry small.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Bit(bool, Probability),
    Symbol { start: u16, width_minus_1: u16 },
}

impl EntropyCoder for Ans {
    #[inline]
    fn encode_bits<const N: usize>(&mut self, bits_with_probabilities: [(bool, Probability); N]) {
        self.ops.extend(
            bits_with_probabilities
                .into_iter()
                .map(|(b, probability)| Op::Bit(b, probability)),
        );
    }

    #[inline]
    fn encode_tree<const N: usize>(&mut self, contexts: &mut [BitContext; N], value: usize) {
        if N < 2 {
            return;
        }
        let range = SymbolRange::for_value(contexts, value);
        self.ops.push(Op::Symbol {
            start: range.start() as u16,
            width_minus_1: (range.width() - 1) as u16,
        });
    }

    #[inline]
    fn encode_incompressible_bytes(&mut self, bytes: &[u8]) {
        self.incompressible_bytes.extend_from_slice(bytes);
    }
}
impl Ans {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        let mut reader = Decoder::from(bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
    /// Convert the encoded value in to a `Vec` of bytes.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        let mut coder = Encoder::new();
        let mut out = Vec::new();
        for op in self.ops.into_iter().rev() {
            match op {
                Op::Bit(b, probability) => {
                    if let Some(byte) = coder.encode(b, probability) {
                        out.push(byte);
                    }
                }
                Op::Symbol {
                    start,
                    width_minus_1,
                } => {
                    let (bytes, state) = coder
                        .state
                        .encode_symbol(start as State, width_minus_1 as State + 1);
                    coder.state = state;
                    out.extend(bytes);
                }
            }
        }
        out.extend(coder.finish_encoding());

        if !self.incompressible_bytes.is_empty() {
            let mut len = self.incompressible_bytes.len();
            // This is a funny tweak on LEB128.  We encode the length as 7-bit
            // bytes that are encoded little-endian, but then reversed and
            // decoded big-endian.  The "final" byte is indicated by the most
            // significant bit being set.
            out.push((len & 127) as u8 | 128);
            len >>= 7;
            while len > 0 {
                out.push((len & 127) as u8);
                len >>= 7;
            }
            out.push(MAGIC_HAS_INCOMPRESSIBLE);
            out.reverse();
            // Add the incompressible bytes in reverse at the end of the output, so
            // that we can read them back without knowing how many incompressible
            // bytes there are.
            out.extend_from_slice(&self.incompressible_bytes);
        } else {
            let last = out.last().copied();
            if last == Some(MAGIC_HAS_INCOMPRESSIBLE) || last == Some(MAGIC_LACKS_INCOMPRESSIBLE) {
                out.push(MAGIC_LACKS_INCOMPRESSIBLE);
            }
            out.reverse();
        }
        out
    }
}
impl From<Ans> for Vec<u8> {
    fn from(value: Ans) -> Self {
        value.into_vec()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Encoder {
    state: StateOnly,
}

impl Encoder {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            state: StateOnly { state: 0 },
        }
    }

    /// Encode a bit using distribution Bernoulli(probability).
    #[inline(always)]
    fn encode(&mut self, b: bool, probability: Probability) -> Option<u8> {
        let (out, state) = self.state.encode(b, probability);
        self.state = state;
        out
    }

    #[inline(always)]
    pub fn finish_encoding(&mut self) -> Bytes {
        let mut bytes = Bytes::default();
        while self.state.state != 0 {
            bytes.push(self.state.state as u8);
            self.state.state >>= 8;
        }
        bytes
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Decoder<'a> {
    state: StateOnly,
    /// The compressed bytes.
    bytes: &'a [u8],
    /// The incompressible set of bytes.
    incompressible: &'a [u8],
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        let mut state: State = 0;
        let first = bytes.first().copied();
        let (bytes, incompressible) = if first == Some(MAGIC_LACKS_INCOMPRESSIBLE) {
            (&bytes[1..], [].as_slice())
        } else if first == Some(MAGIC_HAS_INCOMPRESSIBLE) {
            let mut bytes = &bytes[1..];
            let mut incompressible_len = 0;
            while let Some((&b, rest)) = bytes.split_first() {
                bytes = rest;
                incompressible_len = (incompressible_len << 7) | (b & 127) as usize;
                if b & 127 != b {
                    break;
                }
            }
            bytes.split_at(bytes.len() - incompressible_len)
        } else {
            (bytes, [].as_slice())
        };
        if bytes.len() < STATE_BYTES {
            for &b in bytes.iter() {
                state = state << 8 | State::from(b);
            }
            let state = StateOnly { state };
            Self {
                state,
                bytes: &[],
                incompressible,
            }
        } else {
            let state = State::from_be_bytes(bytes[0..STATE_BYTES].try_into().unwrap());
            let bytes = &bytes[STATE_BYTES..];
            let state = StateOnly { state };
            Self {
                state,
                bytes,
                incompressible,
            }
        }
    }
}

/// One rANS bit-decode step, operating on locals so the caller can keep `state`
/// and the input cursor `bytes` register-resident across a whole batch.
#[inline(always)]
fn decode_step(state: &mut State, bytes: &mut &[u8], probability: Probability) -> bool {
    let ones = State::from(probability);
    let zeros = 256 - ones;
    let z = *state & 255;
    let b = z >= ones;
    let s = *state >> 8;
    // Branchless: compute both paths and select via CMOV.
    let state_b = (s * zeros).wrapping_add(z.wrapping_sub(ones));
    let state_nb = s * ones + z;
    let mut new_s = if b { state_b } else { state_nb };
    if new_s < (1 << (State::BITS - 8)) {
        if let Some((&byte, rest)) = bytes.split_first() {
            *bytes = rest;
            new_s = (new_s << 8) | byte as State;
        }
    }
    *state = new_s;
    b
}

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Whole-tree symbol decode: peek the low [`SymbolRange::BITS`] bits of the
    /// state as the slot, walk the tree to recover value and interval, then do
    /// a single rANS advance + renormalization instead of `log2(N)` of them.
    ///
    /// The bit steps (total 256) and symbol steps (total `M = 2^16`) share the
    /// same normalization interval `[2^24, 2^32)`, so they can interleave
    /// freely in one state/stream; a symbol step may need to pull up to two
    /// bytes where a bit step pulls at most one.
    #[inline(always)]
    fn decode_tree<const N: usize>(&mut self, contexts: &mut [BitContext; N]) -> usize {
        if N < 2 {
            return 0;
        }
        let mut state = self.state.state;
        let mut bytes = self.bytes;
        let slot = state & (SymbolRange::M - 1);
        let (range, value) = SymbolRange::from_slot(contexts, slot);
        state = range.width() * (state >> SymbolRange::BITS) + (slot - range.start());
        while state < (1 << (State::BITS - 8)) {
            let Some((&byte, rest)) = bytes.split_first() else {
                break;
            };
            bytes = rest;
            state = (state << 8) | byte as State;
        }
        self.state.state = state;
        self.bytes = bytes;
        value
    }

    /// Adaptive batch decode, fused into a single pass.
    ///
    /// We pull `state`/`bytes` into locals and do probability-lookup, decode, and
    /// `adapt` in one pass, keeping the coder state register-resident across the
    /// run rather than re-reading the `Decoder` every bit. The contexts are
    /// independent, so adapting bit `i` never changes bit `j`'s probability — the
    /// result is identical to the per-bit default.
    #[inline(always)]
    fn decode_bits<const N: usize>(&mut self, contexts: &mut [BitContext; N]) -> [bool; N] {
        let mut state = self.state.state;
        let mut bytes = self.bytes;
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = decode_step(&mut state, &mut bytes, context.probability());
            *context = context.adapt(bit);
            *b = bit;
        }
        self.state.state = state;
        self.bytes = bytes;
        bits
    }

    #[inline(always)]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        if self.incompressible.len() < bytes.len() {
            return Err(std::io::Error::other(format!(
                "insufficient incompressible bytes: {} < {}",
                self.bytes.len(),
                bytes.len()
            )));
        }
        let (b, incompressible) = self.incompressible.split_at(bytes.len());
        self.incompressible = incompressible;
        bytes.copy_from_slice(b);
        Ok(())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct StateOnly {
    state: State,
}
impl StateOnly {
    #[inline(always)]
    fn encode(mut self, b: bool, probability: Probability) -> (Option<u8>, Self) {
        let mut out = None;
        let ones = State::from(probability);
        let zeros = 256 - ones;
        // we use uniform of size matching the bit value to decode from state first
        let freq = if b { zeros } else { ones };
        // shift data from state to bulk when it grows too much
        if self.state >> (State::BITS - 8) >= freq {
            out = Some(self.state as u8);
            self.state >>= 8;
        }
        // the code really starts here, decode digit from freq base
        let mut z = self.state % freq;
        if b {
            z += ones;
        }
        // now encode new digit from 256 base
        (
            out,
            Self {
                state: (self.state / freq) * 256 + z,
            },
        )
    }

    /// Encode a whole tree symbol occupying `[start, start + width)` of the
    /// total `M = 2^16`. Same rANS scheme as [`StateOnly::encode`] but with a
    /// 16-bit total instead of 8, so renormalization can emit up to two bytes.
    /// Shares the bit steps' normalization interval `[2^24, 2^32)`.
    #[inline(always)]
    fn encode_symbol(mut self, start: State, width: State) -> (Bytes, Self) {
        let mut bytes = Bytes::default();
        // Emit while state >= width << 16 (kept shift-free: width can be 2^16).
        while self.state >> SymbolRange::BITS >= width {
            bytes.push(self.state as u8);
            self.state >>= 8;
        }
        self.state = ((self.state / width) << SymbolRange::BITS) | (self.state % width + start);
        (bytes, self)
    }

    /// The decode counterpart to [`StateOnly::encode_symbol`], for tests; the
    /// trait's `decode_tree` inlines this same logic.
    #[cfg(test)]
    fn decode_symbol(
        mut self,
        start: State,
        width: State,
        mut next_byte: impl FnMut() -> Option<u8>,
    ) -> Self {
        let slot = self.state & (SymbolRange::M - 1);
        debug_assert!(slot >= start && slot < start + width);
        self.state = width * (self.state >> SymbolRange::BITS) + (slot - start);
        while self.state < 1 << (State::BITS - 8) {
            let Some(byte) = next_byte() else { break };
            self.state = (self.state << 8) | State::from(byte);
        }
        self
    }

    /// The decode counterpart to [`StateOnly::encode`]. The `Ans` decoder's
    /// `decode_step` inlines this same logic; this stand-alone version exists so
    /// `check_state_only` can unit-test the encode/decode round-trip directly.
    #[cfg(test)]
    #[inline(always)]
    fn decode(
        mut self,
        probability: Probability,
        next_byte: impl FnOnce() -> Option<u8>,
    ) -> (bool, Self) {
        let ones = State::from(probability);
        let zeros = 256 - ones;
        let z = self.state & 255;
        let b = z >= ones;
        self.state >>= 8;
        // Branchless: compute both paths and select via CMOV.
        // z.wrapping_sub(ones) is only used when b=true (z >= ones), so no actual underflow.
        let state_b = (self.state * zeros).wrapping_add(z.wrapping_sub(ones));
        let state_nb = self.state * ones + z;
        self.state = if b { state_b } else { state_nb };
        if self.state < 1 << (State::BITS - 8) {
            if let Some(u) = next_byte() {
                self.state = (self.state << 8) | State::from(u);
            }
        }
        (b, self)
    }
}

#[test]
fn check_state_only() {
    for probability in (1..255).map(|i| Probability {
        prob: i.try_into().unwrap(),
    }) {
        for state in (0 as State..u16::MAX as State)
            // .chain((0..u16::MAX as State).map(|i| u32::MAX as State - i))
            // .chain((0..u16::MAX as State).map(|i| u32::MAX as State + i))
            .chain((0..u16::MAX as State).map(|i| State::MAX - i))
        {
            for b in [true, false] {
                // println!("Testing with state={state:x} probability={probability:?} bool={b}");
                let (mut next_byte, s) = StateOnly { state }.encode(b, probability);
                let next = || next_byte.take();
                let (bout, again) = s.decode(probability, next);
                assert_eq!(bout, b);
                assert_eq!(again.state, state);
                // If encoding produced a byte, then decoding must consume it.
                assert!(next_byte.is_none());
            }
        }
    }
}

#[test]
fn check_state_only_symbol() {
    // Symbol steps must round-trip from every reachable state region, for
    // every interval shape including extreme widths (the reserve-clamped
    // trees produce widths from 1 up to M/2 and starts across all of M).
    let mut cases: Vec<(State, State)> = vec![
        (0, 1),
        (65535, 1),
        (0, 65536),
        (0, 32768),
        (32768, 32768),
        (255, 256),
    ];
    for _ in 0..200 {
        let start = rand::random::<u32>() % SymbolRange::M;
        let width = 1 + rand::random::<u32>() % (SymbolRange::M - start);
        cases.push((start, width));
    }
    for &(start, width) in &cases {
        for state in (0 as State..u16::MAX as State)
            .chain((0..u16::MAX as State).map(|i| State::MAX - i))
            .step_by(97)
        {
            let (bytes, s) = StateOnly { state }.encode_symbol(start, width);
            // The encoded state's low bits are the slot, inside the interval.
            assert!(s.state & (SymbolRange::M - 1) >= start);
            let mut emitted: Vec<u8> = bytes.iter().copied().collect();
            // decode pulls in the reverse order of emission
            let again = s.decode_symbol(start, width, || emitted.pop());
            assert_eq!(
                again.state, state,
                "symbol round-trip failed for start={start} width={width} state={state:#x}"
            );
            assert!(emitted.is_empty(), "decode must consume all emitted bytes");
        }
    }
}

#[test]
fn check_ans_mixed_bits_and_symbols() {
    // Bits (total 256) and tree symbols (total 2^16) share one state and
    // stream; interleavings of the two must round-trip.
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
        // Adapting contexts: encode and decode must walk identical context
        // state, so use a fixed context array driven by the same data.
        let mut encode_contexts = [BitContext::default(); 256];
        let mut writer = Ans::default();
        for op in &plan {
            match *op {
                Planned::Bit(b, probability) => writer.encode_bit(probability, b),
                Planned::Byte(b) => writer.encode_tree(&mut encode_contexts, b as usize),
            }
        }
        let encoded = writer.into_vec();
        let mut decoder = Decoder::from(encoded.as_slice());
        let mut decode_contexts = [BitContext::default(); 256];
        for (i, op) in plan.iter().enumerate() {
            match *op {
                Planned::Bit(b, probability) => {
                    let bit = decode_step(&mut decoder.state.state, &mut decoder.bytes, probability);
                    assert_eq!(bit, b, "bit {i} of trial {trial}");
                }
                Planned::Byte(b) => {
                    let v = decoder.decode_tree(&mut decode_contexts);
                    assert_eq!(v, b as usize, "byte {i} of trial {trial}");
                }
            }
        }
        assert_eq!(encode_contexts, decode_contexts);
    }
}

#[test]
fn check_ans_coder() {
    for size in (0..32).chain([100, 1_000, 10_000]) {
        println!("testing with size {size}");
        for _ in 0..size.min(1000) + 1000 {
            let mut data = Vec::new();
            data.resize_with(size, rand::random::<bool>);
            let mut distros = Vec::new();
            distros.resize_with(size, rand::random::<Probability>);
            let mut writer = Ans::default();
            for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
                // rev here
                writer.encode_bit(probability, b);
            }
            let bytes = writer.into_vec();
            let mut decoder = Decoder::from(bytes.as_slice());
            for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
                // println!("checking {b} {probability}");
                // `decode_step` is the coder's bit primitive at an arbitrary
                // probability (the trait only exposes context-driven decoding).
                let bit = decode_step(&mut decoder.state.state, &mut decoder.bytes, probability);
                assert_eq!(bit, b);
            }
            assert_eq!(decoder.state.state, 0);
        }
    }
}

#[test]
fn ans_is_reasonable() {
    let data = vec![true; 1024 * 8];
    assert_eq!(super::Range::encode(&data).len(), 12);
    assert_eq!(Ans::decode::<Vec<bool>>(&Ans::encode(&data)).unwrap(), data);
    assert_eq!(Ans::encode(&data).len(), 18);
}

#[cfg(test)]
mod test {
    use super::super::bit_context::BitContext;
    use super::*;

    fn rand_context() -> (BitContext, bool) {
        let value_bool = rand::random::<bool>();
        (rand::random::<BitContext>(), value_bool)
    }

    #[test]
    fn normal() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 256;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_context());
            }
            println!("\n\ntesting {probs:?}");
            let mut encoder = Ans::default();

            for &(p, bit) in &probs {
                encoder.encode_bit(p.probability(), bit);
            }

            let bytes = encoder.into_vec();

            let mut decoder = Decoder::from(bytes.as_slice());

            for &(p, bit) in &probs {
                println!("Decoding before {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
        }
    }

    #[test]
    fn incompressible() {
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 256;
            let mut probs = Vec::new();
            let mut after_probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_context());
                after_probs.push(rand_context());
            }
            let num_inc = rand::random::<usize>() % 9;
            let mut inc = Vec::new();
            for _ in 0..num_inc {
                // Attempt to get random bytes with a wide distribution of
                // number of bits required.
                let mut num_bytes = rand::random::<usize>() % 9;
                if num_bytes == 8 {
                    num_bytes = rand::random::<usize>() % 512;
                    if num_bytes > 500 {
                        num_bytes = rand::random::<usize>() % 512_000;
                    }
                }
                let mut bytes: Vec<u8> = Vec::new();
                for _ in 0..num_bytes {
                    bytes.push(rand::random());
                }
                inc.push(bytes);
            }
            println!("\n\ntesting {probs:?}\n\n{inc:?}");
            let mut encoder = Ans::default();

            for &(p, bit) in &probs {
                encoder.encode_bit(p.probability(), bit);
            }
            for bytes in &inc {
                encoder.encode_incompressible_bytes(bytes);
            }
            for &(p, bit) in &after_probs {
                encoder.encode_bit(p.probability(), bit);
            }

            let bytes = encoder.into_vec();
            println!("\n\nEncoded random as: {bytes:02x?}\n");

            println!(
                "encoded ends with incompressible {:?}",
                &bytes[bytes.len() - inc.iter().map(|x| x.len()).sum::<usize>()..]
            );

            let mut decoder = Decoder::from(bytes.as_slice());

            for &(p, bit) in &probs {
                println!("Decoding before {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
            for b in &inc {
                println!("decoding {b:?}");
                let mut v = vec![0u8; b.len()];
                decoder.decode_incompressible_bytes(&mut v).unwrap();
                assert_eq!(&v, b);
            }
            for &(p, bit) in &after_probs {
                println!("Decoding after {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()), bit);
            }
        }
    }
}
