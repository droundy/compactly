use crate::bit_context::BitContext;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encoder {
    arith: crate::arith::Encoder,
    rng: SplitMix64,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            arith: crate::arith::Encoder::new(),
            rng: SplitMix64::default(),
        }
    }
    pub fn encode(&mut self, value: bool, context: &mut BitContext) {
        self.arith.encode(context.probability(), value);
        *context = context.adapt(value, &mut self.rng);
    }
    pub fn finish(self) -> Vec<u8> {
        self.arith.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder {
    arith: crate::arith::Decoder,
    rng: SplitMix64,
}

impl Decoder {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            arith: crate::arith::Decoder::new(bytes),
            rng: SplitMix64::default(),
        }
    }
    pub fn decode(&mut self, context: &mut BitContext) -> bool {
        let bit = self.arith.decode(context.probability());
        *context = context.adapt(bit, &mut self.rng);
        bit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

impl Default for SplitMix64 {
    fn default() -> Self {
        Self(137)
    }
}
