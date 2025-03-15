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
    pub fn next_byte(&mut self) -> Option<u8> {
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
            Some(lo_byte)
        } else {
            None
        }
    }

    pub fn last_byte(self) -> u8 {
        (self.hi >> 56) as u8
    }

    pub fn encode(&mut self, Probability { prob, shift }: Probability, value: bool) {
        let split = self
            .split(prob, shift)
            .expect("call next_byte enough before encode");
        debug_assert!(split < self.hi - 1);
        debug_assert!(split > 0);
        if value {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        // println!("encoding {prob} {shift} {value:?}   with split {split:016x} gives {self:x?}");
    }

    pub fn decode(&mut self, Probability { prob, shift }: Probability, value: u64) -> bool {
        let split = self
            .split(prob, shift)
            .expect("call next_byte enough before decode");
        let b = value > split;
        // println!("decoded bit {prob} {shift} {b:?}   from {value:016x} and split {split:016x}");
        if b {
            self.lo = split + 1;
        } else {
            self.hi = split;
        }
        b
    }

    fn split(self, prob: u64, shift: u8) -> Option<u64> {
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
        while let Some(byte) = self.state.next_byte() {
            self.bytes.push(byte);
        }
        self.state.encode(probability_of_false, value);
    }
    pub fn finish(mut self) -> Vec<u8> {
        while let Some(byte) = self.state.next_byte() {
            self.bytes.push(byte);
        }
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
        while let Some(byte) = self.state.next_byte() {
            self.write.write(&[byte])?;
        }
        self.state.encode(probability_of_false, value);
        Ok(())
    }
    pub fn finish(mut self) -> std::io::Result<()> {
        while let Some(byte) = self.state.next_byte() {
            self.write.write(&[byte])?;
        }
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
        let out = self.state.decode(p, self.value);
        // println!("after decode: {:x?}", self.state);
        while let Some(_) = self.state.next_byte() {
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
    pub fn new(mut read: R) -> std::io::Result<Self> {
        let mut bytes = [0; 8];
        let mut bytes_to_read = bytes.as_mut_slice();
        while !bytes_to_read.is_empty() {
            let bytes_read = read.read(bytes_to_read)?;
            if bytes_read == 0 {
                // we have a small value and that is find, the remaining bytes are zero.
                break;
            }
            bytes_to_read = &mut bytes_to_read[bytes_read..];
        }
        Ok(Self {
            read,
            state: ArithState::default(),
            value: u64::from_be_bytes(bytes),
        })
    }
    pub fn decode(&mut self, p: Probability) -> std::io::Result<bool> {
        let out = self.state.decode(p, self.value);
        // println!("after decode: {:x?}", self.state);
        while let Some(_) = self.state.next_byte() {
            let mut byte = [0u8; 1];
            self.read.read(&mut byte)?;
            self.value = (self.value << 8) + byte[0] as u64;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_prob() -> (Probability, bool) {
        let value_bool = rand::random::<bool>();
        let shift = 1 + (rand::random::<u8>() % 15);
        let prob = 1 + (rand::random::<u64>() % ((1 << shift) - 1));
        (Probability { prob, shift }, value_bool)
    }

    #[test]
    fn encode_decode_last_byte() {
        let mut s = ArithState::default();
        for _ in 0..10_000 {
            // create a valid state
            s.lo = rand::random();
            if s.lo == u64::MAX {
                s.lo = 0;
            }
            s.hi = s.lo + 1 + (rand::random::<u64>() % (u64::MAX - s.lo));
            println!("initially s is {s:x?}");
            debug_assert!(s.hi > s.lo);
            while let Some(b) = s.next_byte() {
                println!("Got byte {b:x}");
            }
            println!("after regularization s is {s:x?}");
            debug_assert!(s.hi > s.lo);
            let mut decoding_s = s;
            let (p, value_bool) = rand_prob();
            s.encode(p, value_bool);
            let value_chosen = s.lo + (rand::random::<u64>() % (s.hi - s.lo));
            let decoded = decoding_s.decode(p, value_chosen);
            assert_eq!(decoded, value_bool);
            assert_eq!(s, decoding_s);
        }
    }

    #[test]
    fn zero_byte() {
        let mut s = ArithState::default();
        for _ in 0..8 {
            s.encode(
                Probability {
                    prob: 127,
                    shift: 8,
                },
                false,
            );
        }
        assert_eq!(s.next_byte(), Some(0));
    }

    #[test]
    fn one_byte() {
        let mut s = ArithState::default();
        assert_eq!(s.split(128, 8).map(|v| v >> 8), Some((u64::MAX / 2) >> 8));
        for _ in 0..9 {
            s.encode(
                Probability {
                    prob: 127,
                    shift: 8,
                },
                true,
            );
        }
        assert_eq!(s.next_byte(), Some(u8::MAX));
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
