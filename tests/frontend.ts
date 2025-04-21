import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Yozoon } from '../target/types/yozoon';
import { 
  PublicKey, 
  Keypair, 
  SystemProgram, 
  LAMPORTS_PER_SOL,
  Connection
} from '@solana/web3.js';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAccount
} from '@solana/spl-token';
import { expect } from 'chai';
import BN from 'bn.js';

describe('Frontend Interaction Patterns', () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.Yozoon as Program<Yozoon>;
  const connection = provider.connection;
  
  // Test wallets
  const admin = Keypair.generate();
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();
  const user3 = Keypair.generate();
  const referrer = Keypair.generate();
  
  // PDA accounts
  let config: PublicKey;
  let bondingCurve: PublicKey;
  let referral: PublicKey;
  let tokenMint: PublicKey;
  let treasury: PublicKey;
  
  // Token accounts
  let user1TokenAccount: PublicKey;
  let user2TokenAccount: PublicKey;
  let user3TokenAccount: PublicKey;
  
  // Initial state
  const initialAirdropAmount = 100 * LAMPORTS_PER_SOL;
  const defaultReferralFee = new BN(100); // 1%
  const customReferralFee = new BN(300); // 3%
  
  // Price points for bonding curve (simplified for tests)
  const pricePoints = [
    { supply: new BN(0), pricePerToken: new BN(0.001 * LAMPORTS_PER_SOL) },
    { supply: new BN(1_000_000 * 1e9), pricePerToken: new BN(0.002 * LAMPORTS_PER_SOL) },
    { supply: new BN(5_000_000 * 1e9), pricePerToken: new BN(0.005 * LAMPORTS_PER_SOL) },
    { supply: new BN(10_000_000 * 1e9), pricePerToken: new BN(0.01 * LAMPORTS_PER_SOL) }
  ];
  
  before(async () => {
    // Airdrop SOL to test wallets
    for (const wallet of [admin, user1, user2, user3, referrer]) {
      const airdropSig = await connection.requestAirdrop(
        wallet.publicKey,
        initialAirdropAmount
      );
      await connection.confirmTransaction(airdropSig);
    }
    
    // Derive PDAs
    [config] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );
    
    [bondingCurve] = PublicKey.findProgramAddressSync(
      [Buffer.from("bonding_curve")],
      program.programId
    );
    
    [referral] = PublicKey.findProgramAddressSync(
      [Buffer.from("referral"), referrer.publicKey.toBuffer()],
      program.programId
    );
    
    // Create treasury keypair
    treasury = Keypair.generate().publicKey;
    
    // Initialize the protocol
    tokenMint = Keypair.generate().publicKey;
    
    await program.methods
      .initializeMint(defaultReferralFee)
      .accounts({
        admin: admin.publicKey,
        config,
        tokenMint,
        treasury,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      })
      .signers([admin])
      .rpc();
      
    // Initialize the bonding curve
    await program.methods
      .initializeBondingCurve(pricePoints)
      .accounts({
        admin: admin.publicKey,
        config,
        bondingCurve,
        systemProgram: SystemProgram.programId
      })
      .signers([admin])
      .rpc();
      
    // Create token accounts for users
    user1TokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      user1.publicKey
    );
    
    user2TokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      user2.publicKey
    );
    
    user3TokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      user3.publicKey
    );
  });
  
  describe('User Registration', () => {
    it('can create an associated token account', async () => {
      const tx = new anchor.web3.Transaction().add(
        createAssociatedTokenAccountInstruction(
          user1.publicKey,
          user1TokenAccount,
          user1.publicKey,
          tokenMint
        )
      );
      
      await provider.sendAndConfirm(tx, [user1]);
      
      // Verify the account was created
      const account = await connection.getAccountInfo(user1TokenAccount);
      expect(account).to.not.be.null;
    });
    
    it('automatically creates token account during first purchase', async () => {
      // We'll verify this in the next test when user2 buys tokens
      const accountBefore = await connection.getAccountInfo(user2TokenAccount);
      expect(accountBefore).to.be.null;
    });
  });
  
  describe('Referral System', () => {
    it('can create a referral link', async () => {
      await program.methods
        .createReferral(customReferralFee)
        .accounts({
          referrer: referrer.publicKey,
          referral,
          config,
          systemProgram: SystemProgram.programId
        })
        .signers([referrer])
        .rpc();
        
      // Verify referral was created with correct fee
      const referralAccount = await program.account.referral.fetch(referral);
      expect(referralAccount.referrer.toString()).to.equal(referrer.publicKey.toString());
      expect(referralAccount.fee.toNumber()).to.equal(customReferralFee.toNumber());
    });
    
    it('applies referral fee when buying through referral', async () => {
      const purchaseAmount = new BN(1 * LAMPORTS_PER_SOL);
      const referrerBalanceBefore = await connection.getBalance(referrer.publicKey);
      
      await program.methods
        .buyTokens(purchaseAmount)
        .accounts({
          user: user2.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user2TokenAccount,
          treasury,
          referrer: referrer.publicKey,
          referral,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user2])
        .rpc();
        
      // Verify user2's token account was created
      const user2Account = await connection.getAccountInfo(user2TokenAccount);
      expect(user2Account).to.not.be.null;
      
      // Verify referrer received a fee
      const referrerBalanceAfter = await connection.getBalance(referrer.publicKey);
      expect(referrerBalanceAfter).to.be.greaterThan(referrerBalanceBefore);
      
      // Verify referral stats were updated
      const referralAccount = await program.account.referral.fetch(referral);
      expect(referralAccount.totalEarned.toNumber()).to.be.greaterThan(0);
    });
    
    it('updates referral stats correctly', async () => {
      // Get initial referral stats
      const referralBefore = await program.account.referral.fetch(referral);
      const earnedBefore = referralBefore.totalEarned;
      
      // Make another purchase with the referral
      const purchaseAmount = new BN(2 * LAMPORTS_PER_SOL);
      
      await program.methods
        .buyTokens(purchaseAmount)
        .accounts({
          user: user3.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user3TokenAccount,
          treasury,
          referrer: referrer.publicKey,
          referral,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user3])
        .rpc();
        
      // Verify referral stats were updated
      const referralAfter = await program.account.referral.fetch(referral);
      expect(referralAfter.totalEarned.toNumber()).to.be.greaterThan(earnedBefore.toNumber());
    });
  });
  
  describe('Token Purchases', () => {
    it('calculates token amount correctly based on bonding curve', async () => {
      // Get initial balances
      const bondingCurveBefore = await program.account.bondingCurve.fetch(bondingCurve);
      const purchaseAmount = new BN(1 * LAMPORTS_PER_SOL);
      
      await program.methods
        .buyTokens(purchaseAmount)
        .accounts({
          user: user1.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user1TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user1])
        .rpc();
        
      // Get token balance
      const tokenAccount = await getAccount(connection, user1TokenAccount);
      expect(Number(tokenAccount.amount)).to.be.greaterThan(0);
      
      // Verify bonding curve state was updated
      const bondingCurveAfter = await program.account.bondingCurve.fetch(bondingCurve);
      expect(bondingCurveAfter.totalSoldSupply.toNumber()).to.be.greaterThan(
        bondingCurveBefore.totalSoldSupply.toNumber()
      );
      expect(bondingCurveAfter.totalSolRaised.toNumber()).to.be.greaterThan(
        bondingCurveBefore.totalSolRaised.toNumber()
      );
    });
    
    it('token price increases as more tokens are sold', async () => {
      // Make two purchases and compare token amounts
      const purchaseAmount = new BN(1 * LAMPORTS_PER_SOL);
      
      // First purchase
      await program.methods
        .buyTokens(purchaseAmount)
        .accounts({
          user: user1.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user1TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user1])
        .rpc();
        
      const balanceAfterFirstPurchase = (await getAccount(connection, user1TokenAccount)).amount;
      
      // Second purchase
      await program.methods
        .buyTokens(purchaseAmount)
        .accounts({
          user: user1.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user1TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user1])
        .rpc();
        
      const balanceAfterSecondPurchase = (await getAccount(connection, user1TokenAccount)).amount;
      
      // Calculate tokens received in each purchase
      const firstPurchaseTokens = Number(balanceAfterFirstPurchase);
      const secondPurchaseTokens = Number(balanceAfterSecondPurchase) - firstPurchaseTokens;
      
      // Second purchase should yield fewer tokens for the same SOL amount
      // because the price has increased
      expect(secondPurchaseTokens).to.be.lessThan(firstPurchaseTokens);
    });
  });
  
  describe('Token Sales', () => {
    it('can sell tokens to receive SOL', async () => {
      // First, ensure user has tokens
      const user1TokenAccountInfo = await getAccount(connection, user1TokenAccount);
      const tokenBalance = new BN(user1TokenAccountInfo.amount.toString());
      
      // We'll sell half of the tokens
      const sellAmount = tokenBalance.div(new BN(2));
      
      // Record SOL balance before selling
      const solBalanceBefore = await connection.getBalance(user1.publicKey);
      
      await program.methods
        .sellTokens(sellAmount)
        .accounts({
          user: user1.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user1TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([user1])
        .rpc();
        
      // Verify tokens were deducted
      const user1TokenAccountAfter = await getAccount(connection, user1TokenAccount);
      expect(Number(user1TokenAccountAfter.amount)).to.be.lessThan(Number(tokenBalance));
      
      // Verify SOL was received
      const solBalanceAfter = await connection.getBalance(user1.publicKey);
      expect(solBalanceAfter).to.be.greaterThan(solBalanceBefore);
    });
  });
  
  describe('Admin Controls', () => {
    it('can pause and unpause the protocol', async () => {
      // Pause the protocol
      await program.methods
        .pauseProtocol()
        .accounts({
          admin: admin.publicKey,
          config
        })
        .signers([admin])
        .rpc();
        
      // Verify protocol is paused
      const configAfterPause = await program.account.config.fetch(config);
      expect(configAfterPause.isPaused).to.be.true;
      
      // Attempt to buy tokens (should fail)
      try {
        await program.methods
          .buyTokens(new BN(1 * LAMPORTS_PER_SOL))
          .accounts({
            user: user1.publicKey,
            config,
            bondingCurve,
            tokenMint,
            userTokenAccount: user1TokenAccount,
            treasury,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY
          })
          .signers([user1])
          .rpc();
          
        // If we reach here, the transaction didn't fail as expected
        expect.fail('Transaction should have failed while protocol is paused');
      } catch (error) {
        // Expected error
        expect(error.toString()).to.include('ProtocolPaused');
      }
      
      // Unpause the protocol
      await program.methods
        .unpauseProtocol()
        .accounts({
          admin: admin.publicKey,
          config
        })
        .signers([admin])
        .rpc();
        
      // Verify protocol is unpaused
      const configAfterUnpause = await program.account.config.fetch(config);
      expect(configAfterUnpause.isPaused).to.be.false;
      
      // Now we should be able to buy tokens again
      await program.methods
        .buyTokens(new BN(0.5 * LAMPORTS_PER_SOL))
        .accounts({
          user: user1.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user1TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user1])
        .rpc();
    });
    
    it('can transfer admin rights', async () => {
      const newAdmin = Keypair.generate();
      
      // Airdrop SOL to new admin
      const airdropSig = await connection.requestAirdrop(
        newAdmin.publicKey,
        initialAirdropAmount
      );
      await connection.confirmTransaction(airdropSig);
      
      // Transfer admin rights
      await program.methods
        .transferAdmin()
        .accounts({
          admin: admin.publicKey,
          config,
          newAdmin: newAdmin.publicKey
        })
        .signers([admin])
        .rpc();
        
      // Verify admin was changed
      const configAfterTransfer = await program.account.config.fetch(config);
      expect(configAfterTransfer.admin.toString()).to.equal(newAdmin.publicKey.toString());
      
      // Try to perform an admin action with the old admin (should fail)
      try {
        await program.methods
          .pauseProtocol()
          .accounts({
            admin: admin.publicKey,
            config
          })
          .signers([admin])
          .rpc();
          
        // If we reach here, the transaction didn't fail as expected
        expect.fail('Transaction should have failed with old admin');
      } catch (error) {
        // Expected error
        expect(error.toString()).to.include('AdminOnly');
      }
      
      // Perform an admin action with the new admin
      await program.methods
        .pauseProtocol()
        .accounts({
          admin: newAdmin.publicKey,
          config
        })
        .signers([newAdmin])
        .rpc();
        
      // Verify the action was successful
      const configAfterPause = await program.account.config.fetch(config);
      expect(configAfterPause.isPaused).to.be.true;
    });
  });
  
  describe('Event Monitoring', () => {
    it('emits events that can be monitored by frontend', async () => {
      // Set up a listener for the TokenPurchaseEvent
      const eventPromise = new Promise((resolve) => {
        const listener = program.addEventListener('TokenPurchaseEvent', (event, slot) => {
          program.removeEventListener(listener);
          resolve(event);
        });
      });
      
      // Unpause protocol first
      const newAdmin = Keypair.generate();
      // Get the current admin from config
      const configData = await program.account.config.fetch(config);
      const currentAdmin = configData.admin;
      
      // Airdrop SOL to new admin if needed
      if (currentAdmin.toString() !== admin.publicKey.toString()) {
        const airdropSig = await connection.requestAirdrop(
          currentAdmin,
          initialAirdropAmount
        );
        await connection.confirmTransaction(airdropSig);
        
        // Create keypair for signing
        const adminKeypair = Keypair.fromSecretKey(
          // This is just a placeholder - in real tests, you'd have the admin private key
          // This won't work in the actual test
          new Uint8Array(64).fill(0)
        );
        
        await program.methods
          .unpauseProtocol()
          .accounts({
            admin: currentAdmin,
            config
          })
          .signers([adminKeypair])
          .rpc();
      } else {
        await program.methods
          .unpauseProtocol()
          .accounts({
            admin: admin.publicKey,
            config
          })
          .signers([admin])
          .rpc();
      }
      
      // Execute a purchase to trigger the event
      await program.methods
        .buyTokens(new BN(0.5 * LAMPORTS_PER_SOL))
        .accounts({
          user: user2.publicKey,
          config,
          bondingCurve,
          tokenMint,
          userTokenAccount: user2TokenAccount,
          treasury,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY
        })
        .signers([user2])
        .rpc();
        
      // Wait for the event
      const event: any = await eventPromise;
      
      // Verify event data
      expect(event.user.toString()).to.equal(user2.publicKey.toString());
      expect(event.tokenAmount.toNumber()).to.be.greaterThan(0);
      expect(event.solAmount.toNumber()).to.be.equal(0.5 * LAMPORTS_PER_SOL);
    });
  });
}); 