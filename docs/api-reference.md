# Yozoon Protocol API Reference

This document provides a comprehensive reference for all instructions and account structures in the Yozoon protocol.

## Program ID

The program ID for the Yozoon protocol on devnet is:

```
Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
```

Replace this with your actual program ID after deployment.

## Instructions

### 1. Initialize Mint

Initializes the token mint and protocol configuration.

**Parameters:**

- `defaultReferralFee`: u16 - Default referral fee in basis points (100 = 1%)

**Accounts:**

- `admin`: Signer - The admin who initializes the protocol
- `config`: PDA - The configuration account that will be created
- `tokenMint`: Signer - The token mint account that will be created
- `treasury`: AccountInfo - The treasury account that will receive protocol fees
- `systemProgram`: Program - System program
- `tokenProgram`: Program - Token program
- `rent`: Sysvar - Rent sysvar

**PDA Derivation:**

```
[Buffer.from("config")]
```

**Events:**

- `InitializeEvent` - Emitted when protocol is initialized

**Example:**

```typescript
await program.methods
  .initializeMint(100) // 1% referral fee
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
    tokenMint: mintKeypair.publicKey,
    treasury: wallet.publicKey,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .signers([mintKeypair])
  .rpc();
```

### 2. Initialize Bonding Curve

Initializes the bonding curve with supply/price points.

**Parameters:**

- `pricePoints`: Array of { supplyPoint: BN, priceLamports: BN } - At least two price points defining the curve

**Accounts:**

- `admin`: Signer - The admin who initializes the bonding curve
- `config`: PDA - The configuration account
- `bondingCurve`: PDA - The bonding curve account that will be created
- `systemProgram`: Program - System program

**PDA Derivation:**

```
[Buffer.from("bonding_curve")]
```

**Events:**

- `InitializeBondingCurveEvent` - Emitted when bonding curve is initialized

**Example:**

```typescript
const pricePoints = [
  { supplyPoint: new BN(0), priceLamports: new BN(10_000_000) }, // 0.01 SOL
  { supplyPoint: new BN(1_000_000), priceLamports: new BN(20_000_000) }, // 0.02 SOL
  { supplyPoint: new BN(2_000_000), priceLamports: new BN(30_000_000) }, // 0.03 SOL
];

await program.methods
  .initializeBondingCurve(pricePoints)
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### 3. Buy Tokens

Purchases tokens using SOL through the bonding curve.

**Parameters:**

- `solAmount`: BN - Amount of SOL to spend in lamports

**Accounts:**

- `user`: Signer - The user purchasing tokens
- `config`: PDA - The configuration account
- `bondingCurve`: PDA - The bonding curve account
- `tokenMint`: AccountInfo - The token mint
- `tokenAccount`: AccountInfo - The user's token account to receive purchased tokens
- `systemProgram`: Program - System program
- `tokenProgram`: Program - Token program
- `associatedTokenProgram`: Program - Associated Token Program
- `rent`: Sysvar - Rent sysvar
- `referral` (optional): PDA - The referrer's referral account
- `referrer` (optional): AccountInfo - The referrer's wallet address

**Events:**

- `TokenPurchaseEvent` - Emitted when tokens are purchased
- `PriceCalculatedEvent` - Emitted with current price after purchase

**Example:**

```typescript
// Without referral
await program.methods
  .buyTokens(new BN(1_000_000_000)) // 1 SOL
  .accounts({
    user: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMint,
    tokenAccount: tokenAccount,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();

// With referral
await program.methods
  .buyTokens(new BN(1_000_000_000)) // 1 SOL
  .accounts({
    user: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMint,
    tokenAccount: tokenAccount,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
    referral: referralPDA,
    referrer: referrerPublicKey,
  })
  .rpc();
```

### 4. Sell Tokens

Sells tokens back to the bonding curve for SOL.

**Parameters:**

- `tokenAmount`: BN - Amount of tokens to sell (in raw token units)

**Accounts:**

- `user`: Signer - The user selling tokens
- `config`: PDA - The configuration account
- `bondingCurve`: PDA - The bonding curve account
- `tokenMint`: AccountInfo - The token mint
- `tokenAccount`: AccountInfo - The user's token account
- `systemProgram`: Program - System program
- `tokenProgram`: Program - Token program

**Events:**

- `TokenSaleEvent` - Emitted when tokens are sold
- `PriceCalculatedEvent` - Emitted with current price after sale

**Example:**

```typescript
await program.methods
  .sellTokens(new BN(1_000_000_000)) // 1 token (assuming 9 decimals)
  .accounts({
    user: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMint,
    tokenAccount: tokenAccount,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### 5. Create Referral

Creates a referral account for the user to earn referral fees.

**Parameters:**
None

**Accounts:**

- `user`: Signer - The user creating the referral
- `config`: PDA - The configuration account
- `referral`: PDA - The referral account that will be created
- `systemProgram`: Program - System program

**PDA Derivation:**

```
[Buffer.from("referral"), user.toBuffer()]
```

**Events:**

- `ReferralCreatedEvent` - Emitted when a referral is created

**Example:**

```typescript
await program.methods
  .createReferral()
  .accounts({
    user: wallet.publicKey,
    config: configPDA,
    referral: referralPDA,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### 6. Update Referral Fee

Updates the referral fee for a specific referrer (admin only).

**Parameters:**

- `newReferralFee`: u16 - New referral fee in basis points (100 = 1%, max 500 = 5%)

**Accounts:**

- `admin`: Signer - The admin updating the referral fee
- `config`: PDA - The configuration account
- `referral`: PDA - The referral account to update
- `referrer`: AccountInfo - The referrer's wallet address

**Events:**

- `ReferralUpdatedEvent` - Emitted when a referral fee is updated

**Example:**

```typescript
await program.methods
  .updateReferralFee(200) // 2%
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
    referral: referralPDA,
    referrer: referrerPublicKey,
  })
  .rpc();
```

### 7. Update Treasury

Updates the treasury address (admin only).

**Parameters:**

- `newTreasury`: PublicKey - New treasury address

**Accounts:**

- `admin`: Signer - The admin updating the treasury
- `config`: PDA - The configuration account

**Events:**

- `TreasuryUpdatedEvent` - Emitted when treasury is updated

**Example:**

```typescript
await program.methods
  .updateTreasury(newTreasuryPublicKey)
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### 8. Transfer Admin

Transfers admin role to a new address.

**Parameters:**

- `newAdmin`: PublicKey - New admin address

**Accounts:**

- `admin`: Signer - The current admin
- `config`: PDA - The configuration account

**Events:**

- `AdminTransferredEvent` - Emitted when admin role is transferred

**Example:**

```typescript
await program.methods
  .transferAdmin(newAdminPublicKey)
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### 9. Pause Protocol

Pauses the protocol, preventing token purchases and sales (admin only).

**Parameters:**
None

**Accounts:**

- `admin`: Signer - The admin pausing the protocol
- `config`: PDA - The configuration account

**Events:**

- `ProtocolPausedEvent` - Emitted when protocol is paused

**Example:**

```typescript
await program.methods
  .pauseProtocol()
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### 10. Unpause Protocol

Unpauses the protocol, allowing token purchases and sales again (admin only).

**Parameters:**
None

**Accounts:**

- `admin`: Signer - The admin unpausing the protocol
- `config`: PDA - The configuration account

**Events:**

- `ProtocolUnpausedEvent` - Emitted when protocol is unpaused

**Example:**

```typescript
await program.methods
  .unpauseProtocol()
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### 11. Migrate To Raydium

Migrates the bonding curve liquidity to a Raydium pool (admin only or automatic when conditions are met).

**Parameters:**
None

**Accounts:**

- `admin`: Signer - The admin initiating migration (or any user if auto-migration conditions are met)
- `config`: PDA - The configuration account
- `bondingCurve`: PDA - The bonding curve account
- `tokenMint`: AccountInfo - The token mint
- `systemProgram`: Program - System program
- `tokenProgram`: Program - Token program
- `raydiumAmmProgram`: Program - Raydium AMM program
- `raydiumPool`: PDA - The Raydium pool PDA
- `raydiumPoolAuthority`: PDA - The Raydium pool authority PDA
- `raydiumOpenOrders`: PDA - The Raydium open orders PDA
- `solLeg`: Signer - The SOL leg account for the Raydium pool
- `tokenLeg`: Signer - The token leg account for the Raydium pool
- `lpMint`: Signer - The LP token mint
- `feeAccount`: Signer - The fee account

**Events:**

- `MigrationCompletedEvent` - Emitted when migration to Raydium is completed

**Example:**

```typescript
await program.methods
  .migrateToRaydium()
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMint,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    raydiumAmmProgram: RAYDIUM_AMM_PROGRAM_ID,
    raydiumPool: raydiumPoolPDA,
    raydiumPoolAuthority: raydiumPoolAuthorityPDA,
    raydiumOpenOrders: raydiumOpenOrdersPDA,
    solLeg: solLegKeypair.publicKey,
    tokenLeg: tokenLegKeypair.publicKey,
    lpMint: lpMintKeypair.publicKey,
    feeAccount: feeAccountKeypair.publicKey,
  })
  .signers([solLegKeypair, tokenLegKeypair, lpMintKeypair, feeAccountKeypair])
  .rpc();
```

### 12. Create Airdrop

Creates an airdrop distribution (admin only).

**Parameters:**

- `recipients`: Array of { recipient: PublicKey, amount: BN } - Recipients and token amounts

**Accounts:**

- `admin`: Signer - The admin creating the airdrop
- `config`: PDA - The configuration account
- `bondingCurve`: PDA - The bonding curve account
- `tokenMint`: AccountInfo - The token mint
- `systemProgram`: Program - System program
- `tokenProgram`: Program - Token program
- `associatedTokenProgram`: Program - Associated Token Program
- `rent`: Sysvar - Rent sysvar

**Events:**

- `AirdropCreatedEvent` - Emitted when an airdrop is created

**Example:**

```typescript
const recipients = [
  { recipient: recipient1PublicKey, amount: new BN(1_000_000_000) }, // 1 token
  { recipient: recipient2PublicKey, amount: new BN(2_000_000_000) }, // 2 tokens
];

await program.methods
  .createAirdrop(recipients)
  .accounts({
    admin: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMint,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();
```

## Account Structures

### 1. Config

The main configuration account for the protocol.

**PDA Seed:**

```
["config"]
```

**Structure:**

```rust
pub struct Config {
    pub admin: Pubkey,         // Admin wallet
    pub token_mint: Pubkey,    // Token mint address
    pub treasury: Pubkey,      // Treasury address for protocol fees
    pub default_referral_fee: u16, // Default referral fee in basis points
    pub is_paused: bool,       // Protocol pause status
}
```

### 2. BondingCurve

The bonding curve account that manages token sales and purchases.

**PDA Seed:**

```
["bonding_curve"]
```

**Structure:**

```rust
pub struct BondingCurve {
    pub total_sold_supply: u64,       // Total tokens sold
    pub total_sol_raised: u64,        // Total SOL raised in lamports
    pub reserve_balance: u64,         // SOL reserve balance in lamports
    pub current_price: u64,           // Current token price in lamports
    pub price_points: Vec<PricePoint>, // Price/supply points defining the curve
    pub is_migrated: bool,            // Migration status
}

pub struct PricePoint {
    pub supply_point: u64,     // Supply threshold
    pub price_lamports: u64,   // Price at this supply in lamports
}
```

### 3. Referral

The referral account for each referrer.

**PDA Seed:**

```
["referral", referrer.key]
```

**Structure:**

```rust
pub struct Referral {
    pub referrer: Pubkey,          // Referrer wallet
    pub referral_fee: u16,         // Referral fee in basis points
    pub total_referrals: u64,      // Number of referrals
    pub total_rewards: u64,        // Total rewards earned in lamports
}
```

## Events

All events emitted by the protocol:

### 1. InitializeEvent

```rust
pub struct InitializeEvent {
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub default_referral_fee: u16,
    pub timestamp: i64,
}
```

### 2. InitializeBondingCurveEvent

```rust
pub struct InitializeBondingCurveEvent {
    pub admin: Pubkey,
    pub price_points_count: u8,
    pub timestamp: i64,
}
```

### 3. TokenPurchaseEvent

```rust
pub struct TokenPurchaseEvent {
    pub user: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub price: u64,
    pub referrer: Option<Pubkey>,
    pub referral_fee: Option<u16>,
    pub referral_amount: Option<u64>,
    pub timestamp: i64,
}
```

### 4. TokenSaleEvent

```rust
pub struct TokenSaleEvent {
    pub user: Pubkey,
    pub token_amount: u64,
    pub sol_amount: u64,
    pub price: u64,
    pub timestamp: i64,
}
```

### 5. PriceCalculatedEvent

```rust
pub struct PriceCalculatedEvent {
    pub total_sold_supply: u64,
    pub current_price: u64,
    pub timestamp: i64,
}
```

### 6. ReferralCreatedEvent

```rust
pub struct ReferralCreatedEvent {
    pub referrer: Pubkey,
    pub referral_fee: u16,
    pub timestamp: i64,
}
```

### 7. ReferralUpdatedEvent

```rust
pub struct ReferralUpdatedEvent {
    pub referrer: Pubkey,
    pub old_referral_fee: u16,
    pub new_referral_fee: u16,
    pub timestamp: i64,
}
```

### 8. TreasuryUpdatedEvent

```rust
pub struct TreasuryUpdatedEvent {
    pub old_treasury: Pubkey,
    pub new_treasury: Pubkey,
    pub timestamp: i64,
}
```

### 9. AdminTransferredEvent

```rust
pub struct AdminTransferredEvent {
    pub old_admin: Pubkey,
    pub new_admin: Pubkey,
    pub timestamp: i64,
}
```

### 10. ProtocolPausedEvent

```rust
pub struct ProtocolPausedEvent {
    pub admin: Pubkey,
    pub timestamp: i64,
}
```

### 11. ProtocolUnpausedEvent

```rust
pub struct ProtocolUnpausedEvent {
    pub admin: Pubkey,
    pub timestamp: i64,
}
```

### 12. MigrationCompletedEvent

```rust
pub struct MigrationCompletedEvent {
    pub initiator: Pubkey,
    pub is_admin: bool,
    pub total_tokens: u64,
    pub total_sol: u64,
    pub pool_address: Pubkey,
    pub timestamp: i64,
}
```

### 13. AirdropCreatedEvent

```rust
pub struct AirdropCreatedEvent {
    pub admin: Pubkey,
    pub recipient_count: u16,
    pub total_tokens: u64,
    pub timestamp: i64,
}
```

## Error Codes

| Code | Name                     | Description                                         |
| ---- | ------------------------ | --------------------------------------------------- |
| 6000 | InvalidPricePoints       | Fewer than 2 price points or not in ascending order |
| 6001 | ProtocolPaused           | The protocol is currently paused                    |
| 6002 | InsufficientSolAmount    | SOL amount too small for purchase                   |
| 6003 | InsufficientTokenAmount  | Token amount too small for sale                     |
| 6004 | InsufficientReserve      | Not enough SOL in reserve for sale                  |
| 6005 | ExceedsMaxSupply         | Purchase would exceed max token supply              |
| 6006 | AlreadyMigrated          | Bonding curve already migrated to Raydium           |
| 6007 | AdminOnly                | Only the admin can perform this action              |
| 6008 | MigrationInProgress      | Migration is in progress                            |
| 6009 | InvalidReferralFee       | Referral fee exceeds maximum allowed                |
| 6010 | MigrationThresholdNotMet | SOL raised hasn't met migration threshold           |

## Constants

| Name                              | Value     | Description                           |
| --------------------------------- | --------- | ------------------------------------- |
| TOTAL_SUPPLY                      | 1e18      | Maximum token supply                  |
| PRECISION_FACTOR                  | 1e9       | Precision factor for calculations     |
| DEFAULT_REFERRAL_FEE              | 100       | Default referral fee (1%)             |
| MAX_REFERRAL_FEE                  | 500       | Maximum referral fee (5%)             |
| MIN_SOL_RAISED_FOR_AUTO_MIGRATION | 100 SOL   | Minimum SOL raised for auto-migration |
| MIN_TOKEN_SALE_AMOUNT             | 0.1 token | Minimum token amount for sales        |

## Security Considerations

1. **Admin Controls:** The admin has significant control over the protocol. Admin operations should be executed with caution and potentially through a multisig wallet.

2. **Price Manipulation:** Ensure price points are set appropriately to prevent easy manipulation of token prices.

3. **Reentrancy:** The program properly follows the checks-effects-interactions pattern to prevent reentrancy attacks.

4. **Referral System:** Referral fees are capped at 5% to prevent excessive diversion of funds.

5. **Migration:** The migration to Raydium is a one-way process and cannot be reversed. Ensure all conditions are met before migration.

## Transaction Fees

Transactions on Solana incur fees based on:

- Number of signatures (signers)
- Compute units used
- Network congestion

Most operations in the Yozoon protocol require minimal fees, usually under 0.000005 SOL per transaction. Complex operations like migration to Raydium will incur higher fees due to multiple signers and cross-program invocations.
