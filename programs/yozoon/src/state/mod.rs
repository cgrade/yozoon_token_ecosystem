use anchor_lang::prelude::*;

/// Configuration account holding admin info and program settings
#[account]
pub struct Config {
    /// Admin public key for restricted actions
    pub admin: Pubkey,
    
    /// Token mint public key
    pub mint: Pubkey,
    
    /// Bump seed for PDA
    pub bump: u8,
    
    /// Emergency pause flag
    pub paused: bool,
    
    /// Treasury account for project fees
    pub treasury: Pubkey,
    
    /// Pending admin for ownership transfer
    pub pending_admin: Option<Pubkey>,
}

impl Config {
    pub const SPACE: usize = 32 + // admin
                            32 + // mint
                            1 + // bump
                            1 + // paused
                            32 + // treasury
                            1 + // option tag
                            32; // pending_admin
}

/// Bonding curve state account storing price points and supply data
#[account]
pub struct BondingCurve {
    /// Total SOL collected (net of fees)
    pub total_sol_raised: u64,
    
    /// Total tokens sold via bonding curve
    pub total_sold_supply: u64,
    
    /// Precomputed price points for the curve
    pub price_points: Vec<u64>,
    
    /// Bump seed for PDA
    pub bump: u8,
    
    /// Migration status
    pub is_migrated: bool,
}

impl BondingCurve {
    pub const MAX_PRICE_POINTS: usize = 100;
    pub const SPACE: usize = 8 + // total_sol_raised
                            8 + // total_sold_supply
                            4 + // vec length
                            8 * Self::MAX_PRICE_POINTS + // price points
                            1 + // bump
                            1; // is_migrated
}

/// Referral state account storing referrer info and fee percentage
#[account]
pub struct Referral {
    /// Referrer's public key
    pub referrer: Pubkey,
    
    /// Fee percentage (basis points, 10000 = 100%)
    pub fee_percentage: u64,
    
    /// Bump seed for PDA
    pub bump: u8,
}

impl Referral {
    pub const SPACE: usize = 32 + // referrer
                            8 + // fee_percentage
                            1; // bump
}

/// Airdrop ledger account tracking total tokens airdropped
#[account]
pub struct AirdropLedger {
    /// Total tokens airdropped
    pub total_airdropped: u64,
    
    /// Bump seed for PDA
    pub bump: u8,
}
