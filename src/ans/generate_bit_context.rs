//! Generate the `bit_context` module.
//!
//! This generating code creates a finite set of BitContext states and
//! transitions between them based on observations.
use super::ans::Probability;
use std::num::NonZeroU8;

struct BitC {
    name: String,
    probability: Probability,
    next_unlikely: String,
    next_likely: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bucket {
    Count { trues: usize, falses: usize },
}

/// A distribution of Probability
#[derive(Clone, Copy)]
struct Distribution([f64; 256]);

impl Distribution {
    /// This is the expected number of bits required for encoding.
    fn entropy(self, prob: Probability) -> f64 {
        let p0 = prob.as_f64();
        // Number of bits used to encode a zero
        let zero_bits = -p0.log2();
        // Number of bits used to encode a one
        let one_bits = -(1.0 - p0).log2();
        // println!("{zero_bits} and {one_bits}");
        let mut entropy = 0.0;
        for (i, d) in self.0.iter().enumerate() {
            let p = i as f64 / 255.0;
            entropy += d * (p * zero_bits + (1.0 - p) * one_bits);
        }
        entropy
    }

    /// The probability choice that minimizes the encoded size.
    fn best(self) -> (Probability, f64) {
        let mut best_entropy = f64::MAX;
        let mut best_probability = Probability {
            prob: NonZeroU8::new(1).unwrap(),
        };
        for prob in (1..255).map(|p| NonZeroU8::new(p).unwrap()) {
            let prob = Probability { prob };
            let s = self.entropy(prob);
            // println!("{:.8}: {s}   --- best is {best_probability}", prob.as_f64());
            if s < best_entropy {
                best_entropy = s;
                best_probability = prob;
            }
        }
        (best_probability, best_entropy)
    }

    /// The probability choice that minimizes the encoded size.
    fn best_probability(self) -> Probability {
        self.best().0
    }

    #[cfg(test)]
    fn max(self) -> f64 {
        let mut m = 0.0;
        for v in self.0 {
            if v > m {
                m = v;
            }
        }
        m
    }
}

impl Bucket {
    fn name(self) -> String {
        match self {
            Bucket::Count { trues, falses } => format!("True{trues}False{falses}"),
        }
    }
    fn new(trues: usize, falses: usize) -> Self {
        if (1 + trues) * (1 + falses) >= MAX_PRODUCT {
            if trues == 0 {
                Bucket::new(trues, falses - 1)
            } else if falses == 0 {
                Bucket::new(trues - 1, falses)
            } else {
                Bucket::new(trues / 2, falses / 2)
            }
        } else {
            Bucket::Count { trues, falses }
        }
    }

    /// This gives me the normalized Bayesian distribution of the probability of false.
    fn probability_distribution(self) -> Distribution {
        let mut dist = [1.0_f64; 256];
        let Bucket::Count { trues, falses } = self;
        for (i, v) in dist.iter_mut().enumerate() {
            let p = i as f64 / 255.0;
            *v = p.powi(falses as i32) * (1.0 - p).powi(trues as i32);
        }
        let norm = dist.iter().copied().sum::<f64>();
        for v in dist.iter_mut() {
            *v /= norm;
        }
        Distribution(dist)
    }

    fn bitc(self) -> BitC {
        let name = self.name();
        match self {
            Bucket::Count { trues, falses } => {
                let probability = self.probability_distribution().best_probability();
                let next_likely = if probability.likely_bit() {
                    Bucket::new(trues + 1, falses)
                } else {
                    Bucket::new(trues, falses + 1)
                }
                .name();
                let next_unlikely = if probability.likely_bit() {
                    Bucket::new(trues, falses + 1)
                } else {
                    Bucket::new(trues + 1, falses)
                }
                .name();
                BitC {
                    name,
                    probability,
                    next_unlikely,
                    next_likely,
                }
            }
        }
    }
}

fn probability(variants: &[Bucket]) {
    println!(
        r"
        /// Returns the probability of the value being zero.
        #[inline] pub fn probability(self) -> Probability {{
        match self {{"
    );

    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        println!("        {name} => {probability:?},")
    }

    println!(
        r"    }}
}}"
    );
}

fn lookup_probability(variants: &[Bucket]) {
    let sz = variants.len();
    println!(
        r"#[inline] pub fn probability(self) -> Probability {{
        const LOOKUP: [Probability; {sz}] = ["
    );

    for BitC { probability, .. } in variants.iter().map(|b| b.bitc()) {
        println!("        {probability:?},")
    }
    println!(
        "];
    LOOKUP[self as usize]"
    );

    println!(r"}}");
}

fn lookup_bits_required(variants: &[Bucket]) {
    let sz = 2 * variants.len();
    println!(
        r"#[inline] pub fn millibits_required(&mut self, bit: bool) -> u32 {{
        const LOOKUP: [u32; {sz}] = ["
    );

    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        let bits = (-1000.0 * probability.as_f64().log2()).min(u32::MAX as f64) as u32;
        println!("        {bits}, // {name} for false")
    }
    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        let bits = (-1000.0 * (1.0 - probability.as_f64()).log2()).min(u32::MAX as f64) as u32;
        println!("        {bits}, // {name} for true")
    }
    let half_sz = sz / 2;
    println!(
        "];
    let out = LOOKUP[(*self as usize) + (bit as usize)*{half_sz}];
    *self = self.adapt(bit);
    out"
    );

    println!(r"}}");
}

fn print_adapt(variants: &[Bucket]) {
    println!(
        r"
    #[inline] pub fn adapt(self, bit: bool) -> Self {{
        match (bit, self) {{"
    );

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
    } in variants.iter().map(|b| b.bitc())
    {
        let likely_bit = probability.likely_bit();
        let unlikely_bit = !likely_bit;
        println!("            ({likely_bit:?}, {name}) => {next_likely},");
        println!("            ({unlikely_bit:?}, {name}) => {next_unlikely},");
    }

    println!(
        r"        }}
 }}"
    );
}

fn lookup_adapt(variants: &[Bucket]) {
    let sz = variants.len();
    println!(
        r"
    #[inline] pub fn adapt(self, bit: bool) -> Self {{
        const OUTCOMES: [BitContext; 2*{sz}] = ["
    );

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
        ..
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = !probability.likely_bit();
        if am_likely {
            println!("            {next_likely}, // from {name} with false");
        } else {
            println!("            {next_unlikely}, // from {name} with false",);
        }
    }

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
        ..
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = probability.likely_bit();
        if am_likely {
            println!("            {next_likely}, // from {name} with true");
        } else {
            println!("            {next_unlikely}, // from {name} with true");
        }
    }

    println!(
        "    ];
       OUTCOMES[(self as usize) + (bit as usize)*{sz}]"
    );

    println!("}}");
}

const MAX_PRODUCT: usize = 134;
const COUNT_FOR_CONFIDENCE: usize = 4;

/// The program that actually outputs the generated code.
pub fn main() {
    let mut variants = Vec::new();
    for falses in 0..MAX_PRODUCT - 1 {
        let mut trues = 0;
        while (Bucket::Count { trues, falses }) == Bucket::new(trues, falses) {
            variants.push(Bucket::Count { trues, falses });
            trues += 1;
        }
    }

    let confident_name = Bucket::Count {
        trues: 0,
        falses: COUNT_FOR_CONFIDENCE,
    }
    .bitc()
    .name;
    const FILENAME: &str = file!();
    println!(
        r"//! Generated with `{FILENAME}`
use super::ans::Probability;

impl BitContext {{
pub const CONFIDENT: Self = {confident_name};
    }}
"
    );
    println!(
        r"
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {{
    #[default]"
    );

    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        println!("    {name},    // {probability}")
    }

    println!(
        r"}}
use BitContext::*;
"
    );

    println!(
        "
impl BitContext {{"
    );

    if std::env::args().any(|a| a == "--lookup") {
        lookup_probability(&variants);
        // probability(&variants);
        lookup_adapt(&variants);
    } else {
        probability(&variants);
        print_adapt(&variants);
    }
    lookup_bits_required(&variants);

    println!("}}");

    println!(r"// Count of variants: {}", variants.len());
}

#[cfg(test)]
fn test_distribution(trues: usize, falses: usize, prob: f64, expected_bits: f64) {
    let d = Bucket::Count { trues, falses }.probability_distribution();
    println!("{trues} true and {falses} false");
    for v in d.0.into_iter().step_by(8) {
        let wid = (v / d.max() * 80.0) as usize;
        println!("{:wid$}*", "|");
    }
    let (best_prob, bits) = d.best();
    assert_eq!(best_prob.as_f64(), prob);
    assert!(bits > expected_bits - 1e-10, "{bits} > {expected_bits}");
    assert!(bits < expected_bits + 1e-10, "{bits} < {expected_bits}");
}

#[test]
fn distribution_test() {
    test_distribution(32, 32, 0.5, 1.0);
    test_distribution(64, 64, 0.5, 1.0);
    test_distribution(0, 0, 0.5, 1.0);
    test_distribution(1, 0, 0.33203125, 0.9169830942670982);
    test_distribution(0, 1, 0.66796875, 0.9169830942670982);
    test_distribution(2, 0, 0.25, 0.8089518585578784);
    test_distribution(0, 2, 0.75, 0.8089518585578784);
    test_distribution(0, 3, 0.80078125, 0.7187907456421366);
    test_distribution(32, 0, 0.02734375, 0.18195147863889768);
    test_distribution(64, 0, 0.01171875, 0.10211457524295939);
    test_distribution(MAX_PRODUCT - 2, 0, 1.0 / 128.0, 0.05104365326082572);
    test_distribution(MAX_PRODUCT - 1, 0, 1.0 / 256.0, 0.05065909928371242);
}
