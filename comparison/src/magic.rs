#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;

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
    artist: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    artistIds: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    asciiName: Option<Arc<str>>,
    attractionLights: Option<Vec<usize>>,
    #[compactly(LowCardinality)]
    availability: Vec<Arc<str>>,
    #[compactly(LowCardinality)]
    boosterTypes: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    borderColor: Arc<str>,
    #[compactly(LowCardinality)]
    cardParts: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    colorIdentity: Vec<Arc<str>>,
    #[compactly(LowCardinality)]
    colorIndicator: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    colors: Vec<Arc<str>>,
    #[compactly(Small)]
    convertedManaCost: usize,
    #[compactly(LowCardinality)]
    defense: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    duelDeck: Option<Arc<str>>,
    edhrecRank: Option<usize>,
    // edhrecSaltiness: Option<usize>,
    #[compactly(Small)]
    faceConvertedManaCost: Option<usize>,
    #[compactly(LowCardinality)]
    faceFlavorName: Option<Arc<str>>,
    #[compactly(Small)]
    faceManaValue: Option<usize>,
    #[compactly(LowCardinality)]
    faceName: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    finishes: Vec<Arc<str>>,
    #[compactly(LowCardinality)]
    flavorName: Option<Arc<str>>,
    #[compactly(Compressible)]
    flavorText: Option<String>,
    #[compactly(LowCardinality)]
    frameEffects: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    frameVersion: Arc<str>,
    #[compactly(LowCardinality)]
    hand: Option<Arc<str>>,
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
    keywords: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    language: Arc<str>,
    #[compactly(LowCardinality)]
    layout: Arc<str>,
    leadershipSkills: Option<LeadershipSkills>,
    // legalities: Legalities;
    #[compactly(LowCardinality)]
    life: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    loyalty: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    manaCost: Option<Arc<str>>,
    #[compactly(Small)]
    manaValue: usize,
    #[compactly(Compressible)]
    name: String,
    #[compactly(Compressible)]
    number: String,
    #[compactly(LowCardinality)]
    originalPrintings: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    originalReleaseDate: Option<Arc<str>>,
    #[compactly(Compressible)]
    originalText: Option<String>,
    #[compactly(LowCardinality)]
    originalType: Option<Arc<str>>,
    otherFaceIds: Option<Vec<String>>,
    #[compactly(LowCardinality)]
    power: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    printings: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    promoTypes: Option<Vec<Arc<str>>>,
    // purchaseUrls: PurchaseUrls;
    #[compactly(LowCardinality)]
    rarity: Arc<str>,
    // relatedCards?: RelatedCards;
    #[compactly(LowCardinality)]
    rebalancedPrintings: Option<Vec<Arc<str>>>,
    // rulings?: Rulings[];
    // securityStamp: Option<String>,
    #[compactly(LowCardinality)]
    setCode: Arc<str>,
    #[compactly(LowCardinality)]
    side: Option<Arc<str>>,
    #[compactly(Compressible)]
    signature: Option<String>,
    // sourceProducts?: SourceProducts;
    #[compactly(LowCardinality)]
    subsets: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    subtypes: Vec<Arc<str>>,
    #[compactly(LowCardinality)]
    supertypes: Vec<Arc<str>>,
    #[compactly(Compressible)]
    text: Option<String>,
    #[compactly(LowCardinality)]
    toughness: Option<Arc<str>>,
    #[compactly(LowCardinality)]
    types: Vec<Arc<str>>,
    uuid: String,
    #[compactly(LowCardinality)]
    variations: Option<Vec<Arc<str>>>,
    #[compactly(LowCardinality)]
    watermark: Option<Arc<str>>,
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
