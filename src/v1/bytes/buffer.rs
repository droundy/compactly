const CAPACITY: usize = 2 << 23;
const MASK: usize = CAPACITY - 1;

/// A ring buffer that is always full.
#[derive(Debug)]
struct RingBuffer {
    bytes: Box<[u8]>,
    start: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            bytes: vec![0; CAPACITY].into_boxed_slice(),
            start: 0,
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {
        if bytes.len() < CAPACITY - self.start {
            self.bytes[self.start..self.start + bytes.len()].copy_from_slice(bytes);
        } else {
            let (start, end) = self.bytes.split_at_mut(self.start);
            let (b1, b2) = bytes.split_at(end.len());
            end.copy_from_slice(b1);
            start[..b2.len()].copy_from_slice(b2);
        }
        self.start = (self.start + bytes.len()) & MASK;
    }

    pub fn range(&self, range: std::ops::Range<usize>) -> (&[u8], &[u8]) {
        let start = (self.start + range.start) & MASK;
        let end = (self.start + range.end) & MASK;
        if end < start {
            (&self.bytes[start..], &self.bytes[..end])
        } else {
            (&self.bytes[start..end], &[])
        }
    }
}
