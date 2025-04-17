use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::*;
use crate::utils::constants::*;
use crate::utils::constants::seeds::*;

/// Accounts required for initializing the token mint
#[derive(Accounts)]
pub struct InitializeMint<'info> {
    /// Configuration account (PDA)
    #[account(
        init,
        payer = admin,
        space = 8 + Config::SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    
    /// Token mint account
    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = config,
    )]
    pub mint: Account<'info, Mint>,
    
    /// Admin account (pays rent and becomes initial admin)
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts required for initializing the bonding curve
#[derive(Accounts)]
pub struct InitializeBondingCurve<'info> {
    /// Configuration account (PDA)
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        init,
        payer = admin,
        space = 8 + BondingCurve::SPACE,
        seeds = [b"bonding_curve"],
        bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

/// Accounts required for buying tokens
#[derive(Accounts)]
pub struct BuyTokens<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = !config.paused @ YozoonError::ProtocolPaused
    )]
    pub config: Account<'info, Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// Token mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// User's token account to receive tokens
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    /// User account (signs transaction and pays SOL)
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
}

/// Accounts required for setting a referral
#[derive(Accounts)]
pub struct SetReferral<'info> {
    /// User's referral account (PDA)
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Referral::SPACE,
        seeds = [b"referral", user.key().as_ref()],
        bump
    )]
    pub referral: Account<'info, Referral>,
    
    /// User account (signs transaction and pays rent)
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

/// Accounts required for updating referral fee
#[derive(Accounts)]
pub struct UpdateReferralFee<'info> {
    /// Configuration account (PDA) to validate admin
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    /// User's referral account (PDA)
    #[account(mut)]
    pub referral: Account<'info, Referral>,
}

/// Accounts required for airdropping tokens
#[derive(Accounts)]
pub struct AirdropTokens<'info> {
    /// Configuration account (PDA)
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    /// Airdrop ledger account (PDA)
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + 8 + 1, // Discriminator + total_airdropped + bump
        seeds = [AIRDROP_LEDGER],
        bump
    )]
    pub airdrop_ledger: Account<'info, AirdropLedger>,
    
    /// Token mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// Recipient's token account to receive airdropped tokens
    #[account(mut)]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
}

/// Accounts required for migrating to Raydium
#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    /// Configuration account (PDA) to validate admin
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// Pyth price account for SOL/USD
    /// CHECK: Pyth account is validated in the instruction
    pub pyth_price_account: AccountInfo<'info>,
}

/// Accounts required for admin actions
#[derive(Accounts)]
pub struct AdminAction<'info> {
    /// Configuration account (PDA)
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
}

/// Accounts required for accepting admin role
#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    /// Configuration account (PDA)
    #[account(mut)]
    pub pending_admin: Signer<'info>,
    
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.pending_admin == Some(pending_admin.key()) @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
}

/// Accounts required for viewing current price
#[derive(Accounts)]
pub struct GetCurrentPrice<'info> {
    /// Bonding curve account (PDA)
    #[account(
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
}

/// Accounts required for calculating tokens for a SOL amount
#[derive(Accounts)]
pub struct CalculateTokens<'info> {
    /// Bonding curve account (PDA)
    #[account(
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
}
