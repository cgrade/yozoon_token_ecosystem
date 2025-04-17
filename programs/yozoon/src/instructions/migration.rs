use anchor_lang::prelude::*;
use pyth_sdk_solana::load_price_feed_from_account_info;
use crate::utils::constants::{MAX_PRICE_STALENESS, MIGRATION_USD_MIN, MIGRATION_USD_MAX, MIGRATION_SUPPLY_THRESHOLD};
use crate::utils::price::is_price_fresh;
use crate::state::*;
use crate::errors::*;

/// Migrates liquidity to Raydium when conditions are met
pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()> {
    let config = &ctx.accounts.config;
    let curve = &ctx.accounts.bonding_curve;

    // Verify protocol is not paused
    require!(!config.paused, YozoonError::ProtocolPaused);

    // Get current SOL price from Pyth
    let price_feed = load_price_feed_from_account_info(&ctx.accounts.pyth_price_account)?;
    let current_price = price_feed.get_price_unchecked();
    let current_time = Clock::get()?.unix_timestamp;

    // Verify price is fresh
    require!(
        is_price_fresh(current_price.publish_time, current_time, MAX_PRICE_STALENESS),
        YozoonError::StalePrice
    );

    // Calculate total value in USD
    let total_sol = curve.total_sol_raised;
    let total_usd = (total_sol as f64 * current_price.price as f64) / 1e9;

    // Check if migration conditions are met
    let usd_threshold_met = total_usd >= MIGRATION_USD_MIN && total_usd <= MIGRATION_USD_MAX;
    let supply_threshold_met = curve.total_sold_supply >= MIGRATION_SUPPLY_THRESHOLD;

    require!(
        usd_threshold_met && supply_threshold_met,
        YozoonError::MigrationConditionsNotMet
    );

    // Emit migration event
    emit!(MigrationEvent {
        total_sol,
        total_usd: total_usd as u64,
        total_supply: curve.total_sold_supply,
        timestamp: current_time,
    });

    Ok(())
}
