#[derive(Default, Debug, Clone, Copy)]
pub struct Bytes {
    bytes: [u8; 8],
    count: usize,
}

impl Bytes {
    #[inline]
    pub fn push(&mut self, byte: u8) {
        self.bytes[self.count] = byte;
        self.count += 1;
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.bytes[..self.count]
    }
}

impl IntoIterator for Bytes {
    type Item = u8;
    type IntoIter = std::iter::Take<std::array::IntoIter<u8, 8>>;
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter().take(self.count)
    }
}
