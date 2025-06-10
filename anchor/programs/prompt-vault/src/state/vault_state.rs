use anchor_lang::prelude::*;
use super::*;

/// Global vault state account
#[account]
pub struct VaultState {
    /// Admin authority for the vault
    pub admin: Pubkey,
    /// Treasury account for protocol fees
    pub treasury: Pubkey,
    /// Total number of registered prompts
    pub prompt_count: u64,
    /// Protocol fee in basis points (10000 = 100%)
    pub protocol_fee_bps: u16,
    /// Creator royalty share in basis points
    pub creator_share_bps: u16,
    /// Validator reward share in basis points
    pub validator_share_bps: u16,
    /// Emergency pause flag
    pub is_paused: bool,
    /// Vault creation timestamp
    pub created_at: i64,
    /// Last update timestamp
    pub last_updated: i64,
}

impl VaultState {
    pub const SPACE: usize = VAULT_STATE_SIZE;

    /// Check if the vault is operational
    pub fn is_operational(&self) -> bool {
        !self.is_paused
    }

    /// Validate fee distribution adds up to 100%
    pub fn validate_fee_distribution(&self) -> bool {
        let total = self.protocol_fee_bps + self.creator_share_bps + self.validator_share_bps;
        total <= 10000 // Allow for burn percentage
    }

    /// Update the last modified timestamp
    pub fn touch(&mut self) {
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    /// Increment prompt count
    pub fn increment_prompt_count(&mut self) {
        self.prompt_count = self.prompt_count.saturating_add(1);
        self.touch();
    }
}

/// Parameters for initializing the vault
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeParams {
    /// Treasury account for collecting fees
    pub treasury: Pubkey,
    /// Protocol fee in basis points
    pub protocol_fee_bps: u16,
    /// Creator royalty share in basis points
    pub creator_share_bps: u16,
    /// Validator reward share in basis points
    pub validator_share_bps: u16,
}

impl InitializeParams {
    /// Validate the initialization parameters
    pub fn validate(&self) -> Result<()> {
        // Ensure fee distribution doesn't exceed 100%
        let total_bps = self.protocol_fee_bps + self.creator_share_bps + self.validator_share_bps;
        require!(total_bps <= 10000, crate::errors::VaultError::InvalidFeeDistribution);
        
        // Ensure treasury is not the default pubkey
        require!(self.treasury != Pubkey::default(), crate::errors::VaultError::InvalidTreasury);
        
        Ok(())
    }
} 