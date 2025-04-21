# Yozoon Protocol API Documentation

This document provides a comprehensive guide to the Yozoon Protocol API, detailing all the available program instructions, account structures, and integration patterns.

## Table of Contents

1. [Overview](#overview)
2. [Account Structures](#account-structures)
3. [Program Instructions](#program-instructions)
4. [State Transitions](#state-transitions)
5. [Events](#events)
6. [Error Codes](#error-codes)
7. [Integration Examples](#integration-examples)

## Overview

Yozoon is a Solana protocol that implements a bonding curve token sale mechanism with referral capabilities and automatic migration to Raydium liquidity pools once funding thresholds are met.

The protocol follows a lifecycle:

1. Initialization phase - Setting up mint and bonding curve
2. Token Sale phase - Users can buy and sell tokens using the bonding curve
3. Migration phase - Protocol transitions to Raydium once funding thresholds are met
4. Post-migration phase - Trading occurs on Raydium with permanent liquidity

## Account Structures

### Config

The `Config` account stores global configuration for the protocol.

```rust
pub struct Config {
    pub admin: Pubkey,         // Protocol administrator
    pub mint: Pubkey,          // Token mint address
    pub treasury: Pubkey,      // Treasury that receives fees
    pub referral_fee: u16,     // Referral fee in basis points (e.g., 100 = 1%)
    pub treasury_fee: u16,     // Treasury fee in basis points
    pub is_active: bool,       // Whether the protocol is active
    pub total_referrers: u64,  // Total number of referrers
    pub bump: u8,              // PDA bump
}
```

### BondingCurve

The `BondingCurve` account stores the state of the bonding curve.

```rust
pub struct BondingCurve {
    pub token_mint: Pubkey,        // Token mint address
    pub price_points: Vec<u64>,    // Price points in lamports for the bonding curve
    pub supply_points: Vec<u64>,   // Supply points corresponding to price points
    pub total_sold_supply: u64,    // Total tokens sold
    pub total_sol_raised: u64,     // Total SOL raised
    pub reserve_balance: u64,      // Reserve balance for token buybacks
    pub is_migrated: bool,         // Whether migration to Raydium is complete
    pub bump: u8,                  // PDA bump
}
```

### Referral

The `Referral` account stores referral information for a user.

```rust
pub struct Referral {
    pub user: Pubkey,           // User public key
    pub referrer: Pubkey,       // Referrer public key
    pub total_earned: u64,      // Total referral rewards earned
    pub bump: u8,               // PDA bump
}
```

### RaydiumPool

The `RaydiumPool` account stores information about the Raydium pool after migration.

```rust
pub struct RaydiumPool {
    pub token_mint: Pubkey,      // Token mint address
    pub lp_mint: Pubkey,         // LP token mint address
    pub creation_time: i64,      // Pool creation timestamp
    pub bump: u8,                // PDA bump
}
```

## Program Instructions

### Admin Instructions

#### Initialize Mint

Initializes the token mint and global configuration.

```typescript
initializeMint(
  decimals: number,         // Token decimals (usually 9)
  treasury: PublicKey,      // Treasury address to receive fees
  treasuryFee: number,      // Treasury fee in basis points
  referralFee: number,      // Referral fee in basis points
): Promise<TransactionInstruction>
```

Requirements:

- Only callable once
- Must be called by the protocol admin
- `referralFee` must be ≤ 500 basis points (5%)
- `treasuryFee` + `referralFee` must be ≤ 1000 basis points (10%)

#### Initialize Bonding Curve

Sets up the bonding curve with predetermined price points.

```typescript
initializeBondingCurve(
  pricePoints: BN[],        // Price points in lamports
  supplyPoints: BN[],       // Supply points corresponding to price points
): Promise<TransactionInstruction>
```

Requirements:

- Only callable once after mint initialization
- Must be called by the protocol admin
- Must include at least 2 price/supply points
- Price and supply points must be in ascending order

#### Transfer Admin

Transfers admin rights to a new address.

```typescript
transferAdmin(
  newAdmin: PublicKey,      // New admin public key
): Promise<TransactionInstruction>
```

Requirements:

- Only callable by the current admin

#### Update Treasury

Updates the treasury address.

```typescript
updateTreasury(
  newTreasury: PublicKey,   // New treasury public key
): Promise<TransactionInstruction>
```

Requirements:

- Only callable by the admin

#### Update Referral Fee

Updates the referral fee.

```typescript
updateReferralFee(
  newFee: number,           // New referral fee in basis points
): Promise<TransactionInstruction>
```

Requirements:

- Only callable by the admin
- `newFee` must be ≤ 500 basis points (5%)

#### Pause/Unpause Protocol

Pauses or unpauses the protocol.

```typescript
pauseProtocol(): Promise<TransactionInstruction>
unpauseProtocol(): Promise<TransactionInstruction>
```

Requirements:

- Only callable by the admin

### User Instructions

#### Buy Tokens

Allows a user to purchase tokens with SOL.

```typescript
buyTokens(
  amount: BN,               // Amount of SOL in lamports
): Promise<TransactionInstruction>
```

Requirements:

- Protocol must be active and not migrated
- Amount must be greater than minimum purchase
- User must have sufficient SOL

#### Sell Tokens

Allows a user to sell tokens for SOL.

```typescript
sellTokens(
  amount: BN,               // Amount of tokens to sell
): Promise<TransactionInstruction>
```

Requirements:

- Protocol must be active and not migrated
- Amount must be greater than minimum sale
- Bonding curve must have sufficient reserve balance

#### Set Referral

Sets a referrer for a user.

```typescript
setReferral(
  referrer: PublicKey,      // Referrer public key
): Promise<TransactionInstruction>
```

Requirements:

- User cannot set themselves as their own referrer
- Referral can only be set once per user

### Migration Instructions

#### Migrate to Raydium

Migrates the protocol to Raydium.

```typescript
migrateToRaydium(): Promise<TransactionInstruction>
```

Requirements:

- Only callable by the admin
- Migration conditions must be met (SOL raised within threshold)
- Protocol must not be already migrated

### View Functions

#### Calculate Current Price

Calculates the current token price.

```typescript
calculateCurrentPrice(): Promise<BN>
```

#### Calculate Tokens for SOL

Calculates the number of tokens a user would receive for a specific SOL amount.

```typescript
calculateTokensForSol(
  amount: BN,               // Amount of SOL in lamports
): Promise<BN>
```

#### Check Migration Conditions

Checks if migration conditions are met.

```typescript
checkMigrationConditions(): Promise<boolean>
```

## State Transitions

The protocol follows these state transitions:

1. **Uninitialized** → **Initialized** (via `initializeMint` and `initializeBondingCurve`)
2. **Initialized** → **Paused** (via `pauseProtocol`)
3. **Paused** → **Initialized** (via `unpauseProtocol`)
4. **Initialized** → **Migrated** (via `migrateToRaydium` when conditions are met)

Once in the **Migrated** state, the bonding curve functions are disabled and trading occurs on Raydium.

## Events

The protocol emits the following events:

### ProtocolInitializedEvent

Emitted when the protocol is initialized.

```typescript
{
  admin: PublicKey,
  mint: PublicKey,
  treasury: PublicKey,
  treasuryFee: number,
  referralFee: number,
  timestamp: BN,
}
```

### BondingCurveInitializedEvent

Emitted when the bonding curve is initialized.

```typescript
{
  pricePoints: BN[],
  supplyPoints: BN[],
  timestamp: BN,
}
```

### TokenPurchaseEvent

Emitted when tokens are purchased.

```typescript
{
  buyer: PublicKey,
  amountSol: BN,
  amountTokens: BN,
  price: BN,
  referralFee: BN,
  treasuryFee: BN,
  timestamp: BN,
}
```

### TokenSaleEvent

Emitted when tokens are sold.

```typescript
{
  seller: PublicKey,
  amountTokens: BN,
  amountSol: BN,
  price: BN,
  timestamp: BN,
}
```

### ReferralCreatedEvent

Emitted when a referral relationship is created.

```typescript
{
  user: PublicKey,
  referrer: PublicKey,
  timestamp: BN,
}
```

### ReferralPaidEvent

Emitted when a referral fee is paid.

```typescript
{
  referrer: PublicKey,
  amount: BN,
  timestamp: BN,
}
```

### MigrationEvent

Emitted when migration to Raydium is completed.

```typescript
{
  admin: PublicKey,
  tokenMint: PublicKey,
  lpMint: PublicKey,
  solAmount: BN,
  tokenAmount: BN,
  timestamp: BN,
}
```

### PriceCalculatedEvent

Emitted when price calculation is performed.

```typescript
{
  price: BN,
  totalSupply: BN,
  timestamp: BN,
}
```

### TokensCalculatedEvent

Emitted when token calculation is performed.

```typescript
{
  solAmount: BN,
  tokenAmount: BN,
  price: BN,
  timestamp: BN,
}
```

## Error Codes

The protocol defines the following error codes:

| Code | Name                      | Description                                              |
| ---- | ------------------------- | -------------------------------------------------------- |
| 6000 | InvalidPricePoint         | Price points must be in ascending order                  |
| 6001 | InvalidSupplyPoint        | Supply points must be in ascending order                 |
| 6002 | InsufficientPricePoints   | At least 2 price points are required                     |
| 6003 | PriceSupplyMismatch       | Price and supply points arrays must have the same length |
| 6004 | ProtocolPaused            | The protocol is currently paused                         |
| 6005 | InsufficientSolAmount     | SOL amount is below minimum purchase requirement         |
| 6006 | InsufficientReserve       | Insufficient reserve balance for token sale              |
| 6007 | InsufficientTokenAmount   | Token amount is below minimum sale requirement           |
| 6008 | InvalidFeePercentage      | Fee percentage exceeds maximum allowed                   |
| 6009 | AlreadyMigrated           | Protocol has already been migrated to Raydium            |
| 6010 | MigrationConditionsNotMet | Migration conditions are not yet met                     |
| 6011 | MigrationWindowPassed     | Migration window has passed                              |
| 6012 | SelfReferral              | Cannot set yourself as your own referrer                 |
| 6013 | ReferralAlreadyExists     | Referral relationship already exists                     |
| 6014 | UnauthorizedAdmin         | Only the admin can perform this action                   |
| 6015 | InvalidRaydiumPool        | Invalid Raydium pool parameters                          |
| 6016 | CalculationOverflow       | Calculation resulted in overflow                         |

## Integration Examples

### Frontend Integration

```typescript
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { AnchorProvider, Program, BN } from '@project-serum/anchor';
import { getAssociatedTokenAddress } from '@solana/spl-token';
import { IDL } from '../target/types/yozoon';

// Initialize connection and provider
const connection = new Connection('https://api.devnet.solana.com');
const wallet = // ... your wallet adapter
const provider = new AnchorProvider(connection, wallet, {});

// Initialize program
const programId = new PublicKey('YourProgramId');
const program = new Program(IDL, programId, provider);

// Get PDAs
const [configPda] = await PublicKey.findProgramAddress(
  [Buffer.from('config')],
  program.programId
);

const [bondingCurvePda] = await PublicKey.findProgramAddress(
  [Buffer.from('bonding_curve')],
  program.programId
);

// Example: Get current token price
const currentPrice = await program.methods
  .calculateCurrentPrice()
  .accounts({
    bondingCurve: bondingCurvePda,
  })
  .view();

// Example: Buy tokens
const buyAmount = new BN(1_000_000_000); // 1 SOL in lamports
const tx = await program.methods
  .buyTokens(buyAmount)
  .accounts({
    config: configPda,
    bondingCurve: bondingCurvePda,
    mint: mintPubkey, // Get this from config account
    buyer: wallet.publicKey,
    buyerTokenAccount: await getAssociatedTokenAddress(
      mintPubkey,
      wallet.publicKey
    ),
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  })
  .rpc();

console.log('Transaction signature:', tx);
```

### CLI Integration

The project includes a CLI utility for common operations. See `scripts/cli.js` for implementation details.

Example usage:

```bash
# Get protocol status
node scripts/cli.js status

# Buy tokens
node scripts/cli.js buy --amount 1.5

# Set referral
node scripts/cli.js set-referral --referrer 8dHECLdxF5fqGwQCwvmUjdvz4ZmQzBrLq3eFA2NmR3Z3

# Check migration status
node scripts/cli.js migration-status
```

### Monitoring Events

To monitor protocol events:

```typescript
// Subscribe to program logs
const subscriptionId = connection.onLogs(
  programId,
  (logs) => {
    // Parse logs to find events
    const parsedLogs = program.coder.events.parse(logs.logs);

    if (parsedLogs && parsedLogs.name === "TokenPurchaseEvent") {
      console.log("Token Purchase Event:", parsedLogs.data);
      // Handle token purchase event
    }

    if (parsedLogs && parsedLogs.name === "MigrationEvent") {
      console.log("Migration Event:", parsedLogs.data);
      // Handle migration event
    }
  },
  "confirmed"
);

// Later, unsubscribe
connection.removeOnLogsListener(subscriptionId);
```

## Additional Resources

- [Full Migration Test](../tests/full_migration_test.js) - Comprehensive test showing the entire protocol lifecycle
- [CLI Utility](../scripts/cli.js) - Command-line interface for interacting with the protocol
- [Frontend Integration Guide](./frontend-integration-guide.md) - Detailed guide for frontend integration
- [Raydium Integration](./raydium-integration.md) - Details on Raydium pool creation and integration
