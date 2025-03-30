use compactly::v1::Probability;

struct BitC {
    name: String,
    probability: Probability,
    next_unlikely: String,
    next_likely: String,
}

#[derive(Debug, Clone, Copy)]
enum Bucket {
    Count { trues: usize, falses: usize },
}

impl Bucket {
    fn name(self) -> String {
        match self {
            Bucket::Count { trues, falses } => format!("True{trues}False{falses}"),
        }
    }
    fn new(trues: usize, falses: usize) -> Self {
        if trues + falses >= MAX_COUNT {
            Bucket::new(trues / 2, falses / 2)
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
                }
            }
        }
    }
}

fn probability(variants: &[Bucket]) {
    println!(
        r"#[inline] pub fn probability(self) -> Probability {{
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
        probability,
        next_likely,
        next_unlikely,
        ..
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = !probability.likely_bit();
        if am_likely {
            println!("            {next_likely},");
        } else {
            println!("            {next_unlikely},");
        }
    }

    for BitC {
        probability,
        next_likely,
        next_unlikely,
        ..
    } in variants.iter().map(|b| b.bitc())
    {
        let am_likely = probability.likely_bit();
        if am_likely {
            println!("            {next_likely},");
        } else {
            println!("            {next_unlikely},");
        }
    }

    println!(
        "    ];
       OUTCOMES[(self as usize) + (bit as usize)*{sz}]"
    );

    println!("}}");
}

const MAX_COUNT: usize = 50;

fn main() {
    let mut variants = Vec::new();
    for tot in 0..MAX_COUNT {
        for trues in 0..tot + 1 {
            let falses = tot - trues;
            variants.push(Bucket::Count { trues, falses })
        }
    }

    println!(
        r"//! Generated with `src/v1/bit-context.sh`
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
