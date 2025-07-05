use super::bit_context::BitContext;

/// A reader for adaptively encoded bits.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reader<R> {
    arith: super::arith::Reader<R>,
}

impl<R: std::io::Read> Reader<R> {
    /// Create a reader from a [`std::io::Read`].
    ///
    /// This can return an `Err` because we start by reading the first eight
    /// bytes (or the entire file if smaller than 8 bytes) into a buffer.
    pub fn new(read: R) -> Result<Self, std::io::Error> {
        Ok(Self {
            arith: super::arith::Reader::new(read)?,
        })
    }
    /// Decode a single bit with the provided context.
    #[inline]
    pub fn decode(&mut self, context: &mut BitContext) -> Result<bool, std::io::Error> {
        let bit = self.arith.decode(context.probability())?;
        *context = context.adapt(bit);
        Ok(bit)
    }
}
