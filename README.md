# NFT Manager

A Solana program for managing NFT collections with features like minting, fractionalizing, listing, and trading NFTs.

## Features

- ✨ Create and manage NFT collections
- 🖼️ Mint new NFTs with metadata
- 💎 Fractionalize NFTs into multiple parts
- 📜 List NFTs for sale
- 💰 Buy/sell NFTs with SOL
- 🔄 Update listing prices
- 🗑️ Delist NFTs from marketplace
- 🔥 Burn NFTs
- 👑 Collection authority management
- 💸 Fee collection system

## Prerequisites

- [Rust](https://rustup.rs/)
- [Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Yarn](https://yarnpkg.com/)

## Installation

```bash
git clone https://github.com/yourusername/nft-manager
cd nft-manager
yarn install
```

## Build

```bash
anchor build
```

## Test

```bash
anchor test
```

## Program Architecture
**Core Components**

- **NFT Manager**: Main program state controller
- **Minting**: Handles NFT minting
- **Fractionalizer**: Handles NFT fractionalization
- **MarketPlace**: Handles NFT listing and trading

## Instructions


## License
MIT