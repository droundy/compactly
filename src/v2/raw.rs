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
/// let encoded: Vec<u8> = compactly::v2::Raw::encode(&vec![5u64, 4, 3, 2, 1]);
/// assert_eq!(encoded.len(), 41);
/// assert_eq!(compactly::v2::Raw::decode::<Vec<u64>>(&encoded).unwrap()[2], 3);
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Raw {
    bits: Vec<u8>,
    num_bits: u32,
    #[cfg(test)]
    entropy: Millibits,
}

impl EntropyCoder for Raw {
    /// Pack `N` raw bits, ignoring the probabilities. The contexts are still
    /// adapted so they track the decoder's models in lock-step (the packed
    /// bits don't depend on them, but keeping parity matches the other
    /// coders).
    #[inline]
    fn encode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
        bits: [bool; N],
    ) {
        for (bit, context) in bits.into_iter().zip(contexts.iter_mut()) {
            #[cfg(test)]
            {
                self.entropy += context.probability().millibits(bit);
            }
            *context = context.adapt(bit);
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

impl<'a> Decoder<'a> {
    /// Read the next raw bit (LSB-first within each byte), ignoring any entropy
    /// model — `Raw` packs bits verbatim.
    #[inline(always)]
    fn next_bit(&mut self) -> bool {
        let which_byte = self.num_bits as usize / 8;
        let which_bit = self.num_bits & 7;
        let mask = 1u8 << which_bit;
        self.num_bits += 1;
        self.bytes[which_byte] & mask == mask
    }
}

impl<'a> EntropyDecoder for Decoder<'a> {
    /// Read `N` raw bits, ignoring the probabilities. The contexts are still
    /// adapted so they track the encoder's models in lock-step (the bit values
    /// don't depend on them, but keeping parity matches the other coders).
    #[inline(always)]
    fn decode_bits<const N: usize>(
        &mut self,
        contexts: &mut [super::bit_context::BitContext; N],
    ) -> [bool; N] {
        let mut bits = [false; N];
        for (b, context) in bits.iter_mut().zip(contexts.iter_mut()) {
            let bit = self.next_bit();
            *context = context.adapt(bit);
            *b = bit;
        }
        bits
    }

    /// Incompressible bytes are just raw bits too (8 per byte, LSB-first), which
    /// mirrors the default `encode_incompressible_bytes`.
    #[inline(always)]
    fn decode_incompressible_bytes(&mut self, bytes: &mut [u8]) -> Result<(), std::io::Error> {
        for v in bytes {
            let mut b = 0u8;
            for i in 0..8 {
                b |= (self.next_bit() as u8) << i;
            }
            *v = b;
        }
        Ok(())
    }
}
