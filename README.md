# PromptHub Protocol

A decentralized protocol for AI prompt asset management on Solana blockchain.

## Overview

PromptHub Protocol enables the creation, management, and monetization of AI prompts as on-chain assets. Built on Solana using the Anchor framework, it provides a comprehensive infrastructure for prompt creators, users, and validators.

## Features

### Core Functionality
- **Prompt Registration**: Register prompts as on-chain assets with metadata
- **Version Control**: Semantic versioning with IPFS content storage
- **Execution Tracking**: Record prompt executions with cryptographic signatures
- **Fee Distribution**: Automatic revenue sharing (60% creator, 15% DAO, 15% validators, 10% burn)
- **Access Control**: Multiple license types (Public, Token-gated, NFT-gated, Private, Custom)

### Economic Model
- **Staking System**: Token staking for prompt ranking and rewards
- **Royalty Management**: Configurable royalty rates for creators
- **Governance**: DAO-based protocol governance with admin controls
- **Emergency Controls**: Pause/resume functionality for security

### Advanced Features
- **Prompt Forking**: Create derivative works with license validation
- **Whitelist Management**: Fine-grained access control
- **Usage Limits**: Daily usage quotas and rate limiting
- **Performance Analytics**: Execution statistics and success rates

## Architecture

### Programs
- **prompt-vault**: Main program handling prompt management and execution
- **prompt-token**: SPL token for protocol governance and payments
- **prompt-nft**: NFT collections for premium access control
- **prompt-dao**: Governance and proposal management
- **prompt-sig**: Cryptographic signature verification
- **prompt-router**: Cross-program interaction routing

### State Management
- **VaultState**: Global protocol configuration and admin controls
- **PromptData**: Individual prompt metadata and execution stats
- **ExecutionRecord**: Cryptographically signed execution logs
- **StakeAccount**: User staking positions and rewards

## Getting Started

### Prerequisites
- Node.js 18+
- Rust 1.70+
- Solana CLI 1.16+
- Anchor CLI 0.29+

### Installation

1. Clone the repository:
```bash
git clone https://github.com/prompthub/prompthub-protocol
cd prompthub-protocol
```

2. Install dependencies:
```bash
cd anchor
npm install
```

3. Build the programs:
```bash
anchor build
```

4. Run tests:
```bash
anchor test
```

### Deployment

1. Configure your Solana wallet and RPC endpoint
2. Update program IDs in `Anchor.toml`
3. Deploy to your target network:

```bash
# Deploy to localnet
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## Usage Examples

### Initialize the Vault
```typescript
await program.methods
  .initialize(adminPublicKey, 1000, new BN(1000 * 10**9)) // 10% fee, 1000 token min stake
  .accounts({
    vaultState,
    admin: adminPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Register a Prompt
```typescript
await program.methods
  .registerPrompt(
    "my-prompt-v1",
    "My AI Prompt",
    "A powerful prompt for text generation",
    "ipfs://QmHash123",
    ["ai", "text", "generation"],
    { public: {} },
    new BN(100 * 10**9) // 100 tokens
  )
  .accounts({
    promptData,
    vaultState,
    author: authorPublicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Record Execution
```typescript
await program.methods
  .recordExecution(
    "exec-123",
    true, // success
    1500, // execution time in ms
    signatureBytes
  )
  .accounts({
    promptData,
    executionRecord,
    vaultState,
    caller: callerPublicKey,
    callerTokenAccount,
    authorTokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

## Protocol Economics

### Fee Structure
- **Creator Revenue**: 60% of execution fees
- **DAO Treasury**: 15% for protocol development
- **Validator Rewards**: 15% for execution validation
- **Token Burn**: 10% for deflationary pressure

### Staking Rewards
- Stake tokens to boost prompt rankings
- Earn rewards based on prompt performance
- Minimum stake duration for reward eligibility

### Governance
- Token-weighted voting on protocol parameters
- Proposal creation and execution
- Emergency pause/resume controls

## Security Considerations

### Access Control
- Multi-signature admin controls
- Role-based permissions
- Emergency pause functionality

### Validation
- Cryptographic execution signatures
- Content hash verification
- License compliance checks

### Economic Security
- Slashing conditions for malicious behavior
- Stake-weighted governance
- Fee distribution transparency

## Development Roadmap

### Phase 1: Core Infrastructure ✅
- Basic prompt registration and execution
- Fee distribution mechanism
- Access control system

### Phase 2: Advanced Features 🚧
- Staking and governance
- NFT integration
- Cross-chain compatibility

### Phase 3: Ecosystem Integration 📋
- AI model integration
- Developer tools and SDKs
- Marketplace frontend

## Contributing

We welcome contributions to the PromptHub Protocol! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Setup
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- Documentation: [docs.prompthub.io](https://docs.prompthub.io)
- Discord: [discord.gg/prompthub](https://discord.gg/prompthub)
- Twitter: [@PromptHubDAO](https://twitter.com/PromptHubDAO)
- Email: support@prompthub.io

## Acknowledgments

- Solana Foundation for blockchain infrastructure
- Anchor framework for Solana program development
- IPFS for decentralized content storage
- The broader AI and blockchain communities 