use compactly::v0::arith::Probability;

struct BitC {
    name: String,
    probability: Probability,
    next_unlikely: String,
    next_likely: String,
    prob_same: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Bucket {
    Count { trues: usize, falses: usize },
    AllTrue(usize),
    AllFalse(usize),
}

impl Bucket {
    fn name(self) -> String {
        match self {
            Bucket::Count { trues, falses } => format!("True{trues}False{falses}"),
            Bucket::AllTrue(n) => format!("AllTrue{n}"),
            Bucket::AllFalse(n) => format!("AllFalse{n}"),
        }
    }
    fn new(trues: usize, falses: usize) -> Self {
        if trues + falses >= MAX_COUNT {
            if (trues + 1) / 2 == 0 {
                Bucket::AllFalse(MIN_ALL)
            } else if (falses + 1) / 2 == 0 {
                Bucket::AllTrue(MIN_ALL)
            } else {
                Bucket::new((trues + 1) / 2, (falses + 1) / 2)
            }
        } else {
            Bucket::Count { trues, falses }
        }
    }
    fn bitc(self) -> BitC {
        let name = self.name();
        match self {
            Bucket::Count { trues, falses } => {
                let mut prob = if falses == 0 {
                    1 * 256 / ((2 + trues) as u64)
                } else if trues == 0 {
                    (1 + falses) as u64 * 256 / ((2 + falses) as u64)
                } else {
                    falses as u64 * 256 / ((trues + falses) as u64)
                };
                let mut shift = 8;
                while prob & 1 == 0 && shift > 2 {
                    prob >>= 1;
                    shift -= 1;
                }
                let probability = Probability { prob, shift };
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
                    prob_same: None,
                }
            }
            Bucket::AllTrue(n) => {
                let shift = (n as u8) + 1;
                let probability = Probability { prob: 1, shift };
                let next_unlikely = if n == MAX_ALL {
                    self
                } else {
                    Bucket::new(1 << n, 1)
                }
                .name();
                BitC {
                    name,
                    probability,
                    next_unlikely,
                    next_likely: if n < MAX_ALL {
                        Bucket::AllTrue(n + 1)
                    } else {
                        self
                    }
                    .name(),
                    prob_same: Some(u64::MAX - (u64::MAX >> shift)),
                }
            }
            Bucket::AllFalse(n) => {
                let shift = (n as u8) + 1;
                let probability = Probability {
                    prob: (1 << shift) - 1,
                    shift,
                };
                let next_unlikely = if n == MAX_ALL {
                    self
                } else {
                    Bucket::new(1, 1 << n)
                }
                .name();
                BitC {
                    name,
                    probability,
                    next_unlikely,
                    next_likely: if n < MAX_ALL {
                        Bucket::AllFalse(n + 1)
                    } else {
                        self
                    }
                    .name(),
                    prob_same: Some(u64::MAX - (u64::MAX >> shift)),
                }
            }
        }
    }
}

const MAX_COUNT: usize = 21;
const MIN_ALL: usize = MAX_COUNT.ilog2() as usize;
const MAX_ALL: usize = 15;

fn main() {
    let mut variants = Vec::new();
    for tot in 0..MAX_COUNT {
        for trues in 0..tot + 1 {
            let falses = tot - trues;
            variants.push(Bucket::Count { trues, falses })
        }
    }
    for same in MIN_ALL..MAX_ALL + 1 {
        variants.push(Bucket::AllFalse(same));
        variants.push(Bucket::AllTrue(same));
    }

    println!(
        r"//! Generated with `cargo run --example bit-context > src/bit_context.rs`
use crate::adapt::SplitMix64;
use crate::arith::Probability;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {{
    #[default]"
    );

    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        println!("    {name}, // {probability:?} = {probability}")
    }

    println!(
        r"}}
use BitContext::*;
"
    );

    println!(
        r"
impl BitContext {{
    pub fn probability(self) -> Probability {{
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

    println!(
        r"
    pub fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Self {{
        match self {{"
    );

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
        prob_same,
    } in variants.iter().map(|b| b.bitc())
    {
        let likely_bit = probability.likely_bit();
        if next_likely != name && prob_same.is_some() {
            let prob_same = prob_same.unwrap();
            println!(
                "            {name} => {{
                if bit == {likely_bit:?} {{
                    if rng.next() < 0x{prob_same:016x} {{ {next_likely} }} else {{ {name} }}
                }} else {{
                    {next_unlikely}
                }}
            }}"
            );
        } else {
            println!(
                "            {name} => {{
                if bit == {likely_bit:?} {{
                    {next_likely}
                }} else {{
                    {next_unlikely}
                }}
            }}"
            );
        }
    }

    println!(r"        }}");

    println!(
        r"    }}
}}"
    );

    println!(r"// Count of variants: {}", variants.len());
}
