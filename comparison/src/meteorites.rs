use std::collections::BTreeMap;

const CSV: &str = include_str!("meteorites.csv"); // from NASA

#[derive(
    Debug, PartialEq, PartialOrd, compactly::v1::Encode, serde::Serialize, serde::Deserialize,
)]
pub enum NameType {
    Valid,
    Relict,
}

#[derive(
    Debug, PartialEq, PartialOrd, compactly::v1::Encode, serde::Serialize, serde::Deserialize,
)]
pub enum Fall {
    Fell,
    Found,
}

#[derive(
    Debug, PartialEq, PartialOrd, compactly::v1::Encode, serde::Serialize, serde::Deserialize,
)]
pub struct Meteorite {
    name: String,
    nametype: NameType,
    fall: Fall,
    #[compactly(Small)]
    year: u64,
    #[serde(alias = "mass (g)", default)]
    #[compactly(Decimal)]
    mass: f32,
    #[compactly(LowCardinality)]
    recclass: String,
    #[compactly(Decimal)]
    reclat: f32,
    #[compactly(Decimal)]
    reclong: f32,
}

#[derive(
    Debug, PartialEq, PartialOrd, compactly::v1::Encode, serde::Serialize, serde::Deserialize,
)]
pub struct MeteoriteData {
    nametype: NameType,
    fall: Fall,
    #[compactly(Small)]
    year: u64,
    #[serde(alias = "mass (g)", default)]
    #[compactly(Decimal)]
    mass: f32,
    #[compactly(LowCardinality)]
    recclass: String,
    #[compactly(Decimal)]
    reclat: f32,
    #[compactly(Decimal)]
    reclong: f32,
}
impl From<Meteorite> for MeteoriteData {
    fn from(
        Meteorite {
            name: _,
            nametype,
            fall,
            year,
            mass,
            recclass,
            reclat,
            reclong,
        }: Meteorite,
    ) -> Self {
        MeteoriteData {
            nametype,
            fall,
            year,
            mass,
            recclass,
            reclat,
            reclong,
        }
    }
}

pub fn meteorites() -> Vec<MeteoriteData> {
    let mut bytes = CSV.as_bytes();
    let mut rdr = csv::Reader::from_reader(&mut bytes);
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        out.push(result.unwrap());
    }
    out
}

pub fn meteorite_names() -> BTreeMap<String, MeteoriteData> {
    let mut bytes = CSV.as_bytes();
    let mut rdr = csv::Reader::from_reader(&mut bytes);
    let mut out = BTreeMap::new();
    for result in rdr.deserialize() {
        let m: Meteorite = result.unwrap();
        out.insert(m.name.clone(), MeteoriteData::from(m));
    }
    out
}

#[test]
fn check() {
    meteorites();
}
