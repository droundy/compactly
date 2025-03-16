use super::bit_context::BitContext;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Writer<W> {
    arith: super::arith::Writer<W>,
    rng: SplitMix64,
}

impl<W: std::io::Write> Writer<W> {
    pub fn new(write: W) -> Self {
        Self {
            arith: super::arith::Writer::new(write),
            rng: SplitMix64::default(),
        }
    }
    pub fn encode(&mut self, value: bool, context: &mut BitContext) -> Result<(), std::io::Error> {
        self.arith.encode(context.probability(), value)?;
        *context = context.adapt(value, &mut self.rng);
        Ok(())
    }
    pub fn finish(self) -> Result<(), std::io::Error> {
        self.arith.finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reader<R> {
    arith: super::arith::Reader<R>,
    rng: SplitMix64,
}

impl<R: std::io::Read> Reader<R> {
    pub fn new(read: R) -> Result<Self, std::io::Error> {
        Ok(Self {
            arith: super::arith::Reader::new(read)?,
            rng: SplitMix64::default(),
        })
    }
    pub fn decode(&mut self, context: &mut BitContext) -> Result<bool, std::io::Error> {
        let bit = self.arith.decode(context.probability())?;
        *context = context.adapt(bit, &mut self.rng);
        Ok(bit)
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

#[test]
fn encode_size() {
    fn measure_size(bits: &[bool], expected_bytes: usize) {
        let mut context = BitContext::default();
        let mut encoded = Vec::new();
        let mut e = Writer::new(&mut encoded);
        for bit in bits.iter().copied() {
            // println!(
            //     "Context is {context:?} with probability {}",
            //     context.probability()
            // );
            e.encode(bit, &mut context).unwrap();
        }
        e.finish().unwrap();
        assert_eq!(
            encoded.len(),
            expected_bytes,
            "For {bits:?} wrong size for {} bits",
            bits.len()
        );
        let mut decoded = Vec::new();
        let mut encoded_slice = encoded.as_slice();
        let mut decoder = Reader::new(&mut encoded_slice).unwrap();
        let mut decontext = BitContext::default();
        for _ in 0..bits.len() {
            decoded.push(decoder.decode(&mut decontext).unwrap());
        }
        assert_eq!(bits, decoded.as_slice());
    }
    measure_size(&[], 1);
    for i in 0..5 * 1024 {
        measure_size(&vec![true; i], 1);
        measure_size(&vec![false; i], 1);
    }
    // measure_size(&vec![true; 512], 2);
    // measure_size(&vec![false; 256], 2);
    for i in 0..8 {
        let mut bits = Vec::new();
        for x in 0..i {
            bits.push(x & 1 == 0);
        }
        measure_size(&bits, 1);
    }
    measure_size(
        &[
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, false,
        ],
        2,
    );
    measure_size(
        &[
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, false,
        ],
        1,
    );
    measure_size(
        &[
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, false, false, false, false,
        ],
        2, // wow, we just compressed 24 bits into 2 bytes thanks to correlation between the bits!
    );
    measure_size(
        &[
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, false, false, false, false, false,
        ],
        3,
    );
    for i in 8..15 {
        let mut bits = Vec::new();
        for x in 0..i {
            bits.push(x & 1 == 0);
        }
        measure_size(&bits, 2);
    }
}

#[test]
fn write_read_correctly() {
    for _ in 0..10_000 {
        let rand_bits = rand::random::<u128>();
        let mut bits = [false; 128];
        for i in 0..128 {
            bits[i] = ((rand_bits >> i) & 1) == 1;
        }
        for length in 0..128 {
            // println!("\nTesting with {length} bits.");
            let mut context = BitContext::default();
            let mut encoded = Vec::new();
            let mut writer = Writer::new(&mut encoded);
            for &bit in &bits[..length] {
                // println!("Encoding {bit:?} with {context:x?}");
                writer.encode(bit, &mut context).unwrap();
            }
            writer.finish().unwrap();
            // println!("Encoded is {encoded:?}");

            let mut bytes = encoded.as_slice();
            let mut reader = Reader::new(&mut bytes).unwrap();
            let mut context = BitContext::default();
            for i in 0..length {
                // println!("Decoding bit {i} with {context:x?}");
                assert_eq!(reader.decode(&mut context).unwrap(), bits[i]);
            }
        }
    }
}
