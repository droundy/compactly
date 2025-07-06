/// A number of millibits something is encoded with.
///
/// This is used in testing to estimate the encoded size of data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millibits(u32);

impl super::EntropyCoder for Millibits {
    fn encode_bit(&mut self, probability: super::ans::Probability, bit: bool) {
        *self += probability.millibits(bit);
    }
}

impl Millibits {
    /// A given number of bytes
    pub fn bytes(num_bytes: usize) -> Self {
        Self(num_bytes as u32 * 1000 * 8)
    }
    /// A given number of bits
    pub fn bits(num_bits: usize) -> Self {
        Self(num_bits as u32 * 1000)
    }
    /// A given number of millibits
    pub fn new(millibits: usize) -> Self {
        Self(millibits as u32)
    }
}

impl std::ops::AddAssign<Millibits> for Millibits {
    fn add_assign(&mut self, rhs: Millibits) {
        self.0 += rhs.0;
    }
}
