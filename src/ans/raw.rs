pub use super::ans::Probability;
#[cfg(test)]
use super::Millibits;
use super::{EntropyCoder, EntropyDecoder};

/// Raw non-entropy encoding.
///
/// Can be used to encode data without accounting for probabilities.  This is
/// probably inefficient, because all the probability tracking is done anyhow,
/// so it is really intended for testing.
///
/// Raw encoding *will* encode and decode faster than `Ans` or `Range`, and for
/// small data where the adaptive modeling never really gets much information it
/// *may* be useful.  It *is* more compact than encodings such as bincode, because
/// data is encoded in an integer number of *bits* rather than an integer number
/// of *bytes*, and the variable length integers can thus be smaller.  Even
/// unicode is encoded just a little more compactly (beyond ASCII) because it
/// removes a few redundant bits from UTF8.
///
/// # Example
/// ```
/// let encoded: Vec<u8> = compactly::ans::Raw::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 24);
/// assert_eq!(compactly::ans::Raw::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Raw {
    bits: Vec<u8>,
    num_bits: u32,
    #[cfg(test)]
    entropy: Millibits,
}

impl EntropyCoder for Raw {
    #[inline]
    fn encode_bit(&mut self, _probability: Probability, bit: bool) {
        #[cfg(test)]
        {
            self.entropy += _probability.millibits(bit);
        }
        let bit = u8::from(bit);
        let which_bit = self.num_bits & 7;
        if which_bit == 0 {
            self.bits.push(bit);
        } else {
            *self.bits.last_mut().unwrap() |= bit << which_bit;
        }
        self.num_bits += 1;
    }
}
impl Raw {
    /// Encode value directly to a `Vec<u8>`.
    pub fn encode<T: super::Encode>(value: &T) -> Vec<u8> {
        <Self as EntropyCoder>::encode(value).bits
    }
    /// Just count the raw bits.
    pub fn num_bits<T: super::Encode>(value: &T) -> u32 {
        <Self as EntropyCoder>::encode(value).num_bits
    }
    /// Estimate the number of millibits after entropy encoding.
    #[cfg(test)]
    pub fn entropy<T: super::Encode>(value: &T) -> Millibits {
        <Self as EntropyCoder>::encode(value).entropy
    }
    /// Estimate the number of millibits after entropy encoding.
    #[cfg(test)]
    pub fn sizes<T: super::Encode>(value: &T) -> (usize, Millibits) {
        let x = <Self as EntropyCoder>::encode(value);
        (x.num_bits as usize, x.entropy)
    }
    /// Decode some encoded bytes.
    pub fn decode<T: super::Encode>(bytes: &[u8]) -> Option<T> {
        let mut reader = Decoder::from(bytes);
        T::decode(&mut reader, &mut T::Context::default()).ok()
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Decoder<'a> {
    bytes: &'a [u8],
    num_bits: u32,
}

impl<'a> From<&'a [u8]> for Decoder<'a> {
    #[inline(always)]
    fn from(bytes: &'a [u8]) -> Self {
        Self { num_bits: 0, bytes }
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Decode a bit using distribution Bernoulli(probability).
    #[inline(always)]
    fn decode_bit_nonadaptive(
        &mut self,
        _probability: self::Probability,
    ) -> Result<bool, std::io::Error> {
        let which_byte = self.num_bits as usize / 8;
        let which_bit = self.num_bits & 7;
        let mask = 1u8 << which_bit;
        self.num_bits += 1;
        Ok(self.bytes[which_byte] & mask == mask)
    }
}
