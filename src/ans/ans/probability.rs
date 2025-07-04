/// The probability that the bit will be false.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Probability {
    /// The probability is `prob / 256`
    pub prob: u8,
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
        Probability { prob: prob as u8 }
    }
}

impl std::fmt::Debug for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trues = 256 - self.prob as u64;
        let falses = self.prob;
        write!(f, "Probability::new({trues},{falses})")
    }
}

impl Probability {
    /// The more likely value for the bit
    #[inline]
    pub fn likely_bit(&self) -> bool {
        self.prob < (1 << (SHIFT - 1))
    }
    /// The probability of zero as an `f64` value.
    #[inline]
    pub fn as_f64(self) -> f64 {
        self.prob as f64 / (1_u64 << SHIFT) as f64
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.prob as f64 / (1_u64 << SHIFT) as f64;
        write!(f, "{v}")
    }
}
