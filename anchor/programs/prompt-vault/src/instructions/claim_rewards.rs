use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [
            b"stake",
            prompt_data.id.as_bytes(),
            staker.key().as_ref()
        ],
        bump,
        constraint = stake_account.owner == staker.key() @ VaultError::UnauthorizedAuthor
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"prompt", prompt_data.id.as_bytes()],
        bump
    )]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.is_operational() @ VaultError::VaultPaused
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub staker: Signer<'info>,
    
    #[account(
        mut,
        constraint = staker_token_account.owner == staker.key()
    )]
    pub staker_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Reward pool account - validated by seeds
    #[account(
        mut,
        seeds = [b"reward_pool"],
        bump
    )]
    pub reward_pool: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ClaimRewards>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    
    // Check if eligible for rewards (minimum stake duration)
    require!(
        stake_account.is_eligible_for_rewards(86400), // 24 hours minimum
        VaultError::MinimumStakeDurationNotMet
    );
    
    // Calculate pending rewards
    let reward_rate_per_second = 100; // Example rate
    let pending_rewards = stake_account.calculate_pending_rewards(reward_rate_per_second);
    
    require!(pending_rewards > 0, VaultError::NoRewardsAvailable);
    
    // Claim rewards
    stake_account.claim_rewards(pending_rewards);
    
    // Transfer rewards from pool
    // Note: In a full implementation, this would use a PDA signer
    // For now, we'll just log the transfer
    msg!("Claimed {} reward tokens for prompt: {}", pending_rewards, stake_account.prompt_id);
    msg!("Total rewards earned: {}", stake_account.rewards_earned);
    
    Ok(())
} 