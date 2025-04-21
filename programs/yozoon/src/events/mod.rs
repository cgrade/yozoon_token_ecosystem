use anchor_lang::prelude::*;

/// Event emitted when tokens are purchased
#[event]
pub struct TokenPurchaseEvent {
    /// User who purchased tokens
    pub user: Pubkey,
    /// Amount of SOL provided (in lamports)
    pub sol_amount: u64,
    /// Net SOL amount after fees (in lamports)
    pub net_sol: u64,
    /// Number of tokens received
    pub tokens: u64,
    /// Price per token
    pub price: u64,
    /// Unix timestamp of the transaction
    pub timestamp: i64,
}

/// Event emitted when token price is calculated
#[event]
pub struct PriceCalculatedEvent {
    /// Current token supply
    pub supply: u64,
    /// Calculated price
    pub price: u64,
    /// Unix timestamp of calculation
    pub timestamp: i64,
}

/// Event emitted when calculating tokens for a given SOL amount
#[event]
pub struct TokenCalculationEvent {
    /// Input SOL amount
    pub sol_amount: u64,
    /// Net SOL after referral fees
    pub net_sol: u64,
    /// Calculated token amount
    pub tokens: u64,
    /// Unix timestamp of calculation
    pub timestamp: i64,
}

/// Event emitted when a referral is created
#[event]
pub struct ReferralCreatedEvent {
    /// User who was referred
    pub user: Pubkey,
    /// Referrer's public key
    pub referrer: Pubkey,
    /// Fee percentage in basis points
    pub fee_percentage: u64,
}

/// Event emitted when tokens are airdropped
#[event]
pub struct AirdropEvent {
    /// Recipient of airdropped tokens
    pub recipient: Pubkey,
    /// Amount of tokens airdropped
    pub amount: u64,
    /// Unix timestamp of airdrop
    pub timestamp: i64,
}

/// Event emitted when migration to Raydium is completed
#[event]
pub struct MigrationCompletedEvent {
    /// Total SOL value in treasury
    pub sol_value: u64,
    /// Total tokens sold
    pub tokens_sold: u64,
    /// Admin who performed migration
    pub admin: Pubkey,
    /// Unix timestamp of migration
    pub timestamp: i64,
}

/// Event emitted when admin transfer is initiated
#[event]
pub struct AdminTransferInitiatedEvent {
    /// Current admin address
    pub current_admin: Pubkey,
    /// Proposed new admin address
    pub proposed_admin: Pubkey,
}

/// Event emitted when admin transfer is completed
#[event]
pub struct AdminTransferCompletedEvent {
    /// New admin address
    pub new_admin: Pubkey,
}

/// Event emitted when pause state is changed
#[event]
pub struct PauseStateChangedEvent {
    /// New pause state (true = paused)
    pub paused: bool,
    /// Admin who changed pause state
    pub admin: Pubkey,
}

#[event]
pub struct MintInitializedEvent {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AdminTransferredEvent {
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct AdminAcceptedEvent {
    pub admin: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct BondingCurveInitializedEvent {
    pub admin: Pubkey,
    pub price_points: Vec<u64>,
    pub timestamp: i64,
}

#[event]
pub struct TokensPurchasedEvent {
    pub buyer: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub price: u64,
    pub timestamp: i64,
}

#[event]
pub struct ReferralSetEvent {
    pub user: Pubkey,
    pub referrer: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ReferralFeeUpdatedEvent {
    pub user: Pubkey,
    pub old_fee: u64,
    pub new_fee: u64,
    pub timestamp: i64,
}

#[event]
pub struct MigrationEvent {
    pub total_sol: u64,
    pub total_usd: u64,
    pub total_supply: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokenSaleEvent {
    pub user: Pubkey,
    pub token_amount: u64,
    pub sol_amount: u64,
    pub price: u64,
    pub timestamp: i64,
}
