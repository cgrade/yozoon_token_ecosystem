use crate::utils::constants::TOTAL_SUPPLY;

/// Calculate token price at a specific supply level
/// 
/// Uses linear interpolation between price points
///
/// # Arguments
/// 
/// * `supply` - Current token supply
/// * `price_points` - Vector of price points for the bonding curve
/// 
/// # Returns
/// 
/// The calculated token price at the given supply
pub fn calculate_token_price(supply: u64, price_points: &[u64]) -> u64 {
    let n = price_points.len() as u64;
    let w = TOTAL_SUPPLY / n;  // Width of each segment
    let m = supply / w;        // Current segment index
    let r_x = supply % w;      // Position within segment
    
    // Linear interpolation between price points
    let p_m = price_points[m as usize]; // Current price point
    let p_m1 = if m + 1 < n {
        price_points[(m + 1) as usize] // Next price point
    } else {
        p_m // Use current price point if at the end
    };
    
    // Linear interpolation formula: p_m + ((p_m1 - p_m) * r_x) / w
    p_m + ((p_m1 - p_m) * r_x) / w
}

/// Calculate tokens to be received for a specific SOL amount
/// 
/// # Arguments
/// 
/// * `sol_amount` - Amount of SOL (in lamports)
/// * `fee_percentage` - Referral fee percentage (basis points)
/// * `price` - Current token price
/// * `precision_factor` - Precision factor for calculation
/// 
/// # Returns
/// 
/// Number of tokens to be received
pub fn calculate_tokens_for_sol_amount(
    sol_amount: u64,
    fee_percentage: u64,
    price: u64,
    precision_factor: u64
) -> (u64, u64) {
    // Calculate fee and net SOL
    let fee = sol_amount * fee_percentage / 10_000;
    let net_sol = sol_amount - fee;
    
    // Calculate tokens with precision
    let tokens = net_sol * precision_factor / price;
    
    (tokens, net_sol)
}

/// Calculate referral fee split between referrer and project
/// 
/// # Arguments
/// 
/// * `fee` - Total fee amount
/// 
/// # Returns
/// 
/// Tuple of (referrer_share, project_share)
pub fn calculate_fee_split(fee: u64) -> (u64, u64) {
    let referrer_share = fee / 2;  // 50% to referrer
    let project_share = fee - referrer_share; // Remaining to project
    
    (referrer_share, project_share)
}

/// Check if a price feed is stale
/// 
/// # Arguments
/// 
/// * `price_timestamp` - Timestamp of the price feed
/// * `current_time` - Current timestamp
/// * `max_staleness` - Maximum allowed staleness in seconds
/// 
/// # Returns
/// 
/// `true` if price is fresh, `false` if stale
pub fn is_price_fresh(price_timestamp: i64, current_time: i64, max_staleness: i64) -> bool {
    current_time - price_timestamp < max_staleness
}
