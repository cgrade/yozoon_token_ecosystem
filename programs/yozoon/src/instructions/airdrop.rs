use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::instructions::contexts::*;
use crate::utils::constants::*;
use crate::utils::constants::seeds::*;

/// Airdrops tokens to a recipient without affecting the bonding curve
pub fn airdrop_tokens(
    ctx: Context<AirdropTokens>,
    amount: u64
) -> Result<()> {
    // Check if protocol is paused
    require!(!ctx.accounts.config.paused, ErrorCode::Paused);
    
    // Verify amount is non-zero
    require!(amount > 0, ErrorCode::InvalidParameter);
    
    let ledger = &mut ctx.accounts.airdrop_ledger;
    
    // Ensure total supply (sold + airdropped) isn't exceeded
    require!(
        ledger.total_airdropped + amount + ctx.accounts.bonding_curve.total_sold_supply <= TOTAL_SUPPLY,
        ErrorCode::SupplyExceeded
    );
    
    // Mint tokens to the recipient
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&[CONFIG, &[ctx.accounts.config.bump]]],
        ),
        amount,
    )?;
    
    // Update airdrop ledger
    ledger.total_airdropped += amount;
    
    // Emit event for frontend tracking
    emit!(AirdropEvent {
        recipient: ctx.accounts.recipient.key(),
        amount,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Airdropped {} tokens to {}", amount, ctx.accounts.recipient.key());
    Ok(())
}
