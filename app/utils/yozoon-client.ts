import { 
  Program, 
  AnchorProvider, 
  BN, 
  web3, 
  EventParser 
} from '@project-serum/anchor';
import { 
  PublicKey, 
  Transaction, 
  SystemProgram, 
  TransactionInstruction, 
  Connection 
} from '@solana/web3.js';
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddress 
} from '@solana/spl-token';
import { YozoonIDL } from '../idl/types';
import { getYozoonProgram } from '../idl/types';

export interface PricePoint {
  supply: BN;
  pricePerToken: BN;
}

export class YozoonClient {
  private program: Program<YozoonIDL>;
  private provider: AnchorProvider;

  constructor(provider: AnchorProvider, programId: PublicKey) {
    this.provider = provider;
    this.program = getYozoonProgram(provider, programId);
  }

  // --- PDA DERIVATION METHODS ---

  /**
   * Get the config address for the protocol
   */
  async getConfigAddress(): Promise<PublicKey> {
    const [configAddress] = await PublicKey.findProgramAddress(
      [Buffer.from('config')],
      this.program.programId
    );
    return configAddress;
  }

  /**
   * Get the bonding curve address
   */
  async getBondingCurveAddress(): Promise<PublicKey> {
    const [bondingCurveAddress] = await PublicKey.findProgramAddress(
      [Buffer.from('bonding_curve')],
      this.program.programId
    );
    return bondingCurveAddress;
  }

  /**
   * Get the referral address for a specific referrer
   */
  async getReferralAddress(referrer: PublicKey): Promise<PublicKey> {
    const [referralAddress] = await PublicKey.findProgramAddress(
      [Buffer.from('referral'), referrer.toBuffer()],
      this.program.programId
    );
    return referralAddress;
  }

  /**
   * Get the airdrop address for a specific recipient
   */
  async getAirdropAddress(recipient: PublicKey): Promise<PublicKey> {
    const [airdropAddress] = await PublicKey.findProgramAddress(
      [Buffer.from('airdrop'), recipient.toBuffer()],
      this.program.programId
    );
    return airdropAddress;
  }

  /**
   * Get the token mint address
   */
  async getTokenMintAddress(): Promise<PublicKey> {
    const configAddress = await this.getConfigAddress();
    const configData = await this.fetchConfigData(configAddress);
    return configData.tokenMint;
  }

  // --- DATA FETCHING METHODS ---

  /**
   * Fetch config account data
   */
  async fetchConfigData(configAddress: PublicKey) {
    return await this.program.account.config.fetch(configAddress);
  }

  /**
   * Fetch bonding curve account data
   */
  async fetchBondingCurveData(bondingCurveAddress: PublicKey) {
    return await this.program.account.bondingCurve.fetch(bondingCurveAddress);
  }

  /**
   * Fetch referral account data
   */
  async fetchReferralData(referralAddress: PublicKey) {
    return await this.program.account.referral.fetch(referralAddress);
  }

  /**
   * Fetch airdrop account data
   */
  async fetchAirdropData(airdropAddress: PublicKey) {
    return await this.program.account.airdrop.fetch(airdropAddress);
  }

  /**
   * Check if the protocol has been migrated to Raydium
   */
  async isMigratedToRaydium(bondingCurveAddress: PublicKey): Promise<boolean> {
    const bondingCurveData = await this.fetchBondingCurveData(bondingCurveAddress);
    return bondingCurveData.isMigrated;
  }

  /**
   * Get the current token price
   */
  async getCurrentTokenPrice(bondingCurveAddress: PublicKey): Promise<BN> {
    const bondingCurveData = await this.fetchBondingCurveData(bondingCurveAddress);
    
    // Find the current price point based on total sold supply
    const totalSoldSupply = bondingCurveData.totalSoldSupply;
    const pricePoints = bondingCurveData.pricePoints;
    
    let currentPrice = pricePoints[0].pricePerToken;
    
    for (let i = 0; i < pricePoints.length; i++) {
      if (totalSoldSupply.gte(pricePoints[i].supply)) {
        currentPrice = pricePoints[i].pricePerToken;
      } else {
        break;
      }
    }
    
    return currentPrice;
  }

  /**
   * Calculate the number of tokens a user would receive for a given SOL amount
   */
  async calculateTokensForSol(
    bondingCurveAddress: PublicKey,
    solAmount: BN
  ): Promise<BN> {
    const bondingCurveData = await this.fetchBondingCurveData(bondingCurveAddress);
    const currentPrice = await this.getCurrentTokenPrice(bondingCurveAddress);
    
    // This is a simplified calculation - the actual protocol uses a more precise approach
    // that accounts for crossing price points
    return solAmount.mul(new BN(1_000_000_000)).div(currentPrice);
  }

  /**
   * Calculate the price at a specific supply level
   */
  async calculatePriceAtSupply(
    bondingCurveAddress: PublicKey,
    supply: BN
  ): Promise<BN> {
    const bondingCurveData = await this.fetchBondingCurveData(bondingCurveAddress);
    const pricePoints = bondingCurveData.pricePoints;
    
    let price = pricePoints[0].pricePerToken;
    
    for (let i = 0; i < pricePoints.length; i++) {
      if (supply.gte(pricePoints[i].supply)) {
        price = pricePoints[i].pricePerToken;
      } else {
        break;
      }
    }
    
    return price;
  }

  // --- INSTRUCTION BUILDING METHODS ---

  /**
   * Build instruction to initialize the mint (admin only)
   */
  async buildInitializeMintInstruction(
    admin: PublicKey,
    defaultReferralFee: BN
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    
    return this.program.methods
      .initializeMint(defaultReferralFee)
      .accounts({
        config: configAddress,
        admin: admin,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  /**
   * Build instruction to initialize the bonding curve (admin only)
   */
  async buildInitializeBondingCurveInstruction(
    admin: PublicKey,
    pricePoints: PricePoint[]
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const bondingCurveAddress = await this.getBondingCurveAddress();
    const tokenMint = await this.getTokenMintAddress();
    
    return this.program.methods
      .initializeBondingCurve(pricePoints)
      .accounts({
        config: configAddress,
        bondingCurve: bondingCurveAddress,
        tokenMint: tokenMint,
        admin: admin,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  /**
   * Build instruction to buy tokens
   */
  async buildBuyTokensInstruction(
    buyer: PublicKey,
    solAmount: BN,
    minTokensExpected: BN
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const bondingCurveAddress = await this.getBondingCurveAddress();
    const tokenMint = await this.getTokenMintAddress();
    
    const buyerTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      buyer,
      false
    );
    
    return this.program.methods
      .buyTokens(solAmount, minTokensExpected)
      .accounts({
        config: configAddress,
        bondingCurve: bondingCurveAddress,
        tokenMint: tokenMint,
        buyer: buyer,
        buyerTokenAccount: buyerTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .instruction();
  }

  /**
   * Build instruction to buy tokens with a referral
   */
  async buildBuyTokensWithReferralInstruction(
    buyer: PublicKey,
    solAmount: BN,
    minTokensExpected: BN,
    referrer: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const bondingCurveAddress = await this.getBondingCurveAddress();
    const tokenMint = await this.getTokenMintAddress();
    const referralAddress = await this.getReferralAddress(referrer);
    
    const buyerTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      buyer,
      false
    );
    
    return this.program.methods
      .buyTokensWithReferral(solAmount, minTokensExpected)
      .accounts({
        config: configAddress,
        bondingCurve: bondingCurveAddress,
        tokenMint: tokenMint,
        buyer: buyer,
        buyerTokenAccount: buyerTokenAccount,
        referral: referralAddress,
        referrer: referrer,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .instruction();
  }

  /**
   * Build instruction to sell tokens
   */
  async buildSellTokensInstruction(
    seller: PublicKey,
    tokenAmount: BN,
    minSolExpected: BN
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const bondingCurveAddress = await this.getBondingCurveAddress();
    const tokenMint = await this.getTokenMintAddress();
    
    const sellerTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      seller,
      false
    );
    
    return this.program.methods
      .sellTokens(tokenAmount, minSolExpected)
      .accounts({
        config: configAddress,
        bondingCurve: bondingCurveAddress,
        tokenMint: tokenMint,
        seller: seller,
        sellerTokenAccount: sellerTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();
  }

  /**
   * Build instruction to create a referral
   */
  async buildCreateReferralInstruction(
    payer: PublicKey,
    referrer: PublicKey,
    referralFee: BN
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const referralAddress = await this.getReferralAddress(referrer);
    
    return this.program.methods
      .createReferral(referralFee)
      .accounts({
        config: configAddress,
        referral: referralAddress,
        referrer: referrer,
        payer: payer,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  /**
   * Build instruction to create an airdrop (admin only)
   */
  async buildCreateAirdropInstruction(
    admin: PublicKey,
    recipient: PublicKey,
    amount: BN
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const airdropAddress = await this.getAirdropAddress(recipient);
    const tokenMint = await this.getTokenMintAddress();
    
    return this.program.methods
      .createAirdrop(amount)
      .accounts({
        config: configAddress,
        airdrop: airdropAddress,
        tokenMint: tokenMint,
        admin: admin,
        recipient: recipient,
        systemProgram: SystemProgram.programId,
      })
      .instruction();
  }

  /**
   * Build instruction to claim an airdrop
   */
  async buildClaimAirdropInstruction(
    recipient: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const airdropAddress = await this.getAirdropAddress(recipient);
    const tokenMint = await this.getTokenMintAddress();
    
    const recipientTokenAccount = await getAssociatedTokenAddress(
      tokenMint,
      recipient,
      false
    );
    
    return this.program.methods
      .claimAirdrop()
      .accounts({
        config: configAddress,
        airdrop: airdropAddress,
        tokenMint: tokenMint,
        recipient: recipient,
        recipientTokenAccount: recipientTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .instruction();
  }

  /**
   * Build instruction to pause the protocol (admin only)
   */
  async buildPauseProtocolInstruction(
    admin: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    
    return this.program.methods
      .pauseProtocol()
      .accounts({
        config: configAddress,
        admin: admin,
      })
      .instruction();
  }

  /**
   * Build instruction to unpause the protocol (admin only)
   */
  async buildUnpauseProtocolInstruction(
    admin: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    
    return this.program.methods
      .unpauseProtocol()
      .accounts({
        config: configAddress,
        admin: admin,
      })
      .instruction();
  }

  /**
   * Build instruction to transfer admin (admin only)
   */
  async buildTransferAdminInstruction(
    admin: PublicKey,
    newAdmin: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    
    return this.program.methods
      .transferAdmin()
      .accounts({
        config: configAddress,
        admin: admin,
        newAdmin: newAdmin,
      })
      .instruction();
  }

  /**
   * Build instruction to update treasury (admin only)
   */
  async buildUpdateTreasuryInstruction(
    admin: PublicKey,
    newTreasury: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    
    return this.program.methods
      .updateTreasury()
      .accounts({
        config: configAddress,
        admin: admin,
        newTreasury: newTreasury,
      })
      .instruction();
  }

  /**
   * Build instruction to migrate to Raydium (admin only)
   */
  async buildMigrateToRaydiumInstruction(
    admin: PublicKey
  ): Promise<TransactionInstruction> {
    const configAddress = await this.getConfigAddress();
    const bondingCurveAddress = await this.getBondingCurveAddress();
    const tokenMint = await this.getTokenMintAddress();
    
    // Note: In a real implementation, you would need to include all the Raydium accounts
    // This is a simplified version
    return this.program.methods
      .migrateToRaydium()
      .accounts({
        config: configAddress,
        bondingCurve: bondingCurveAddress,
        tokenMint: tokenMint,
        admin: admin,
        // ... other Raydium accounts would be specified here
      })
      .instruction();
  }

  // --- EVENT SUBSCRIPTION METHODS ---

  /**
   * Subscribe to token purchase events
   */
  subscribeToTokenPurchaseEvents(callback: (event: any) => void) {
    const eventParser = new EventParser(this.program.programId, this.program.coder);
    
    const listener = this.provider.connection.onLogs(
      this.program.programId,
      (logs) => {
        if (logs.err) return;
        
        const events = eventParser.parseLogs(logs.logs);
        for (const event of events) {
          if (event.name === 'TokenPurchaseEvent') {
            callback(event.data);
          }
        }
      }
    );
    
    return listener;
  }

  /**
   * Subscribe to price calculation events
   */
  subscribeToPriceCalculatedEvents(callback: (event: any) => void) {
    const eventParser = new EventParser(this.program.programId, this.program.coder);
    
    const listener = this.provider.connection.onLogs(
      this.program.programId,
      (logs) => {
        if (logs.err) return;
        
        const events = eventParser.parseLogs(logs.logs);
        for (const event of events) {
          if (event.name === 'PriceCalculatedEvent') {
            callback(event.data);
          }
        }
      }
    );
    
    return listener;
  }

  /**
   * Generic unsubscribe method for event listeners
   */
  async unsubscribe(listener: number) {
    await this.provider.connection.removeOnLogsListener(listener);
  }
} 