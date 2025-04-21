import { BN, AnchorProvider, Program } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';

export type YozoonIDL = {
  version: '0.1.0';
  name: 'yozoon';
  instructions: [
    {
      name: 'initializeMint';
      accounts: [
        {
          name: 'authority';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'treasury';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'defaultReferralFee';
          type: 'u64';
        }
      ];
    },
    {
      name: 'transferAdmin';
      accounts: [
        {
          name: 'admin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'newAdmin';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'updateTreasury';
      accounts: [
        {
          name: 'admin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'newTreasury';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'initializeBondingCurve';
      accounts: [
        {
          name: 'admin';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bondingCurve';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'pricePoints';
          type: {
            vec: {
              defined: 'PricePoint';
            };
          };
        }
      ];
    },
    {
      name: 'buyTokens';
      accounts: [
        {
          name: 'buyer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bondingCurve';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'buyerTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'treasury';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'solAmount';
          type: 'u64';
        },
        {
          name: 'minTokensExpected';
          type: 'u64';
        }
      ];
    },
    {
      name: 'buyTokensWithReferral';
      accounts: [
        {
          name: 'buyer';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bondingCurve';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'buyerTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'treasury';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'referral';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'referrer';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'solAmount';
          type: 'u64';
        },
        {
          name: 'minTokensExpected';
          type: 'u64';
        }
      ];
    },
    {
      name: 'sellTokens';
      accounts: [
        {
          name: 'seller';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bondingCurve';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'sellerTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'treasury';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'tokenAmount';
          type: 'u64';
        },
        {
          name: 'minSolExpected';
          type: 'u64';
        }
      ];
    },
    {
      name: 'pauseProtocol';
      accounts: [
        {
          name: 'admin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'unpauseProtocol';
      accounts: [
        {
          name: 'admin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'createReferral';
      accounts: [
        {
          name: 'authority';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'referrer';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'referral';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'referralFee';
          type: 'u64';
        }
      ];
    },
    {
      name: 'updateReferralFee';
      accounts: [
        {
          name: 'admin';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'referrer';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'referral';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'referralFee';
          type: 'u64';
        }
      ];
    },
    {
      name: 'createAirdrop';
      accounts: [
        {
          name: 'admin';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'recipient';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'airdrop';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: 'amount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'claimAirdrop';
      accounts: [
        {
          name: 'recipient';
          isMut: false;
          isSigner: true;
        },
        {
          name: 'recipientTokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'airdrop';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'migrateToRaydium';
      accounts: [
        {
          name: 'admin';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'bondingCurve';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'treasury';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'systemProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'raydiumAmm';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'rent';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    }
  ];
  accounts: [
    {
      name: 'Config';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'admin';
            type: 'publicKey';
          },
          {
            name: 'tokenMint';
            type: 'publicKey';
          },
          {
            name: 'treasury';
            type: 'publicKey';
          },
          {
            name: 'defaultReferralFee';
            type: 'u64';
          },
          {
            name: 'isPaused';
            type: 'bool';
          },
          {
            name: 'bump';
            type: 'u8';
          }
        ];
      };
    },
    {
      name: 'BondingCurve';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'totalSoldSupply';
            type: 'u64';
          },
          {
            name: 'totalSolRaised';
            type: 'u64';
          },
          {
            name: 'pricePoints';
            type: {
              vec: {
                defined: 'PricePoint';
              };
            };
          },
          {
            name: 'isMigratedToRaydium';
            type: 'bool';
          },
          {
            name: 'bump';
            type: 'u8';
          }
        ];
      };
    },
    {
      name: 'Referral';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'referrer';
            type: 'publicKey';
          },
          {
            name: 'referralFee';
            type: 'u64';
          },
          {
            name: 'totalEarned';
            type: 'u64';
          },
          {
            name: 'bump';
            type: 'u8';
          }
        ];
      };
    },
    {
      name: 'Airdrop';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'recipient';
            type: 'publicKey';
          },
          {
            name: 'amount';
            type: 'u64';
          },
          {
            name: 'claimed';
            type: 'bool';
          },
          {
            name: 'bump';
            type: 'u8';
          }
        ];
      };
    }
  ];
  types: [
    {
      name: 'PricePoint';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'supply';
            type: 'u64';
          },
          {
            name: 'pricePerToken';
            type: 'u64';
          }
        ];
      };
    }
  ];
  events: [
    {
      name: 'TokenPurchaseEvent';
      fields: [
        {
          name: 'buyer';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'solAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'tokenAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'TokenSaleEvent';
      fields: [
        {
          name: 'seller';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'tokenAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'solAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'PriceCalculatedEvent';
      fields: [
        {
          name: 'totalSupply';
          type: 'u64';
          index: false;
        },
        {
          name: 'price';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'ReferralCreatedEvent';
      fields: [
        {
          name: 'referrer';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'referralFee';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'ReferralPaymentEvent';
      fields: [
        {
          name: 'referrer';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'buyer';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'solAmount';
          type: 'u64';
          index: false;
        },
        {
          name: 'referralFee';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'MigrationCompletedEvent';
      fields: [
        {
          name: 'totalTokens';
          type: 'u64';
          index: false;
        },
        {
          name: 'totalSol';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    },
    {
      name: 'AirdropClaimedEvent';
      fields: [
        {
          name: 'recipient';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'amount';
          type: 'u64';
          index: false;
        },
        {
          name: 'timestamp';
          type: 'i64';
          index: false;
        }
      ];
    }
  ];
  errors: [
    {
      code: 6000;
      name: 'Unauthorized';
      msg: 'You are not authorized to perform this action';
    },
    {
      code: 6001;
      name: 'InsufficientSol';
      msg: 'Insufficient SOL for this transaction';
    },
    {
      code: 6002;
      name: 'InsufficientSupply';
      msg: 'Insufficient token supply for this transaction';
    },
    {
      code: 6003;
      name: 'ProtocolPaused';
      msg: 'Protocol is currently paused';
    },
    {
      code: 6004;
      name: 'InvalidReferralFee';
      msg: 'Referral fee exceeds maximum allowed';
    },
    {
      code: 6005;
      name: 'InvalidPricePoints';
      msg: 'Invalid price points configuration';
    },
    {
      code: 6006;
      name: 'SlippageExceeded';
      msg: 'Slippage tolerance exceeded';
    },
    {
      code: 6007;
      name: 'AlreadyMigrated';
      msg: 'Protocol already migrated to Raydium';
    },
    {
      code: 6008;
      name: 'InsufficientFundsForMigration';
      msg: 'Insufficient funds raised for migration';
    },
    {
      code: 6009;
      name: 'AirdropAlreadyClaimed';
      msg: 'Airdrop already claimed';
    }
  ];
};

export function getYozoonProgram(
  provider: AnchorProvider,
  programId: PublicKey
): Program<YozoonIDL> {
  return new Program<YozoonIDL>(
    // @ts-ignore: IDL matches the actual interface, it's just not recognized by TypeScript due to complex typing
    {}, 
    programId, 
    provider
  );
} 