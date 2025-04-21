// Constants for the Yozoon token ecosystem
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

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

/// Minimum SOL value for migration ($60k in SOL)
pub const MIGRATION_SOL_MIN: u64 = 60_000 * LAMPORTS_PER_SOL;

/// Maximum SOL value for migration ($63k in SOL)
pub const MIGRATION_SOL_MAX: u64 = 63_000 * LAMPORTS_PER_SOL;

/// Supply threshold for migration (1B tokens)
pub const MIGRATION_SUPPLY_THRESHOLD: u64 = 1_000_000_000;

/// Minimum SOL purchase amount (0.001 SOL in lamports)
pub const MINIMUM_SOL_PURCHASE: u64 = 1_000_000;

/// Maximum staleness period for oracle data (5 minutes in seconds)
pub const MAX_PRICE_STALENESS: i64 = 300;

/// Raydium swap program ID (for migration)
pub const RAYDIUM_SWAP_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

/// Raydium AMM program ID
pub const RAYDIUM_AMM_PROGRAM_ID: &str = "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8";

/// Raydium liquidity pool fee (basis points, 0.25%)
pub const RAYDIUM_POOL_FEE: u64 = 25;

/// Raydium LP NFT Fee key program
pub const RAYDIUM_FEE_KEY_PROGRAM_ID: &str = "FeeKedCBd6AvpXjWUFLa8rZwJTASXTJXwSK89JS6QFmQ";

/// SOL token Mint (Wrapped SOL)
pub const WRAPPED_SOL_MINT: &str = "So11111111111111111111111111111111111111112";

/// Liquidity lock period in seconds (permanent lock, represented by u64::MAX)
pub const LIQUIDITY_LOCK_PERIOD: u64 = u64::MAX;

/// Common PDA seeds
pub mod seeds {
    pub const CONFIG: &[u8] = b"config";
    pub const BONDING_CURVE: &[u8] = b"bonding_curve";
    pub const REFERRAL_PREFIX: &[u8] = b"referral";
    pub const AIRDROP_LEDGER: &[u8] = b"airdrop_ledger";
    pub const RAYDIUM_POOL: &[u8] = b"raydium_pool";
    pub const NFT_FEE_KEY: &[u8] = b"nft_fee_key";
}

pub const MINIMUM_TOKEN_SALE: u64 = 1000; // Minimum amount of tokens that can be sold
