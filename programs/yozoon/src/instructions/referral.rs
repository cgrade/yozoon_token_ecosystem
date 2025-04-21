use anchor_lang::prelude::*;
use crate::errors::YozoonError;
use crate::events::*;
use crate::instructions::contexts::*;
use crate::utils::constants::*;

/// Sets a referrer for a user
pub fn set_referral(
    ctx: Context<SetReferral>,
    referrer: Pubkey
) -> Result<()> {
    // Check if user is trying to refer themselves
    require!(
        ctx.accounts.user.key() != referrer,
        YozoonError::SelfReferral
    );
    
    let referral = &mut ctx.accounts.referral;
    
    // Update referral
    referral.referrer = referrer;
    referral.fee_percentage = DEFAULT_REFERRAL_FEE;
    referral.bump = *ctx.bumps.get("referral").unwrap();
    
    // Emit event
    emit!(ReferralSetEvent {
        user: ctx.accounts.user.key(),
        referrer,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Referral set for {} to {}", ctx.accounts.user.key(), referrer);
    Ok(())
}

/// Updates the referral fee percentage for a user (admin only)
pub fn update_referral_fee(
    ctx: Context<UpdateReferralFee>,
    new_fee_percentage: u64
) -> Result<()> {
    let referral = &mut ctx.accounts.referral;
    let old_fee = referral.fee_percentage;
    
    // Check if new fee is too high
    require!(
        new_fee_percentage <= MAX_REFERRAL_FEE,
        YozoonError::FeeTooHigh
    );
    
    // Update fee
    referral.fee_percentage = new_fee_percentage;
    
    // Emit event
    emit!(ReferralFeeUpdatedEvent {
        user: ctx.accounts.referral.key(),
        old_fee,
        new_fee: new_fee_percentage,
        timestamp: Clock::get()?.unix_timestamp,
    });
    
    msg!("Referral fee updated for {} to {}bps", ctx.accounts.referral.key(), new_fee_percentage);
    
    Ok(())
}
