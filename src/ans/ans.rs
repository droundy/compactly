mod probability;
pub use probability::Probability;
mod bytes;
use bytes::Bytes;

type State = u64;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnsCoder {
    bits: Vec<(bool, Probability)>,
}

impl super::EntropyCoder for AnsCoder {
    fn encode(&mut self, probability: self::Probability, bit: bool) {
        self.bits.push((bit, probability));
    }
}
impl AnsCoder {
    #[inline]
    pub fn encode(&mut self, probability_of_false: Probability, value: bool) {
        self.bits.push((value, probability_of_false));
    }
    #[inline]
    pub fn finish(self) -> Vec<u8> {
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

#[derive(Eq, PartialEq, Debug)]
pub struct Encoder {
    state: State,
}

impl Encoder {
    pub fn new() -> Self {
        Self { state: 255 }
    }

    /// Encode a bit using distribution Bernoulli(probability).
    fn encode(&mut self, b: bool, probability: Probability) -> Option<u8> {
        let mut out = None;
        let zeros = State::from(probability);
        let ones = 256 - zeros;
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
        self.state = (self.state / freq) * 256 + z;
        out
    }

    pub fn finish_encoding(&mut self) -> Bytes {
        let mut bytes = Bytes::default();
        while self.state > 0 {
            bytes.push(self.state as u8);
            self.state >>= 8;
        }
        bytes
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Decoder<'a> {
    state: State,
    bytes: &'a [u8],
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let mut state: State = 0;
        if bytes.len() < 8 {
            for &b in bytes {
                state = (state << 8) | State::from(b);
            }

            Self { state, bytes: &[] }
        } else {
            let state = State::from_be_bytes(bytes[0..8].try_into().unwrap());
            let bytes = &bytes[8..];
            Self { state, bytes }
        }
    }

    /// Decode a bit using distribution Bernoulli(probability).
    pub fn decode(&mut self, probability: Probability) -> bool {
        let zeros = State::from(probability);
        let ones = 256 - zeros;
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
            if let Some(&u) = self.bytes.split_off_first() {
                self.state = (self.state << 8) | State::from(u);
            }
        }
        b
    }
}

#[test]
fn check_ans_coder() {
    let mut data = Vec::new();
    const SIZE: usize = 1000;
    data.resize_with(SIZE, || rand::random::<bool>());
    let mut distros = Vec::new();
    distros.resize_with(SIZE, rand::random::<Probability>);
    let mut writer = AnsCoder::default();
    for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
        // rev here
        writer.encode(probability, b);
    }
    let bytes = writer.finish();
    let mut decoder = Decoder::new(&bytes);
    for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
        println!("checking {b} {probability}");
        assert_eq!(decoder.decode(probability), b);
    }
    assert_eq!(decoder.state, 255);
}
