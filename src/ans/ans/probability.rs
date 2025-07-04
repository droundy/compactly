use std::num::NonZeroU8;

/// The probability that the bit will be false.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Probability {
    /// The probability is `prob / 256`
    pub prob: NonZeroU8,
}

impl From<Probability> for super::State {
    fn from(value: Probability) -> Self {
        Self::from(value.prob.get())
    }
}

pub const SHIFT: u8 = 8;

impl Probability {
    /// Create a new probability based on a given number of true and false observations
    pub const fn new(trues: u64, falses: u64) -> Self {
        let prob = if falses == 0 {
            1 * 256 / ((2 + trues) as u64)
        } else if trues == 0 {
            (1 + falses) as u64 * 256 / ((2 + falses) as u64)
        } else {
            falses as u64 * 256 / ((trues + falses) as u64)
        };
        let prob = prob as u8;
        Probability {
            prob: NonZeroU8::new(prob).unwrap(),
        }
    }
}

impl std::fmt::Debug for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trues = 256 - self.prob.get() as u64;
        let falses = self.prob;
        write!(f, "Probability::new({trues},{falses})")
    }
}

impl Probability {
    /// The more likely value for the bit
    #[inline]
    pub fn likely_bit(&self) -> bool {
        self.prob.get() < (1 << (SHIFT - 1))
    }
    /// The probability of zero as an `f64` value.
    #[inline]
    pub fn as_f64(self) -> f64 {
        self.prob.get() as f64 / (1_u64 << SHIFT) as f64
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.prob.get() as f64 / (1_u64 << SHIFT) as f64;
        write!(f, "{v}")
    }
}

#[cfg(test)]
mod test {
    use std::num::NonZeroU8;

    use super::Probability;
    use rand::{distributions::Standard, prelude::*};

    impl Distribution<Probability> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Probability {
            let prob = rng.gen_range(1u8..255);
            let prob = NonZeroU8::new(prob).unwrap();
            Probability { prob }
        }
    }
}
