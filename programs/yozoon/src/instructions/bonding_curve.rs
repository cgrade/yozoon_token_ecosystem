use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::errors::YozoonError;
use crate::events::*;
use crate::instructions::contexts::*;
use crate::utils::constants::*;
use crate::utils::helpers::*;

/// Sets up the bonding curve with predefined price points
pub fn initialize_bonding_curve(
    ctx: Context<InitializeBondingCurve>,
    price_points: Vec<u64>
) -> Result<()> {
    // Validate number of price points
    require!(
        price_points.len() <= MAX_PRICE_POINTS,
        YozoonError::TooManyPricePoints
    );
    
    // Validate we have at least 2 price points for interpolation
    require!(
        price_points.len() >= 2,
        YozoonError::InvalidParameter
    );
    
    // Validate price points are in ascending order (non-decreasing)
    for i in 1..price_points.len() {
        require!(
            price_points[i] >= price_points[i - 1],
            YozoonError::InvalidParameter
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

/// Allows users to buy tokens with SOL, applying referral fees if set
pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
    // Check if protocol is paused
    require!(!ctx.accounts.config.paused, YozoonError::ProtocolPaused);
    
    let curve = &mut ctx.accounts.bonding_curve;
    
    // Prevent purchases after migration
    require!(!curve.is_migrated, YozoonError::Migrated);
    
    // Validate minimum purchase amount
    require!(sol_amount >= MINIMUM_SOL_PURCHASE, YozoonError::AmountTooSmall);
    
    // Calculate current token price
    let supply = curve.total_sold_supply;
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Calculate tokens to mint
    let tokens = sol_amount * PRECISION_FACTOR / price;
    
    // Ensure non-zero tokens (prevent dust amounts)
    require!(tokens > 0, YozoonError::DustAmount);
    
    // Check supply limit
    require!(supply + tokens <= TOTAL_SUPPLY, YozoonError::SupplyExceeded);
    
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
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&[
                crate::utils::constants::seeds::CONFIG,
                &[ctx.accounts.config.bump]
            ]],
        ),
        tokens,
    )?;
    
    // Update bonding curve state
    curve.total_sold_supply += tokens;
    curve.total_sol_raised += net_sol;
    
    // Emit event for frontend tracking
    emit!(TokenPurchaseEvent {
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

/// Calculate current token price at the current supply level
pub fn calculate_current_price(ctx: Context<GetCurrentPrice>) -> Result<u64> {
    let curve = &ctx.accounts.bonding_curve;
    let supply = curve.total_sold_supply;
    
    // Use extracted helper function
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Emit event for frontend tracking
    emit!(PriceCalculatedEvent {
        supply,
        price,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Current price: {} at supply: {}", price, supply);
    Ok(price)
}

/// Calculate tokens to be received for a specific SOL amount
pub fn calculate_tokens_for_sol(
    ctx: Context<CalculateTokens>,
    sol_amount: u64
) -> Result<u64> {
    let curve = &ctx.accounts.bonding_curve;
    
    // Prevent calculations after migration
    require!(!curve.is_migrated, YozoonError::Migrated);
    
    let supply = curve.total_sold_supply;
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Calculate tokens without referral fee since this is just a calculation
    let fee_percentage = 0; // No referral fee for calculations
    let (tokens, net_sol) = calculate_tokens_for_sol_amount(
        sol_amount,
        fee_percentage,
        price,
        PRECISION_FACTOR
    );
    
    // Ensure non-zero tokens (prevent dust amounts)
    require!(tokens > 0, YozoonError::DustAmount);
    
    // Check supply limit
    require!(supply + tokens <= TOTAL_SUPPLY, YozoonError::SupplyExceeded);
    
    // Emit event for frontend tracking
    emit!(TokenCalculationEvent {
        sol_amount,
        net_sol,
        tokens,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Calculated {} tokens for {} SOL", tokens, sol_amount);
    Ok(tokens)
}

/// Allows users to sell tokens for SOL
pub fn sell_tokens(ctx: Context<SellTokens>, token_amount: u64) -> Result<()> {
    let curve = &mut ctx.accounts.bonding_curve;
    
    // Prevent sales after migration
    require!(!curve.is_migrated, YozoonError::Migrated);
    
    // Validate minimum sale amount
    require!(token_amount >= MINIMUM_TOKEN_SALE, YozoonError::AmountTooSmall);
    
    // Calculate current token price
    let supply = curve.total_sold_supply;
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Calculate SOL to return
    let sol_amount = token_amount * price / PRECISION_FACTOR;
    
    // Ensure non-zero SOL (prevent dust amounts)
    require!(sol_amount > 0, YozoonError::DustAmount);
    
    // Check reserve balance
    require!(ctx.accounts.reserve.lamports() >= sol_amount, YozoonError::InsufficientReserve);
    
    // Burn tokens from the user's token account
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
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
    curve.total_sol_raised -= sol_amount;
    
    // Emit event for frontend tracking
    emit!(TokenSaleEvent {
        user: ctx.accounts.seller.key(),
        token_amount,
        sol_amount,
        price,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("User sold {} tokens for {} SOL", token_amount, sol_amount);
    Ok(())
}
