#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
pub struct FullSet {
    meta: Meta,
    pub data: Set,
}

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
pub struct Meta {
    version: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
pub struct Set {
    baseSetSize: usize,
    block: Option<String>,
    code: String,
    totalSetSize: usize,
    pub cards: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
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
    convertedManaCost: usize,
    #[compactly(LowCardinality)]
    defense: Option<String>,
    duelDeck: Option<String>,
    edhrecRank: Option<usize>,
    // edhrecSaltiness: Option<usize>,
    faceConvertedManaCost: Option<usize>,
    #[compactly(LowCardinality)]
    faceFlavorName: Option<String>,
    faceManaValue: Option<usize>,
    #[compactly(LowCardinality)]
    faceName: Option<String>,
    #[compactly(LowCardinality)]
    finishes: Vec<String>,
    #[compactly(LowCardinality)]
    flavorName: Option<String>,
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
    manaValue: usize,
    name: String,
    number: String,
    #[compactly(LowCardinality)]
    originalPrintings: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    originalReleaseDate: Option<String>,
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
    signature: Option<String>,
    // sourceProducts?: SourceProducts;
    #[compactly(LowCardinality)]
    subsets: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    subtypes: Vec<String>,
    #[compactly(LowCardinality)]
    supertypes: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
pub struct LeadershipSkills {
    brawl: bool,
    commander: bool,
    oathbreaker: bool,
}

#[derive(Debug, Serialize, Deserialize, compactly::v0::Encode, compactly::v1::Encode, Clone)]
pub struct Identifiers {
    abuId: Option<String>,
    cardKingdomEtchedId: Option<String>,
    cardKingdomFoilId: Option<String>,
    cardKingdomId: Option<String>,
    cardsphereId: Option<String>,
    cardsphereFoilId: Option<String>,
    cardtraderId: Option<String>,
    csiId: Option<String>,
    mcmId: Option<String>,
    mcmMetaId: Option<String>,
    miniaturemarketId: Option<String>,
    mtgArenaId: Option<String>,
    mtgjsonFoilVersionId: Option<String>,
    mtgjsonNonFoilVersionId: Option<String>,
    mtgjsonV4Id: Option<String>,
    mtgoFoilId: Option<String>,
    mtgoId: Option<String>,
    multiverseId: Option<String>,
    scgId: Option<String>,
    scryfallId: Option<String>,
    scryfallCardBackId: Option<String>,
    scryfallOracleId: Option<String>,
    scryfallIllustrationId: Option<String>,
    tcgplayerProductId: Option<String>,
    tcgplayerEtchedProductId: Option<String>,
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
