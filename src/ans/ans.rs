mod probability;

use super::{EntropyCoder, EntropyDecoder};
pub use probability::Probability;
mod bytes;
use bytes::Bytes;

type State = u32;
const STATE_BYTES: usize = std::mem::size_of::<State>();

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
}

impl EntropyCoder for Ans {
    #[inline]
    fn encode_bit(&mut self, probability: self::Probability, bit: bool) {
        self.bits.push((bit, probability));
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
        out.reverse();
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
    bytes: &'a [u8],
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        let mut state: State = 0;
        if bytes.len() < STATE_BYTES {
            for &b in bytes.iter() {
                state = state << 8 | State::from(b);
            }
            let state = StateOnly { state };
            Self { state, bytes: &[] }
        } else {
            let state = State::from_be_bytes(bytes[0..STATE_BYTES].try_into().unwrap());
            let bytes = &bytes[STATE_BYTES..];
            let state = StateOnly { state };
            Self { state, bytes }
        }
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Decode a bit using distribution Bernoulli(probability).
    #[inline(always)]
    fn decode_bit_nonadaptive(
        &mut self,
        probability: self::Probability,
    ) -> Result<bool, std::io::Error> {
        let (b, state) = self
            .state
            .decode(probability, || self.bytes.split_off_first().copied());
        self.state = state;
        Ok(b)
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
