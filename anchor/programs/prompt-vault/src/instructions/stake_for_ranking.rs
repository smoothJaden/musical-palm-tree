use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct StakeForRanking<'info> {
    #[account(
        init,
        payer = staker,
        space = StakeAccount::SPACE,
        seeds = [
            b"stake",
            prompt_data.id.as_bytes(),
            staker.key().as_ref()
        ],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"prompt", prompt_data.id.as_bytes()],
        bump,
        constraint = prompt_data.is_active() @ VaultError::PromptNotActive
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
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<StakeForRanking>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::StakeAmountBelowMinimum);
    require!(
        ctx.accounts.staker_token_account.amount >= amount,
        VaultError::InsufficientStake
    );
    
    let stake_account = &mut ctx.accounts.stake_account;
    let prompt_data = &ctx.accounts.prompt_data;
    
    // Initialize stake account
    *stake_account = StakeAccount::new(
        ctx.accounts.staker.key(),
        prompt_data.id.clone(),
        amount,
    );
    
    // Transfer tokens to stake pool
    let transfer_to_pool = Transfer {
        from: ctx.accounts.staker_token_account.to_account_info(),
        to: ctx.accounts.stake_pool.to_account_info(),
        authority: ctx.accounts.staker.to_account_info(),
    };
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_pool,
        ),
        amount,
    )?;
    
    msg!("Staked {} tokens for prompt: {}", amount, prompt_data.id);
    msg!("Staker: {}", ctx.accounts.staker.key());
    
    Ok(())
} 