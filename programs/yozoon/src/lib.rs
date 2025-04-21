use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("3J6Hu2iwgwuU4gvQACsHqEsfrRog6EzoaEWo1oZ8NHNx");

pub mod errors;
pub mod events;
pub mod state;
pub mod utils;
pub mod raydium;
pub mod instructions;

#[program]
pub mod yozoon {
    use super::*;
    
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Initializing mint");
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        config.mint = ctx.accounts.mint.key();
        config.bump = *ctx.bumps.get("config").unwrap();
        config.paused = false;
        config.total_supply = 0;
        config.total_value = 0;
        config.treasury = ctx.accounts.admin.key();
        config.pending_admin = None;
        
        Ok(())
    }
    
    pub fn initialize_bonding_curve(ctx: Context<InitializeBondingCurve>, price_points: Vec<u64>) -> Result<()> {
        let curve = &mut ctx.accounts.bonding_curve;
        curve.price_points = price_points;
        curve.total_sold_supply = 0;
        curve.total_sol_raised = 0;
        curve.bump = *ctx.bumps.get("bonding_curve").unwrap();
        curve.is_migrated = false;
        
        Ok(())
    }
    
    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        let curve = &mut ctx.accounts.bonding_curve;
        
        // Example validation
        require!(!curve.is_migrated, errors::YozoonError::Migrated);
        
        // Example calculation
        let tokens = sol_amount * 1_000_000_000;
        
        // Mint tokens
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                &[&[b"config", &[ctx.accounts.config.bump]]],
            ),
            tokens,
        )?;
        
        // Update state
        curve.total_sold_supply += tokens;
        curve.total_sol_raised += sol_amount;
        
        Ok(())
    }
    
    pub fn migrate_to_raydium(ctx: Context<MigrateContext>) -> Result<()> {
        let curve = &mut ctx.accounts.bonding_curve;
        curve.is_migrated = true;
        
        // Emit migration event
        emit!(events::MigrationEvent {
            total_sol: curve.total_sol_raised,
            total_usd: 0,
            total_supply: curve.total_sold_supply,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + state::Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, state::Config>,
    
    #[account(
        init,
        payer = admin,
        mint::decimals = 9,
        mint::authority = config,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeBondingCurve<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + state::BondingCurve::LEN,
        seeds = [b"bonding_curve"],
        bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, state::Config>,
    
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct MigrateContext<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, state::Config>,
    
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    #[account(
        constraint = admin.key() == config.admin @ errors::YozoonError::Unauthorized
    )]
    pub admin: Signer<'info>,
}
