# Solana Token Staking Program

A fully on-chain staking program built and tested with [Solana](https://solana.com), [Anchor framework](https://book.anchor-lang.com/). This program allows users to stake SPL tokens for a fixed duration and earn rewards over time.

---

## Features

- **Create and Mint**: Create SPL tokens and mint successfully
- **Stake**: Lock tokens for a defined time and earn rewards
- **Unstake**: Retrieve staked tokens after the staking period ends
- **Claim Reward**: Claim accumulated staking rewards
- **Reward Calculation**: Based on time and proportional stake amount

---

## Program Architecture

### Accounts

| Account           | Description                              |
|------------------|------------------------------------------|
| `AdminStake`     | Global staking configuration              |
| `UserStake`      | User-specific staking record              |
| `Vault`          | PDA-owned token account for staking       |
| `VaultAuthority` | PDA used to authorize token withdrawals   |

### Instructions

| Instruction        | Purpose                                   |
|--------------------|--------------------------------------------|
| `createTokenMint`  | Create an SPL token with metadata          |
| `mintToken`        | Mint tokens to a user's ATA                |
| `createStake`      | Initialize a stake configuration for token |
| `stakeToken`       | Stake a fixed amount for a lock duration   |
| `unstakeToken`     | Withdraw staked tokens (after lock ends)   |
| `claimReward`      | Claim accumulated rewards                  |

---

## Installation

### Prerequisites

- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Node.js](https://nodejs.org)
- [Yarn](https://classic.yarnpkg.com/en/docs/install/)
- [Anchor CLI](https://book.anchor-lang.com/chapter_2/anchor_installation.html)

```bash
# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
```

## Build and Deloy

```bash
anchor build
anchor deploy
anchor test
```

## Security Notes

- The vault and authority accounts are PDAs — users cannot transfer tokens arbitrarily.
- All token transfers are performed through CPIs with validated signer seeds.
- Reward calculations use fixed-point math via REWARD_PRECISION.

## Author

Luja Chitrakar
