# Yozoon Protocol API Documentation

This document provides comprehensive documentation for integrating with the Yozoon protocol.

## Overview

Yozoon is a Solana-based protocol that implements a bonding curve for token distribution, a referral system, and migration capabilities to Raydium.

## Account Structure

### Config Account

Stores the protocol configuration including admin keys and global parameters.

```
PDA Seed: ["config"]
```

Fields:

- `admin`: PublicKey - The protocol administrator
- `treasury`: PublicKey - Treasury account for protocol fees
- `tokenMint`: PublicKey - The token mint address
- `defaultReferralFee`: u64 - Default referral fee in basis points (e.g., 100 = 1%)
- `isPaused`: bool - Protocol pause status
- `bump`: u8 - PDA bump seed

### Bonding Curve Account

Stores the bonding curve state and parameters.

```
PDA Seed: ["bonding_curve"]
```

Fields:

- `totalSoldSupply`: u64 - Total tokens sold through the bonding curve
- `totalSolRaised`: u64 - Total SOL received through token purchases
- `isMigratedToRaydium`: bool - Whether migration to Raydium has occurred
- `pricePoints`: PricePoint[] - Array of price points defining the curve
- `bump`: u8 - PDA bump seed

### Referral Account

Tracks referrer details and earnings.

```
PDA Seed: ["referral", referrer_pubkey]
```

Fields:

- `referrer`: PublicKey - The referrer's public key
- `fee`: u64 - Referrer's custom fee in basis points
- `totalEarned`: u64 - Total SOL earned from referrals
- `bump`: u8 - PDA bump seed

### Airdrop Account

Tracks airdrop allocations.

```
PDA Seed: ["airdrop", recipient_pubkey]
```

Fields:

- `recipient`: PublicKey - Airdrop recipient
- `amount`: u64 - Token amount to be airdropped
- `claimed`: bool - Whether the airdrop has been claimed
- `bump`: u8 - PDA bump seed

## Instructions

### Initialize Mint

Creates the token mint and initializes the protocol.

```typescript
await program.methods
  .initializeMint(
    defaultReferralFee, // basis points (e.g., 100 = 1%)
    pricePoints // Array of {supply: BN, pricePerToken: BN}
  )
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: mintKeypair.publicKey,
    treasury: treasuryKeypair.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  })
  .signers([mintKeypair, treasuryKeypair])
  .rpc();
```

### Buy Tokens

Purchases tokens through the bonding curve.

```typescript
await program.methods
  .buyTokens(
    new BN(solAmount), // lamports
    minTokensExpected // minimum tokens to receive (slippage protection)
  )
  .accounts({
    buyer: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMintPDA,
    buyerTokenAccount: buyerTokenAccount,
    treasury: treasuryPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### Buy Tokens With Referral

Purchases tokens with a referral fee.

```typescript
await program.methods
  .buyTokensWithReferral(
    new BN(solAmount), // lamports
    minTokensExpected // minimum tokens to receive
  )
  .accounts({
    buyer: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMintPDA,
    buyerTokenAccount: buyerTokenAccount,
    treasury: treasuryPDA,
    referral: referralPDA,
    referrer: referrerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### Sell Tokens

Sells tokens back to the bonding curve.

```typescript
await program.methods
  .sellTokens(
    new BN(tokenAmount), // token amount to sell
    minSolExpected // minimum SOL to receive
  )
  .accounts({
    seller: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMintPDA,
    sellerTokenAccount: sellerTokenAccount,
    treasury: treasuryPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### Create Referral

Creates a new referral account.

```typescript
await program.methods
  .createReferral(
    new BN(referralFee) // basis points (e.g., 200 = 2%)
  )
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    referrer: wallet.publicKey, // usually the same as authority
    referral: referralPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Update Referral Fee

Updates an existing referral fee.

```typescript
await program.methods
  .updateReferralFee(
    new BN(newReferralFee) // basis points
  )
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    referrer: wallet.publicKey,
    referral: referralPDA,
  })
  .rpc();
```

### Create Airdrop

Creates an airdrop allocation.

```typescript
await program.methods
  .createAirdrop(new BN(tokenAmount))
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    recipient: recipientPublicKey,
    airdrop: airdropPDA,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### Claim Airdrop

Claims an airdrop allocation.

```typescript
await program.methods
  .claimAirdrop()
  .accounts({
    recipient: wallet.publicKey,
    recipientTokenAccount: recipientTokenAccount,
    config: configPDA,
    airdrop: airdropPDA,
    tokenMint: tokenMintPDA,
    tokenProgram: TOKEN_PROGRAM_ID,
  })
  .rpc();
```

### Pause Protocol

Pauses all protocol operations except for admin functions.

```typescript
await program.methods
  .pauseProtocol()
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### Unpause Protocol

Resumes protocol operations.

```typescript
await program.methods
  .unpauseProtocol()
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
  })
  .rpc();
```

### Transfer Admin

Transfers admin rights to a new address.

```typescript
await program.methods
  .transferAdmin()
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    newAdmin: newAdminPublicKey,
  })
  .rpc();
```

### Update Treasury

Updates the treasury address.

```typescript
await program.methods
  .updateTreasury()
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    newTreasury: newTreasuryPublicKey,
  })
  .rpc();
```

### Migrate To Raydium

Migrates liquidity to a Raydium pool when conditions are met.

```typescript
await program.methods
  .migrateToRaydium()
  .accounts({
    authority: wallet.publicKey,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint: tokenMintPDA,
    treasury: treasuryPDA,
    treasuryTokenAccount: treasuryTokenAccount,
    raydiumPoolId: raydiumPoolPDA,
    // Additional Raydium accounts...
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    raydiumProgramId: RAYDIUM_PROGRAM_ID,
  })
  .rpc();
```

## Events

### TokenPurchaseEvent

Emitted when tokens are purchased.

Fields:

- `user`: PublicKey - Buyer address
- `tokenAmount`: u64 - Amount of tokens purchased
- `solAmount`: u64 - Amount of SOL spent
- `timestamp`: u64 - Unix timestamp

### TokenSaleEvent

Emitted when tokens are sold.

Fields:

- `user`: PublicKey - Seller address
- `tokenAmount`: u64 - Amount of tokens sold
- `solAmount`: u64 - Amount of SOL received
- `timestamp`: u64 - Unix timestamp

### PriceCalculatedEvent

Emitted when a token price is calculated.

Fields:

- `totalSupply`: u64 - Current total supply
- `tokenPrice`: u64 - Calculated price per token
- `timestamp`: u64 - Unix timestamp

### ReferralCreatedEvent

Emitted when a referral is created.

Fields:

- `referrer`: PublicKey - Referrer address
- `fee`: u64 - Referral fee in basis points
- `timestamp`: u64 - Unix timestamp

### ReferralPaymentEvent

Emitted when a referral payment is made.

Fields:

- `referrer`: PublicKey - Referrer who received payment
- `user`: PublicKey - User who made the purchase
- `amount`: u64 - Amount of SOL paid as referral
- `timestamp`: u64 - Unix timestamp

### MigrationCompletedEvent

Emitted when migration to Raydium is completed.

Fields:

- `totalSolMigrated`: u64 - Amount of SOL migrated
- `totalTokensMigrated`: u64 - Amount of tokens migrated
- `raydiumPoolAddress`: PublicKey - Address of the Raydium pool
- `timestamp`: u64 - Unix timestamp

### AirdropClaimedEvent

Emitted when an airdrop is claimed.

Fields:

- `recipient`: PublicKey - Recipient address
- `amount`: u64 - Amount of tokens claimed
- `timestamp`: u64 - Unix timestamp

## Error Codes

- `Unauthorized`: Authority is not the admin
- `InvalidPricePoints`: Price points must be in ascending order
- `InsufficientSol`: Not enough SOL for purchase
- `InvalidReferralFee`: Referral fee exceeds maximum
- `ProtocolPaused`: Protocol operations are paused
- `ProtocolNotPaused`: Protocol is not paused
- `InsufficientTokens`: Not enough tokens for sale
- `SlippageExceeded`: Transaction exceeds slippage tolerance
- `InsufficientReserve`: Treasury doesn't have enough SOL
- `AirdropAlreadyClaimed`: Airdrop has already been claimed
- `MigrationNotReady`: Conditions for migration are not met
- `AlreadyMigrated`: Protocol has already migrated to Raydium

## Utility Functions

### Calculating Token Price

```typescript
// Example function to calculate token price based on the bonding curve
function calculateTokenPrice(
  totalSupply: BN,
  pricePoints: PricePoint[],
  tokenAmount: BN
): BN {
  // Find applicable price points
  let lowerPoint = pricePoints[0];
  let upperPoint = pricePoints[pricePoints.length - 1];

  for (let i = 0; i < pricePoints.length - 1; i++) {
    if (
      totalSupply.gte(pricePoints[i].supply) &&
      totalSupply.lt(pricePoints[i + 1].supply)
    ) {
      lowerPoint = pricePoints[i];
      upperPoint = pricePoints[i + 1];
      break;
    }
  }

  // Linear interpolation between price points
  const supplyRange = upperPoint.supply.sub(lowerPoint.supply);
  const priceRange = upperPoint.pricePerToken.sub(lowerPoint.pricePerToken);
  const supplyOffset = totalSupply.sub(lowerPoint.supply);

  const priceFactor = supplyOffset.mul(priceRange).div(supplyRange);
  const currentPrice = lowerPoint.pricePerToken.add(priceFactor);

  return currentPrice.mul(tokenAmount).div(new BN(1e9)); // Apply precision factor
}
```

### Deriving PDAs

```typescript
// Derive Config PDA
const [configPDA] = await PublicKey.findProgramAddress(
  [Buffer.from("config")],
  programId
);

// Derive Bonding Curve PDA
const [bondingCurvePDA] = await PublicKey.findProgramAddress(
  [Buffer.from("bonding_curve")],
  programId
);

// Derive Referral PDA
const [referralPDA] = await PublicKey.findProgramAddress(
  [Buffer.from("referral"), referrerPublicKey.toBuffer()],
  programId
);

// Derive Airdrop PDA
const [airdropPDA] = await PublicKey.findProgramAddress(
  [Buffer.from("airdrop"), recipientPublicKey.toBuffer()],
  programId
);
```
