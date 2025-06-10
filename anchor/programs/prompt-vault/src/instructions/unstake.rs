use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Unstake<'info> {
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
    
    /// CHECK: Stake pool account - validated by seeds
    #[account(
        mut,
        seeds = [b"stake_pool"],
        bump
    )]
    pub stake_pool: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    
    require!(amount > 0, VaultError::StakeAmountBelowMinimum);
    require!(
        stake_account.staked_amount >= amount,
        VaultError::InsufficientStake
    );
    
    // Remove stake amount
    stake_account.remove_stake(amount)?;
    
    // Transfer tokens back from stake pool
    // Note: In a full implementation, this would use a PDA signer
    // For now, we'll just log the transfer
    msg!("Unstaking {} tokens from prompt: {}", amount, stake_account.prompt_id);
    msg!("Remaining stake: {}", stake_account.staked_amount);
    
    Ok(())
} 