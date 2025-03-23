use compactly::v1::Probability;

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

fn probability(variants: &[Bucket]) {
    println!(
        r"pub fn probability(self) -> Probability {{
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
        r"pub fn probability(self) -> Probability {{
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

fn print_adapt(variants: &[Bucket]) {
    println!(
        r"
    pub fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Self {{
        match (bit, self) {{"
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
        let unlikely_bit = !likely_bit;
        if next_likely != name && prob_same.is_some() {
            let prob_same = prob_same.unwrap();
            println!(
                "            ({likely_bit:?}, {name}) => {{
                    if rng.next() < 0x{prob_same:016x} {{ {next_likely} }} else {{ {name} }}
            }}"
            );
            println!(
                "            ({unlikely_bit:?}, {name}) => {{
                    {next_unlikely}
            }}"
            );
        } else {
            println!("            ({likely_bit:?}, {name}) => {next_likely},");
            println!("            ({unlikely_bit:?}, {name}) => {next_unlikely},");
        }
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
    pub fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Self {{
        struct Outcome {{
            a: BitContext,
            b: BitContext,
            prob_a: u64,
        }}
        const OUTCOMES: [Outcome; 2*{sz}] = ["
    );

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
        prob_same,
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = !probability.likely_bit();
        if am_likely {
            if next_likely != name && prob_same.is_some() {
                let prob_same = prob_same.unwrap();
                println!(
                    "            Outcome {{ a: {next_likely}, b: {name}, prob_a: 0x{prob_same:016x} }},"
                );
            } else {
                println!(
                    "            Outcome {{ a: {next_likely}, b: {next_likely}, prob_a: 0 }},"
                );
            }
        } else {
            println!(
                "            Outcome {{ a: {next_unlikely}, b: {next_unlikely}, prob_a: 0 }},"
            );
        }
    }

    for BitC {
        name,
        probability,
        next_likely,
        next_unlikely,
        prob_same,
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = probability.likely_bit();
        if am_likely {
            if next_likely != name && prob_same.is_some() {
                let prob_same = prob_same.unwrap();
                println!(
                    "            Outcome {{ a: {next_likely}, b: {name}, prob_a: 0x{prob_same:016x} }},"
                );
            } else {
                println!(
                    "            Outcome {{ a: {next_likely}, b: {next_likely}, prob_a: 0 }},"
                );
            }
        } else {
            println!(
                "            Outcome {{ a: {next_unlikely}, b: {next_unlikely}, prob_a: 0 }},"
            );
        }
    }

    println!(
        "    ];
                let idx = (self as usize) + (bit as usize)*{sz};
       let Outcome {{ a, b, prob_a }} = OUTCOMES[idx];
       if prob_a == 0 {{ a }} else if rng.next() < prob_a {{ a}} else {{b}}"
    );

    println!("}}");
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
        r"//! Generated with `src/v1/bit-context.sh`
use super::adapt::SplitMix64;
use super::arith::Probability;

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

    println!("}}");

    println!(r"// Count of variants: {}", variants.len());
}
