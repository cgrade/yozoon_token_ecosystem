# Yozoon Protocol Frontend Integration Guide

This guide provides comprehensive examples for integrating the Yozoon protocol into your frontend applications.

## Setup

### Prerequisites

- Node.js (v14+)
- A Solana wallet (Phantom, Sollet, etc.)
- Anchor framework understanding

### Installation

Install the required dependencies:

```bash
npm install @solana/web3.js @solana/spl-token @project-serum/anchor @solana/wallet-adapter-react @solana/wallet-adapter-wallets
```

### Basic Configuration

```typescript
import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program, BN } from "@project-serum/anchor";
import { useWallet } from "@solana/wallet-adapter-react";
import { IDL } from "../app/idl/yozoon";

// Program ID from your Anchor.toml
const PROGRAM_ID = new PublicKey(
  "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
);

// Setup Anchor provider and program
function useYozoonProgram() {
  const { wallet, publicKey, signTransaction, signAllTransactions } =
    useWallet();
  const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
  );

  const provider = new AnchorProvider(
    connection,
    {
      publicKey,
      signTransaction,
      signAllTransactions,
    } as any,
    { commitment: "confirmed" }
  );

  const program = new Program(IDL, PROGRAM_ID, provider);
  return { program, provider, connection };
}
```

## Finding PDAs

```typescript
// Helper function to find Config PDA
async function findConfigPDA(programId: PublicKey) {
  const [configPDA] = await PublicKey.findProgramAddress(
    [Buffer.from("config")],
    programId
  );
  return configPDA;
}

// Helper function to find BondingCurve PDA
async function findBondingCurvePDA(programId: PublicKey) {
  const [bondingCurvePDA] = await PublicKey.findProgramAddress(
    [Buffer.from("bonding_curve")],
    programId
  );
  return bondingCurvePDA;
}

// Helper function to find Referral PDA
async function findReferralPDA(user: PublicKey, programId: PublicKey) {
  const [referralPDA] = await PublicKey.findProgramAddress(
    [Buffer.from("referral"), user.toBuffer()],
    programId
  );
  return referralPDA;
}
```

## Core Functionality Implementation

### 1. Initializing the Protocol (Admin Only)

```typescript
async function initializeProtocol() {
  const { program, provider } = useYozoonProgram();
  const admin = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);

  // Create mint
  const tokenMint = Keypair.generate();

  // Initialize mint with default referral fee (100 = 1%)
  const defaultReferralFee = 100; // 1% in basis points

  try {
    const tx = await program.methods
      .initializeMint(defaultReferralFee)
      .accounts({
        admin,
        config: configPDA,
        tokenMint: tokenMint.publicKey,
        treasury: admin, // Treasury defaults to admin initially
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([tokenMint])
      .rpc();

    console.log("Protocol initialized:", tx);
    return { tx, configPDA, tokenMint: tokenMint.publicKey };
  } catch (error) {
    console.error("Error initializing protocol:", error);
    throw error;
  }
}
```

### 2. Initialize Bonding Curve (Admin Only)

```typescript
async function initializeBondingCurve() {
  const { program, provider } = useYozoonProgram();
  const admin = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);
  const bondingCurvePDA = await findBondingCurvePDA(program.programId);

  // Define price points for the bonding curve
  // Example: Linear curve where price increases by 0.01 SOL for each 1,000,000 tokens
  const pricePoints = [
    { supplyPoint: new BN(0), priceLamports: new BN(10_000_000) }, // 0.01 SOL
    { supplyPoint: new BN(1_000_000), priceLamports: new BN(20_000_000) }, // 0.02 SOL
    { supplyPoint: new BN(2_000_000), priceLamports: new BN(30_000_000) }, // 0.03 SOL
    { supplyPoint: new BN(3_000_000), priceLamports: new BN(40_000_000) }, // 0.04 SOL
    { supplyPoint: new BN(4_000_000), priceLamports: new BN(50_000_000) }, // 0.05 SOL
  ];

  try {
    const tx = await program.methods
      .initializeBondingCurve(pricePoints)
      .accounts({
        admin,
        config: configPDA,
        bondingCurve: bondingCurvePDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Bonding curve initialized:", tx);
    return { tx, bondingCurvePDA };
  } catch (error) {
    console.error("Error initializing bonding curve:", error);
    throw error;
  }
}
```

### 3. Buy Tokens

```typescript
async function buyTokens(solAmount: number, referrerPubkey?: PublicKey) {
  const { program, provider } = useYozoonProgram();
  const user = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);
  const bondingCurvePDA = await findBondingCurvePDA(program.programId);

  // Get token mint from config
  const configAccount = await program.account.config.fetch(configPDA);
  const tokenMint = configAccount.tokenMint;

  // Get or create token account for user
  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    provider.connection,
    user,
    tokenMint,
    user,
    true
  );

  // Convert SOL amount to lamports
  const solAmountLamports = new BN(solAmount * LAMPORTS_PER_SOL);

  // Prepare accounts object
  const accounts = {
    user,
    config: configPDA,
    bondingCurve: bondingCurvePDA,
    tokenMint,
    tokenAccount: tokenAccount.address,
    systemProgram: SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    rent: SYSVAR_RENT_PUBKEY,
  };

  // Add referral accounts if provided
  if (referrerPubkey) {
    const referralPDA = await findReferralPDA(
      referrerPubkey,
      program.programId
    );
    accounts.referral = referralPDA;
    accounts.referrer = referrerPubkey;
  }

  try {
    const tx = await program.methods
      .buyTokens(solAmountLamports)
      .accounts(accounts)
      .rpc();

    console.log("Tokens purchased:", tx);
    return { tx };
  } catch (error) {
    console.error("Error buying tokens:", error);
    throw error;
  }
}
```

### 4. Sell Tokens

```typescript
async function sellTokens(tokenAmount: number) {
  const { program, provider } = useYozoonProgram();
  const user = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);
  const bondingCurvePDA = await findBondingCurvePDA(program.programId);

  // Get token mint from config
  const configAccount = await program.account.config.fetch(configPDA);
  const tokenMint = configAccount.tokenMint;

  // Get user's token account
  const tokenAccount = await getAssociatedTokenAddress(tokenMint, user);

  // Convert token amount to raw units
  const tokenAmountRaw = new BN(tokenAmount * Math.pow(10, 9)); // Assuming 9 decimals

  try {
    const tx = await program.methods
      .sellTokens(tokenAmountRaw)
      .accounts({
        user,
        config: configPDA,
        bondingCurve: bondingCurvePDA,
        tokenMint,
        tokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Tokens sold:", tx);
    return { tx };
  } catch (error) {
    console.error("Error selling tokens:", error);
    throw error;
  }
}
```

### 5. Create Referral

```typescript
async function createReferral() {
  const { program, provider } = useYozoonProgram();
  const user = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);
  const referralPDA = await findReferralPDA(user, program.programId);

  try {
    const tx = await program.methods
      .createReferral()
      .accounts({
        user,
        config: configPDA,
        referral: referralPDA,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("Referral created:", tx);
    return { tx, referralPDA };
  } catch (error) {
    console.error("Error creating referral:", error);
    throw error;
  }
}
```

## UI Integration Examples

### 1. Protocol Info Component

```tsx
import React, { useEffect, useState } from "react";
import { useYozoonProgram } from "../hooks/useYozoonProgram";
import { findConfigPDA, findBondingCurvePDA } from "../utils/pda";

export function ProtocolInfo() {
  const { program } = useYozoonProgram();
  const [config, setConfig] = useState(null);
  const [bondingCurve, setBondingCurve] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchProtocolInfo() {
      try {
        setLoading(true);

        // Find PDAs
        const configPDA = await findConfigPDA(program.programId);
        const bondingCurvePDA = await findBondingCurvePDA(program.programId);

        // Fetch accounts
        const configAccount = await program.account.config.fetch(configPDA);
        const bondingCurveAccount = await program.account.bondingCurve.fetch(
          bondingCurvePDA
        );

        setConfig(configAccount);
        setBondingCurve(bondingCurveAccount);
      } catch (error) {
        console.error("Error fetching protocol info:", error);
      } finally {
        setLoading(false);
      }
    }

    fetchProtocolInfo();
  }, [program]);

  if (loading) {
    return <div>Loading protocol info...</div>;
  }

  return (
    <div className="protocol-info">
      <h2>Protocol Info</h2>

      <div className="card">
        <h3>Configuration</h3>
        <p>
          <strong>Admin:</strong> {config.admin.toString()}
        </p>
        <p>
          <strong>Token Mint:</strong> {config.tokenMint.toString()}
        </p>
        <p>
          <strong>Default Referral Fee:</strong>{" "}
          {config.defaultReferralFee / 100}%
        </p>
        <p>
          <strong>Status:</strong> {config.isPaused ? "Paused" : "Active"}
        </p>
      </div>

      <div className="card">
        <h3>Bonding Curve</h3>
        <p>
          <strong>Total Sold Supply:</strong>{" "}
          {bondingCurve.totalSoldSupply.toString()}
        </p>
        <p>
          <strong>Total SOL Raised:</strong>{" "}
          {bondingCurve.totalSolRaised.toNumber() / LAMPORTS_PER_SOL} SOL
        </p>
        <p>
          <strong>Current Price:</strong>{" "}
          {bondingCurve.currentPrice.toNumber() / LAMPORTS_PER_SOL} SOL
        </p>
        <p>
          <strong>Migration Status:</strong>{" "}
          {bondingCurve.isMigrated ? "Migrated to Raydium" : "Not Migrated"}
        </p>

        <h4>Price Points</h4>
        <ul>
          {bondingCurve.pricePoints.map((point, index) => (
            <li key={index}>
              Supply: {point.supplyPoint.toString()} tokens - Price:{" "}
              {point.priceLamports.toNumber() / LAMPORTS_PER_SOL} SOL
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
```

### 2. Buy Tokens Component

```tsx
import React, { useState } from "react";
import { PublicKey } from "@solana/web3.js";
import { useYozoonProgram } from "../hooks/useYozoonProgram";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";

export function BuyTokens() {
  const { connected } = useWallet();
  const [solAmount, setSolAmount] = useState(1);
  const [referrer, setReferrer] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState("");

  const handleBuyTokens = async () => {
    setLoading(true);
    setError("");
    setSuccess("");

    try {
      // Validate referrer if provided
      let referrerPubkey = null;
      if (referrer) {
        try {
          referrerPubkey = new PublicKey(referrer);
        } catch (e) {
          setError("Invalid referrer address");
          setLoading(false);
          return;
        }
      }

      // Import locally to avoid dependency issues
      const { buyTokens } = await import("../utils/protocol");

      // Buy tokens
      const { tx } = await buyTokens(solAmount, referrerPubkey);

      setSuccess(`Successfully purchased tokens! Transaction: ${tx}`);
    } catch (err) {
      console.error(err);
      setError(`Failed to buy tokens: ${err.message}`);
    } finally {
      setLoading(false);
    }
  };

  if (!connected) {
    return (
      <div className="buy-tokens">
        <h2>Buy Tokens</h2>
        <p>Connect your wallet to buy tokens</p>
        <WalletMultiButton />
      </div>
    );
  }

  return (
    <div className="buy-tokens">
      <h2>Buy Tokens</h2>

      <div className="form-group">
        <label>SOL Amount:</label>
        <input
          type="number"
          min="0.1"
          step="0.1"
          value={solAmount}
          onChange={(e) => setSolAmount(parseFloat(e.target.value))}
        />
      </div>

      <div className="form-group">
        <label>Referrer (optional):</label>
        <input
          type="text"
          placeholder="Referrer wallet address"
          value={referrer}
          onChange={(e) => setReferrer(e.target.value)}
        />
      </div>

      <button
        className="buy-button"
        onClick={handleBuyTokens}
        disabled={loading}
      >
        {loading ? "Processing..." : `Buy Tokens for ${solAmount} SOL`}
      </button>

      {error && <div className="error">{error}</div>}
      {success && <div className="success">{success}</div>}
    </div>
  );
}
```

### 3. Price Chart Component

```tsx
import React, { useEffect, useState } from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
} from "recharts";
import { useYozoonProgram } from "../hooks/useYozoonProgram";
import { findBondingCurvePDA } from "../utils/pda";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

export function PriceChart() {
  const { program, connection } = useYozoonProgram();
  const [chartData, setChartData] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function generateChartData() {
      try {
        setLoading(true);

        // Find PDAs
        const bondingCurvePDA = await findBondingCurvePDA(program.programId);

        // Fetch bonding curve account
        const bondingCurveAccount = await program.account.bondingCurve.fetch(
          bondingCurvePDA
        );
        const pricePoints = bondingCurveAccount.pricePoints;

        // Generate chart data from price points
        const data = [];
        for (let i = 0; i < pricePoints.length - 1; i++) {
          const startSupply = pricePoints[i].supplyPoint.toNumber();
          const endSupply = pricePoints[i + 1].supplyPoint.toNumber();
          const startPrice =
            pricePoints[i].priceLamports.toNumber() / LAMPORTS_PER_SOL;
          const endPrice =
            pricePoints[i + 1].priceLamports.toNumber() / LAMPORTS_PER_SOL;

          // Generate 10 points between each price point for smoother curve
          const step = (endSupply - startSupply) / 10;
          const priceStep = (endPrice - startPrice) / 10;

          for (let j = 0; j <= 10; j++) {
            const supply = startSupply + step * j;
            const price = startPrice + priceStep * j;
            data.push({ supply, price });
          }
        }

        setChartData(data);
      } catch (error) {
        console.error("Error generating chart data:", error);
      } finally {
        setLoading(false);
      }
    }

    generateChartData();
  }, [program]);

  if (loading) {
    return <div>Loading price chart...</div>;
  }

  return (
    <div className="price-chart">
      <h2>Token Price Chart</h2>

      <LineChart width={600} height={300} data={chartData}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis
          dataKey="supply"
          label={{
            value: "Token Supply",
            position: "insideBottom",
            offset: -5,
          }}
        />
        <YAxis
          label={{ value: "Price (SOL)", angle: -90, position: "insideLeft" }}
        />
        <Tooltip formatter={(value) => [`${value} SOL`, "Price"]} />
        <Legend />
        <Line
          type="monotone"
          dataKey="price"
          stroke="#8884d8"
          activeDot={{ r: 8 }}
          name="Token Price"
        />
      </LineChart>
    </div>
  );
}
```

### 4. Referral Component

```tsx
import React, { useEffect, useState } from "react";
import { useYozoonProgram } from "../hooks/useYozoonProgram";
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { findReferralPDA } from "../utils/pda";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

export function ReferralDashboard() {
  const { connected, publicKey } = useWallet();
  const { program } = useYozoonProgram();
  const [referral, setReferral] = useState(null);
  const [loading, setLoading] = useState(true);
  const [referralLink, setReferralLink] = useState("");
  const [createReferralStatus, setCreateReferralStatus] = useState({
    loading: false,
    error: "",
    success: "",
  });

  useEffect(() => {
    async function fetchReferralInfo() {
      if (!connected || !publicKey) return;

      try {
        setLoading(true);

        // Find Referral PDA
        const referralPDA = await findReferralPDA(publicKey, program.programId);

        try {
          // Try to fetch the referral account
          const referralAccount = await program.account.referral.fetch(
            referralPDA
          );
          setReferral(referralAccount);

          // Generate referral link
          const baseUrl = window.location.origin;
          setReferralLink(`${baseUrl}?ref=${publicKey.toString()}`);
        } catch (err) {
          // Referral account doesn't exist yet
          setReferral(null);
        }
      } catch (error) {
        console.error("Error fetching referral info:", error);
      } finally {
        setLoading(false);
      }
    }

    fetchReferralInfo();
  }, [connected, publicKey, program]);

  const handleCreateReferral = async () => {
    setCreateReferralStatus({ loading: true, error: "", success: "" });

    try {
      // Import locally to avoid dependency issues
      const { createReferral } = await import("../utils/protocol");

      // Create referral
      const { tx } = await createReferral();

      setCreateReferralStatus({
        loading: false,
        error: "",
        success: `Referral created successfully! Transaction: ${tx}`,
      });

      // Refresh referral info
      const referralPDA = await findReferralPDA(publicKey, program.programId);
      const referralAccount = await program.account.referral.fetch(referralPDA);
      setReferral(referralAccount);

      // Generate referral link
      const baseUrl = window.location.origin;
      setReferralLink(`${baseUrl}?ref=${publicKey.toString()}`);
    } catch (err) {
      console.error(err);
      setCreateReferralStatus({
        loading: false,
        error: `Failed to create referral: ${err.message}`,
        success: "",
      });
    }
  };

  if (!connected) {
    return (
      <div className="referral-dashboard">
        <h2>Referral Dashboard</h2>
        <p>Connect your wallet to view and create referrals</p>
        <WalletMultiButton />
      </div>
    );
  }

  if (loading) {
    return <div className="referral-dashboard">Loading referral info...</div>;
  }

  return (
    <div className="referral-dashboard">
      <h2>Referral Dashboard</h2>

      {!referral ? (
        <div className="create-referral">
          <p>You don't have a referral link yet.</p>
          <button
            onClick={handleCreateReferral}
            disabled={createReferralStatus.loading}
          >
            {createReferralStatus.loading
              ? "Creating..."
              : "Create Referral Link"}
          </button>

          {createReferralStatus.error && (
            <div className="error">{createReferralStatus.error}</div>
          )}
          {createReferralStatus.success && (
            <div className="success">{createReferralStatus.success}</div>
          )}
        </div>
      ) : (
        <div className="referral-info">
          <div className="card">
            <h3>Your Referral Statistics</h3>
            <p>
              <strong>Referral Fee:</strong> {referral.referralFee / 100}%
            </p>
            <p>
              <strong>Total Referrals:</strong>{" "}
              {referral.totalReferrals.toString()}
            </p>
            <p>
              <strong>Total Rewards Earned:</strong>{" "}
              {referral.totalRewards.toNumber() / LAMPORTS_PER_SOL} SOL
            </p>
          </div>

          <div className="referral-link">
            <h3>Your Referral Link</h3>
            <div className="link-container">
              <input type="text" readOnly value={referralLink} />
              <button
                onClick={() => {
                  navigator.clipboard.writeText(referralLink);
                  alert("Referral link copied to clipboard!");
                }}
              >
                Copy
              </button>
            </div>
            <p className="hint">
              Share this link with others. When they buy tokens using your link,
              you'll earn {referral.referralFee / 100}% of their purchase
              amount!
            </p>
          </div>
        </div>
      )}
    </div>
  );
}
```

## Migration to Raydium (Admin Only)

```typescript
async function migrateToRaydium() {
  const { program, provider } = useYozoonProgram();
  const admin = provider.wallet.publicKey;

  // Find PDAs
  const configPDA = await findConfigPDA(program.programId);
  const bondingCurvePDA = await findBondingCurvePDA(program.programId);

  // Get token mint from config
  const configAccount = await program.account.config.fetch(configPDA);
  const tokenMint = configAccount.tokenMint;

  // Find Raydium pool PDAs - These would typically be calculated based on your specific setup
  // Note: In practice, these would be derived correctly based on the Raydium program
  const raydiumPoolPDA = await PublicKey.findProgramAddress(
    [Buffer.from("amm_pool"), tokenMint.toBuffer()],
    RAYDIUM_AMM_PROGRAM_ID
  );

  const raydiumPoolAuthorityPDA = await PublicKey.findProgramAddress(
    [Buffer.from("amm_authority")],
    RAYDIUM_AMM_PROGRAM_ID
  );

  const raydiumOpenOrdersPDA = await PublicKey.findProgramAddress(
    [Buffer.from("open_orders"), tokenMint.toBuffer()],
    RAYDIUM_AMM_PROGRAM_ID
  );

  // Create token accounts for pool legs
  const solLeg = await createAccount(
    provider.connection,
    admin,
    SystemProgram.programId,
    admin
  );

  const tokenLeg = await createAccount(
    provider.connection,
    admin,
    tokenMint,
    admin
  );

  // Create LP mint
  const lpMint = Keypair.generate();

  // Create fee account
  const feeAccount = Keypair.generate();

  try {
    const tx = await program.methods
      .migrateToRaydium()
      .accounts({
        admin,
        config: configPDA,
        bondingCurve: bondingCurvePDA,
        tokenMint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        raydiumAmmProgram: RAYDIUM_AMM_PROGRAM_ID,
        raydiumPool: raydiumPoolPDA[0],
        raydiumPoolAuthority: raydiumPoolAuthorityPDA[0],
        raydiumOpenOrders: raydiumOpenOrdersPDA[0],
        solLeg: solLeg.publicKey,
        tokenLeg: tokenLeg.publicKey,
        lpMint: lpMint.publicKey,
        feeAccount: feeAccount.publicKey,
      })
      .signers([lpMint, feeAccount])
      .rpc();

    console.log("Migrated to Raydium:", tx);
    return { tx, raydiumPoolPDA: raydiumPoolPDA[0] };
  } catch (error) {
    console.error("Error migrating to Raydium:", error);
    throw error;
  }
}
```

## Event Monitoring

```typescript
// Set up a listener for token purchase events
function setupTokenPurchaseEventListener(callback) {
  const { program } = useYozoonProgram();

  // Get program's event parser
  const eventParser = program.coder.events;

  // Set up a websocket connection
  const wsConnection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed"
  );

  // Listen for program logs
  const subscriptionId = wsConnection.onLogs(program.programId, (logs) => {
    if (!logs.logs || logs.logs.length === 0) return;

    try {
      // Try to parse the logs as an event
      const event = eventParser.parse(logs.logs[0]);

      // Check if it's a TokenPurchaseEvent
      if (event && event.name === "TokenPurchaseEvent") {
        // Call the callback with the parsed event
        callback(event.data);
      }
    } catch (error) {
      // Not all logs will be parseable events, so just ignore errors
    }
  });

  // Return a function to unsubscribe when needed
  return () => {
    wsConnection.removeOnLogsListener(subscriptionId);
  };
}

// Usage example in a React component
function TokenPurchaseFeed() {
  const [purchases, setPurchases] = useState([]);

  useEffect(() => {
    // Set up listener
    const unsubscribe = setupTokenPurchaseEventListener((eventData) => {
      setPurchases((prev) => [eventData, ...prev].slice(0, 10)); // Keep last 10 purchases
    });

    // Clean up on unmount
    return unsubscribe;
  }, []);

  return (
    <div className="purchase-feed">
      <h3>Recent Purchases</h3>
      {purchases.length === 0 ? (
        <p>No purchases yet</p>
      ) : (
        <ul>
          {purchases.map((purchase, index) => (
            <li key={index}>
              User {purchase.user.toString().substr(0, 6)}...
              {purchase.user.toString().substr(-4)}
              bought {purchase.tokenAmount.toNumber() / 1e9} tokens for{" "}
              {purchase.solAmount.toNumber() / LAMPORTS_PER_SOL} SOL
              {purchase.referrer &&
                ` (Referred by ${purchase.referrer
                  .toString()
                  .substr(0, 6)}...)`}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
```

## Best Practices and Error Handling

### 1. Handling Transaction Errors

Always wrap your transaction calls in try/catch blocks and provide meaningful error messages:

```typescript
try {
  // Transaction code here
} catch (error) {
  // Check for specific error codes
  if (error.message.includes("6001")) {
    return "The protocol is currently paused. Please try again later.";
  } else if (error.message.includes("6002")) {
    return "Insufficient SOL amount provided for this purchase.";
  } else if (error.message.includes("6007")) {
    return "Only the admin can perform this action.";
  } else {
    // Generic error handling
    console.error("Transaction error:", error);
    return `Error: ${error.message}`;
  }
}
```

### 2. Estimating Transaction Costs

To provide better UX, estimate the SOL cost of transactions:

```typescript
async function estimateTransactionCost(transaction) {
  const { connection } = useYozoonProgram();

  // Get the recent blockhash for fee calculation
  const { blockhash } = await connection.getLatestBlockhash();
  transaction.recentBlockhash = blockhash;

  // Get the fee for this transaction
  const fee = await connection.getFeeForMessage(transaction.compileMessage());

  return fee;
}
```

### 3. Transaction Confirmation and UI Feedback

Always provide good feedback during transactions:

```tsx
function TransactionButton({ onClick, children }) {
  const [status, setStatus] = useState({
    loading: false,
    error: null,
    success: false,
    signature: null,
  });

  const handleClick = async () => {
    setStatus({ loading: true, error: null, success: false, signature: null });

    try {
      const signature = await onClick();
      setStatus({
        loading: false,
        error: null,
        success: true,
        signature,
      });
    } catch (error) {
      setStatus({
        loading: false,
        error: error.message,
        success: false,
        signature: null,
      });
    }
  };

  return (
    <div>
      <button
        onClick={handleClick}
        disabled={status.loading}
        className={status.loading ? "loading" : ""}
      >
        {status.loading ? "Processing..." : children}
      </button>

      {status.error && <div className="error">{status.error}</div>}
      {status.success && (
        <div className="success">
          Transaction successful!
          <a
            href={`https://explorer.solana.com/tx/${status.signature}?cluster=devnet`}
            target="_blank"
            rel="noopener noreferrer"
          >
            View on Explorer
          </a>
        </div>
      )}
    </div>
  );
}
```

## Conclusion

This guide provides comprehensive examples for integrating the Yozoon protocol into your frontend applications. By following these patterns, you can build intuitive UIs for users to interact with the bonding curve, referral system, and other protocol features.

Remember to always validate user inputs, handle errors gracefully, and provide clear feedback throughout the transaction lifecycle. For production applications, consider implementing additional security measures and thorough testing.

For more details about the protocol's API, refer to the API Reference documentation.
