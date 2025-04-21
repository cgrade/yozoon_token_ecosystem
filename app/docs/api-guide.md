# Yozoon Protocol API Guide

## Overview

The Yozoon Protocol is a Solana-based token protocol that implements a bonding curve mechanism for token distribution and includes features like referrals, airdrops, and eventual migration to Raydium AMM for enhanced liquidity.

This guide explains how to integrate with the Yozoon protocol from your frontend application.

## Installation

1. Install the required dependencies:

```bash
npm install @project-serum/anchor @solana/web3.js @solana/spl-token
```

2. Import the Yozoon client and types:

```typescript
import { YozoonClient } from "../utils/yozoon-client";
import { YozoonIDL } from "../idl/types";
```

## Key Concepts

### Bonding Curve

The protocol uses a bonding curve pricing mechanism where:

- Token price increases as more tokens are sold
- Price is determined by pre-configured price points that define the curve
- Each price point consists of a supply level and a price per token

### Referrals

The protocol supports referrals where:

- Users can create referral links tied to their address
- When other users buy tokens through their referral, the referrer gets a percentage fee
- Referral fees are configurable (with a maximum cap)

### Migration to Raydium

Once sufficient funds are raised, the protocol can migrate to Raydium AMM:

- Migration creates a permanent liquidity pool
- The migration threshold is pre-configured
- After migration, tokens can be traded on Raydium DEX

## Initializing the Client

```typescript
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { AnchorProvider, Wallet } from "@project-serum/anchor";
import { YozoonClient } from "../utils/yozoon-client";

// Set up provider
const connection = new Connection("https://api.devnet.solana.com");
const wallet = new Wallet(Keypair.generate()); // Replace with actual wallet
const provider = new AnchorProvider(connection, wallet, {});

// Initialize Yozoon client
const programId = new PublicKey("YOUR_PROGRAM_ID");
const client = new YozoonClient(provider, programId);
```

## Core Functions

### Getting Protocol Information

```typescript
// Get configuration info
const configAddress = await client.getConfigAddress();
const configData = await client.fetchConfigData(configAddress);

// Get bonding curve info
const bondingCurveAddress = await client.getBondingCurveAddress();
const bondingCurveData = await client.fetchBondingCurveData(
  bondingCurveAddress
);

// Check if protocol is migrated to Raydium
const isMigrated = await client.isMigratedToRaydium(bondingCurveAddress);

// Get current token price
const currentPrice = await client.getCurrentTokenPrice(bondingCurveAddress);
```

### Buy and Sell Tokens

```typescript
// Buy tokens
const solAmount = new BN(1_000_000_000); // 1 SOL in lamports
const minTokensExpected = new BN(1_000_000); // Minimum tokens expected (slippage protection)
const buyIx = await client.buildBuyTokensInstruction(
  wallet.publicKey,
  solAmount,
  minTokensExpected
);

// Buy tokens with referral
const referrerPublicKey = new PublicKey("REFERRER_ADDRESS");
const buyWithReferralIx = await client.buildBuyTokensWithReferralInstruction(
  wallet.publicKey,
  solAmount,
  minTokensExpected,
  referrerPublicKey
);

// Sell tokens
const tokenAmount = new BN(1_000_000);
const minSolExpected = new BN(900_000_000); // Minimum SOL expected (slippage protection)
const sellIx = await client.buildSellTokensInstruction(
  wallet.publicKey,
  tokenAmount,
  minSolExpected
);
```

### Referral System

```typescript
// Create a referral
const referrerPublicKey = wallet.publicKey;
const referralFee = new BN(100); // 1% in basis points (100/10,000)
const createReferralIx = await client.buildCreateReferralInstruction(
  wallet.publicKey,
  referrerPublicKey,
  referralFee
);

// Fetch referral info
const referralAddress = await client.getReferralAddress(referrerPublicKey);
const referralData = await client.fetchReferralData(referralAddress);
```

### Airdrops

```typescript
// Admin: Create an airdrop
const recipientPublicKey = new PublicKey("RECIPIENT_ADDRESS");
const airdropAmount = new BN(1_000_000);
const createAirdropIx = await client.buildCreateAirdropInstruction(
  wallet.publicKey,
  recipientPublicKey,
  airdropAmount
);

// Recipient: Claim an airdrop
const claimAirdropIx = await client.buildClaimAirdropInstruction(
  wallet.publicKey
);

// Check if an airdrop exists
const airdropAddress = await client.getAirdropAddress(wallet.publicKey);
const airdropData = await client.fetchAirdropData(airdropAddress);
```

### Admin Functions

```typescript
// Initialize the mint
const defaultReferralFee = new BN(100); // 1% in basis points
const initMintIx = await client.buildInitializeMintInstruction(
  wallet.publicKey,
  defaultReferralFee
);

// Initialize bonding curve
const pricePoints = [
  { supply: new BN(1_000_000), pricePerToken: new BN(10_000_000) },
  { supply: new BN(10_000_000), pricePerToken: new BN(20_000_000) },
  { supply: new BN(100_000_000), pricePerToken: new BN(50_000_000) },
];
const initBondingCurveIx = await client.buildInitializeBondingCurveInstruction(
  wallet.publicKey,
  pricePoints
);

// Pause/unpause protocol
const pauseIx = await client.buildPauseProtocolInstruction(wallet.publicKey);
const unpauseIx = await client.buildUnpauseProtocolInstruction(
  wallet.publicKey
);

// Transfer admin
const newAdminPublicKey = new PublicKey("NEW_ADMIN_ADDRESS");
const transferAdminIx = await client.buildTransferAdminInstruction(
  wallet.publicKey,
  newAdminPublicKey
);

// Update treasury
const newTreasury = new PublicKey("NEW_TREASURY_ADDRESS");
const updateTreasuryIx = await client.buildUpdateTreasuryInstruction(
  wallet.publicKey,
  newTreasury
);

// Migrate to Raydium (when conditions are met)
const migrateIx = await client.buildMigrateToRaydiumInstruction(
  wallet.publicKey
);
```

## Event Listeners

The Yozoon client provides methods to subscribe to various events emitted by the protocol:

```typescript
// Subscribe to token purchase events
const purchaseListener = client.subscribeToTokenPurchaseEvents((event) => {
  console.log("Token purchase:", {
    buyer: event.buyer.toBase58(),
    solAmount: event.solAmount.toString(),
    tokenAmount: event.tokenAmount.toString(),
    timestamp: new Date(event.timestamp.toNumber() * 1000),
  });
});

// Subscribe to price calculation events
const priceListener = client.subscribeToPriceCalculatedEvents((event) => {
  console.log("Price update:", {
    totalSupply: event.totalSupply.toString(),
    price: event.price.toString(),
    timestamp: new Date(event.timestamp.toNumber() * 1000),
  });
});

// Don't forget to unsubscribe when done
client.unsubscribe(purchaseListener);
client.unsubscribe(priceListener);
```

## Error Handling

The protocol defines several error codes you should handle in your application:

```typescript
// Example of error handling
try {
  // Attempt to buy tokens
  // ...
} catch (error) {
  if (error.code === 6001) {
    console.error("Insufficient SOL for this transaction");
  } else if (error.code === 6002) {
    console.error("Insufficient token supply");
  } else if (error.code === 6003) {
    console.error("Protocol is currently paused");
  } else if (error.code === 6006) {
    console.error("Slippage tolerance exceeded");
  } else {
    console.error("Unknown error:", error);
  }
}
```

## Best Practices

1. **Slippage Protection**: Always specify realistic `minTokensExpected` and `minSolExpected` values to protect users from price changes during transaction confirmation.

2. **Event Monitoring**: Subscribe to price calculation events to keep your UI updated with the latest token price.

3. **Migration Awareness**: Check if the protocol has migrated to Raydium before attempting to buy or sell through the bonding curve.

4. **Account Prefetch**: To improve UX, prefetch account data when possible to avoid unnecessary loading states.

5. **Referral Integration**: Integrate the referral system to incentivize user growth and provide additional value to users.

## Advanced Usage

### Calculating Price Impact

```typescript
// Calculate price impact for a potential purchase
async function calculatePriceImpact(solAmount) {
  const bondingCurveAddress = await client.getBondingCurveAddress();
  const bondingCurveData = await client.fetchBondingCurveData(
    bondingCurveAddress
  );

  const currentPrice = await client.getCurrentTokenPrice(bondingCurveAddress);
  const tokensToReceive = await client.calculateTokensForSol(
    bondingCurveAddress,
    solAmount
  );

  const newSupply = bondingCurveData.totalSoldSupply.add(tokensToReceive);
  const newPrice = await client.calculatePriceAtSupply(
    bondingCurveAddress,
    newSupply
  );

  const priceImpact = newPrice
    .sub(currentPrice)
    .mul(new BN(100))
    .div(currentPrice);
  return priceImpact;
}
```

### Multi-step Transactions

For complex interactions, you can combine multiple instructions:

```typescript
// Example: Create referral and buy tokens in one transaction
async function createReferralAndBuy(referralFee, solAmount, minTokensExpected) {
  const createReferralIx = await client.buildCreateReferralInstruction(
    wallet.publicKey,
    wallet.publicKey,
    referralFee
  );

  const buyIx = await client.buildBuyTokensInstruction(
    wallet.publicKey,
    solAmount,
    minTokensExpected
  );

  const transaction = new Transaction().add(createReferralIx).add(buyIx);

  return transaction;
}
```

## Need More Help?

Refer to the test files in the `tests/` directory for more examples of using the Yozoon client in different scenarios.
