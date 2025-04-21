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
} 