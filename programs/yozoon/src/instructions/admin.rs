use anchor_lang::prelude::*;
use crate::errors::ErrorCode;
use crate::events::*;
use crate::instructions::contexts::*;

/// Initialize the token mint and configuration account
pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Set up the config account
    config.admin = ctx.accounts.admin.key();
    config.mint = ctx.accounts.mint.key();
    config.bump = *ctx.bumps.get("config").unwrap();
    config.paused = false;
    config.treasury = ctx.accounts.treasury.key();
    config.pending_admin = None;
    
    msg!("Yozoon token initialized with admin: {}", config.admin);
    Ok(())
}

/// Transfer admin role to a new account (two-step process)
pub fn transfer_admin(ctx: Context<AdminAction>, new_admin: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Verify not setting to the same admin
    require!(config.admin != new_admin, ErrorCode::InvalidParameter);
    
    // Set pending admin
    config.pending_admin = Some(new_admin);
    
    // Emit event for frontend tracking
    emit!(AdminTransferInitiatedEvent {
        current_admin: ctx.accounts.admin.key(),
        proposed_admin: new_admin,
    });
    
    msg!("Admin transfer initiated to: {}", new_admin);
    Ok(())
}

/// Accept admin role (must be called by pending admin)
pub fn accept_admin(ctx: Context<AcceptAdmin>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Verify pending admin
    require!(
        config.pending_admin == Some(ctx.accounts.new_admin.key()),
        ErrorCode::Unauthorized
    );
    
    // Update admin
    config.admin = ctx.accounts.new_admin.key();
    config.pending_admin = None;
    
    // Emit event for frontend tracking
    emit!(AdminTransferCompletedEvent {
        new_admin: config.admin,
    });
    
    msg!("Admin transfer completed to: {}", config.admin);
    Ok(())
}

/// Emergency pause/unpause of the protocol
pub fn set_pause_state(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Don't update if already in the requested state
    if config.paused == paused {
        return Ok(());
    }
    
    // Update pause state
    config.paused = paused;
    
    // Emit event for frontend tracking
    emit!(PauseStateChangedEvent {
        paused,
        admin: ctx.accounts.admin.key(),
    });
    
    msg!("Protocol {} by admin", if paused { "paused" } else { "unpaused" });
    Ok(())
}
