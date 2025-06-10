# PromptHub Protocol Architecture

## Overview

PromptHub Protocol is a comprehensive decentralized system for managing AI prompts as on-chain assets on the Solana blockchain. The protocol implements a sophisticated economic model with staking, governance, and automated fee distribution.

## Core Components

### 1. Smart Contract Programs

#### prompt-vault (Main Program)
The core program handling all prompt-related operations:

**State Management:**
- `VaultState`: Global protocol configuration and admin controls
- `PromptData`: Individual prompt metadata, versioning, and execution statistics
- `ExecutionRecord`: Cryptographically signed execution logs with PromptSig
- `StakeAccount`: User staking positions and reward tracking

**Key Instructions:**
- `initialize`: Set up the protocol with admin and fee configuration
- `register_prompt`: Register new prompts with metadata and licensing
- `record_execution`: Log prompt executions with automatic fee distribution
- `create_version`: Manage prompt versioning with IPFS content storage
- `stake_for_ranking`: Token staking for prompt ranking and rewards
- `fork_prompt`: Create derivative works with license validation
- `emergency_pause/resume_operations`: Admin controls for security

#### Additional Programs (Planned)
- `prompt-token`: SPL token for governance and payments
- `prompt-nft`: NFT collections for premium access control
- `prompt-dao`: Governance and proposal management
- `prompt-sig`: Cryptographic signature verification system
- `prompt-router`: Cross-program interaction routing

### 2. Economic Model

#### Fee Distribution (Per Execution)
- **Creator Revenue**: 60% - Direct payment to prompt authors
- **DAO Treasury**: 15% - Protocol development and governance
- **Validator Rewards**: 15% - Execution validation incentives
- **Token Burn**: 10% - Deflationary mechanism

#### Staking System
- **Ranking Boost**: Stake tokens to improve prompt visibility
- **Reward Mechanism**: Earn rewards based on prompt performance
- **Minimum Duration**: 24-hour minimum stake period for rewards
- **Compound Interest**: Time-weighted reward calculation

### 3. Access Control System

#### License Types
- **Public**: Open access for all users
- **TokenGated**: Requires minimum token balance
- **NftGated**: Requires specific NFT ownership
- **Private**: Whitelist-only access
- **Custom**: Flexible custom licensing terms

#### Access Management
- **Whitelist Control**: Up to 100 addresses per prompt
- **Usage Limits**: Daily execution quotas
- **Token Requirements**: Configurable minimum balances
- **Time-based Access**: Temporal access controls

### 4. Version Control System

#### Semantic Versioning
- **Format**: Major.Minor.Patch (e.g., 1.2.3)
- **Compatibility**: Major version compatibility checking
- **History Tracking**: Up to 10 recent versions stored
- **Content Verification**: IPFS hash validation

#### Content Storage
- **IPFS Integration**: Decentralized content storage
- **Hash Verification**: Content integrity validation
- **URI Validation**: Proper IPFS URI format checking
- **Changelog Support**: Version change documentation

### 5. Execution Tracking (PromptSig)

#### Cryptographic Verification
- **Signature Validation**: Ed25519 signature verification
- **Execution Proof**: Cryptographic proof of execution
- **Performance Metrics**: Execution time and success rate tracking
- **Revenue Tracking**: Automatic fee calculation and distribution

#### Analytics
- **Success Rate**: Track prompt performance over time
- **Execution Time**: Average and time-weighted performance metrics
- **Usage Patterns**: Daily, weekly, and monthly usage statistics
- **Revenue Analytics**: Creator earnings and protocol fees

## Technical Implementation

### 1. Program Derived Addresses (PDAs)

```rust
// Vault state (global)
seeds = [b"vault_state"]

// Individual prompt data
seeds = [b"prompt", prompt_id.as_bytes()]

// Execution records
seeds = [b"execution", execution_id.as_bytes()]

// Staking accounts
seeds = [b"stake", prompt_id.as_bytes(), staker.key().as_ref()]

// Pool accounts
seeds = [b"stake_pool"] // or [b"reward_pool"]
```

### 2. Account Structure

#### VaultState (Global Configuration)
```rust
pub struct VaultState {
    pub admin: Pubkey,              // Protocol administrator
    pub fee_bps: u16,               // Fee in basis points
    pub min_stake_amount: u64,      // Minimum staking amount
    pub total_prompts: u64,         // Total registered prompts
    pub total_executions: u64,      // Total executions recorded
    pub total_revenue: u64,         // Total protocol revenue
    pub is_paused: bool,            // Emergency pause state
    pub created_at: i64,            // Creation timestamp
    pub updated_at: i64,            // Last update timestamp
}
```

#### PromptData (Individual Prompts)
```rust
pub struct PromptData {
    pub id: String,                 // Unique prompt identifier
    pub title: String,              // Human-readable title
    pub description: String,        // Detailed description
    pub author: Pubkey,             // Creator's public key
    pub content_uri: String,        // IPFS content URI
    pub current_version: String,    // Current semantic version
    pub license_type: LicenseType,  // Access control type
    pub price: u64,                 // Execution price in tokens
    pub tags: Vec<String>,          // Searchable tags
    pub status: PromptStatus,       // Active/Deprecated/Archived
    pub execution_stats: ExecutionStats, // Performance metrics
    pub access_control: AccessControl,   // Access restrictions
    pub royalty_config: RoyaltyConfig,   // Revenue sharing
    pub version_history: Vec<VersionEntry>, // Version tracking
    pub created_at: i64,            // Creation timestamp
    pub updated_at: i64,            // Last update timestamp
}
```

### 3. Error Handling

Comprehensive error system with specific error codes for:
- **Validation Errors**: Input validation and format checking
- **Authorization Errors**: Access control and permission validation
- **State Errors**: Account state and lifecycle validation
- **Economic Errors**: Fee calculation and token transfer validation
- **Operational Errors**: System state and emergency controls

### 4. Security Features

#### Access Control
- **Multi-signature Admin**: Secure admin key management
- **Role-based Permissions**: Granular permission system
- **Emergency Pause**: Circuit breaker for security incidents

#### Validation
- **Input Sanitization**: Comprehensive input validation
- **Cryptographic Verification**: Signature and hash validation
- **State Consistency**: Account state validation and constraints

#### Economic Security
- **Slashing Conditions**: Penalties for malicious behavior
- **Stake-weighted Governance**: Economic alignment in governance
- **Transparent Fee Distribution**: Auditable fee allocation

## Integration Points

### 1. Frontend Integration
- **Web3 Wallet Connection**: Phantom, Solflare, etc.
- **Transaction Building**: Anchor client integration
- **Real-time Updates**: WebSocket subscriptions for state changes

### 2. AI Model Integration
- **Execution Endpoints**: RESTful API for prompt execution
- **Signature Generation**: PromptSig cryptographic proof
- **Performance Monitoring**: Real-time execution metrics

### 3. IPFS Integration
- **Content Storage**: Decentralized prompt content storage
- **Metadata Pinning**: Persistent metadata availability
- **Gateway Access**: Multiple IPFS gateway support

## Deployment Architecture

### 1. Network Configuration
- **Localnet**: Development and testing
- **Devnet**: Staging and integration testing
- **Mainnet**: Production deployment

### 2. Program Deployment
- **Anchor Framework**: Streamlined deployment process
- **Program Upgrades**: Controlled upgrade mechanism
- **State Migration**: Safe state transition procedures

### 3. Monitoring and Analytics
- **On-chain Analytics**: Transaction and state monitoring
- **Performance Metrics**: Execution time and success rate tracking
- **Economic Metrics**: Fee distribution and staking analytics

## Future Enhancements

### 1. Cross-chain Compatibility
- **Bridge Integration**: Multi-chain prompt access
- **State Synchronization**: Cross-chain state consistency
- **Universal Access**: Chain-agnostic prompt execution

### 2. Advanced AI Integration
- **Model Marketplace**: AI model discovery and integration
- **Automated Optimization**: Performance-based prompt optimization
- **Collaborative Filtering**: Community-driven prompt recommendations

### 3. Governance Evolution
- **Decentralized Governance**: Community-driven protocol evolution
- **Proposal System**: On-chain governance proposals
- **Voting Mechanisms**: Token-weighted and quadratic voting

This architecture provides a robust foundation for the PromptHub Protocol, enabling secure, scalable, and economically sustainable management of AI prompts as digital assets. 