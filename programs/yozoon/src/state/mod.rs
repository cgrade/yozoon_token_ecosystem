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
    
    /// Total value of the project
    pub total_value: u64,
    
    /// Total supply of the project
    pub total_supply: u64,
}

impl Config {
    pub const LEN: usize = 32 + // admin
                            32 + // mint
                            1 + // bump
                            1 + // paused
                            32 + // treasury
                            1 + // option tag
                            32 + // pending_admin
                            8 + // total_value
                            8; // total_supply
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
    pub const LEN: usize = 8 + // total_sold_supply
                            8 + // total_sol_raised
                            4 + // vec length
                            (32 * 10) + // price_points (max 10 points)
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
    pub const LEN: usize = 32 + // referrer
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

impl AirdropLedger {
    pub const LEN: usize = 8 + // total_airdropped
                            1; // bump
}
