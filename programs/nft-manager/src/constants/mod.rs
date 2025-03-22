use anchor_lang::constant;

#[constant]
pub const NFT_MANAGER_TAG: &[u8] = b"nftmg";

#[constant]
pub const FEES_COLLECTOR_TAG: &[u8] = b"fcolt";

#[constant]
pub const MINT_FEES_COLLECTOR_TAG: &[u8] = b"mfcolt";

#[constant]
pub const USER_TAG: &[u8] = b"usert";

#[constant]
pub const MINT_TAG: &[u8] = b"mintt";

#[constant]
pub const COLLECTION_TAG: &[u8] = b"collt";

#[constant]
pub const COLLECTION_TOKEN_ACCOUNT_TAG: &[u8] = b"coltat";

#[constant]
pub const LISTING_TAG: &[u8] = b"listt";

#[constant]
pub const LISTING_TOKEN_ACCOUNT_TAG: &[u8] = b"listtat";

#[constant]
pub const FINALIZE_MINT_DATA_TAG: &[u8] = b"finmdt";

#[constant]
pub const FINALIZE_FRACTIONALIZE_DATA_TAG: &[u8] = b"finfdt";

// TODO: Update this constant
#[constant]
pub const MAX_AGE: u64 = 259_200;

#[constant]
pub const WEIGHT_KEY: &str = "weight";

#[constant]
pub const DISCRIMINANT_KEY: &str = "discriminant";

#[constant]
pub const COLLECTION_KEY: &str = "collection";
