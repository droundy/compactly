#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct FullSet {
    meta: Meta,
    pub data: Set,
}

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct Meta {
    version: String,
    date: String,
}

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct Set {
    baseSetSize: usize,
    block: Option<String>,
    code: String,
    totalSetSize: usize,
    pub cards: Vec<Card>,
}

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct Card {
    #[compactly(LowCardinality)]
    artist: Option<String>,
    #[compactly(LowCardinality)]
    artistIds: Option<Vec<String>>,
    asciiName: Option<String>,
    attractionLights: Option<Vec<usize>>,
    #[compactly(LowCardinality)]
    availability: Vec<String>,
    #[compactly(LowCardinality)]
    boosterTypes: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    borderColor: String,
    cardParts: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    colorIdentity: Vec<String>,
    #[compactly(LowCardinality)]
    colorIndicator: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    colors: Vec<String>,
    #[compactly(Small)]
    convertedManaCost: usize,
    #[compactly(LowCardinality)]
    defense: Option<String>,
    duelDeck: Option<String>,
    edhrecRank: Option<usize>,
    // edhrecSaltiness: Option<usize>,
    #[compactly(Small)]
    faceConvertedManaCost: Option<usize>,
    #[compactly(LowCardinality)]
    faceFlavorName: Option<String>,
    #[compactly(Small)]
    faceManaValue: Option<usize>,
    #[compactly(LowCardinality)]
    faceName: Option<String>,
    #[compactly(LowCardinality)]
    finishes: Vec<String>,
    #[compactly(LowCardinality)]
    flavorName: Option<String>,
    #[compactly(Compressible)]
    flavorText: Option<String>,
    #[compactly(LowCardinality)]
    frameEffects: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    frameVersion: String,
    #[compactly(LowCardinality)]
    hand: Option<String>,
    hasAlternativeDeckLimit: Option<bool>,
    hasContentWarning: Option<bool>,
    hasFoil: bool,
    hasNonFoil: bool,
    identifiers: Identifiers,
    isAlternative: Option<bool>,
    isFullArt: Option<bool>,
    isFunny: Option<bool>,
    isOnlineOnly: Option<bool>,
    isOversized: Option<bool>,
    isPromo: Option<bool>,
    isRebalanced: Option<bool>,
    isReprint: Option<bool>,
    isReserved: Option<bool>,
    isStarter: Option<bool>,
    isStorySpotlight: Option<bool>,
    isTextless: Option<bool>,
    isTimeshifted: Option<bool>,
    #[compactly(LowCardinality)]
    keywords: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    language: String,
    #[compactly(LowCardinality)]
    layout: String,
    leadershipSkills: Option<LeadershipSkills>,
    // legalities: Legalities;
    #[compactly(LowCardinality)]
    life: Option<String>,
    #[compactly(LowCardinality)]
    loyalty: Option<String>,
    #[compactly(LowCardinality)]
    manaCost: Option<String>,
    #[compactly(Small)]
    manaValue: usize,
    #[compactly(Compressible)]
    name: String,
    #[compactly(Compressible)]
    number: String,
    #[compactly(LowCardinality)]
    originalPrintings: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    originalReleaseDate: Option<String>,
    #[compactly(Compressible)]
    originalText: Option<String>,
    #[compactly(LowCardinality)]
    originalType: Option<String>,
    otherFaceIds: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    power: Option<String>,
    #[compactly(LowCardinality)]
    printings: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    promoTypes: Option<Vec<String>>,
    // purchaseUrls: PurchaseUrls;
    #[compactly(LowCardinality)]
    rarity: String,
    // relatedCards?: RelatedCards;
    #[compactly(LowCardinality)]
    rebalancedPrintings: Option<Vec<String>>,
    // rulings?: Rulings[];
    // securityStamp: Option<String>,
    #[compactly(LowCardinality)]
    setCode: String,
    #[compactly(LowCardinality)]
    side: Option<String>,
    #[compactly(Compressible)]
    signature: Option<String>,
    // sourceProducts?: SourceProducts;
    #[compactly(LowCardinality)]
    subsets: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    subtypes: Vec<String>,
    #[compactly(LowCardinality)]
    supertypes: Vec<String>,
    #[compactly(Compressible)]
    text: Option<String>,
    #[compactly(LowCardinality)]
    toughness: Option<String>,
    #[compactly(LowCardinality)]
    types: Vec<String>,
    uuid: String,
    #[compactly(LowCardinality)]
    variations: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    watermark: Option<String>,
}

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct LeadershipSkills {
    brawl: bool,
    commander: bool,
    oathbreaker: bool,
}

#[derive(
    Debug, Serialize, Deserialize, compactly::v1::Encode, compactly::v2::Encode, Clone, PartialEq,
)]
pub struct Identifiers {
    #[compactly(Compressible)]
    abuId: Option<String>,
    #[compactly(Compressible)]
    cardKingdomEtchedId: Option<String>,
    #[compactly(Compressible)]
    cardKingdomFoilId: Option<String>,
    #[compactly(Compressible)]
    cardKingdomId: Option<String>,
    #[compactly(Compressible)]
    cardsphereId: Option<String>,
    #[compactly(Compressible)]
    cardsphereFoilId: Option<String>,
    #[compactly(Compressible)]
    cardtraderId: Option<String>,
    #[compactly(Compressible)]
    csiId: Option<String>,
    #[compactly(Compressible)]
    mcmId: Option<String>,
    #[compactly(Compressible)]
    mcmMetaId: Option<String>,
    #[compactly(Compressible)]
    miniaturemarketId: Option<String>,
    #[compactly(Compressible)]
    mtgArenaId: Option<String>,
    #[compactly(Compressible)]
    mtgjsonFoilVersionId: Option<String>,
    #[compactly(Compressible)]
    mtgjsonNonFoilVersionId: Option<String>,
    #[compactly(Compressible)]
    mtgjsonV4Id: Option<String>,
    #[compactly(Compressible)]
    mtgoFoilId: Option<String>,
    #[compactly(Compressible)]
    mtgoId: Option<String>,
    #[compactly(Compressible)]
    multiverseId: Option<String>,
    #[compactly(Compressible)]
    scgId: Option<String>,
    #[compactly(Compressible)]
    scryfallId: Option<String>,
    #[compactly(Compressible)]
    scryfallCardBackId: Option<String>,
    #[compactly(Compressible)]
    scryfallOracleId: Option<String>,
    #[compactly(Compressible)]
    scryfallIllustrationId: Option<String>,
    #[compactly(Compressible)]
    tcgplayerProductId: Option<String>,
    #[compactly(Compressible)]
    tcgplayerEtchedProductId: Option<String>,
    #[compactly(Compressible)]
    tntId: Option<String>,
}

const TENTH_EDITION_JSON: &str = include_str!("ten.json");
pub fn tenth_edition() -> FullSet {
    serde_json::from_str(TENTH_EDITION_JSON).unwrap()
}

#[test]
fn ten() {
    let value: FullSet = tenth_edition();
    println!("{value:#?}");
}
