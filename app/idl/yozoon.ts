import { PublicKey } from '@solana/web3.js';
import { BN } from '@project-serum/anchor';

export type YozoonIDL = {
  version: '0.1.0';
  name: 'yozoon';
  instructions: [
    {
      name: 'initializeMint';
      accounts: [
        {
          name: 'admin';
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
          isSigner: false;
        },
        {
          name: 'treasury';
          isMut: false;
          isSigner: false;
          isOptional: true;
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
          type: 'u16';
        }
      ];
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
          name: 'user';
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
          name: 'tokenAccount';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'referral';
          isMut: true;
          isSigner: false;
          isOptional: true;
        },
        {
          name: 'referrer';
          isMut: true;
          isSigner: false;
          isOptional: true;
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
          name: 'associatedTokenProgram';
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
          name: 'solAmount';
          type: 'u64';
        }
      ];
    },
    {
      name: 'sellTokens';
      accounts: [
        {
          name: 'user';
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
          name: 'tokenAccount';
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
        }
      ];
    },
    {
      name: 'createReferral';
      accounts: [
        {
          name: 'user';
          isMut: true;
          isSigner: true;
        },
        {
          name: 'config';
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
      args: [];
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
          name: 'config';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'newAdmin';
          isMut: false;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: 'acceptAdmin';
      accounts: [
        {
          name: 'pendingAdmin';
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
          name: 'raydiumAmmProgram';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'raydiumPool';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'raydiumPoolAuthority';
          isMut: false;
          isSigner: false;
        },
        {
          name: 'raydiumOpenOrders';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'solLeg';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'tokenLeg';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'lpMint';
          isMut: true;
          isSigner: false;
        },
        {
          name: 'feeAccount';
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    }
  ];
  accounts: [
    {
      name: 'config';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'admin';
            type: 'publicKey';
          },
          {
            name: 'pendingAdmin';
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
            type: 'u16';
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
      name: 'bondingCurve';
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
            name: 'isMigrated';
            type: 'bool';
          },
          {
            name: 'currentPrice';
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
      name: 'referral';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'user';
            type: 'publicKey';
          },
          {
            name: 'referralFee';
            type: 'u16';
          },
          {
            name: 'totalReferrals';
            type: 'u64';
          },
          {
            name: 'totalRewards';
            type: 'u64';
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
            name: 'supplyPoint';
            type: 'u64';
          },
          {
            name: 'priceLamports';
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
          name: 'user';
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
          name: 'referrer';
          type: {
            option: 'publicKey';
          };
          index: false;
        },
        {
          name: 'referralAmount';
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
          name: 'user';
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
      name: 'PriceCalculatedEvent';
      fields: [
        {
          name: 'currentPrice';
          type: 'u64';
          index: false;
        },
        {
          name: 'totalSupply';
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
          name: 'poolAddress';
          type: 'publicKey';
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
          name: 'user';
          type: 'publicKey';
          index: false;
        },
        {
          name: 'referralFee';
          type: 'u16';
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
      name: 'InvalidPricePoints';
      msg: 'Price points are invalid (need at least 2)';
    },
    {
      code: 6001;
      name: 'ProtocolPaused';
      msg: 'The protocol is paused';
    },
    {
      code: 6002;
      name: 'InsufficientSolAmount';
      msg: 'Insufficient SOL amount provided';
    },
    {
      code: 6003;
      name: 'InsufficientTokenBalance';
      msg: 'Insufficient token balance for sale';
    },
    {
      code: 6004;
      name: 'NotEnoughTokensInReserve';
      msg: 'Not enough tokens in reserve for sale';
    },
    {
      code: 6005;
      name: 'AlreadyMigrated';
      msg: 'Bonding curve already migrated';
    },
    {
      code: 6006;
      name: 'MigrationThresholdNotReached';
      msg: 'Migration threshold not reached';
    },
    {
      code: 6007;
      name: 'AdminOnly';
      msg: 'Only admin can perform this action';
    },
    {
      code: 6008;
      name: 'InvalidFeePercentage';
      msg: 'Referral fee percentage is invalid';
    },
    {
      code: 6009;
      name: 'UnauthorizedSigner';
      msg: 'Unauthorized signer for this action';
    },
    {
      code: 6010;
      name: 'MigrationFailed';
      msg: 'Migration to Raydium failed';
    }
  ];
};

export const IDL: YozoonIDL = {
  version: '0.1.0',
  name: 'yozoon',
  instructions: [
    {
      name: 'initializeMint',
      accounts: [
        {
          name: 'admin',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenMint',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'treasury',
          isMut: false,
          isSigner: false,
          isOptional: true,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [
        {
          name: 'defaultReferralFee',
          type: 'u16',
        }
      ],
    },
    {
      name: 'initializeBondingCurve',
      accounts: [
        {
          name: 'admin',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'bondingCurve',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [
        {
          name: 'pricePoints',
          type: {
            vec: {
              defined: 'PricePoint',
            },
          },
        }
      ],
    },
    {
      name: 'buyTokens',
      accounts: [
        {
          name: 'user',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'bondingCurve',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenMint',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'referral',
          isMut: true,
          isSigner: false,
          isOptional: true,
        },
        {
          name: 'referrer',
          isMut: true,
          isSigner: false,
          isOptional: true,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'associatedTokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'rent',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [
        {
          name: 'solAmount',
          type: 'u64',
        }
      ],
    },
    {
      name: 'sellTokens',
      accounts: [
        {
          name: 'user',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'bondingCurve',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenMint',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenAccount',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [
        {
          name: 'tokenAmount',
          type: 'u64',
        }
      ],
    },
    {
      name: 'createReferral',
      accounts: [
        {
          name: 'user',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'referral',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'transferAdmin',
      accounts: [
        {
          name: 'admin',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'newAdmin',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'acceptAdmin',
      accounts: [
        {
          name: 'pendingAdmin',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'updateTreasury',
      accounts: [
        {
          name: 'admin',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'newTreasury',
          isMut: false,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'pauseProtocol',
      accounts: [
        {
          name: 'admin',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'unpauseProtocol',
      accounts: [
        {
          name: 'admin',
          isMut: false,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: true,
          isSigner: false,
        }
      ],
      args: [],
    },
    {
      name: 'migrateToRaydium',
      accounts: [
        {
          name: 'admin',
          isMut: true,
          isSigner: true,
        },
        {
          name: 'config',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'bondingCurve',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenMint',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'systemProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'tokenProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'raydiumAmmProgram',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'raydiumPool',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'raydiumPoolAuthority',
          isMut: false,
          isSigner: false,
        },
        {
          name: 'raydiumOpenOrders',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'solLeg',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'tokenLeg',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'lpMint',
          isMut: true,
          isSigner: false,
        },
        {
          name: 'feeAccount',
          isMut: true,
          isSigner: false,
        }
      ],
      args: [],
    }
  ],
  accounts: [
    {
      name: 'config',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'admin',
            type: 'publicKey',
          },
          {
            name: 'pendingAdmin',
            type: 'publicKey',
          },
          {
            name: 'tokenMint',
            type: 'publicKey',
          },
          {
            name: 'treasury',
            type: 'publicKey',
          },
          {
            name: 'defaultReferralFee',
            type: 'u16',
          },
          {
            name: 'isPaused',
            type: 'bool',
          },
          {
            name: 'bump',
            type: 'u8',
          }
        ],
      },
    },
    {
      name: 'bondingCurve',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'totalSoldSupply',
            type: 'u64',
          },
          {
            name: 'totalSolRaised',
            type: 'u64',
          },
          {
            name: 'pricePoints',
            type: {
              vec: {
                defined: 'PricePoint',
              },
            },
          },
          {
            name: 'isMigrated',
            type: 'bool',
          },
          {
            name: 'currentPrice',
            type: 'u64',
          },
          {
            name: 'bump',
            type: 'u8',
          }
        ],
      },
    },
    {
      name: 'referral',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'user',
            type: 'publicKey',
          },
          {
            name: 'referralFee',
            type: 'u16',
          },
          {
            name: 'totalReferrals',
            type: 'u64',
          },
          {
            name: 'totalRewards',
            type: 'u64',
          },
          {
            name: 'bump',
            type: 'u8',
          }
        ],
      },
    }
  ],
  types: [
    {
      name: 'PricePoint',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'supplyPoint',
            type: 'u64',
          },
          {
            name: 'priceLamports',
            type: 'u64',
          }
        ],
      },
    }
  ],
  events: [
    {
      name: 'TokenPurchaseEvent',
      fields: [
        {
          name: 'user',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'solAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'tokenAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'referrer',
          type: {
            option: 'publicKey',
          },
          index: false,
        },
        {
          name: 'referralAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'timestamp',
          type: 'i64',
          index: false,
        }
      ],
    },
    {
      name: 'TokenSaleEvent',
      fields: [
        {
          name: 'user',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'solAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'tokenAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'timestamp',
          type: 'i64',
          index: false,
        }
      ],
    },
    {
      name: 'PriceCalculatedEvent',
      fields: [
        {
          name: 'currentPrice',
          type: 'u64',
          index: false,
        },
        {
          name: 'totalSupply',
          type: 'u64',
          index: false,
        },
        {
          name: 'timestamp',
          type: 'i64',
          index: false,
        }
      ],
    },
    {
      name: 'MigrationCompletedEvent',
      fields: [
        {
          name: 'tokenAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'solAmount',
          type: 'u64',
          index: false,
        },
        {
          name: 'poolAddress',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'timestamp',
          type: 'i64',
          index: false,
        }
      ],
    },
    {
      name: 'ReferralCreatedEvent',
      fields: [
        {
          name: 'user',
          type: 'publicKey',
          index: false,
        },
        {
          name: 'referralFee',
          type: 'u16',
          index: false,
        },
        {
          name: 'timestamp',
          type: 'i64',
          index: false,
        }
      ],
    }
  ],
  errors: [
    {
      code: 6000,
      name: 'InvalidPricePoints',
      msg: 'Price points are invalid (need at least 2)',
    },
    {
      code: 6001,
      name: 'ProtocolPaused',
      msg: 'The protocol is paused',
    },
    {
      code: 6002,
      name: 'InsufficientSolAmount',
      msg: 'Insufficient SOL amount provided',
    },
    {
      code: 6003,
      name: 'InsufficientTokenBalance',
      msg: 'Insufficient token balance for sale',
    },
    {
      code: 6004,
      name: 'NotEnoughTokensInReserve',
      msg: 'Not enough tokens in reserve for sale',
    },
    {
      code: 6005,
      name: 'AlreadyMigrated',
      msg: 'Bonding curve already migrated',
    },
    {
      code: 6006,
      name: 'MigrationThresholdNotReached',
      msg: 'Migration threshold not reached',
    },
    {
      code: 6007,
      name: 'AdminOnly',
      msg: 'Only admin can perform this action',
    },
    {
      code: 6008,
      name: 'InvalidFeePercentage',
      msg: 'Referral fee percentage is invalid',
    },
    {
      code: 6009,
      name: 'UnauthorizedSigner',
      msg: 'Unauthorized signer for this action',
    },
    {
      code: 6010,
      name: 'MigrationFailed',
      msg: 'Migration to Raydium failed',
    }
  ],
};

// Helper type for PricePoint data
export type PricePoint = {
  supplyPoint: BN;
  priceLamports: BN;
};

// Helper types for account data
export type ConfigAccount = {
  admin: PublicKey;
  pendingAdmin: PublicKey;
  tokenMint: PublicKey;
  treasury: PublicKey;
  defaultReferralFee: number;
  isPaused: boolean;
  bump: number;
};

export type BondingCurveAccount = {
  totalSoldSupply: BN;
  totalSolRaised: BN;
  pricePoints: PricePoint[];
  isMigrated: boolean;
  currentPrice: BN;
  bump: number;
};

export type ReferralAccount = {
  user: PublicKey;
  referralFee: number;
  totalReferrals: BN;
  totalRewards: BN;
  bump: number;
};

// Helper types for events
export type TokenPurchaseEvent = {
  user: PublicKey;
  solAmount: BN;
  tokenAmount: BN;
  referrer: PublicKey | null;
  referralAmount: BN;
  timestamp: BN;
};

export type TokenSaleEvent = {
  user: PublicKey;
  solAmount: BN;
  tokenAmount: BN;
  timestamp: BN;
};

export type PriceCalculatedEvent = {
  currentPrice: BN;
  totalSupply: BN;
  timestamp: BN;
};

export type MigrationCompletedEvent = {
  tokenAmount: BN;
  solAmount: BN;
  poolAddress: PublicKey;
  timestamp: BN;
};

export type ReferralCreatedEvent = {
  user: PublicKey;
  referralFee: number;
  timestamp: BN;
}; 