use anchor_lang::prelude::*;
use super::*;

/// Main prompt data account
#[account]
pub struct PromptData {
    /// Unique prompt identifier
    pub id: String,
    /// Author's wallet address
    pub author: Pubkey,
    /// IPFS URI containing full DSL definition
    pub metadata_uri: String,
    /// Current semantic version
    pub current_version: String,
    /// License type
    pub license_type: LicenseType,
    /// Usage fee amount in lamports
    pub fee_amount: u64,
    /// Optional SPL token for access control
    pub token_gate: Option<Pubkey>,
    /// Total execution count
    pub execution_count: u64,
    /// Current status
    pub status: PromptStatus,
    /// Creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub last_updated: i64,
    /// Number of versions
    pub version_count: u8,
    /// Recent version history (limited to 10 most recent)
    pub recent_versions: Vec<VersionEntry>,
    /// Execution statistics
    pub execution_stats: ExecutionStats,
    /// Royalty configuration
    pub royalty_config: RoyaltyConfig,
    /// Tags for categorization
    pub tags: Vec<PromptTag>,
    /// Access control settings
    pub access_control: AccessControl,
}

impl PromptData {
    pub const SPACE: usize = PROMPT_DATA_SIZE;

    /// Check if the prompt is active and usable
    pub fn is_active(&self) -> bool {
        matches!(self.status, PromptStatus::Active)
    }

    /// Check if the prompt is accessible (active or deprecated)
    pub fn is_accessible(&self) -> bool {
        matches!(self.status, PromptStatus::Active | PromptStatus::Deprecated)
    }

    /// Check if a user has access to this prompt
    pub fn has_access(&self, user: &Pubkey, user_token_balance: u64) -> bool {
        match self.license_type {
            LicenseType::Public => true,
            LicenseType::TokenGated => user_token_balance >= self.access_control.min_token_balance,
            LicenseType::Private => self.access_control.whitelist.contains(user),
            _ => false, // Other types require additional validation
        }
    }

    /// Add a new version to the prompt
    pub fn add_version(&mut self, version: VersionEntry) -> Result<()> {
        // Limit to 10 most recent versions
        if self.recent_versions.len() >= 10 {
            self.recent_versions.remove(0);
        }
        
        self.recent_versions.push(version.clone());
        self.current_version = version.version;
        self.metadata_uri = version.metadata_uri;
        self.version_count = self.version_count.saturating_add(1);
        self.touch();
        
        Ok(())
    }

    /// Record an execution
    pub fn record_execution(&mut self, execution_time_ms: u32, success: bool, revenue: u64) {
        self.execution_count = self.execution_count.saturating_add(1);
        self.execution_stats.total_executions = self.execution_stats.total_executions.saturating_add(1);
        
        if success {
            self.execution_stats.total_revenue = self.execution_stats.total_revenue.saturating_add(revenue);
        }
        
        // Update average execution time
        let total_time = (self.execution_stats.avg_execution_time as u64) * (self.execution_stats.total_executions - 1);
        self.execution_stats.avg_execution_time = ((total_time + execution_time_ms as u64) / self.execution_stats.total_executions) as u32;
        
        // Update success rate
        let successful_executions = if success {
            ((self.execution_stats.success_rate as u64) * (self.execution_stats.total_executions - 1) / 10000) + 1
        } else {
            (self.execution_stats.success_rate as u64) * (self.execution_stats.total_executions - 1) / 10000
        };
        self.execution_stats.success_rate = ((successful_executions * 10000) / self.execution_stats.total_executions) as u16;
        
        self.execution_stats.last_execution = Clock::get().unwrap().unix_timestamp;
        self.touch();
    }

    /// Update the last modified timestamp
    pub fn touch(&mut self) {
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    /// Calculate fee distribution based on royalty config
    pub fn calculate_fee_distribution(&self, total_fee: u64) -> (u64, u64, u64, u64) {
        let creator_amount = (total_fee * self.royalty_config.creator_share_bps as u64) / 10000;
        let dao_amount = (total_fee * self.royalty_config.dao_share_bps as u64) / 10000;
        let validator_amount = (total_fee * self.royalty_config.validator_share_bps as u64) / 10000;
        let burn_amount = total_fee - creator_amount - dao_amount - validator_amount;
        
        (creator_amount, dao_amount, validator_amount, burn_amount)
    }

    /// Get the latest version entry
    pub fn get_latest_version(&self) -> Option<&VersionEntry> {
        self.recent_versions.last()
    }

    /// Check if prompt has a specific tag
    pub fn has_tag(&self, tag_name: &str) -> bool {
        self.tags.iter().any(|tag| tag.name == tag_name)
    }

    /// Add a tag to the prompt
    pub fn add_tag(&mut self, tag: PromptTag) -> Result<()> {
        require!(self.tags.len() < 5, crate::errors::VaultError::TooManyTags);
        require!(!self.has_tag(&tag.name), crate::errors::VaultError::DuplicateTag);
        
        self.tags.push(tag);
        self.touch();
        Ok(())
    }

    /// Remove a tag from the prompt
    pub fn remove_tag(&mut self, tag_name: &str) -> Result<()> {
        let initial_len = self.tags.len();
        self.tags.retain(|tag| tag.name != tag_name);
        
        require!(self.tags.len() < initial_len, crate::errors::VaultError::TagNotFound);
        self.touch();
        Ok(())
    }
}

/// Parameters for registering a new prompt
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RegisterPromptParams {
    /// Unique prompt identifier
    pub id: String,
    /// IPFS URI containing the prompt definition
    pub metadata_uri: String,
    /// Initial version string
    pub version: String,
    /// License type
    pub license_type: LicenseType,
    /// Usage fee amount
    pub fee_amount: u64,
    /// Optional token gate
    pub token_gate: Option<Pubkey>,
    /// Content hash for verification
    pub content_hash: [u8; 32],
    /// Initial tags
    pub tags: Vec<PromptTag>,
    /// Access control settings
    pub access_control: AccessControl,
    /// Custom royalty configuration (optional)
    pub royalty_config: Option<RoyaltyConfig>,
}

impl RegisterPromptParams {
    /// Validate the registration parameters
    pub fn validate(&self) -> Result<()> {
        require!(!self.id.is_empty(), crate::errors::VaultError::EmptyPromptId);
        require!(self.id.len() <= 64, crate::errors::VaultError::PromptIdTooLong);
        require!(!self.metadata_uri.is_empty(), crate::errors::VaultError::EmptyMetadataUri);
        require!(self.metadata_uri.len() <= 256, crate::errors::VaultError::MetadataUriTooLong);
        require!(!self.version.is_empty(), crate::errors::VaultError::EmptyVersion);
        require!(self.version.len() <= 32, crate::errors::VaultError::VersionTooLong);
        require!(self.tags.len() <= 5, crate::errors::VaultError::TooManyTags);
        
        // Validate royalty config if provided
        if let Some(ref config) = self.royalty_config {
            let total = config.creator_share_bps + config.dao_share_bps + 
                       config.validator_share_bps + config.burn_share_bps;
            require!(total == 10000, crate::errors::VaultError::InvalidRoyaltyDistribution);
        }
        
        Ok(())
    }
}

/// Parameters for creating a new version
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateVersionParams {
    /// Prompt ID
    pub prompt_id: String,
    /// New version string
    pub version: String,
    /// IPFS URI for the new version
    pub metadata_uri: String,
    /// Content hash for verification
    pub content_hash: [u8; 32],
}

/// Parameters for updating metadata
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateMetadataParams {
    /// Prompt ID
    pub prompt_id: String,
    /// New metadata URI
    pub metadata_uri: Option<String>,
    /// Tags to add
    pub add_tags: Vec<PromptTag>,
    /// Tags to remove (by name)
    pub remove_tags: Vec<String>,
}

/// Parameters for updating license
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateLicenseParams {
    /// Prompt ID
    pub prompt_id: String,
    /// New license type
    pub license_type: Option<LicenseType>,
    /// New fee amount
    pub fee_amount: Option<u64>,
    /// New token gate
    pub token_gate: Option<Option<Pubkey>>,
    /// New access control settings
    pub access_control: Option<AccessControl>,
    /// New royalty configuration
    pub royalty_config: Option<RoyaltyConfig>,
}

/// Parameters for updating status
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateStatusParams {
    /// Prompt ID
    pub prompt_id: String,
    /// New status
    pub status: PromptStatus,
}

/// Parameters for forking a prompt
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ForkPromptParams {
    /// Original prompt ID
    pub original_prompt_id: String,
    /// New prompt ID for the fork
    pub new_prompt_id: String,
    /// IPFS URI for the forked version
    pub metadata_uri: String,
    /// Initial version for the fork
    pub version: String,
    /// Content hash
    pub content_hash: [u8; 32],
    /// License type for the fork
    pub license_type: LicenseType,
    /// Fee amount for the fork
    pub fee_amount: u64,
    /// Token gate for the fork
    pub token_gate: Option<Pubkey>,
    /// Tags for the fork
    pub tags: Vec<PromptTag>,
    /// Access control for the fork
    pub access_control: AccessControl,
} 