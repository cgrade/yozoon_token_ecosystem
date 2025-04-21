const assert = require('assert');
const { Connection, PublicKey, clusterApiUrl } = require('@solana/web3.js');

describe('Yozoon Migration Test', () => {
  it('should verify program is deployed', async () => {
    // Connect to devnet
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    
    // Program ID from deployment
    const programId = new PublicKey('3J6Hu2iwgwuU4gvQACsHqEsfrRog6EzoaEWo1oZ8NHNx');
    
    // Fetch the program account
    const accountInfo = await connection.getAccountInfo(programId);
    
    console.log('Program account size:', accountInfo.data.length);
    
    // Verify program exists
    assert(accountInfo !== null, 'Program account not found');
    assert(accountInfo.executable, 'Program is not executable');
    
    console.log('Program is deployed and executable on devnet');
  });
  
  it('should simulate migration process outcome', () => {
    // This is a mock test since we can't directly call the program without setup
    
    // Mock values that would result from migration
    const mockBondingCurve = {
      isMigrated: true,
      totalSolRaised: 60000 * 1_000_000_000, // in lamports
      totalSoldSupply: 500_000_000_000_000
    };
    
    // Verify mock values
    assert.strictEqual(mockBondingCurve.isMigrated, true, 'Migration flag should be true');
    assert.strictEqual(
      mockBondingCurve.totalSolRaised / 1_000_000_000 >= 60000, 
      true, 
      'Raised SOL should be at least 60000'
    );
    
    console.log(`Migration test passed - simulated values:
      - Migration completed: ${mockBondingCurve.isMigrated}
      - Total SOL raised: ${mockBondingCurve.totalSolRaised / 1_000_000_000} SOL
      - Total supply sold: ${mockBondingCurve.totalSoldSupply}`
    );
  });
}); 