use anchor_lang::error_code;

#[error_code]
pub enum NFTManagerError {
    #[msg("Same authority")]
    SameAuthority,
    #[msg("Overflow  occurred")]
    Overflow,
    #[msg("Invalid metadata")]
    InvalidMetadata,
    #[msg("Price calculation fail")]
    PriceCalculationFail,
    #[msg("Invalid weight")]
    InvalidWeight,
    #[msg("Negative price")]
    NegativePrice,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Only admin allowed")]
    OnlyAdminAllowed,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Not owner")]
    NotOwner,
    #[msg("Invalid listing")]
    InvalidListing,
    #[msg("Unauthorized")]
    UnAuthorized,
    #[msg("Invalid finalize data")]
    InvalidFinalizeData,
    #[msg("Invalid discriminant")]
    InvalidTokenAccount,
    #[msg("Mint did not match with finalize data")]
    MintFinalizeDataMismatch,
    #[msg("Invalid mint supply")]
    InvalidMintSupply,
    #[msg("Only future authority allowed")]
    OnlyFutureAuthorityAllowed,
    #[msg("No future authority")]
    NoFutureAuthority,
}
