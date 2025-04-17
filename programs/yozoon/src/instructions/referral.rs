use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::instructions::contexts::*;
use crate::utils::constants::*;

/// Sets a referrer for a user
pub fn set_referral(
    ctx: Context<SetReferral>,
    referrer: Pubkey
) -> Result<()> {
    // Prevent self-referral
    require!(
        ctx.accounts.user.key() != referrer,
        ErrorCode::SelfReferral
    );
    
    // Verify referrer matches account provided
    require!(
        ctx.accounts.referrer.key() == referrer,
        ErrorCode::InvalidParameter
    );
    
    let referral = &mut ctx.accounts.referral;
    
    // Store referrer and fee information
    referral.referrer = referrer;
    referral.fee_percentage = DEFAULT_REFERRAL_FEE;
    referral.bump = *ctx.bumps.get("referral").unwrap();
    
    // Emit event for frontend tracking
    emit!(ReferralCreatedEvent {
        user: ctx.accounts.user.key(),
        referrer,
        fee_percentage: DEFAULT_REFERRAL_FEE,
    });
    
    msg!("Referral set for {} to referrer {}", ctx.accounts.user.key(), referrer);
    Ok(())
}

/// Updates the referral fee percentage for a user (admin only)
pub fn update_referral_fee(
    ctx: Context<UpdateReferralFee>,
    new_fee_percentage: u64
) -> Result<()> {
    // Validate fee percentage is within limits
    require!(
        new_fee_percentage <= MAX_REFERRAL_FEE,
        ErrorCode::FeeTooHigh
    );
    
    let referral = &mut ctx.accounts.referral;
    
    // Update fee percentage
    referral.fee_percentage = new_fee_percentage;
    
    // Emit event for frontend tracking
    emit!(ReferralFeeUpdatedEvent {
        user: ctx.accounts.user.key(),
        fee_percentage: new_fee_percentage,
    });
    
    msg!("Referral fee updated for {} to {}bps", ctx.accounts.user.key(), new_fee_percentage);
    Ok(())
}
