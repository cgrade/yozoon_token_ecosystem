use anchor_lang::prelude::*;
use crate::errors::*;
use crate::events::MigrationEvent;
use crate::state::*;

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    #[account(
        mut,
        has_one = admin @ YozoonError::Unauthorized
    )]
    pub config: Account<'info, Config>,
    
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    
    pub admin: Signer<'info>,
}

/// Migrates liquidity to Raydium when conditions are met
pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Calculate total values
    let total_sol = ctx.accounts.treasury.lamports();
    let total_usd = config.total_value;
    let total_supply = config.total_supply;
    
    // Emit migration event
    emit!(MigrationEvent {
        total_sol,
        total_usd,
        total_supply,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    Ok(())
}
