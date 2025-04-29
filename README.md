# PromptHub Protocol

<div align="center">
  <img src="https://pbs.twimg.com/profile_banners/1916101667487117312/1745677140/1500x500" alt="PromptHub Logo" width="1500"/>
  <h2>The Next-Generation AI Asset Protocol on Blockchain</h2>
  <h4>Unleashing Prompt Potential, Pioneering the AI Economic Revolution</h4>

  
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
  [![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
  [![TypeScript](https://img.shields.io/badge/typescript-5.0%2B-blue.svg)](https://www.typescriptlang.org)
  [![Solana](https://img.shields.io/badge/solana-1.14-purple.svg)](https://solana.com)
  [![Discord](https://img.shields.io/discord/1234567890?color=7389D8&label=discord&logo=discord&logoColor=ffffff)](https://discord.gg/prompthub)
  [![Twitter Follow](https://img.shields.io/twitter/follow/PromptHub?style=social)](https://x.com/prompthub3)
    [![Solana](https://img.shields.io/twitter/follow/PromptHub?style=social)](https://solana.com)

</div>

## 📚 Table of Contents

- [Introduction](#introduction)
- [Repository Organization](#repository-organization)
- [Repository Structure](#repository-structure)
- [Architecture Overview](#architecture-overview)
- [Prerequisites](#prerequisites)
- [Development Setup](#development-setup)
- [Smart Contract Development](#smart-contract-development)
- [SDK Development](#sdk-development)
- [Frontend Development](#frontend-development)
- [Testing](#testing)
- [Deployment](#deployment)
- [API Documentation](#api-documentation)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)
- [Contact](#contact)

## 🌐 Introduction

PromptHub is a protocol layer and trading system that transforms prompts from static text into programmable, version-controlled, and tradable assets. This repository contains the implementation of the core protocol components as described in the [PromptHub Whitepaper](./PromptHub_Whitepaper.md).

**Core Components:**

- **PromptDSL**: A machine-readable language for defining structured prompts
- **PromptModule**: Standard execution units with consistent interface protocols
- **PromptVault**: A decentralized prompt registry on Solana
- **PromptSig**: Cryptographically verifiable execution logging
- **PromptRouter**: Semantic resolution layer for prompt discovery
- **PromptDAG**: Workflow orchestration for multi-step prompt execution
- **PromptMCP**: Protocol implementation that standardizes model interactions, context handling, and tool connections across various LLM providers, enabling model-agnostic prompt execution and consistent AI interfaces

## 🔄 Repository Organization

The PromptHub ecosystem is distributed across several repositories, each with a specific focus and primary programming language:

| Repository | Description | Primary Languages | Secondary Languages |
|------------|-------------|-------------------|---------------------|
| [prompthub-protocol](https://github.com/PromptHubLabs/prompthub-protocol) | Core protocol definitions and smart contracts | Rust (Anchor) | TypeScript |
| [prompthub-sdk](https://github.com/PromptHubLabs/prompthub-sdk) | Client libraries and development kit | TypeScript | JavaScript |
| [prompthub-app](https://github.com/PromptHubLabs/prompthub-app) | Frontend application and UI components | TypeScript (React) | CSS, HTML |
| [prompthub-indexer](https://github.com/PromptHubLabs/prompthub-indexer) | Off-chain indexing service | TypeScript (Node.js) | GraphQL, SQL |
| [prompthub-mcp](https://github.com/PromptHubLabs/prompthub-mcp) | Model Context Protocol implementation | TypeScript | Python, Rust |
| [prompthub-examples](https://github.com/PromptHubLabs/prompthub-examples) | Example implementations and tutorials | JavaScript | Python, Rust |
| [prompthub-docs](https://github.com/PromptHubLabs/prompthub-docs) | Documentation website and resources | Markdown | JavaScript |

### Repository Details

#### prompthub-protocol

The core protocol repository contains the foundational components of the PromptHub ecosystem:

- **Languages**: Primarily Rust (Anchor framework for Solana smart contracts)
- **Key Components**: 
  - PromptVault smart contracts (Rust/Anchor)
  - Contract test suite (Rust, TypeScript)
  - Protocol specification (Markdown)
- **Responsibility**: Defines the on-chain behavior and contract interfaces

#### prompthub-sdk

The SDK repository provides developer tools for interacting with the protocol:

- **Languages**: TypeScript (100%)
- **Key Components**:
  - PromptDSL parser and validator
  - Web3 client adapters for blockchain interaction
  - Model provider integrations
  - **MCP Connector adapters** that bridge between PromptHub and MCP
- **Responsibility**: Enables developers to build applications on top of PromptHub

#### prompthub-app

The frontend application repository hosts the user-facing interfaces:

- **Languages**: TypeScript (85%), CSS/SCSS (10%), HTML (5%) 
- **Frameworks**: React, Next.js
- **Key Components**:
  - Asset marketplace UI
  - PromptDSL visual editor
  - DAG builder interface
  - Wallet integration
- **Responsibility**: Provides end-users with access to the protocol features

#### prompthub-indexer

The indexer service repository manages off-chain data:

- **Languages**: TypeScript (80%), GraphQL (15%), SQL (5%)
- **Frameworks**: Node.js, PostgreSQL, Apollo Server
- **Key Components**:
  - Blockchain event listener
  - Metadata indexing engine
  - GraphQL API server
- **Responsibility**: Creates searchable indexes of on-chain data and metadata

#### prompthub-mcp

This repository is dedicated to the Model Context Protocol implementation and integration with PromptHub:

- **Languages**: TypeScript (70%), Python (20%), Rust (10%)
- **Key Components**:
  - **MCP Core**: The core implementation of the Model Context Protocol
  - **MCP Adapters**: Connectors for various LLM providers (OpenAI, Anthropic, etc.)
  - **PromptHub Bridge**: Connection layer between PromptHub and MCP
  - **Context Management**: Standardized context handling across model providers
  - **Tool Connectors**: Standardized interfaces for external tool access
- **Responsibility**: Provides the standardized protocol layer for AI model interactions
- **Integration Points**:
  - Directly integrated with PromptModule for execution
  - Used by PromptRouter for model-agnostic prompt routing
  - Leveraged in the DAG execution flow for multi-step processes
  - Enables standardized tool access across model providers

#### prompthub-examples

The examples repository showcases implementation patterns:

- **Languages**: JavaScript (60%), Python (25%), Rust (15%)
- **Key Components**:
  - Integration examples with various AI frameworks
  - Code templates for common use cases
  - Sample applications
  - **MCP integration examples** showing how to use PromptHub with MCP
- **Responsibility**: Demonstrates how to build with PromptHub

#### prompthub-docs

The documentation repository contains detailed guides and specifications:

- **Languages**: Markdown (90%), JavaScript (10%)
- **Frameworks**: Docusaurus
- **Key Components**:
  - Technical documentation
  - API references
  - Tutorials and guides
  - **MCP integration guides**
- **Responsibility**: Provides comprehensive documentation for developers

### Cross-Repository MCP Integration

The Model Context Protocol integration spans multiple repositories in the PromptHub ecosystem:

1. **prompthub-mcp**: Contains the core MCP implementation and standardized interfaces

2. **prompthub-sdk**: 
   - `sdk/core/module/mcp-adapter.ts` - Adapter that converts PromptModule calls to MCP format
   - `sdk/providers/model-providers.ts` - Model provider implementations using MCP

3. **prompthub-protocol**:
   - `anchor/programs/prompt_vault/src/lib.rs` - On-chain validation of MCP execution logs

4. **prompthub-examples**:
   - `examples/mcp-integration/` - Example implementations of MCP with PromptHub

5. **prompthub-app**:
   - `app/src/components/execution/MCPModelSelector.tsx` - UI for selecting MCP-compatible models

The MCP integration follows this flow:
1. PromptDSL definitions are compiled into MCP-compatible formats
2. PromptModule execution leverages MCP for standardized model interactions
3. PromptRouter directs requests to appropriate models via MCP
4. PromptSig captures execution details from MCP context for verification

## 📂 Repository Structure

```
prompthub/
├── anchor/                   # Solana smart contracts (PromptVault)
│   ├── programs/             # Anchor programs
│   │   ├── prompt_vault/     # Main PromptVault contract
│   │   ├── prompt_nft/       # NFT implementation
│   │   └── prompt_dao/       # Governance implementation
│   ├── tests/                # Contract test suite
│   └── Anchor.toml           # Anchor configuration
├── sdk/                      # TypeScript/JavaScript SDK
│   ├── core/                 # Core functionality
│   │   ├── promptdsl/        # PromptDSL implementation
│   │   ├── module/           # PromptModule implementation
│   │   ├── router/           # PromptRouter implementation
│   │   └── sig/              # PromptSig implementation
│   ├── client/               # Client implementation
│   └── examples/             # Usage examples
├── indexer/                  # Off-chain indexing service
│   ├── src/                  # Indexer source code
│   └── docker/               # Docker configuration
├── app/                      # Frontend application
│   ├── src/                  # React application
│   ├── public/               # Static assets
│   └── package.json          # Dependencies
├── docs/                     # Documentation
│   ├── api/                  # API documentation
│   ├── architecture/         # Architecture diagrams
│   └── tutorials/            # Tutorials and guides
├── .github/                  # GitHub configuration
│   ├── workflows/            # CI/CD pipelines
│   └── ISSUE_TEMPLATE/       # Issue templates
├── scripts/                  # Utility scripts
├── .gitignore                # Git ignore file
├── LICENSE                   # MIT License
├── package.json              # Project configuration
└── README.md                 # This file
```

## 🏗️ Architecture Overview

PromptHub follows a multi-layer architecture that spans both on-chain and off-chain components:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         PROMPTHUB ARCHITECTURE                              │
│                                                                             │
│  ┌───────────────┐    ┌────────────────┐    ┌──────────────┐    ┌─────────┐ │
│  │               │    │                │    │              │    │         │ │
│  │  Application  │    │  SDK           │    │ Smart        │    │ Indexer │ │
│  │  Layer        │    │  Layer         │    │ Contracts    │    │ Service │ │
│  │               │    │                │    │              │    │         │ │
│  └───────┬───────┘    └────────┬───────┘    └──────┬───────┘    └────┬────┘ │
│          │                     │                    │                 │      │
│          │                     │                    │                 │      │
│          ▼                     ▼                    ▼                 ▼      │
│  ┌───────────────┐    ┌────────────────┐    ┌──────────────┐    ┌─────────┐ │
│  │               │    │                │    │              │    │         │ │
│  │  React UI     │    │  TypeScript    │    │ Solana/      │    │ GraphQL │ │
│  │  Components   │    │  Libraries     │    │ Anchor       │    │ API     │ │
│  │               │    │                │    │              │    │         │ │
│  └───────────────┘    └────────────────┘    └──────────────┘    └─────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

The primary data flow follows this sequence:

1. **Prompt Registration**: Author creates a PromptDSL definition → SDK validates and pins to IPFS → Anchor program registers on-chain → Indexer updates searchable store
2. **Prompt Execution**: Client resolves prompt via Router → Module executes prompt with model → PromptSig generates verifiable log → On-chain record of execution
3. **Monetization Flow**: Execution triggers fee collection → Smart contract distributes to stakeholders → Royalties paid to prompt creator

## 🛠️ Prerequisites

Before you begin development, ensure your environment meets these requirements:

- **Node.js**: v16.x or later
- **npm**: v8.x or later
- **Rust**: Latest stable version
- **Solana CLI**: v1.14.x or later
- **Anchor**: v0.26.0 or later
- **Git**: Latest version

Optional but recommended:
- **Docker**: For running the indexer service
- **Python**: v3.10 or later (for certain scripts)

## 🚀 Development Setup

### Clone the Repository

```bash
git clone https://github.com/PromptHubLabs/prompthub-protocol.git
cd prompthub-protocol
npm install
```

### Configure Environment

Create environment configuration files:

```bash
# Root directory
cp .env.example .env

# For Anchor development
cd anchor
cp .env.example .env

# For SDK development
cd ../sdk
cp .env.example .env

# For frontend development
cd ../app
cp .env.example .env
```

Edit each `.env` file with your specific configuration values.

### Initialize Development Environment

```bash
# Generate Solana keypair (if you don't have one)
solana-keygen new -o ./keypair.json

# Start local validator
solana-test-validator

# Build and initialize (in a new terminal)
npm run init
```

This script will:
1. Build Anchor programs
2. Deploy to local Solana validator
3. Initialize SDK with local contract addresses
4. Set up local development configuration

## 🔗 Smart Contract Development

The Solana smart contracts are built using the Anchor framework and are located in the `anchor` directory.

### Build and Deploy Contracts

```bash
cd anchor
anchor build
anchor deploy
```

### Key Contract Components

#### PromptVault Program

The primary Anchor program that manages prompt registration, versioning, and execution tracking:

```rust
// Implementation example from anchor/programs/prompt_vault/src/lib.rs
#[program]
pub mod prompt_vault {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        let vault_state = &mut ctx.accounts.vault_state;
        vault_state.admin = ctx.accounts.admin.key();
        vault_state.treasury = params.treasury;
        vault_state.prompt_count = 0;
        vault_state.protocol_fee_bps = params.protocol_fee_bps;
        vault_state.creator_share_bps = params.creator_share_bps;
        vault_state.validator_share_bps = params.validator_share_bps;
        
        Ok(())
    }
    
    pub fn register_prompt(
        ctx: Context<RegisterPrompt>,
        id: String,
        metadata_uri: String,
        version: String,
        license_type: u8,
        fee_amount: Option<u64>,
        token_gate: Option<Pubkey>,
    ) -> Result<()> {
        // Implementation details
        // ...
        
        Ok(())
    }
    
    // Other functions
    // ...
}
```

### Contract Testing

```bash
# Run all tests
cd anchor
anchor test

# Run specific test
anchor test --skip-build -- --nocapture test_register_prompt
```

## 📦 SDK Development

The TypeScript SDK provides client libraries for interacting with the PromptHub protocol.

### Build the SDK

```bash
cd sdk
npm install
npm run build
```

### Core Modules

#### PromptDSL

The `PromptDSL` module provides tools for creating, validating, and managing prompt definitions:

```typescript
// Implementation example from sdk/core/promptdsl/index.ts
export class PromptDSL {
  constructor(private definition: PromptDefinition) {
    this.validate();
  }
  
  static create(definition: PromptDefinition): PromptDSL {
    return new PromptDSL(definition);
  }
  
  validate(): boolean {
    // Validation logic
    return true;
  }
  
  compile(): CompiledPrompt {
    // Compilation logic
    return {
      // ...compiled prompt data
    };
  }
  
  // Additional methods
}
```

#### PromptModule

The `PromptModule` provides execution interfaces for prompt modules:

```typescript
// Implementation example from sdk/core/module/index.ts
export class PromptModule {
  constructor(
    private id: string,
    private version: string,
    private client: PromptHubClient
  ) {}
  
  async execute(
    input: Record<string, any>,
    context?: ExecutionContext
  ): Promise<ModuleResponse> {
    // Execution logic
    return {
      // ...execution response
    };
  }
  
  // Additional methods
}
```

### Client Example

```typescript
// Example usage
import { PromptHubClient } from '@prompthub/sdk';

const client = new PromptHubClient({
  cluster: 'devnet',
  wallet: new NodeWallet(keypair),
  promptVaultProgramId: PROGRAM_ID,
});

// Register a prompt
const promptId = await client.registerPrompt({
  name: 'Legal Contract Analyzer',
  description: 'Extracts key information from legal contracts',
  inputs: {
    contract: { type: 'string', required: true },
    focus_areas: { type: 'array', default: ['liability', 'termination'] }
  },
  template: 'Analyze this contract and extract information about {{focus_areas}}: {{contract}}',
  output_schema: {
    type: 'object',
    properties: {
      sections: { type: 'array' }
    }
  }
});

// Execute a prompt
const result = await client.executePrompt('legal_analyzer_v1', {
  contract: 'This agreement is made between...',
  focus_areas: ['payment terms', 'confidentiality']
});
```

## 🖥️ Frontend Development

The frontend application provides user interfaces for interacting with the PromptHub protocol.

### Run the Frontend

```bash
cd app
npm install
npm run dev
```

The application will be available at `http://localhost:3000`.

### Key UI Components

- **PromptEditor**: Visual editor for creating PromptDSL definitions
- **PromptStore**: Browsing and discovery interface for the prompt marketplace
- **DAGBuilder**: Visual tool for creating multi-step prompt workflows
- **ExecutionConsole**: Testing and monitoring interface for prompt execution

## 🧪 Testing

### Unit Testing

```bash
# Run all tests
npm test

# Run specific test suite
npm test -- --testPathPattern=promptdsl
```

### Integration Testing

```bash
# Run integration tests
npm run test:integration
```

### End-to-End Testing

```bash
# Start local development environment
npm run dev

# In another terminal
npm run test:e2e
```

## 📤 Deployment

### Deploying Smart Contracts

```bash
# Deploy to devnet
npm run deploy:devnet

# Deploy to mainnet
npm run deploy:mainnet
```

### Deploying Frontend

```bash
# Build production version
cd app
npm run build

# Deploy to production
npm run deploy
```

## 📚 API Documentation

Comprehensive API documentation is available in the `docs/api` directory:

- **PromptDSL API**: [docs/api/promptdsl.md](docs/api/promptdsl.md)
- **PromptModule API**: [docs/api/promptmodule.md](docs/api/promptmodule.md)
- **Client API**: [docs/api/client.md](docs/api/client.md)
- **Smart Contract API**: [docs/api/contracts.md](docs/api/contracts.md)

## 🤝 Contributing

We welcome contributions to the PromptHub protocol! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and commit: `git commit -m "Add my feature"`
4. Push to your fork: `git push origin feature/my-feature`
5. Open a pull request

### Coding Standards

- **Rust**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **TypeScript**: Follow [Google TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
- **Testing**: All code should have unit tests with >80% coverage
- **Documentation**: All public APIs should be documented with JSDoc/RustDoc

## 🔒 Security

### Audit Status

The PromptHub protocol is currently undergoing security audits. The current status:

- **Smart Contracts**: In audit (ETA: Q3 2025)
- **SDK**: In audit (ETA: Q2 2025)

### Reporting Security Issues

Please report security issues through our [Security Policy](SECURITY.md).

## 📄 License

The PromptHub protocol is licensed under the [MIT License](LICENSE).

## 📞 Contact

- **Website**: [https://prompthub3.com](https://prompthub3.com)
- **GitHub**: [https://github.com/smoothJaden/prompthub-protocol](https://github.com/smoothJaden/prompthub-protocol)
- **X / Twitter**: [@prompthub3](https://x.com/prompthub3)
- **Email**: prompthub3@gmail.com
- **Discord**: [Join our community](https://discord.gg/prompthub)

---

## Technical Reference

### PromptDSL Schema

```typescript
interface PromptDefinition {
  id: string;                      // Unique identifier
  name: string;                    // Human-readable name
  description: string;             // Description of functionality
  version: string;                 // Semantic version (e.g., "1.2.0")
  author: string;                  // Creator's wallet address
  license: string;                 // License type (e.g., "CC-BY-SA")
  inputs: Record<string, InputParameter>;  // Input parameters schema
  template: string;                // The prompt template with placeholders
  output_schema: OutputSchema;     // Expected output structure
  dependencies?: string[];         // Other modules this depends on
  execution_settings?: Record<string, any>; // Model-specific parameters
  tags?: string[];                 // Categorical tags
  models?: string[];               // Compatible models
}

interface InputParameter {
  type: "string" | "number" | "boolean" | "array" | "object";
  required?: boolean;
  default?: any;
  description?: string;
  minLength?: number;  // For strings
  maxLength?: number;  // For strings
  pattern?: string;    // Regex pattern for strings
  minimum?: number;    // For numbers
  maximum?: number;    // For numbers
  enum?: any[];        // Limited value set
  items?: InputParameter;  // For arrays
  properties?: Record<string, InputParameter>;  // For objects
}
```

### Smart Contract Data Structures

```rust
// Main prompt data structure
#[account]
pub struct PromptData {
    pub id: String,                // Unique prompt identifier
    pub author: Pubkey,            // Author's wallet address
    pub metadata_uri: String,      // IPFS URI containing full DSL
    pub current_version: String,   // Current semantic version
    pub license_type: u8,          // License code
    pub fee_amount: u64,           // Usage fee (in lamports)
    pub token_gate: Option<Pubkey>, // Optional SPL token for access
    pub execution_count: u64,      // Total execution count
    pub status: u8,                // Status code
    pub created_at: i64,           // Unix timestamp
    pub last_updated: i64,         // Last update timestamp
    pub version_count: u8,
    pub recent_versions: Vec<VersionEntry>,
}

// Version entry structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VersionEntry {
    pub version: String,
    pub metadata_uri: String,
    pub timestamp: i64,
}

// Execution record structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ExecutionRecord {
    pub caller: Pubkey,
    pub prompt_id: String,
    pub version: String,
    pub input_hash: [u8; 32],
    pub output_hash: [u8; 32],
    pub timestamp: i64,
    pub signature: [u8; 64],
}
```

### SDK Client Configuration

```typescript
interface PromptHubClientConfig {
  // Connection settings
  cluster: 'mainnet-beta' | 'devnet' | 'testnet' | 'localnet';
  endpoint?: string;  // Custom RPC endpoint
  commitment?: Commitment;
  
  // Program IDs
  promptVaultProgramId: string;
  promptNftProgramId?: string;
  promptDaoProgramId?: string;
  
  // Wallet/identity
  wallet: Wallet | WalletAdapter;
  
  // Advanced options
  ipfsGateway?: string;
  cacheStrategy?: CacheStrategy;
  retryConfig?: RetryConfig;
  logger?: LoggerConfig;
}
```

### Error Handling Pattern

```typescript
// Error types
export enum PromptHubErrorCode {
  VALIDATION_ERROR = 'validation_error',
  EXECUTION_ERROR = 'execution_error',
  CONNECTION_ERROR = 'connection_error',
  PERMISSION_ERROR = 'permission_error',
  NOT_FOUND = 'not_found',
  INTERNAL_ERROR = 'internal_error',
}

export class PromptHubError extends Error {
  constructor(
    public code: PromptHubErrorCode,
    message: string,
    public details?: any
  ) {
    super(message);
    this.name = 'PromptHubError';
  }
}

// Error handling example
try {
  await client.executePrompt('prompt_id', inputs);
} catch (error) {
  if (error instanceof PromptHubError) {
    switch (error.code) {
      case PromptHubErrorCode.VALIDATION_ERROR:
        console.error('Invalid inputs:', error.details);
        break;
      case PromptHubErrorCode.PERMISSION_ERROR:
        console.error('Access denied:', error.message);
        break;
      // Handle other error types
      default:
        console.error('Unknown error:', error);
    }
  }
}
``` 
