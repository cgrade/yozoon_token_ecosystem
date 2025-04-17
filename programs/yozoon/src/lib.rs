use anchor_lang::prelude::*;

// Import modular components
mod state;
mod errors;
mod events;
mod instructions;
mod utils;

// Re-export components for external use
pub use state::*;
pub use events::*;
pub use instructions::*;
pub use utils::*;

// Declare the program ID
declare_id!("7giegFn7Wy4McS1eKr1cpjhpE9TibEywydG57PSao9bM");

/// Yozoon token ecosystem program
#[program]
pub mod yozoon {
    use super::*;

    // Admin instructions
    
    /// Initializes the token mint and configuration account
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        instructions::initialize_mint(ctx)
    }
    
    /// Transfer admin role to a new account (two-step process)
    pub fn transfer_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
        instructions::transfer_admin(ctx, new_admin)
    }

    /// Accept admin role (must be called by pending admin)
    pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
        instructions::accept_admin(ctx)
    }

    /// Emergency pause/unpause of the protocol
    pub fn set_pause_state(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
        instructions::set_pause_state(ctx, paused)
    }

    // Bonding curve instructions
    
    /// Sets up the bonding curve with predefined price points
    pub fn initialize_bonding_curve(
        ctx: Context<InitializeBondingCurve>,
        price_points: Vec<u64>
    ) -> Result<()> {
        instructions::initialize_bonding_curve(ctx, price_points)
    }

    /// Allows users to buy tokens with SOL, applying referral fees if set
    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        instructions::buy_tokens(ctx, sol_amount)
    }

    /// Calculate current token price at the current supply level
    pub fn calculate_current_price(ctx: Context<GetCurrentPrice>) -> Result<u64> {
        instructions::calculate_current_price(ctx)
    }

    /// Calculate tokens to be received for a specific SOL amount
    pub fn calculate_tokens_for_sol(
        ctx: Context<CalculateTokens>,
        sol_amount: u64
    ) -> Result<u64> {
        instructions::calculate_tokens_for_sol(ctx, sol_amount)
    }

    // Referral instructions
    
    /// Sets a referrer for a user
    pub fn set_referral(ctx: Context<SetReferral>, referrer: Pubkey) -> Result<()> {
        instructions::set_referral(ctx, referrer)
    }

    /// Updates the referral fee percentage for a user (admin only)
    pub fn update_referral_fee(
        ctx: Context<UpdateReferralFee>,
        new_fee_percentage: u64
    ) -> Result<()> {
        instructions::update_referral_fee(ctx, new_fee_percentage)
    }

    // Airdrop instructions
    
    /// Airdrops tokens to a recipient without affecting the bonding curve
    pub fn airdrop_tokens(ctx: Context<AirdropTokens>, amount: u64) -> Result<()> {
        instructions::airdrop_tokens(ctx, amount)
    }

    // Migration instructions
    
    /// Migrates liquidity to Raydium when conditions are met
    pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()> {
        instructions::migrate_to_raydium(ctx)
    }
}