#!/usr/bin/env node

const { program } = require('commander');
const { Connection, PublicKey, Keypair, LAMPORTS_PER_SOL } = require('@solana/web3.js');
const anchor = require('@project-serum/anchor');
const fs = require('fs');
const path = require('path');
const readline = require('readline');

// Load the IDL file
const idlPath = path.join(__dirname, '../target/idl/yozoon.json');
let idl = {};
try {
  idl = JSON.parse(fs.readFileSync(idlPath, 'utf8'));
} catch (error) {
  console.error(`Error loading IDL file: ${error.message}`);
  console.error('Please ensure you have built the program and generated the IDL.');
  process.exit(1);
}

// Set up Anchor provider
const configureProvider = (keypairPath, rpcUrl) => {
  // Load the keypair from the provided path
  let keypair;
  try {
    const keypairData = JSON.parse(fs.readFileSync(keypairPath, 'utf8'));
    keypair = Keypair.fromSecretKey(Uint8Array.from(keypairData));
  } catch (error) {
    console.error(`Error loading keypair: ${error.message}`);
    process.exit(1);
  }

  // Connect to the specified RPC URL
  const connection = new Connection(rpcUrl, 'confirmed');
  
  // Create Anchor provider
  const provider = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(keypair),
    { commitment: 'confirmed' }
  );
  
  // Initialize the program with the IDL
  const programId = new PublicKey(idl.metadata.address);
  const program = new anchor.Program(idl, programId, provider);
  
  return { provider, program, keypair, connection };
};

// Interactive prompts for common operations
const promptForConfirmation = async (message) => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) => {
    rl.question(`${message} (y/N): `, (answer) => {
      rl.close();
      resolve(answer.toLowerCase() === 'y');
    });
  });
};

// Helper to display account information
const displayAccountInfo = (accountName, account) => {
  console.log(`\n-- ${accountName} Account Information --`);
  for (const [key, value] of Object.entries(account)) {
    if (value instanceof anchor.BN) {
      console.log(`${key}: ${value.toString()}`);
    } else if (value instanceof PublicKey) {
      console.log(`${key}: ${value.toString()}`);
    } else if (Array.isArray(value) && value.length > 0 && value[0] instanceof anchor.BN) {
      console.log(`${key}: [${value.map(v => v.toString()).join(', ')}]`);
    } else {
      console.log(`${key}: ${value}`);
    }
  }
};

// Define the CLI commands
program
  .name('yozoon-cli')
  .description('CLI utility for managing Yozoon protocol')
  .version('1.0.0');

// Global options
program
  .option('-k, --keypair <path>', 'Path to keypair file', '~/.config/solana/id.json')
  .option('-u, --url <url>', 'RPC URL', 'https://api.devnet.solana.com');

// Get current protocol status
program
  .command('status')
  .description('Get current status of the Yozoon protocol')
  .action(async () => {
    const { provider, program } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Derive Config PDA
      const [configPda] = await PublicKey.findProgramAddress(
        [Buffer.from('config')],
        program.programId
      );
      
      // Derive Bonding Curve PDA
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Fetch account data
      const config = await program.account.config.fetch(configPda);
      const bondingCurve = await program.account.bondingCurve.fetch(bondingCurvePda);
      
      // Display Config Info
      displayAccountInfo('Config', config);
      
      // Display Bonding Curve Info
      displayAccountInfo('Bonding Curve', bondingCurve);
      
      // Calculate and display migration progress
      const MIGRATION_SOL_MIN = 60000 * LAMPORTS_PER_SOL;
      const migrationProgress = (bondingCurve.totalSolRaised.toNumber() / MIGRATION_SOL_MIN) * 100;
      
      console.log('\n-- Migration Status --');
      console.log(`Total SOL Raised: ${bondingCurve.totalSolRaised.toNumber() / LAMPORTS_PER_SOL} SOL`);
      console.log(`Migration Threshold: ${MIGRATION_SOL_MIN / LAMPORTS_PER_SOL} SOL`);
      console.log(`Progress: ${migrationProgress.toFixed(2)}%`);
      console.log(`Is Migrated: ${bondingCurve.isMigrated}`);
      
      // Calculate current price
      const price = await program.methods
        .calculateCurrentPrice()
        .accounts({
          bondingCurve: bondingCurvePda,
        })
        .view();
      
      console.log(`\nCurrent Token Price: ${price.toNumber() / LAMPORTS_PER_SOL} SOL`);
      
    } catch (error) {
      console.error('Error fetching protocol status:', error);
    }
  });

// Buy tokens
program
  .command('buy')
  .description('Buy tokens from the bonding curve')
  .requiredOption('-a, --amount <number>', 'Amount of SOL to spend (in SOL)')
  .option('-r, --referrer <pubkey>', 'Referrer public key (optional)')
  .action(async (options) => {
    const { provider, program, keypair } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Convert SOL amount to lamports
      const amountLamports = parseFloat(options.amount) * LAMPORTS_PER_SOL;
      
      // Derive necessary PDAs
      const [configPda] = await PublicKey.findProgramAddress(
        [Buffer.from('config')],
        program.programId
      );
      
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Get config to find mint
      const config = await program.account.config.fetch(configPda);
      const mint = config.mint;
      
      // Find ATA for the user
      const { associatedTokenProgram, getAssociatedTokenAddress } = require('@solana/spl-token');
      const tokenAccount = await getAssociatedTokenAddress(
        mint,
        keypair.publicKey
      );
      
      // Calculate expected tokens
      const expectedTokens = await program.methods
        .calculateTokensForSol(new anchor.BN(amountLamports))
        .accounts({
          bondingCurve: bondingCurvePda,
        })
        .view();
      
      // Show the user what they're about to do
      console.log(`You are about to purchase tokens with ${options.amount} SOL`);
      console.log(`Expected tokens to receive: ${expectedTokens.toNumber() / (10 ** 9)}`);
      
      const confirmed = await promptForConfirmation('Proceed with purchase?');
      if (!confirmed) {
        console.log('Purchase cancelled by user.');
        return;
      }
      
      // Set up accounts for transaction
      let accounts = {
        config: configPda,
        bondingCurve: bondingCurvePda,
        mint: mint,
        buyer: keypair.publicKey,
        buyerTokenAccount: tokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: associatedTokenProgram,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      };
      
      // Add referrer if provided
      if (options.referrer) {
        const referrerPubkey = new PublicKey(options.referrer);
        const [referrerPda] = await PublicKey.findProgramAddress(
          [Buffer.from('referral'), referrerPubkey.toBuffer()],
          program.programId
        );
        
        const [userReferralPda] = await PublicKey.findProgramAddress(
          [Buffer.from('referral'), keypair.publicKey.toBuffer()],
          program.programId
        );
        
        accounts.referral = userReferralPda;
        accounts.referrer = referrerPda;
      }
      
      // Execute the buy transaction
      const tx = await program.methods
        .buyTokens(new anchor.BN(amountLamports))
        .accounts(accounts)
        .rpc();
      
      console.log(`Transaction successful! Signature: ${tx}`);
      console.log(`Tokens purchased: ${expectedTokens.toNumber() / (10 ** 9)}`);
      
    } catch (error) {
      console.error('Error buying tokens:', error);
    }
  });

// Sell tokens
program
  .command('sell')
  .description('Sell tokens back to the bonding curve')
  .requiredOption('-a, --amount <number>', 'Amount of tokens to sell (in token units)')
  .action(async (options) => {
    const { provider, program, keypair } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Convert token amount to raw units (assuming 9 decimals)
      const tokenAmount = new anchor.BN(parseFloat(options.amount) * (10 ** 9));
      
      // Derive necessary PDAs
      const [configPda] = await PublicKey.findProgramAddress(
        [Buffer.from('config')],
        program.programId
      );
      
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Get config to find mint
      const config = await program.account.config.fetch(configPda);
      const mint = config.mint;
      
      // Find ATA for the user
      const { getAssociatedTokenAddress } = require('@solana/spl-token');
      const tokenAccount = await getAssociatedTokenAddress(
        mint,
        keypair.publicKey
      );
      
      // Show the user what they're about to do
      console.log(`You are about to sell ${options.amount} tokens back to the bonding curve`);
      
      const confirmed = await promptForConfirmation('Proceed with sale?');
      if (!confirmed) {
        console.log('Sale cancelled by user.');
        return;
      }
      
      // Execute the sell transaction
      const tx = await program.methods
        .sellTokens(tokenAmount)
        .accounts({
          config: configPda,
          bondingCurve: bondingCurvePda,
          mint: mint,
          seller: keypair.publicKey,
          sellerTokenAccount: tokenAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        })
        .rpc();
      
      console.log(`Transaction successful! Signature: ${tx}`);
      console.log(`Tokens sold: ${options.amount}`);
      
    } catch (error) {
      console.error('Error selling tokens:', error);
    }
  });

// Set referral
program
  .command('set-referral')
  .description('Set a user as your referrer')
  .requiredOption('-r, --referrer <pubkey>', 'Referrer public key')
  .action(async (options) => {
    const { provider, program, keypair } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Parse referrer pubkey
      const referrerPubkey = new PublicKey(options.referrer);
      
      // Derive PDAs
      const [configPda] = await PublicKey.findProgramAddress(
        [Buffer.from('config')],
        program.programId
      );
      
      const [userReferralPda] = await PublicKey.findProgramAddress(
        [Buffer.from('referral'), keypair.publicKey.toBuffer()],
        program.programId
      );
      
      // Execute the set referral transaction
      const tx = await program.methods
        .setReferral(referrerPubkey)
        .accounts({
          config: configPda,
          user: keypair.publicKey,
          userReferral: userReferralPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      
      console.log(`Referral set successfully! Signature: ${tx}`);
      console.log(`Your referrer is now: ${referrerPubkey.toString()}`);
      
    } catch (error) {
      console.error('Error setting referral:', error);
    }
  });

// Check migration status
program
  .command('migration-status')
  .description('Check detailed migration status and eligibility')
  .action(async () => {
    const { provider, program } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Derive PDAs
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Fetch bonding curve data
      const bondingCurve = await program.account.bondingCurve.fetch(bondingCurvePda);
      
      // Migration thresholds
      const MIGRATION_SOL_MIN = 60000 * LAMPORTS_PER_SOL;
      const MIGRATION_SOL_MAX = 63000 * LAMPORTS_PER_SOL;
      
      // Calculate migration metrics
      const totalSolRaised = bondingCurve.totalSolRaised.toNumber();
      const migrationProgress = (totalSolRaised / MIGRATION_SOL_MIN) * 100;
      const isEligibleForMigration = totalSolRaised >= MIGRATION_SOL_MIN && totalSolRaised <= MIGRATION_SOL_MAX;
      const solNeededForMigration = MIGRATION_SOL_MIN - totalSolRaised;
      
      console.log('\n-- Detailed Migration Status --');
      console.log(`Total SOL Raised: ${totalSolRaised / LAMPORTS_PER_SOL} SOL`);
      console.log(`Migration Min Threshold: ${MIGRATION_SOL_MIN / LAMPORTS_PER_SOL} SOL`);
      console.log(`Migration Max Threshold: ${MIGRATION_SOL_MAX / LAMPORTS_PER_SOL} SOL`);
      console.log(`Progress: ${migrationProgress.toFixed(2)}%`);
      console.log(`Is Migrated: ${bondingCurve.isMigrated}`);
      
      if (bondingCurve.isMigrated) {
        console.log('\nMigration has already been completed!');
        
        // Try to find Raydium pool info
        const [raydiumPoolPda] = await PublicKey.findProgramAddress(
          [Buffer.from('raydium_pool'), bondingCurve.tokenMint.toBuffer()],
          program.programId
        );
        
        try {
          const raydiumPool = await program.account.raydiumPool.fetch(raydiumPoolPda);
          console.log('\n-- Raydium Pool Information --');
          console.log(`LP Mint: ${raydiumPool.lpMint.toString()}`);
          console.log(`Pool Creation Time: ${new Date(raydiumPool.creationTime.toNumber() * 1000)}`);
          console.log(`Trading now available on Raydium!`);
        } catch (e) {
          console.log('Could not fetch Raydium pool information.');
        }
        
      } else if (isEligibleForMigration) {
        console.log('\n✅ ELIGIBLE FOR MIGRATION!');
        console.log('The protocol has reached the migration threshold and can be migrated to Raydium.');
        console.log('Only the admin can execute the migration.');
      } else if (totalSolRaised < MIGRATION_SOL_MIN) {
        console.log(`\n❌ NOT YET ELIGIBLE FOR MIGRATION`);
        console.log(`Need ${solNeededForMigration / LAMPORTS_PER_SOL} more SOL to reach migration threshold.`);
      } else {
        console.log(`\n⚠️ MIGRATION WINDOW PASSED`);
        console.log(`The protocol has exceeded the migration window. Please contact the admin.`);
      }
      
    } catch (error) {
      console.error('Error checking migration status:', error);
    }
  });

// Execute migration (admin only)
program
  .command('migrate')
  .description('Execute migration to Raydium (admin only)')
  .action(async () => {
    const { provider, program, keypair } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Derive PDAs
      const [configPda] = await PublicKey.findProgramAddress(
        [Buffer.from('config')],
        program.programId
      );
      
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Fetch config and bonding curve
      const config = await program.account.config.fetch(configPda);
      const bondingCurve = await program.account.bondingCurve.fetch(bondingCurvePda);
      
      // Verify admin
      if (!config.admin.equals(keypair.publicKey)) {
        console.error('Error: Only the admin can execute migration.');
        return;
      }
      
      // Check if already migrated
      if (bondingCurve.isMigrated) {
        console.error('Error: Protocol has already been migrated to Raydium.');
        return;
      }
      
      // Check migration eligibility
      const MIGRATION_SOL_MIN = 60000 * LAMPORTS_PER_SOL;
      const MIGRATION_SOL_MAX = 63000 * LAMPORTS_PER_SOL;
      const totalSolRaised = bondingCurve.totalSolRaised.toNumber();
      
      if (totalSolRaised < MIGRATION_SOL_MIN || totalSolRaised > MIGRATION_SOL_MAX) {
        console.error('Error: Protocol is not eligible for migration.');
        console.log(`Current SOL raised: ${totalSolRaised / LAMPORTS_PER_SOL} SOL`);
        console.log(`Migration threshold: ${MIGRATION_SOL_MIN / LAMPORTS_PER_SOL} - ${MIGRATION_SOL_MAX / LAMPORTS_PER_SOL} SOL`);
        return;
      }
      
      // Derive Raydium pool PDA
      const [raydiumPoolPda] = await PublicKey.findProgramAddress(
        [Buffer.from('raydium_pool'), config.mint.toBuffer()],
        program.programId
      );
      
      // Derive Fee Key NFT PDA
      const [feeKeyNftPda] = await PublicKey.findProgramAddress(
        [Buffer.from('fee_key_nft'), config.mint.toBuffer()],
        program.programId
      );
      
      // Show warning about permanent migration
      console.log('⚠️ WARNING: MIGRATION IS PERMANENT ⚠️');
      console.log('Once executed, all token trading will move to Raydium and the bonding curve will be disabled.');
      console.log('All liquidity will be permanently locked on Raydium.');
      
      const confirmed = await promptForConfirmation('Are you absolutely sure you want to proceed with migration?');
      if (!confirmed) {
        console.log('Migration cancelled by user.');
        return;
      }
      
      // TODO: Complete implementation with actual Raydium accounts once details are finalized
      console.log('⚠️ Implementation note: This is a placeholder for the migration command.');
      console.log('To implement the actual migration, you need to include all Raydium-specific accounts.');
      console.log('Please refer to the frontend-integration-guide.md for full implementation details.');
      
    } catch (error) {
      console.error('Error executing migration:', error);
    }
  });

// Calculate tokens for given SOL
program
  .command('calculate')
  .description('Calculate tokens for a given SOL amount')
  .requiredOption('-a, --amount <number>', 'Amount of SOL (in SOL)')
  .action(async (options) => {
    const { program } = configureProvider(
      program.opts().keypair,
      program.opts().url
    );
    
    try {
      // Convert SOL amount to lamports
      const amountLamports = parseFloat(options.amount) * LAMPORTS_PER_SOL;
      
      // Derive bonding curve PDA
      const [bondingCurvePda] = await PublicKey.findProgramAddress(
        [Buffer.from('bonding_curve')],
        program.programId
      );
      
      // Calculate expected tokens
      const expectedTokens = await program.methods
        .calculateTokensForSol(new anchor.BN(amountLamports))
        .accounts({
          bondingCurve: bondingCurvePda,
        })
        .view();
      
      console.log(`\nCalculation Results:`);
      console.log(`SOL Amount: ${options.amount} SOL`);
      console.log(`Tokens to Receive: ${expectedTokens.toNumber() / (10 ** 9)} tokens`);
      console.log(`Rate: ${(expectedTokens.toNumber() / (10 ** 9)) / parseFloat(options.amount)} tokens per SOL`);
      
    } catch (error) {
      console.error('Error calculating tokens:', error);
    }
  });

// Parse arguments
program.parse(); 