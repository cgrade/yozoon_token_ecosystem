use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::errors::ErrorCode;
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
        ErrorCode::TooManyPricePoints
    );
    
    // Validate we have at least 2 price points for interpolation
    require!(
        price_points.len() >= 2,
        ErrorCode::InvalidParameter
    );
    
    // Validate price points are in ascending order (non-decreasing)
    for i in 1..price_points.len() {
        require!(
            price_points[i] >= price_points[i - 1],
            ErrorCode::InvalidParameter
        );
    }
    
    let curve = &mut ctx.accounts.bonding_curve;
    
    // Initialize bonding curve state
    curve.total_sold_supply = 0;
    curve.total_net_sol = 0;
    curve.price_points = price_points;
    curve.bump = *ctx.bumps.get("bonding_curve").unwrap();
    curve.is_migrated = false;
    
    msg!("Bonding curve initialized with {} price points", curve.price_points.len());
    Ok(())
}

/// Allows users to buy tokens with SOL, applying referral fees if set
pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()> {
    // Check if protocol is paused
    require!(!ctx.accounts.config.paused, ErrorCode::Paused);
    
    let curve = &mut ctx.accounts.bonding_curve;
    
    // Prevent purchases after migration
    require!(!curve.is_migrated, ErrorCode::Migrated);
    
    // Validate minimum purchase amount
    require!(sol_amount >= MINIMUM_SOL_PURCHASE, ErrorCode::AmountTooSmall);
    
    let referral = &ctx.accounts.referral;
    
    // Calculate referral fee
    let fee = sol_amount * referral.fee_percentage / 10_000;
    let net_sol = sol_amount - fee;
    
    // Calculate current token price
    let supply = curve.total_sold_supply;
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Calculate tokens to mint
    let tokens = net_sol * PRECISION_FACTOR / price;
    
    // Ensure non-zero tokens (prevent dust amounts)
    require!(tokens > 0, ErrorCode::DustAmount);
    
    // Check supply limit
    require!(supply + tokens <= TOTAL_SUPPLY, ErrorCode::SupplyExceeded);
    
    // Transfer SOL to reserve account
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.reserve.to_account_info(),
            },
        ),
        net_sol,
    )?;
    
    // Distribute referral fees if applicable
    if fee > 0 {
        // Calculate fee split
        let (referrer_share, yozoon_share) = calculate_fee_split(fee);
        
        // Transfer to referrer if applicable
        if referrer_share > 0 {
            anchor_lang::system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.referrer_account.to_account_info(),
                    },
                ),
                referrer_share,
            )?;
        }
        
        // Transfer to project treasury
        if yozoon_share > 0 {
            anchor_lang::system_program::transfer(
                CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.treasury_account.to_account_info(),
                    },
                ),
                yozoon_share,
            )?;
        }
    }
    
    // Mint tokens to the user's token account
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
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
    curve.total_net_sol += net_sol;
    
    // Emit event for frontend tracking
    emit!(TokenPurchaseEvent {
        user: ctx.accounts.user.key(),
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
    // Validate minimum purchase amount
    require!(sol_amount >= MINIMUM_SOL_PURCHASE, ErrorCode::AmountTooSmall);
    
    let curve = &ctx.accounts.bonding_curve;
    
    // Prevent calculations after migration
    require!(!curve.is_migrated, ErrorCode::Migrated);
    
    let supply = curve.total_sold_supply;
    let price = calculate_token_price(supply, &curve.price_points);
    
    // Calculate tokens with referral fee deduction
    let referral = &ctx.accounts.referral;
    let (tokens, net_sol) = calculate_tokens_for_sol_amount(
        sol_amount,
        referral.fee_percentage,
        price,
        PRECISION_FACTOR
    );
    
    // Ensure non-zero tokens (prevent dust amounts)
    require!(tokens > 0, ErrorCode::DustAmount);
    
    // Check supply limit
    require!(supply + tokens <= TOTAL_SUPPLY, ErrorCode::SupplyExceeded);
    
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
