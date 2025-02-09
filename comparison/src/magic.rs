#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
pub struct FullSet {
    meta: Meta,
    data: Set,
}

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
pub struct Meta {
    version: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
pub struct Set {
    baseSetSize: usize,
    block: Option<String>,
    code: String,
    totalSetSize: usize,
    cards: Vec<CardSet>,
}

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
pub struct CardSet {
    artist: Option<String>,
    artistIds: Option<Vec<String>>,
    asciiName: Option<String>,
    attractionLights: Option<Vec<usize>>,
    availability: Vec<String>,
    boosterTypes: Option<Vec<String>>,
    borderColor: String,
    cardParts: Option<Vec<String>>,
    colorIdentity: Vec<String>,
    colorIndicator: Option<Vec<String>>,
    colors: Vec<String>,
    convertedManaCost: usize,
    defense: Option<String>,
    duelDeck: Option<String>,
    edhrecRank: Option<usize>,
    // edhrecSaltiness: Option<usize>,
    faceConvertedManaCost: Option<usize>,
    faceFlavorName: Option<String>,
    faceManaValue: Option<usize>,
    faceName: Option<String>,
    finishes: Vec<String>,
    flavorName: Option<String>,
    flavorText: Option<String>,
    frameEffects: Option<Vec<String>>,
    frameVersion: String,
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
    keywords: Option<Vec<String>>,
    language: String,
    layout: String,
    leadershipSkills: Option<LeadershipSkills>,
    // legalities: Legalities;
    life: Option<String>,
    loyalty: Option<String>,
    manaCost: Option<String>,
    manaValue: usize,
    name: String,
    number: String,
    originalPrintings: Option<Vec<String>>,
    originalReleaseDate: Option<String>,
    originalText: Option<String>,
    originalType: Option<String>,
    otherFaceIds: Option<Vec<String>>,
    power: Option<String>,
    printings: Option<Vec<String>>,
    promoTypes: Option<Vec<String>>,
    // purchaseUrls: PurchaseUrls;
    rarity: String,
    // relatedCards?: RelatedCards;
    rebalancedPrintings: Option<Vec<String>>,
    // rulings?: Rulings[];
    // securityStamp: Option<String>,
    setCode: String,
    side: Option<String>,
    signature: Option<String>,
    // sourceProducts?: SourceProducts;
    subsets: Option<Vec<String>>,
    subtypes: Vec<String>,
    supertypes: Vec<String>,
    text: Option<String>,
    toughness: Option<String>,
    types: Vec<String>,
    uuid: String,
    variations: Option<Vec<String>>,
    watermark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
pub struct LeadershipSkills {
    brawl: bool,
    commander: bool,
    oathbreaker: bool,
}

#[derive(Debug, Serialize, Deserialize, compactly::Encode, Clone)]
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
    println!("{value:?}");
}
