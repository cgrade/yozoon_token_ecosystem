use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::{invoke, invoke_signed};
use anchor_lang::solana_program::instruction::Instruction;
use anchor_spl::token::{Mint, Token, TokenAccount};
use std::str::FromStr;
use crate::errors::*;
use crate::events::*;
use crate::state::*;
use crate::utils::constants::*;
use crate::raydium::*;

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [seeds::BONDING_CURVE],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
    
    /// Token mint (project token)
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// Wrapped SOL mint
    #[account(
        address = Pubkey::from_str(WRAPPED_SOL_MINT).unwrap()
    )]
    pub wrapped_sol: Account<'info, Mint>,
    
    /// Treasury account (source of SOL for liquidity)
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    
    /// Project token account for pool liquidity
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    /// SOL token account for pool liquidity
    #[account(mut)]
    pub sol_token_account: Account<'info, TokenAccount>,
    
    /// LP token mint (to be created)
    #[account(mut)]
    pub lp_mint: Account<'info, Mint>,
    
    /// Fee account for pool fees
    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>,
    
    /// NFT mint for fee key
    #[account(mut)]
    pub nft_mint: Account<'info, Mint>,
    
    /// Raydium pool account (PDA to be created)
    #[account(
        init,
        payer = admin,
        space = 8 + RaydiumPool::LEN,
        seeds = [seeds::RAYDIUM_POOL, mint.key().as_ref()],
        bump
    )]
    pub raydium_pool: Account<'info, RaydiumPool>,
    
    /// Fee key NFT account (PDA to be created)
    #[account(
        init,
        payer = admin,
        space = 8 + FeeKeyNft::LEN,
        seeds = [seeds::NFT_FEE_KEY, raydium_pool.key().as_ref()],
        bump
    )]
    pub fee_key_nft: Account<'info, FeeKeyNft>,
    
    /// Admin account who triggers migration
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Raydium AMM program
    pub raydium_program: Program<'info, Raydium>,
    
    /// Raydium Fee Key program
    pub fee_key_program: Program<'info, FeeKeyProgram>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

/// Verify if migration conditions are met
pub fn check_migration_conditions(
    ctx: &Context<MigrateToRaydium>
) -> Result<()> {
    let curve = &ctx.accounts.bonding_curve;
    
    // Check if we've reached the SOL threshold
    let total_sol = curve.total_sol_raised;
    msg!("Current SOL raised: {}", total_sol);
    
    require!(
        total_sol >= MIGRATION_SOL_MIN && total_sol <= MIGRATION_SOL_MAX,
        YozoonError::MigrationThresholdNotReached
    );
    
    // Check if already migrated
    require!(
        !curve.is_migrated,
        YozoonError::AlreadyMigrated
    );
    
    Ok(())
}

/// Migrates liquidity to Raydium when conditions are met
pub fn migrate_to_raydium(ctx: &mut Context<MigrateToRaydium>) -> Result<()> {
    // 1. Verify migration conditions
    check_migration_conditions(ctx)?;
    
    // 2. Mark as migrated to prevent further buying/selling via bonding curve
    let curve = &mut ctx.accounts.bonding_curve;
    curve.is_migrated = true;
    
    // 3. Calculate total values for the pool
    let total_sol = curve.total_sol_raised;
    let total_supply = curve.total_sold_supply;
    
    // 4. Create Raydium pool with permanent liquidity locking
    create_raydium_pool(ctx, total_supply, total_sol)?;
    
    // 5. Create NFT fee key for fee distribution
    create_fee_key_nft(ctx)?;
    
    // 6. Emit migration event
    emit!(MigrationEvent {
        total_sol,
        total_usd: 0, // Not using USD value anymore
        total_supply,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    // 7. Emit more detailed migration event with pool information
    emit!(MigrationCompletedEvent {
        sol_value: total_sol,
        tokens_sold: total_supply,
        admin: ctx.accounts.admin.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Migration to Raydium completed. Total SOL: {}, Total tokens: {}", total_sol, total_supply);
    msg!("Raydium pool created with permanent liquidity locking");
    msg!("NFT fee key created for fee distribution");
    
    Ok(())
}

/// Creates a Raydium liquidity pool with permanent liquidity locking
fn create_raydium_pool(
    ctx: &mut Context<MigrateToRaydium>,
    token_amount: u64,
    sol_amount: u64
) -> Result<()> {
    let pool = &mut ctx.accounts.raydium_pool;
    
    // Initialize pool data
    pool.authority = ctx.accounts.config.key();
    pool.token_a_mint = ctx.accounts.mint.key();
    pool.token_b_mint = ctx.accounts.wrapped_sol.key();
    pool.token_a_account = ctx.accounts.token_account.key();
    pool.token_b_account = ctx.accounts.sol_token_account.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.fee_account = ctx.accounts.fee_account.key();
    pool.init_timestamp = Clock::get()?.unix_timestamp;
    pool.bump = *ctx.bumps.get("raydium_pool").unwrap();
    pool.is_initialized = true;
    
    // Create instruction data for Raydium pool creation
    let instruction_data = create_pool_instruction_data(
        token_amount,
        sol_amount,
        RAYDIUM_POOL_FEE,
        LIQUIDITY_LOCK_PERIOD
    );
    
    // Create the CPI instruction
    let raydium_program_id = ctx.accounts.raydium_program.key();
    let accounts = vec![
        AccountMeta::new(ctx.accounts.config.key(), false),
        AccountMeta::new(ctx.accounts.mint.key(), false),
        AccountMeta::new(ctx.accounts.wrapped_sol.key(), false),
        AccountMeta::new(ctx.accounts.token_account.key(), false),
        AccountMeta::new(ctx.accounts.sol_token_account.key(), false),
        AccountMeta::new(ctx.accounts.lp_mint.key(), false),
        AccountMeta::new(ctx.accounts.fee_account.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
    ];
    
    let ix = Instruction {
        program_id: raydium_program_id,
        accounts,
        data: instruction_data,
    };
    
    // Execute the CPI with PDA signer
    invoke_signed(
        &ix,
        &[
            ctx.accounts.config.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.wrapped_sol.to_account_info(),
            ctx.accounts.token_account.to_account_info(),
            ctx.accounts.sol_token_account.to_account_info(),
            ctx.accounts.lp_mint.to_account_info(),
            ctx.accounts.fee_account.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[
            b"config",
            &[ctx.accounts.config.bump]
        ]]
    )?;
    
    msg!("Raydium pool created with permanent liquidity locking");
    Ok(())
}

/// Creates an NFT fee key for fee distribution
fn create_fee_key_nft(
    ctx: &mut Context<MigrateToRaydium>
) -> Result<()> {
    let fee_key = &mut ctx.accounts.fee_key_nft;
    
    // Initialize fee key data
    fee_key.mint = ctx.accounts.nft_mint.key();
    fee_key.owner = ctx.accounts.admin.key();
    fee_key.pool = ctx.accounts.raydium_pool.key();
    fee_key.fee_percentage = 10000; // 100% of fees
    fee_key.last_claimed = Clock::get()?.unix_timestamp;
    fee_key.bump = *ctx.bumps.get("fee_key_nft").unwrap();
    
    // Create instruction data for NFT fee key creation
    let instruction_data = create_nft_fee_key_instruction_data(10000);
    
    // Create the CPI instruction
    let fee_key_program_id = ctx.accounts.fee_key_program.key();
    let accounts = vec![
        AccountMeta::new(ctx.accounts.nft_mint.key(), false),
        AccountMeta::new(ctx.accounts.admin.key(), true),
        AccountMeta::new(ctx.accounts.raydium_pool.key(), false),
        AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
    ];
    
    let ix = Instruction {
        program_id: fee_key_program_id,
        accounts,
        data: instruction_data,
    };
    
    // Execute the CPI
    invoke(
        &ix,
        &[
            ctx.accounts.nft_mint.to_account_info(),
            ctx.accounts.admin.to_account_info(),
            ctx.accounts.raydium_pool.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
        ]
    )?;
    
    msg!("NFT fee key created for fee distribution");
    Ok(())
}

/// Automatic migration check that can be called on every token purchase
pub fn check_auto_migration(ctx: &mut Context<CheckAutoMigration>) -> Result<()> {
    let curve = &ctx.accounts.bonding_curve;
    
    // Check if we've reached the SOL threshold
    let total_sol = curve.total_sol_raised;
    
    // If we're within the migration threshold, emit an event to notify frontends
    if total_sol >= MIGRATION_SOL_MIN && total_sol <= MIGRATION_SOL_MAX {
        emit!(MigrationReadyEvent {
            total_sol,
            total_supply: curve.total_sold_supply,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Migration threshold reached: {} SOL. Ready for migration to Raydium.", total_sol);
    }
    
    Ok(())
}

/// Accounts required for automatic migration checking
#[derive(Accounts)]
pub struct CheckAutoMigration<'info> {
    /// Bonding curve account (PDA)
    #[account(
        seeds = [seeds::BONDING_CURVE],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, BondingCurve>,
}

/// Event emitted when migration threshold is reached
#[event]
pub struct MigrationReadyEvent {
    pub total_sol: u64,
    pub total_supply: u64,
    pub timestamp: i64,
}
