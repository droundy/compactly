use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithState {
    lo: u64,
    hi: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Probability {
    pub prob: u64,
    pub shift: u8,
}

impl Probability {
    pub fn likely_bit(&self) -> bool {
        self.prob < (1 << (self.shift - 1))
    }
    pub fn as_f64(self) -> f64 {
        self.prob as f64 / (1_u64 << self.shift) as f64
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.prob as f64 / (1_u64 << self.shift) as f64;
        write!(f, "{v}")
    }
}

#[test]
fn likely_bit() {
    assert_eq!((Probability { prob: 1, shift: 2 }).likely_bit(), true);
    assert_eq!((Probability { prob: 2, shift: 2 }).likely_bit(), false);
    assert_eq!((Probability { prob: 3, shift: 2 }).likely_bit(), false);
}

impl Default for ArithState {
    fn default() -> Self {
        ArithState {
            lo: 0,
            hi: u64::MAX,
        }
    }
}

impl ArithState {
    fn ready_bytes(&mut self) -> Bytes {
        if self.hi == self.lo {
            let bytes = self.hi.to_be_bytes();
            self.lo = 0;
            self.hi = u64::MAX;
            Bytes { bytes, count: 8 }
        } else {
            let mut bytes = Bytes::default();
            for _ in 0..7 {
                let lo_byte = (self.lo >> 56) as u8;
                let hi_byte = (self.hi >> 56) as u8;
                debug_assert_ne!(self.lo, self.hi);
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
            bytes
        }
    }

    pub fn last_byte(self) -> u8 {
        (self.hi >> 56) as u8
    }

    /// Returns a set of bytes to be written out.
    #[must_use]
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
        let split = self
            .split(prob)
            .expect("call next_byte enough before encode");
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
    pub fn decode(&mut self, prob: Probability, value: u64) -> (bool, usize) {
        if self.hi == self.lo + 1 {
            let bit = value == self.hi;
            self.hi = u64::MAX;
            self.lo = 0;
            return (bit, 8);
        }
        let split = self
            .split(prob)
            .expect("call next_byte enough before decode");
        let b = value > split;
        // println!("decoded bit {prob} {shift} {b:?}   from {value:016x} and split {split:016x}");
        if b {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        (b, self.ready_bytes().count)
    }

    fn split(self, Probability { prob, shift }: Probability) -> Option<u64> {
        debug_assert!(prob < 1 << shift);
        debug_assert!(self.hi > self.lo);
        let width = self.hi - self.lo;
        if self.lo >> 56 == self.hi >> 56 {
            None
        } else {
            Some(self.lo + (width >> shift) * prob)
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Bytes {
    bytes: [u8; 8],
    count: usize,
}

impl Bytes {
    fn push(&mut self, byte: u8) {
        self.bytes[self.count] = byte;
        self.count += 1;
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.bytes[..self.count]
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = std::iter::Take<std::array::IntoIter<u8, 8>>;
    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().take(self.count)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoder {
    bytes: Vec<u8>,
    state: ArithState,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            state: ArithState::default(),
        }
    }
    pub fn encode(&mut self, probability_of_false: Probability, value: bool) {
        self.bytes
            .extend_from_slice(&self.state.encode(probability_of_false, value));
    }
    pub fn finish(mut self) -> Vec<u8> {
        self.bytes.push(self.state.last_byte());
        self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Writer<W> {
    write: W,
    state: ArithState,
}

impl<W: Write> Writer<W> {
    pub fn new(write: W) -> Self {
        Self {
            write,
            state: ArithState::default(),
        }
    }
    pub fn encode(
        &mut self,
        probability_of_false: Probability,
        value: bool,
    ) -> std::io::Result<()> {
        let bytes = self.state.encode(probability_of_false, value);
        if bytes.count > 0 {
            self.write.write(&bytes)?;
        }
        Ok(())
    }
    pub fn finish(mut self) -> std::io::Result<()> {
        self.write.write(&[self.state.last_byte()])?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder {
    bytes: Vec<u8>,
    state: ArithState,
    value: u64,
}

impl Decoder {
    pub fn new(mut bytes: Vec<u8>) -> Self {
        bytes.reverse();
        let mut value = 0;
        for _ in 0..8 {
            value = (value << 8) + bytes.pop().unwrap_or_default() as u64;
        }
        Self {
            bytes,
            state: ArithState::default(),
            value,
        }
    }
    pub fn decode(&mut self, p: Probability) -> bool {
        let (out, sz) = self.state.decode(p, self.value);
        for _ in 0..sz {
            self.value = (self.value << 8) + self.bytes.pop().unwrap_or_default() as u64;
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reader<R> {
    read: R,
    state: ArithState,
    value: u64,
}

impl<R: Read> Reader<R> {
    fn read_bytes(&mut self, sz: usize) -> Result<(), std::io::Error> {
        if sz == 0 {
            return Ok(());
        }
        let mut bytes = [0; 8];
        let mut bytes_to_read = &mut bytes[(8 - sz)..];
        while !bytes_to_read.is_empty() {
            let bytes_read = self.read.read(bytes_to_read)?;
            if bytes_read == 0 {
                // we have a small value and that is find, the remaining bytes are zero.
                break;
            }
            bytes_to_read = &mut bytes_to_read[bytes_read..];
        }
        let value = u64::from_be_bytes(bytes);
        if sz == 8 {
            self.value = value;
        } else {
            self.value = value + (self.value << (8 * sz));
        }
        Ok(())
    }
    pub fn new(read: R) -> std::io::Result<Self> {
        let mut r = Self {
            value: 0,
            read,
            state: ArithState::default(),
        };
        r.read_bytes(8)?;
        Ok(r)
    }
    pub fn decode(&mut self, p: Probability) -> std::io::Result<bool> {
        let (out, sz) = self.state.decode(p, self.value);
        self.read_bytes(sz)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn rand_prob() -> (Probability, bool) {
        let value_bool = rand::random::<bool>();
        let shift = 1 + (rand::random::<u8>() % 15);
        let prob = 1 + (rand::random::<u64>() % ((1 << shift) - 1));
        (Probability { prob, shift }, value_bool)
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

                let split = original_s.split(p).unwrap();

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
                        prob: 127,
                        shift: 8,
                    },
                    false,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: 127,
                shift: 8,
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
                prob: 128,
                shift: 8
            })
            .map(|v| v >> 8),
            Some((u64::MAX / 2) >> 8)
        );
        for _ in 0..8 {
            assert_eq!(
                s.encode(
                    Probability {
                        prob: 127,
                        shift: 8,
                    },
                    true,
                )
                .count,
                0
            );
        }
        let bytes = s.encode(
            Probability {
                prob: 127,
                shift: 8,
            },
            true,
        );
        assert_eq!(bytes.count, 1);
        assert_eq!(bytes.bytes, [255, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn encode_decode() {
        for probs in [
            vec![],
            vec![(1u64, 1u8, false)],
            vec![(1u64, 1u8, true)],
            vec![
                (1u64, 1u8, true),
                (1u64, 1u8, true),
                (1u64, 1u8, true),
                (1u64, 1u8, true),
                (1u64, 1u8, true),
            ],
            vec![
                (1u64, 1u8, true),
                (1u64, 1u8, false),
                (1u64, 1u8, true),
                (1u64, 1u8, false),
                (1u64, 1u8, true),
            ],
            vec![(1u64, 2u8, false)],
            vec![(1u64, 2u8, true)],
            vec![(2063, 13, false), (46, 7, true), (441, 12, true)],
            vec![
                (3, 2, true),
                (5, 9, false),
                (6997, 14, false),
                (16, 5, false),
                (4, 5, false),
                (28478, 15, false),
                (14625, 15, false),
                (103, 7, false),
                (1, 1, false),
                (3, 2, true),
                (178, 10, false),
            ],
        ] {
            println!("\nTest {probs:?}");
            let mut encoder = Encoder::new();
            for &(prob, shift, bit) in &probs {
                let p = Probability { prob, shift };
                encoder.encode(p, bit);
            }
            println!("{encoder:x?}");
            let bytes = encoder.finish();
            println!("\n\nEncoded: {bytes:02x?}\n");
            let mut decoder = Decoder::new(bytes);
            for &(prob, shift, bit) in &probs {
                println!("Decoding {prob} {shift} {bit:?}");
                let p = Probability { prob, shift };
                assert_eq!(decoder.decode(p), bit);
            }
        }
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 32 * 8;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_prob());
            }
            println!("\n\ntesting {probs:?}");
            let mut encoder = Encoder::new();
            for &(p, bit) in &probs {
                encoder.encode(p, bit);
            }
            let bytes = encoder.finish();
            println!("\n\nEncoded random as: {bytes:02x?}\n");
            let mut decoder = Decoder::new(bytes);
            for &(p, bit) in &probs {
                println!("Decoding {p:?} {bit:?}");
                assert_eq!(decoder.decode(p), bit);
            }
        }
    }
}
