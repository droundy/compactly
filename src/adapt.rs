use crate::arith::Probability;

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
        todo!()
    }
    pub fn finish(mut self) -> Vec<u8> {
        self.arith.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Decoder {
    arith: crate::arith::Decoder,
    rng: SplitMix64,
}

impl Decoder {
    pub fn new(mut bytes: Vec<u8>) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BitContext {
    Start,
    /// Have seen just one false
    P1_0,
    /// Have seen just one true
    P0_1,
    /// Have seen a false and a true
    P1_1,
}
use BitContext::*;

impl BitContext {
    pub(crate) const fn probability(self) -> Probability {
        match self {
            Start => Probability { prob: 1, shift: 1 },
            P1_0 => Probability { prob: 1, shift: 2 },
            P0_1 => Probability { prob: 3, shift: 2 },
            P1_1 => Probability { prob: 1, shift: 1 },
        }
    }

    pub(crate) const fn adapt(self, bit: bool, _rng: &mut SplitMix64) -> Self {
        if bit {
            match self {
                Start => P0_1,
                P1_0 => P1_1,
                P0_1 => P0_1, // FIXME
                P1_1 => P1_1, // FIXME
            }
        } else {
            match self {
                Start => P1_0,
                P0_1 => P1_1,
                P1_0 => P1_0, // FIXME
                P1_1 => P1_1, // FIXME
            }
        }
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
