mod probability;
pub use probability::Probability;

type State = u32;

#[derive(Eq, PartialEq, Debug)]
pub struct Coder {
    state: State,
    bulk: Vec<u8>,
}

impl Coder {
    pub fn new() -> Self {
        Self {
            state: 255,
            bulk: Vec::new(),
        }
    }

    /// Encode a bit using distribution Bernoulli(probability).
    pub fn encode(&mut self, b: bool, probability: Probability) {
        let zeros = State::from(probability);
        let ones = 256 - zeros;
        // we use uniform of size matching the bit value to decode from state first
        let freq = if b { zeros } else { ones };
        // shift data from state to bulk when it grows too much
        if self.state >> (State::BITS - 8) >= freq {
            self.bulk.push(self.state as u8);
            self.state >>= 8;
        }
        // the code really starts here, decode digit from freq base
        let mut z = self.state % freq;
        if b {
            z += ones;
        }
        // now encode new digit from 256 base
        self.state = (self.state / freq) * 256 + z;
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
            if let Some(u) = self.bulk.pop() {
                self.state = (self.state << 8) | u as State;
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
    let mut coder = Coder::new();
    for (b, probability) in data.iter().copied().zip(distros.iter().copied()).rev() {
        // rev here
        coder.encode(b, probability);
    }
    for (b, probability) in data.iter().copied().zip(distros.iter().copied()) {
        assert_eq!(coder.decode(probability), b);
    }
    assert_eq!(coder, Coder::new());
}
