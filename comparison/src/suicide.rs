use std::{collections::HashMap, u8};

use serde::{Deserialize, Serialize};

const CSV: &str = include_str!("suicide.csv");

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, compactly::Encode, Serialize, Deserialize,
)]
pub struct Age {
    min: u8,
    max: u8,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, compactly::Encode, Serialize, Deserialize,
)]
pub enum Sex {
    Male,
    Female,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, compactly::Encode, Serialize, Deserialize,
)]
pub enum Race {
    White,
    Black,
    NativeAmerican,
    Asian,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, compactly::Encode, Serialize, Deserialize,
)]
pub struct SuicideFactors {
    sex: Sex,
    race: Race,
    age: Age,
    year: u16,
}

pub fn suicides_per_million() -> HashMap<SuicideFactors, u16> {
    let mut out = HashMap::new();
    for line in CSV.lines() {
        if let [sex_race_age, year, rate] = line.split(',').collect::<Vec<_>>().as_slice() {
            let year: u16 = year.parse().unwrap();
            if let [sex, race, age] = sex_race_age.split(": ").collect::<Vec<_>>().as_slice() {
                let sex = match *sex {
                    "Male" => Sex::Male,
                    "Female" => Sex::Female,
                    _ => panic!("Invalid sex {sex}"),
                };
                let race = match *race {
                    "White" => Race::White,
                    "Black or African American" => Race::Black,
                    "American Indian or Alaska Native" => Race::NativeAmerican,
                    "Asian or Pacific Islander" => Race::Asian,
                    _ => panic!("Invalid race {race}"),
                };
                let age = match *age {
                    "15-24 years" => Age { min: 15, max: 24 },
                    "25-44 years" => Age { min: 25, max: 44 },
                    "45-64 years" => Age { min: 45, max: 64 },
                    "65 years and over" => Age {
                        min: 65,
                        max: u8::MAX,
                    },
                    "65-74 years" => Age { min: 65, max: 74 },
                    "75-84 years" => Age { min: 75, max: 84 },
                    "85 years and over" => Age {
                        min: 85,
                        max: u8::MAX,
                    },
                    _ => panic!("Invalid age {age:?}"),
                };
                if !rate.is_empty() {
                    let rate: f64 = rate.parse().unwrap();
                    out.insert(
                        SuicideFactors {
                            sex,
                            race,
                            age,
                            year,
                        },
                        (rate * 10.0).round() as u16,
                    );
                }
            }
        }
    }
    out
}

#[test]
fn suicide_works() {
    suicides_per_million();
}
