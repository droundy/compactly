mod probability;

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
/// let encoded: Vec<u8> = compactly::ans::Ans::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 24);
/// assert_eq!(compactly::ans::Ans::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ans {
    bits: Vec<(bool, Probability)>,
    incompressible_bytes: Vec<u8>,
}

impl EntropyCoder for Ans {
    #[inline]
    fn encode_bit(&mut self, probability: Probability, bit: bool) {
        self.bits.push((bit, probability));
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
        for (b, probability) in self.bits.into_iter().rev() {
            if let Some(byte) = coder.encode(b, probability) {
                out.push(byte);
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

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Decode a bit using distribution Bernoulli(probability).
    #[inline(always)]
    fn decode_bit_nonadaptive(&mut self, probability: Probability) -> Result<bool, std::io::Error> {
        let (b, state) = self
            .state
            .decode(probability, || self.bytes.split_off_first().copied());
        self.state = state;
        Ok(b)
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

    #[inline(always)]
    fn decode(
        mut self,
        probability: Probability,
        next_byte: impl FnOnce() -> Option<u8>,
    ) -> (bool, Self) {
        let ones = State::from(probability);
        let zeros = 256 - ones;
        let mut z = self.state % 256;
        let b = z >= ones;
        self.state /= 256;
        if b {
            z -= ones;
            self.state = self.state * zeros + z;
        } else {
            self.state = self.state * ones + z;
        }
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
fn check_ans_coder() {
    for size in (0..32).chain([100, 1_000, 10_000]) {
        println!("testing with size {size}");
        for _ in 0..size.min(1000) + 1000 {
            let mut data = Vec::new();
            data.resize_with(size, || rand::random::<bool>());
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
                assert_eq!(decoder.decode_bit_nonadaptive(probability).unwrap(), b);
            }
            assert_eq!(decoder.state.state, 0);
        }
    }
}

#[test]
fn ans_is_reasonable() {
    let data = vec![true; 1024 * 8];
    assert_eq!(super::Range::encode(&data).len(), 16);
    assert_eq!(Ans::decode::<Vec<bool>>(&Ans::encode(&data)).unwrap(), data);
    assert_eq!(Ans::encode(&data).len(), 16);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ans::bit_context::BitContext;

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
                assert_eq!(decoder.decode_bit(&mut p.clone()).unwrap(), bit);
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
                assert_eq!(decoder.decode_bit(&mut p.clone()).unwrap(), bit);
            }
            for b in &inc {
                println!("decoding {b:?}");
                let mut v = vec![0u8; b.len()];
                decoder.decode_incompressible_bytes(&mut v).unwrap();
                assert_eq!(&v, b);
            }
            for &(p, bit) in &after_probs {
                println!("Decoding after {p:?} {bit:?}");
                assert_eq!(decoder.decode_bit(&mut p.clone()).unwrap(), bit);
            }
        }
    }
}
