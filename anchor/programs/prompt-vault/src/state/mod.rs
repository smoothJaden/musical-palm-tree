use anchor_lang::prelude::*;

pub mod vault_state;
pub mod prompt_data;
pub mod execution_record;
pub mod stake_account;

pub use vault_state::*;
pub use prompt_data::*;
pub use execution_record::*;
pub use stake_account::*;

/// License types for prompts
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum LicenseType {
    /// Free to use by anyone
    Public = 0,
    /// Requires token payment per use
    TokenGated = 1,
    /// Requires NFT ownership
    NftGated = 2,
    /// Private/restricted access
    Private = 3,
    /// Custom license terms
    Custom = 4,
}

impl Default for LicenseType {
    fn default() -> Self {
        LicenseType::Public
    }
}

/// Prompt status enumeration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum PromptStatus {
    /// Draft state, not yet published
    Draft = 0,
    /// Active and available for use
    Active = 1,
    /// Deprecated but still accessible
    Deprecated = 2,
    /// Suspended due to violations
    Suspended = 3,
    /// Permanently removed
    Removed = 4,
}

impl Default for PromptStatus {
    fn default() -> Self {
        PromptStatus::Draft
    }
}

/// Version entry for tracking prompt versions
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct VersionEntry {
    /// Semantic version string
    pub version: String,
    /// IPFS URI for this version
    pub metadata_uri: String,
    /// Creation timestamp
    pub timestamp: i64,
    /// Content hash for verification
    pub content_hash: [u8; 32],
}

/// Execution statistics
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ExecutionStats {
    /// Total number of executions
    pub total_executions: u64,
    /// Total revenue generated
    pub total_revenue: u64,
    /// Average execution time (milliseconds)
    pub avg_execution_time: u32,
    /// Success rate (basis points, 10000 = 100%)
    pub success_rate: u16,
    /// Last execution timestamp
    pub last_execution: i64,
}

/// Royalty configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoyaltyConfig {
    /// Creator share in basis points (10000 = 100%)
    pub creator_share_bps: u16,
    /// DAO treasury share in basis points
    pub dao_share_bps: u16,
    /// Validator share in basis points
    pub validator_share_bps: u16,
    /// Burn share in basis points
    pub burn_share_bps: u16,
}

impl Default for RoyaltyConfig {
    fn default() -> Self {
        Self {
            creator_share_bps: 6000,  // 60%
            dao_share_bps: 1500,      // 15%
            validator_share_bps: 1500, // 15%
            burn_share_bps: 1000,     // 10%
        }
    }
}

/// Tag for categorizing prompts
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PromptTag {
    pub name: String,
    pub category: String,
}

/// Access control configuration
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct AccessControl {
    /// Minimum token balance required
    pub min_token_balance: u64,
    /// Required NFT mint address
    pub required_nft_mint: Option<Pubkey>,
    /// Whitelist of allowed users
    pub whitelist: Vec<Pubkey>,
    /// Maximum uses per user per day
    pub daily_usage_limit: Option<u32>,
}

/// Constants for account sizes
pub const VAULT_STATE_SIZE: usize = 8 + // discriminator
    32 + // admin
    32 + // treasury
    8 + // prompt_count
    2 + // protocol_fee_bps
    2 + // creator_share_bps
    2 + // validator_share_bps
    1 + // is_paused
    8 + // created_at
    8 + // last_updated
    64; // padding

pub const PROMPT_DATA_SIZE: usize = 8 + // discriminator
    64 + // id (max length)
    32 + // author
    256 + // metadata_uri (max length)
    32 + // current_version (max length)
    1 + // license_type
    8 + // fee_amount
    33 + // token_gate (Option<Pubkey>)
    8 + // execution_count
    1 + // status
    8 + // created_at
    8 + // last_updated
    1 + // version_count
    (32 + 256 + 8 + 32) * 10 + // recent_versions (max 10)
    (8 + 8 + 4 + 2 + 8) + // execution_stats
    (2 + 2 + 2 + 2) + // royalty_config
    4 + (64 + 64) * 5 + // tags (max 5)
    (8 + 33 + 4 + 32 * 10 + 5) + // access_control
    128; // padding

pub const EXECUTION_RECORD_SIZE: usize = 8 + // discriminator
    64 + // prompt_id
    32 + // caller
    32 + // version (max length)
    32 + // input_hash
    32 + // output_hash
    8 + // timestamp
    64 + // signature
    8 + // execution_time_ms
    1 + // success
    256 + // error_message (optional)
    64; // padding

pub const STAKE_ACCOUNT_SIZE: usize = 8 + // discriminator
    32 + // owner
    64 + // prompt_id
    8 + // staked_amount
    8 + // rewards_earned
    8 + // last_claim
    8 + // stake_timestamp
    64; // padding 