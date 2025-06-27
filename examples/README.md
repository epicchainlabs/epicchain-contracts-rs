# EpicChain Rust Smart Contract Examples

This directory contains comprehensive, production-ready examples demonstrating the full capabilities of the EpicChain Rust smart contract development framework.

## 🎉 **Production Status: 100% Success Rate**

All 13 examples are fully functional and production-ready:
- ✅ **100% build success rate** - All examples compile successfully
- ✅ **Complete NEF generation** - All examples generate valid NEF files
- ✅ **Proper manifest generation** - All examples create correct manifest files
- ✅ **Standards compliance** - Full NEP-17, NEP-11, and NEP-24 support

## 📚 Example Categories

### 🟢 **Beginner Examples**
Learn the basics of EpicChain smart contract development with Rust.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [01-hello-world](./01-hello-world/) | Basic contract structure | Contract attributes, simple methods |
| [02-simple-storage](./02-simple-storage/) | Storage operations | StorageMap, data persistence |
| [03-counter](./03-counter/) | State management | Increment/decrement, witness checking |

### 🟡 **Token Examples**
Implement standard token contracts following NEP specifications.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [04-nep17-token](./04-nep17-token/) | Fungible token (NEP-17) | Transfer, mint, burn, allowances |
| [05-nep11-nft](./05-nep11-nft/) | Non-fungible token (NEP-11) | Unique tokens, metadata, enumeration |
| [06-nep24-royalty-nft](./06-nep24-royalty-nft/) | NFT with royalties (NEP-24) | Creator royalties, marketplace integration |

### 🟠 **DeFi Examples**
Decentralized finance applications and protocols.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [07-crowdfunding](./07-crowdfunding/) | Crowdfunding platform | Goal-based funding, refunds, milestones |
| [08-staking](./08-staking/) | Token staking contract | Rewards calculation, lock periods |
| [09-simple-dex](./09-simple-dex/) | Decentralized exchange | Token swaps, liquidity pools, AMM |

### 🔴 **Advanced Examples**
Complex contracts demonstrating advanced EpicChain features.

| Example | Description | Key Features |
|---------|-------------|--------------|
| [10-multisig-wallet](./10-multisig-wallet/) | Multi-signature wallet | M-of-N signatures, proposal system |
| [11-governance](./11-governance/) | Decentralized governance | Proposals, voting, execution |
| [12-oracle-price-feed](./12-oracle-price-feed/) | Oracle integration | External data, price feeds |
| [13-nft-marketplace](./13-nft-marketplace/) | NFT marketplace | Modular architecture, listings, auctions |

## 🚀 **Getting Started**

### Prerequisites

1. **Rust toolchain** with `wasm32-unknown-unknown` target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **epicchain-wasm compiler** (included in this repository):
   ```bash
   cd epicchain-wasm
   go build -o epicchain-wasm ./cmd
   ```

### Building Examples

Each example can be built using the provided Makefile:

```bash
cd examples/01-hello-world
make all
```

This will:
1. Compile Rust to WASM
2. Generate NEF file from WASM
3. Create contract manifest

Or manually with cargo:

```bash
export RUSTFLAGS="-Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152"
cargo build --target wasm32-unknown-unknown --release
```

### Testing Examples

Run the comprehensive test suite:

```bash
cargo test
```

## 📖 **Learning Path**

### 1. **Start with Basics** (Examples 01-03)
- Understand contract structure and attributes
- Learn storage operations and state management
- Practice witness checking and authorization

### 2. **Master Token Standards** (Examples 04-06)
- Implement NEP-17 fungible tokens
- Create NEP-11 non-fungible tokens
- Add royalty features with NEP-24

### 3. **Build DeFi Applications** (Examples 07-09)
- Create crowdfunding mechanisms
- Implement staking and rewards
- Build decentralized exchanges

### 4. **Advanced Patterns** (Examples 10-13)
- Multi-signature security patterns
- Governance and voting systems
- Oracle integration techniques
- Complex marketplace logic

## 🛠 **Development Tools**

### Makefile Commands

Each example includes a Makefile with these commands:

```bash
make wasm       # Compile Rust to WASM
make nef        # Generate NEF file from WASM
make manifest   # Generate contract manifest
make all        # Complete build pipeline
make clean      # Clean build artifacts
make test       # Run tests
```

### VS Code Integration

Recommended VS Code extensions:
- `rust-analyzer` - Rust language support
- `CodeLLDB` - Debugging support
- `Better TOML` - Cargo.toml syntax highlighting

## 📋 **Best Practices Demonstrated**

### Security
- ✅ Proper witness checking and authorization
- ✅ Input validation and sanitization
- ✅ Overflow protection in arithmetic operations
- ✅ Reentrancy protection patterns

### Performance
- ✅ Efficient storage patterns
- ✅ Gas optimization techniques
- ✅ Minimal memory allocations
- ✅ Optimized serialization

### Code Quality
- ✅ Comprehensive error handling
- ✅ Clear documentation and comments
- ✅ Modular and reusable code
- ✅ Extensive test coverage

## 🔗 **Additional Resources**

- [EpicChain Documentation](https://epic-chain.org/docs/getting-started/)
- [EpicChain Rust Framework Documentation](../docs/)

## 🤝 **Contributing**

Found an issue or want to improve an example? Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 **License**

All examples are licensed under the MIT License. See [LICENSE](../LICENSE) for details.
