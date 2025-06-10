use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = VaultState::SPACE,
        seeds = [b"vault_state"],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    // Validate parameters
    params.validate()?;
    
    let vault_state = &mut ctx.accounts.vault_state;
    let clock = Clock::get()?;
    
    // Initialize vault state
    vault_state.admin = ctx.accounts.admin.key();
    vault_state.treasury = params.treasury;
    vault_state.prompt_count = 0;
    vault_state.protocol_fee_bps = params.protocol_fee_bps;
    vault_state.creator_share_bps = params.creator_share_bps;
    vault_state.validator_share_bps = params.validator_share_bps;
    vault_state.is_paused = false;
    vault_state.created_at = clock.unix_timestamp;
    vault_state.last_updated = clock.unix_timestamp;
    
    msg!("PromptVault initialized with admin: {}", ctx.accounts.admin.key());
    msg!("Treasury: {}", params.treasury);
    msg!("Protocol fee: {}bps", params.protocol_fee_bps);
    
    Ok(())
} 