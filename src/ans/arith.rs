pub use super::ans::Probability;
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
                    self.lo = self.lo << 8;
                    self.hi = self.hi << 8;
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
        // println!("decoded bit {prob} {shift} {b:?}   from {value:016x} and split {split:016x}");
        if b {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        (b, self.ready_bytes().count)
    }

    #[inline]
    fn split(self, Probability { prob }: Probability) -> u64 {
        // debug_assert!(prob < 1 << SHIFT);
        debug_assert!(self.hi > self.lo);
        let width = self.hi - self.lo;
        debug_assert!(self.lo >> 56 != self.hi >> 56);
        self.lo + (width >> SHIFT) * prob.get() as u64
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
/// let encoded: Vec<u8> = compactly::ans::Range::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 23);
/// assert_eq!(compactly::ans::Range::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    bytes: Vec<u8>,
    state: ArithState,
}

impl EntropyCoder for Range {
    #[inline]
    fn encode_bit(&mut self, probability_of_false: Probability, value: bool) {
        self.bytes
            .extend_from_slice(&self.state.encode(probability_of_false, value));
    }
}

impl Range {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).into()
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        let mut reader = super::arith::Decoder::new(&bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
    /// Convert the encoded value in to a `Vec` of bytes.
    #[inline]
    pub fn into_vec(mut self) -> Vec<u8> {
        self.bytes.push(self.state.last_byte());
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

    fn pop_next_byte(&mut self) -> u8 {
        if let Some((&b, r)) = self.bytes.split_first() {
            self.bytes = r;
            b
        } else {
            0
        }
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    #[inline]
    fn decode_bit_nonadaptive(
        &mut self,
        probability: super::ans::Probability,
    ) -> Result<bool, std::io::Error> {
        let (out, sz) = self.state.decode(probability, self.value);
        for _ in 0..sz {
            self.value = (self.value << 8) + self.pop_next_byte() as u64;
        }
        Ok(out)
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
                assert_eq!(decoder.decode_bit_nonadaptive(p).unwrap(), bit);
            }
        }
    }
}
