use anchor_lang::prelude::*;

/// Error codes for the Yozoon program
#[error_code]
pub enum YozoonError {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,

    #[msg("The protocol is currently paused")]
    ProtocolPaused,

    #[msg("The price data is stale")]
    StalePrice,

    #[msg("Migration conditions are not met")]
    MigrationConditionsNotMet,

    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,

    #[msg("Invalid price points")]
    InvalidPricePoints,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    #[msg("Invalid SOL amount")]
    InvalidSolAmount,

    #[msg("Insufficient SOL balance")]
    InsufficientSolBalance,

    #[msg("Insufficient token balance")]
    InsufficientTokenBalance,

    #[msg("Migration already completed")]
    MigrationAlreadyCompleted,

    #[msg("Invalid referrer")]
    InvalidReferrer,

    #[msg("Invalid recipient")]
    InvalidRecipient,

    #[msg("Invalid mint")]
    InvalidMint,

    #[msg("Invalid treasury")]
    InvalidTreasury,
}
