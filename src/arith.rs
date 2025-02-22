#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithState {
    lo: u64,
    hi: u64,
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
        #[cfg(test)]
        {
            let width = self.hi - self.lo;
            println!("width = {width:016x}");
            println!("  min = {:016x}", u64::MAX >> 8);
            println!("lo_byte {lo_byte:02x}");
            println!("hi_byte {hi_byte:02x}");
        }
        if lo_byte == hi_byte {
            self.lo = self.lo << 8;
            self.hi = self.hi << 8;
            #[cfg(test)]
            {
                println!("next_byte resetting to {self:x?}");
            }
            Some(lo_byte)
        } else {
            None
        }
    }

    pub fn last_bytes(self) -> [u8; 8] {
        self.lo.to_be_bytes()
        // Some((self.lo >> 56) as u8)
        // if self.hi >> 63 == self.lo >> 63 {
        //     Some((self.lo >> 56) as u8)
        // } else {
        //     None
        // }
    }

    pub fn encode(&mut self, prob: u64, shift: u8, value: bool) {
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
        println!("encoding {prob} {shift} {value:?}   with split {split:016x} gives {self:x?}");
    }

    pub fn decode(&mut self, prob: u64, shift: u8, value: u64) -> bool {
        let split = self
            .split(prob, shift)
            .expect("call next_byte enough before decode");
        let b = value > split;
        println!("decoded bit {prob} {shift} {b:?}   from {value:016x} and split {split:016x}");
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
        let min = u64::MAX >> 16;
        #[cfg(test)]
        {
            println!(" self = {self:x?}");
            println!("width = {width:016x}");
            println!("  min = {min:016x}");
        }
        if width < min {
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
    pub fn encode(&mut self, prob: u64, shift: u8, value: bool) {
        while let Some(byte) = self.state.next_byte() {
            self.bytes.push(byte);
        }
        self.state.encode(prob, shift, value);
    }
    pub fn finish(mut self) -> Vec<u8> {
        while let Some(byte) = self.state.next_byte() {
            self.bytes.push(byte);
        }
        self.bytes.extend_from_slice(&self.state.last_bytes());
        // if let Some(byte) = self.state.last_byte() {
        //     self.bytes.push(byte);
        // }
        self.bytes
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
    pub fn decode(&mut self, prob: u64, shift: u8) -> bool {
        let out = self.state.decode(prob, shift, self.value);
        println!("after decode: {:x?}", self.state);
        while let Some(_) = self.state.next_byte() {
            self.value = (self.value << 8) + self.bytes.pop().unwrap_or_default() as u64;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_prob() -> (u64, u8, bool) {
        let value_bool = rand::random::<bool>();
        let shift = 1 + (rand::random::<u8>() % 15);
        let prob = 1 + (rand::random::<u64>() % ((1 << shift) - 1));
        (prob, shift, value_bool)
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
            let (prob, shift, value_bool) = rand_prob();
            s.encode(dbg!(prob), dbg!(shift), dbg!(value_bool));
            let value_chosen = s.lo + (rand::random::<u64>() % (s.hi - s.lo));
            let decoded = decoding_s.decode(prob, shift, value_chosen);
            assert_eq!(decoded, value_bool);
            assert_eq!(s, decoding_s);
        }
    }

    #[test]
    fn zero_byte() {
        let mut s = ArithState::default();
        for _ in 0..8 {
            s.encode(128, 8, false);
        }
        assert_eq!(s.next_byte(), Some(0));
        // assert_eq!(s, ArithState::default()); // this is only approximately true due to truncation
    }

    #[test]
    fn one_byte() {
        let mut s = ArithState::default();
        assert_eq!(s.split(128, 8).map(|v| v >> 8), Some((u64::MAX / 2) >> 8));
        for _ in 0..9 {
            s.encode(128, 8, true);
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
        ] {
            println!("\nTest {probs:?}");
            let mut encoder = Encoder::new();
            for &(prob, shift, bit) in &probs {
                encoder.encode(prob, shift, bit);
            }
            println!("{encoder:x?}");
            let bytes = encoder.finish();
            println!("\n\nEncoded: {bytes:02x?}\n");
            let mut decoder = Decoder::new(bytes);
            for &(prob, shift, bit) in &probs {
                println!("Decoding {prob} {shift} {bit:?}");
                assert_eq!(decoder.decode(prob, shift), bit);
            }
        }
        for _ in 0..10_000 {
            let num_bits = rand::random::<usize>() % 16;
            let mut probs = Vec::new();
            for _ in 0..num_bits {
                probs.push(rand_prob());
            }
            println!("\n\ntesting {probs:?}");
            let mut encoder = Encoder::new();
            for &(prob, shift, bit) in &probs {
                encoder.encode(prob, shift, bit);
            }
            let bytes = encoder.finish();
            println!("\n\nEncoded random as: {bytes:02x?}\n");
            let mut decoder = Decoder::new(bytes);
            for &(prob, shift, bit) in &probs {
                println!("Decoding {prob} {shift} {bit:?}");
                assert_eq!(decoder.decode(prob, shift), bit);
            }
        }
    }
}
