use compactly::arith::Probability;

struct BitC {
    name: String,
    probability: Probability,
    next_unlikely: String,
    next_likely: String,
    prob_same: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Bucket {
    Start,
    AllTrue(usize),
    AllFalse(usize),
}

impl Bucket {
    fn name(self) -> String {
        match self {
            Bucket::Start => "Start".to_string(),
            Bucket::AllTrue(n) => format!("AllTrue{n}"),
            Bucket::AllFalse(n) => format!("AllFalse{n}"),
        }
    }
    fn bitc(self) -> BitC {
        let name = self.name();
        match self {
            Bucket::Start => BitC {
                name,
                probability: Probability { prob: 1, shift: 1 },
                next_unlikely: Bucket::AllTrue(1).name(),
                next_likely: Bucket::AllTrue(0).name(),
                prob_same: None,
            },
            Bucket::AllTrue(n) => {
                let shift = (n as u8) + 1;
                BitC {
                    name,
                    probability: Probability { prob: 1, shift },
                    next_unlikely: Bucket::Start.name(),
                    next_likely: Bucket::AllTrue(n + 1).name(),
                    prob_same: Some(u64::MAX - (u64::MAX >> shift)),
                }
            }
            Bucket::AllFalse(n) => {
                let shift = (n as u8) + 1;
                BitC {
                    name,
                    probability: Probability {
                        prob: 1 << shift,
                        shift,
                    },
                    next_unlikely: Bucket::Start.name(),
                    next_likely: Bucket::AllFalse(n + 1).name(),
                    prob_same: Some(u64::MAX - (u64::MAX >> shift)),
                }
            }
        }
    }
}

fn main() {
    let variants = vec![
        Bucket::Start,
        Bucket::AllFalse(1),
        Bucket::AllFalse(2),
        Bucket::AllTrue(1),
        Bucket::AllTrue(2),
    ];

    println!(
        r"
pub enum BitContext {{"
    );

    for BitC {
        name, probability, ..
    } in variants.iter().map(|b| b.bitc())
    {
        println!("    {name}, // {probability:?}")
    }

    println!(
        r"}}
use BitC::*;
"
    );

    println!(
        r"
impl BitContext {{
    fn probability(self) -> Probability {{
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
impl BitContext {{
    fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Probability {{
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
                    if rng.next() < {prob_same:016x} {{ {next_likely} }} else {{ {name} }}
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
}
