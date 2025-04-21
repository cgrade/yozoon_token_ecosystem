use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::utils::constants::*;
use std::str::FromStr;

/// Raydium Liquidity Pool state
#[account]
pub struct RaydiumPool {
    /// Pool authority
    pub authority: Pubkey,
    
    /// Token A mint (project token)
    pub token_a_mint: Pubkey,
    
    /// Token B mint (SOL)
    pub token_b_mint: Pubkey,
    
    /// Token A pool account
    pub token_a_account: Pubkey,
    
    /// Token B pool account
    pub token_b_account: Pubkey,
    
    /// LP token mint
    pub lp_mint: Pubkey,
    
    /// Fee account (for fee distribution)
    pub fee_account: Pubkey,
    
    /// Pool initialization timestamp
    pub init_timestamp: i64,
    
    /// Bump for PDA derivation
    pub bump: u8,
    
    /// Whether the pool has been initialized
    pub is_initialized: bool,
}

impl RaydiumPool {
    pub const LEN: usize = 32 + // authority
                           32 + // token_a_mint
                           32 + // token_b_mint
                           32 + // token_a_account
                           32 + // token_b_account
                           32 + // lp_mint
                           32 + // fee_account
                           8 +  // init_timestamp
                           1 +  // bump
                           1;   // is_initialized
}

/// Raydium Fee Key NFT state
#[account]
pub struct FeeKeyNft {
    /// NFT mint address
    pub mint: Pubkey,
    
    /// Owner of the fee key
    pub owner: Pubkey,
    
    /// Associated pool
    pub pool: Pubkey,
    
    /// Fee percentage in basis points (e.g., 500 = 5%)
    pub fee_percentage: u64,
    
    /// Last claimed timestamp
    pub last_claimed: i64,
    
    /// Bump for PDA derivation
    pub bump: u8,
}

impl FeeKeyNft {
    pub const LEN: usize = 32 + // mint
                           32 + // owner
                           32 + // pool
                           8 +  // fee_percentage
                           8 +  // last_claimed
                           1;   // bump
}

/// Accounts required for creating a Raydium liquidity pool
#[derive(Accounts)]
pub struct CreateRaydiumPool<'info> {
    /// Pool state account (PDA)
    #[account(
        init,
        payer = payer,
        space = 8 + RaydiumPool::LEN,
        seeds = [seeds::RAYDIUM_POOL, token_a_mint.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, RaydiumPool>,
    
    /// Token A mint (project token)
    pub token_a_mint: Account<'info, Mint>,
    
    /// Token B mint (SOL)
    pub token_b_mint: Account<'info, Mint>,
    
    /// Token A pool account (will hold project tokens)
    #[account(mut)]
    pub token_a_account: Account<'info, TokenAccount>,
    
    /// Token B pool account (will hold SOL)
    #[account(mut)]
    pub token_b_account: Account<'info, TokenAccount>,
    
    /// LP token mint (to be created)
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    
    /// Fee account (for collecting fees)
    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>,
    
    /// Raydium AMM program
    pub raydium_program: Program<'info, Raydium>,
    
    /// User paying for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts required for creating a Fee Key NFT
#[derive(Accounts)]
pub struct CreateFeeKeyNft<'info> {
    /// Fee Key NFT state account (PDA)
    #[account(
        init,
        payer = payer,
        space = 8 + FeeKeyNft::LEN,
        seeds = [seeds::NFT_FEE_KEY, pool.key().as_ref()],
        bump
    )]
    pub fee_key_nft: Account<'info, FeeKeyNft>,
    
    /// NFT mint account
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    
    /// Associated pool
    pub pool: Account<'info, RaydiumPool>,
    
    /// Owner of the NFT (usually the admin)
    #[account(mut)]
    pub owner: SystemAccount<'info>,
    
    /// User paying for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// Raydium Fee Key program
    pub fee_key_program: Program<'info, FeeKeyProgram>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Raydium program (just for type-checking)
#[derive(Clone)]
pub struct Raydium;

impl anchor_lang::Id for Raydium {
    fn id() -> Pubkey {
        Pubkey::from_str(RAYDIUM_AMM_PROGRAM_ID).unwrap()
    }
}

/// Fee Key program (just for type-checking)
#[derive(Clone)]
pub struct FeeKeyProgram;

impl anchor_lang::Id for FeeKeyProgram {
    fn id() -> Pubkey {
        Pubkey::from_str(RAYDIUM_FEE_KEY_PROGRAM_ID).unwrap()
    }
}

/// Helper function to find the Raydium pool PDA
pub fn find_raydium_pool_pda(token_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[seeds::RAYDIUM_POOL, token_mint.as_ref()],
        &crate::ID
    )
}

/// Helper function to find the Fee Key NFT PDA
pub fn find_fee_key_nft_pda(pool: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[seeds::NFT_FEE_KEY, pool.as_ref()],
        &crate::ID
    )
}

/// Helper function to create Raydium pool instruction data
pub fn create_pool_instruction_data(
    token_a_amount: u64,
    token_b_amount: u64,
    fee_rate: u64,
    lock_period: u64
) -> Vec<u8> {
    let mut data = Vec::new();
    
    // Instruction index for create_pool
    data.push(1);
    
    // Token amounts
    data.extend_from_slice(&token_a_amount.to_le_bytes());
    data.extend_from_slice(&token_b_amount.to_le_bytes());
    
    // Fee rate
    data.extend_from_slice(&fee_rate.to_le_bytes());
    
    // Lock period
    data.extend_from_slice(&lock_period.to_le_bytes());
    
    data
}

/// Helper function to create NFT fee key instruction data
pub fn create_nft_fee_key_instruction_data(
    fee_percentage: u64
) -> Vec<u8> {
    let mut data = Vec::new();
    
    // Instruction index for create_nft_fee_key
    data.push(1);
    
    // Fee percentage
    data.extend_from_slice(&fee_percentage.to_le_bytes());
    
    data
} 