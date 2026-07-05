/// A number of millibits something is encoded with.
///
/// This is used in testing to estimate the encoded size of data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millibits(u32);

impl super::EntropyCoder for Millibits {
    fn encode_bits<const N: usize>(
        &mut self,
        bits_with_probabilities: [(bool, super::ans::Probability); N],
    ) {
        for (bit, probability) in bits_with_probabilities {
            *self += probability.millibits(bit);
        }
    }

    /// A whole tree symbol costs `-log2(width / M)` bits: one exact estimate
    /// for the symbol rather than `log2(N)` separately-rounded per-bit
    /// estimates, matching what the single-step coders actually pay.
    fn encode_tree<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        value: usize,
    ) {
        use super::symbol::SymbolRange;
        let width = SymbolRange::for_value(contexts, value).width();
        let millibits = (SymbolRange::BITS as f64 - (width as f64).log2()) * 1000.0;
        *self += Millibits(millibits.round() as u32);
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

    #[cfg(test)]
    pub(crate) fn as_bits(self) -> String {
        ((self.0 + 500) / 1000).to_string()
    }
}

impl std::fmt::Display for Millibits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} mb", self.0)
    }
}

impl std::ops::AddAssign<Millibits> for Millibits {
    fn add_assign(&mut self, rhs: Millibits) {
        self.0 += rhs.0;
    }
}
