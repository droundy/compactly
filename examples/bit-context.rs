use compactly::arith::Probability;

struct BitC {
    name: String,
    probability: Probability,
    next_unlikely: String,
    next_likely: String,
    prob_likely: u64,
}

fn main() {
    let variants = vec![
        BitC {
            name: "Start".to_string(),
            probability: Probability { prob: 1, shift: 1 },
            next_unlikely: "Start".to_string(),
            next_likely: "P1_0".to_string(),
            prob_likely: u64::MAX / 2,
        },
        BitC {
            name: "P1_0".to_string(),
            probability: Probability { prob: 1, shift: 2 },
            next_unlikely: "Start".to_string(),
            next_likely: "P1_0".to_string(),
            prob_likely: u64::MAX / 2,
        },
    ];

    println!(
        r"
pub enum BitContext {{"
    );

    for BitC {
        name, probability, ..
    } in &variants
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
    } in &variants
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
        prob_likely,
    } in &variants
    {
        let likely_bit = probability.likely_bit();
        if next_likely != name {
            println!(
                "            {name} => {{
                if bit == {likely_bit:?} {{
                    if rng.next() < {prob_likely:x} {{ {next_likely} }} else {{ {name} }}
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
