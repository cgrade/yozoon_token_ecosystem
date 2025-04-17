// Constants for the Yozoon token ecosystem

/// Total token supply (1e18 tokens)
pub const TOTAL_SUPPLY: u64 = 1_000_000_000_000_000_000;

/// Precision factor used for calculations (1e9)
pub const PRECISION_FACTOR: u64 = 1_000_000_000;

/// Default referral fee (1% in basis points: 100/10_000)
pub const DEFAULT_REFERRAL_FEE: u64 = 100;

/// Maximum referral fee (5% in basis points: 500/10_000)
pub const MAX_REFERRAL_FEE: u64 = 500;

/// Maximum number of price points allowed for bonding curve
pub const MAX_PRICE_POINTS: usize = 100;

/// Minimum USD value for migration ($100k with decimals)
pub const MIGRATION_USD_MIN: f64 = 100_000.0;

/// Maximum USD value for migration ($1M with decimals)
pub const MIGRATION_USD_MAX: f64 = 1_000_000.0;

/// Supply threshold for migration (1B tokens)
pub const MIGRATION_SUPPLY_THRESHOLD: u64 = 1_000_000_000;

/// Minimum SOL purchase amount (0.001 SOL in lamports)
pub const MINIMUM_SOL_PURCHASE: u64 = 1_000_000;

/// Maximum staleness period for oracle data (5 minutes in seconds)
pub const MAX_PRICE_STALENESS: i64 = 300;

/// Raydium swap program ID (for migration)
pub const RAYDIUM_SWAP_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

/// Common PDA seeds
pub mod seeds {
    pub const CONFIG: &[u8] = b"config";
    pub const BONDING_CURVE: &[u8] = b"bonding_curve";
    pub const REFERRAL_PREFIX: &[u8] = b"referral";
    pub const AIRDROP_LEDGER: &[u8] = b"airdrop_ledger";
}
