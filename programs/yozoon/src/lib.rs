use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("3J6Hu2iwgwuU4gvQACsHqEsfrRog6EzoaEWo1oZ8NHNx");

pub mod errors;
pub mod events;
pub mod state;
pub mod utils;

#[program]
pub mod yozoon {
    use super::*;

    // Admin instruction
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        // Initialize configuration
        let config = &mut ctx.accounts.config;
        config.admin = ctx.accounts.admin.key();
        config.pending_admin = None;
        config.bump = *ctx.bumps.get("config").unwrap();
        config.paused = false;
        config.total_supply = 0;
        config.total_value = 0;
        config.mint = ctx.accounts.mint.key();
        config.treasury = ctx.accounts.admin.key(); // Default treasury to admin
        
        msg!("Initialized with admin: {}", config.admin);
        
        Ok(())
    }
    
    // Transfer admin function
    pub fn transfer_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
        // Set pending admin
        let config = &mut ctx.accounts.config;
        config.pending_admin = Some(new_admin);
        
        msg!("Admin transfer initiated to: {}", new_admin);
        
        Ok(())
    }
    
    // Accept admin function
    pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
        // Transfer admin role to pending admin
        let config = &mut ctx.accounts.config;
        
        // Update admin
        config.admin = ctx.accounts.pending_admin.key();
        config.pending_admin = None;
        
        msg!("Admin role transferred to: {}", config.admin);
        
        Ok(())
    }
    
    // Update treasury function
    pub fn update_treasury(ctx: Context<AdminAction>, new_treasury: Pubkey) -> Result<()> {
        // Update treasury address
        let config = &mut ctx.accounts.config;
        config.treasury = new_treasury;
        
        msg!("Treasury updated to: {}", new_treasury);
        
        Ok(())
    }
    
    // Pause/unpause protocol
    pub fn set_paused(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
        // Set pause state
        let config = &mut ctx.accounts.config;
        config.paused = paused;
        
        msg!("Protocol paused state set to: {}", paused);
        
        Ok(())
    }
    
    // Bonding curve instructions
    
    // Initialize bonding curve with price points
    pub fn initialize_bonding_curve(
        ctx: Context<InitializeBondingCurve>,
        price_points: Vec<u64>
    ) -> Result<()> {
        // Validate number of price points
        require!(
            price_points.len() <= utils::constants::MAX_PRICE_POINTS,
            errors::YozoonError::TooManyPricePoints
        );
        
        // Validate we have at least 2 price points for interpolation
        require!(
            price_points.len() >= 2,
            errors::YozoonError::InvalidParameter
        );
        
        // Validate price points are in ascending order (non-decreasing)
        for i in 1..price_points.len() {
            require!(
                price_points[i] >= price_points[i - 1],
                errors::YozoonError::InvalidParameter
            );
        }
        
        let curve = &mut ctx.accounts.bonding_curve;
        
        // Initialize bonding curve state
        curve.total_sold_supply = 0;
        curve.total_sol_raised = 0;
        curve.price_points = price_points;
        curve.bump = *ctx.bumps.get("bonding_curve").unwrap();
        curve.is_migrated = false;
        
        msg!("Bonding curve initialized with {} price points", curve.price_points.len());
        Ok(())
    }
    
    // Buy tokens with SOL
    pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
        // Check if protocol is paused
        require!(!ctx.accounts.config.paused, errors::YozoonError::ProtocolPaused);
        
        let curve = &mut ctx.accounts.bonding_curve;
        
        // Prevent purchases after migration
        require!(!curve.is_migrated, errors::YozoonError::Migrated);
        
        // Validate minimum purchase amount
        require!(sol_amount >= utils::constants::MINIMUM_SOL_PURCHASE, errors::YozoonError::AmountTooSmall);
        
        // Calculate current token price
        let supply = curve.total_sold_supply;
        let price = utils::helpers::calculate_token_price(supply, &curve.price_points);
        
        // Calculate tokens to mint
        let tokens = sol_amount * utils::constants::PRECISION_FACTOR / price;
        
        // Ensure non-zero tokens (prevent dust amounts)
        require!(tokens > 0, errors::YozoonError::DustAmount);
        
        // Check supply limit
        require!(
            supply + tokens <= utils::constants::TOTAL_SUPPLY, 
            errors::YozoonError::SupplyExceeded
        );
        
        // Calculate fees if referral exists
        let (net_sol, referral_fee) = if let Some(referral) = &ctx.accounts.referral {
            let fee = sol_amount * referral.fee_percentage / 10_000;
            (sol_amount - fee, fee)
        } else {
            (sol_amount, 0)
        };
        
        // Transfer SOL to treasury
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.buyer.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                },
            ),
            net_sol,
        )?;
        
        // Transfer referral fee if applicable
        if referral_fee > 0 {
            if let Some(referrer) = &ctx.accounts.referrer {
                anchor_lang::system_program::transfer(
                    CpiContext::new(
                        ctx.accounts.system_program.to_account_info(),
                        anchor_lang::system_program::Transfer {
                            from: ctx.accounts.buyer.to_account_info(),
                            to: referrer.to_account_info(),
                        },
                    ),
                    referral_fee,
                )?;
            }
        }
        
        // Mint tokens to the user's token account
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                &[&[
                    b"config",
                    &[ctx.accounts.config.bump]
                ]],
            ),
            tokens,
        )?;
        
        // Update bonding curve state
        curve.total_sold_supply += tokens;
        curve.total_sol_raised += net_sol;
        
        // Emit event for frontend tracking
        emit!(events::TokenPurchaseEvent {
            user: ctx.accounts.buyer.key(),
            sol_amount,
            net_sol,
            tokens,
            price,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("User purchased {} tokens for {} SOL", tokens, sol_amount);
        Ok(())
    }
    
    // Calculate current price
    pub fn calculate_current_price(ctx: Context<GetCurrentPrice>) -> Result<u64> {
        let curve = &ctx.accounts.bonding_curve;
        let supply = curve.total_sold_supply;
        
        // Use extracted helper function
        let price = utils::helpers::calculate_token_price(supply, &curve.price_points);
        
        // Emit event for frontend tracking
        emit!(events::PriceCalculatedEvent {
            supply,
            price,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Current price: {} at supply: {}", price, supply);
        Ok(price)
    }
    
    // Calculate tokens for sol
    pub fn calculate_tokens_for_sol(
        ctx: Context<CalculateTokens>,
        sol_amount: u64
    ) -> Result<u64> {
        let curve = &ctx.accounts.bonding_curve;
        
        // Prevent calculations after migration
        require!(!curve.is_migrated, errors::YozoonError::Migrated);
        
        let supply = curve.total_sold_supply;
        let price = utils::helpers::calculate_token_price(supply, &curve.price_points);
        
        // Calculate tokens without referral fee since this is just a calculation
        let fee_percentage = 0; // No referral fee for calculations
        let (tokens, net_sol) = utils::helpers::calculate_tokens_for_sol_amount(
            sol_amount,
            fee_percentage,
            price,
            utils::constants::PRECISION_FACTOR
        );
        
        // Ensure non-zero tokens (prevent dust amounts)
        require!(tokens > 0, errors::YozoonError::DustAmount);
        
        // Check supply limit
        require!(supply + tokens <= utils::constants::TOTAL_SUPPLY, errors::YozoonError::SupplyExceeded);
        
        // Emit event for frontend tracking
        emit!(events::TokenCalculationEvent {
            sol_amount,
            net_sol,
            tokens,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Calculated {} tokens for {} SOL", tokens, sol_amount);
        Ok(tokens)
    }
    
    // Sell tokens
    pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
        let curve = &mut ctx.accounts.bonding_curve;
        
        // Prevent sales after migration
        require!(!curve.is_migrated, errors::YozoonError::Migrated);
        
        // Validate minimum sale amount
        require!(token_amount >= utils::constants::MINIMUM_TOKEN_SALE, errors::YozoonError::AmountTooSmall);
        
        // Calculate current token price
        let supply = curve.total_sold_supply;
        let price = utils::helpers::calculate_token_price(supply, &curve.price_points);
        
        // Calculate SOL to return
        let sol_amount = token_amount * price / utils::constants::PRECISION_FACTOR;
        
        // Ensure non-zero SOL (prevent dust amounts)
        require!(sol_amount > 0, errors::YozoonError::DustAmount);
        
        // Check reserve balance
        require!(ctx.accounts.reserve.lamports() >= sol_amount, errors::YozoonError::InsufficientReserve);
        
        // Burn tokens from the user's token account
        anchor_spl::token::burn(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.seller_token_account.to_account_info(),
                    authority: ctx.accounts.seller.to_account_info(),
                },
            ),
            token_amount,
        )?;
        
        // Transfer SOL from reserve to user
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.reserve.to_account_info(),
                    to: ctx.accounts.seller.to_account_info(),
                },
            ),
            sol_amount,
        )?;
        
        // Update bonding curve state
        curve.total_sold_supply -= token_amount;
        
        // Emit event for frontend tracking
        emit!(events::TokenSaleEvent {
            user: ctx.accounts.seller.key(),
            token_amount,
            sol_amount,
            price,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("User sold {} tokens for {} SOL", token_amount, sol_amount);
        Ok(())
    }
    
    // Set referral for a user
    pub fn set_referral(
        ctx: Context<SetReferral>,
        referrer: Pubkey,
        fee_percentage: u64
    ) -> Result<()> {
        // Check if user is trying to refer themselves
        require!(
            ctx.accounts.user.key() != referrer,
            errors::YozoonError::SelfReferral
        );
        
        // Check fee percentage is within allowed range
        require!(
            fee_percentage <= 2000, // Max 20%
            errors::YozoonError::FeeTooHigh
        );
        
        // Initialize referral
        let referral = &mut ctx.accounts.referral;
        referral.referrer = referrer;
        referral.fee_percentage = fee_percentage;
        referral.bump = *ctx.bumps.get("referral").unwrap();
        
        msg!("Referral set for user {} with referrer {} and fee {}", 
            ctx.accounts.user.key(), 
            referrer,
            fee_percentage
        );
        
        emit!(events::ReferralCreatedEvent {
            user: ctx.accounts.user.key(),
            referrer,
            fee_percentage,
        });
        
        Ok(())
    }
    
    // Update referral fee (admin only)
    pub fn update_referral_fee(
        ctx: Context<UpdateReferralFee>,
        fee_percentage: u64
    ) -> Result<()> {
        // Check fee percentage is within allowed range
        require!(
            fee_percentage <= 2000, // Max 20%
            errors::YozoonError::FeeTooHigh
        );
        
        // Update fee
        let referral = &mut ctx.accounts.referral;
        let old_fee = referral.fee_percentage;
        referral.fee_percentage = fee_percentage;
        
        msg!("Referral fee updated to: {}", fee_percentage);
        
        emit!(events::ReferralFeeUpdatedEvent {
            user: ctx.accounts.config.admin,
            old_fee,
            new_fee: fee_percentage,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    // Airdrop tokens
    pub fn airdrop_tokens(
        ctx: Context<AirdropTokens>,
        amount: u64
    ) -> Result<()> {
        // Check if protocol is paused
        require!(!ctx.accounts.config.paused, errors::YozoonError::ProtocolPaused);
        
        // Mint tokens to recipient
        anchor_spl::token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                &[&[
                    b"config",
                    &[ctx.accounts.config.bump]
                ]],
            ),
            amount,
        )?;
        
        // Update airdrop ledger
        let ledger = &mut ctx.accounts.airdrop_ledger;
        ledger.total_airdropped += amount;
        ledger.bump = *ctx.bumps.get("airdrop_ledger").unwrap_or(&0);
        
        // Update total supply in config
        let config = &mut ctx.accounts.config;
        config.total_supply += amount;
        
        // Emit event for tracking
        emit!(events::AirdropEvent {
            recipient: ctx.accounts.recipient_token_account.owner,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Airdropped {} tokens to {}", 
            amount, 
            ctx.accounts.recipient_token_account.owner
        );
        
        Ok(())
    }
    
    // Migrate to Raydium
    pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()> {
        // Check if admin
        require!(
            ctx.accounts.config.admin == ctx.accounts.admin.key(),
            errors::YozoonError::Unauthorized
        );
        
        // Mark as migrated
        let curve = &mut ctx.accounts.bonding_curve;
        curve.is_migrated = true;
        
        // Calculate total values
        let total_sol = curve.total_sol_raised;
        let total_supply = curve.total_sold_supply;
        
        // Approximate USD value (for example purposes)
        let sol_price_usd = 100; // Example: $100 per SOL
        let total_usd = total_sol * sol_price_usd / anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
        
        // Emit migration event
        emit!(events::MigrationEvent {
            total_sol,
            total_usd,
            total_supply,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        msg!("Migration to Raydium completed. Total SOL: {}, Total USD: {}", total_sol, total_usd);
        Ok(())
    }
}

// Account contexts for all instructions

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    /// Configuration account (PDA)
    #[account(
        init,
        payer = admin,
        space = 8 + state::Config::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, state::Config>,
    
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

#[derive(Accounts)]
pub struct AdminAction<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        has_one = admin @ errors::YozoonError::Unauthorized
    )]
    pub config: Account<'info, state::Config>,
    
    /// Admin account (pays rent and becomes initial admin)
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAdmin<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        constraint = config.pending_admin == Some(pending_admin.key()) @ errors::YozoonError::Unauthorized
    )]
    pub config: Account<'info, state::Config>,
    
    /// Pending admin account (signs transaction to accept role)
    pub pending_admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeBondingCurve<'info> {
    /// Bonding curve account (PDA)
    #[account(
        init,
        payer = admin,
        space = 8 + state::BondingCurve::LEN,
        seeds = [b"bonding_curve"],
        bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    /// Admin account (pays rent)
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, state::Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
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
    
    /// Treasury account
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    
    /// Referral account (optional)
    #[account(mut)]
    pub referral: Option<Account<'info, state::Referral>>,
    
    /// Referrer account (optional)
    #[account(mut)]
    pub referrer: Option<SystemAccount<'info>>,
    
    /// System program
    pub system_program: Program<'info, System>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct SetReferral<'info> {
    /// User's referral account (PDA)
    #[account(
        init,
        payer = user,
        space = 8 + state::Referral::LEN,
        seeds = [b"referral", user.key().as_ref()],
        bump
    )]
    pub referral: Account<'info, state::Referral>,
    
    /// User account (signs transaction and pays rent)
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AirdropTokens<'info> {
    /// Configuration account (PDA)
    #[account(
        mut,
        has_one = admin @ errors::YozoonError::Unauthorized
    )]
    pub config: Account<'info, state::Config>,
    
    /// Admin account
    #[account(mut)]
    pub admin: Signer<'info>,
    
    /// Airdrop ledger account (PDA)
    #[account(
        init_if_needed,
        payer = admin,
        space = 8 + state::AirdropLedger::LEN,
        seeds = [b"airdrop_ledger"],
        bump
    )]
    pub airdrop_ledger: Account<'info, state::AirdropLedger>,
    
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

#[derive(Accounts)]
pub struct GetCurrentPrice<'info> {
    /// Bonding curve account (PDA)
    #[account(
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
}

#[derive(Accounts)]
pub struct CalculateTokens<'info> {
    /// Bonding curve account (PDA)
    #[account(
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
}

#[derive(Accounts)]
pub struct SellTokens<'info> {
    /// User account (signs transaction and receives SOL)
    #[account(mut)]
    pub seller: Signer<'info>,
    
    /// User's token account to burn tokens from
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key()
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    /// Token mint account
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    /// Reserve account that holds SOL for token buybacks
    #[account(mut)]
    pub reserve: SystemAccount<'info>,
    
    /// Token program
    pub token_program: Program<'info, Token>,
    
    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateReferralFee<'info> {
    /// Configuration account (PDA) to validate admin
    #[account(
        has_one = admin @ errors::YozoonError::Unauthorized
    )]
    pub config: Account<'info, state::Config>,
    
    /// Referral account to update
    #[account(mut)]
    pub referral: Account<'info, state::Referral>,
    
    /// Admin account
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct MigrateToRaydium<'info> {
    /// Configuration account (PDA)
    #[account(mut)]
    pub config: Account<'info, state::Config>,
    
    /// Bonding curve account (PDA)
    #[account(
        mut,
        seeds = [b"bonding_curve"],
        bump = bonding_curve.bump
    )]
    pub bonding_curve: Account<'info, state::BondingCurve>,
    
    /// Admin account
    pub admin: Signer<'info>,
}
