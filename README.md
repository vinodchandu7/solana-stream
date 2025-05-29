# Solana Stream SDK

A collection of Rust and TypeScript packages for Solana stream data, operated by ValidatorsDAO. This repository is published as open-source software (OSS) and is freely available for anyone to use.

## Overview

This project provides libraries and tools for streaming real-time data from the Solana blockchain. It supports both Geyser and Shreds approaches, making it easier for developers to access Solana data streams.

## Package Structure

### Rust Clients

- **client/geyser-rs/**: Rust client using Geyser plugin
- **client/shreds-rs/**: Rust client using Shreds

### TypeScript Clients

- **client/geyser-ts/**: TypeScript client using Geyser plugin
- **client/shreds-ts/**: TypeScript client using Shreds

### SDK Packages

- **crate/solana-stream-sdk/**: Rust SDK for Solana stream functionality
- **package/solana-stream-sdk/**: TypeScript SDK for Solana stream functionality

## Getting Started

### Prerequisites

- Node.js (for TypeScript packages)
- Rust (for Rust packages)
- pnpm (for package management)

### Installation

For the entire workspace:

```bash
pnpm install
```

### Geyser Client Example - TypeScript

Set `client/geyser-ts/.env` with your X_TOKEN and other environment variables, then run:

```bash
pnpm -F client/geyser-ts dev
```

For specific packages, navigate to the package directory and install dependencies.

## Development

This project uses a monorepo structure with both Rust and TypeScript components:

- **Rust packages**: Managed with Cargo
- **TypeScript packages**: Managed with pnpm workspaces
- **Unified configuration**: Shared TypeScript and Prettier configurations

### Building

```bash
# Build all TypeScript packages
pnpm build

# Build Rust packages
cargo build
```

## Usage

Each package contains its own documentation and usage examples. Please refer to the individual package READMEs for specific implementation details.

## Contributing

We welcome contributions from the community! This project is continuously updated and improved.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

This project is open-source and available under the appropriate license terms. Please see individual package directories for specific license information.

## About ValidatorsDAO

This project is operated and maintained by ValidatorsDAO, focused on providing robust tools and infrastructure for the Solana ecosystem.

https://discord.gg/pw7kuJNDKq

## Updates

This repository is actively maintained and will receive continuous updates to improve functionality and add new features.
