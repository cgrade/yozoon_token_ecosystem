# Yozoon Token Ecosystem

A Solana-based token ecosystem featuring a bonding curve mechanism, referral system, and migration capabilities to Raydium.

## Features

- **Bonding Curve**: Dynamic token pricing based on supply
- **Referral System**: Incentivized user acquisition
- **Migration to Raydium**: Automatic liquidity migration when conditions are met
- **Admin Controls**: Secure admin management with two-step transfer process
- **Emergency Pause**: Protocol-wide pause functionality
- **Airdrop Capability**: Token distribution without affecting bonding curve
- **Pyth Oracle Integration**: Real-time price feeds for migration decisions

## Technical Specifications

### Dependencies

- Solana: 1.18.26
- Anchor: 0.29.0
- SPL Token: 4.0.0
- Pyth SDK: 0.10.0

### Program Structure

```
programs/
└── yozoon/
    ├── src/
    │   ├── lib.rs          # Program entry point
    │   ├── state/          # Account state definitions
    │   ├── instructions/   # Program instructions
    │   ├── errors/         # Custom error definitions
    │   ├── events/         # Program events
    │   └── utils/          # Utility functions
    └── Cargo.toml          # Program dependencies
```

## Getting Started

### Prerequisites

- Rust 1.79.0 or later
- Solana CLI 1.18.26
- Anchor 0.29.0

### Installation

1. Clone the repository:

```bash
git clone https://github.com/cgrade/yozoon_token_ecosystem.git
cd yozoon_token_ecosystem
```

2. Install dependencies:

```bash
cargo build
```

3. Build the program:

```bash
anchor build
```

## Usage

### Initialize Mint

```rust
pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()>
```

### Buy Tokens

```rust
pub fn buy_tokens(ctx: Context<BuyTokens>, sol_amount: u64) -> Result<()>
```

### Set Referral

```rust
pub fn set_referral(ctx: Context<SetReferral>, referrer: Pubkey) -> Result<()>
```

### Migrate to Raydium

```rust
pub fn migrate_to_raydium(ctx: Context<MigrateToRaydium>) -> Result<()>
```

## Constants

- `TOTAL_SUPPLY`: 1,000,000,000,000,000,000 tokens
- `PRECISION_FACTOR`: 1,000,000,000
- `DEFAULT_REFERRAL_FEE`: 1% (100 basis points)
- `MAX_REFERRAL_FEE`: 5% (500 basis points)
- `MIGRATION_USD_MIN`: $100,000
- `MIGRATION_USD_MAX`: $1,000,000
- `MIGRATION_SUPPLY_THRESHOLD`: 1,000,000,000 tokens

## Security

- Two-step admin transfer process
- Emergency pause functionality
- Referral fee limits
- Migration conditions verification
- Price feed staleness checks

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Solana Foundation
- Anchor Framework
- Pyth Network
- Raydium Protocol
