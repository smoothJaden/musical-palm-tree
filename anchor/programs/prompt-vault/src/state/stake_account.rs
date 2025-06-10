use anchor_lang::prelude::*;
use super::*;

/// Stake account for prompt ranking and rewards
#[account]
pub struct StakeAccount {
    /// Owner of the stake
    pub owner: Pubkey,
    /// Prompt ID being staked for
    pub prompt_id: String,
    /// Amount of tokens staked
    pub staked_amount: u64,
    /// Accumulated rewards
    pub rewards_earned: u64,
    /// Last time rewards were claimed
    pub last_claim: i64,
    /// When the stake was created
    pub stake_timestamp: i64,
}

impl StakeAccount {
    pub const SPACE: usize = STAKE_ACCOUNT_SIZE;

    /// Create a new stake account
    pub fn new(owner: Pubkey, prompt_id: String, initial_stake: u64) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            owner,
            prompt_id,
            staked_amount: initial_stake,
            rewards_earned: 0,
            last_claim: now,
            stake_timestamp: now,
        }
    }

    /// Add more tokens to the stake
    pub fn add_stake(&mut self, amount: u64) {
        self.staked_amount = self.staked_amount.saturating_add(amount);
    }

    /// Remove tokens from the stake
    pub fn remove_stake(&mut self, amount: u64) -> Result<()> {
        require!(
            self.staked_amount >= amount,
            crate::errors::VaultError::InsufficientStake
        );
        self.staked_amount = self.staked_amount.saturating_sub(amount);
        Ok(())
    }

    /// Calculate pending rewards based on time staked and amount
    pub fn calculate_pending_rewards(&self, reward_rate_per_second: u64) -> u64 {
        let current_time = Clock::get().unwrap().unix_timestamp;
        let time_since_last_claim = (current_time - self.last_claim) as u64;
        
        // Simple linear reward calculation
        // In practice, this would be more sophisticated
        (self.staked_amount * reward_rate_per_second * time_since_last_claim) / 1_000_000
    }

    /// Claim accumulated rewards
    pub fn claim_rewards(&mut self, reward_amount: u64) {
        self.rewards_earned = self.rewards_earned.saturating_add(reward_amount);
        self.last_claim = Clock::get().unwrap().unix_timestamp;
    }

    /// Check if the stake is eligible for rewards (minimum time requirement)
    pub fn is_eligible_for_rewards(&self, minimum_stake_duration: i64) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time - self.stake_timestamp >= minimum_stake_duration
    }

    /// Get the total value of the stake including rewards
    pub fn get_total_value(&self) -> u64 {
        self.staked_amount.saturating_add(self.rewards_earned)
    }

    /// Check if the stake has been active for a certain duration
    pub fn has_been_staked_for(&self, duration_seconds: i64) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time - self.stake_timestamp >= duration_seconds
    }
} 