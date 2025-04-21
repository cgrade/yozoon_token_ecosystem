use anchor_lang::prelude::*;

#[error_code]
pub enum YozoonError {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Protocol is paused")]
    ProtocolPaused,
    #[msg("Invalid parameter")]
    InvalidParameter,
    #[msg("Too many price points")]
    TooManyPricePoints,
    #[msg("Amount too small")]
    AmountTooSmall,
    #[msg("Protocol has been migrated")]
    Migrated,
    #[msg("Dust amount")]
    DustAmount,
    #[msg("Supply exceeded")]
    SupplyExceeded,
    #[msg("Self referral not allowed")]
    SelfReferral,
    #[msg("Referral fee too high")]
    FeeTooHigh,
    #[msg("Insufficient reserve")]
    InsufficientReserve,
    #[msg("Migration threshold not reached")]
    MigrationThresholdNotReached,
    #[msg("Protocol already migrated to Raydium")]
    AlreadyMigrated,
    #[msg("Failed to create Raydium pool")]
    RaydiumPoolCreationFailed,
    #[msg("Failed to create NFT fee key")]
    NftFeeKeyCreationFailed,
    #[msg("Invalid SOL amount for migration")]
    InvalidMigrationAmount,
    #[msg("Raydium liquidity lock failed")]
    LiquidityLockFailed,
} 