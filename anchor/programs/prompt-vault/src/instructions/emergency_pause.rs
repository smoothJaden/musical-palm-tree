use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.admin == admin.key() @ VaultError::UnauthorizedAdmin
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<EmergencyPause>) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    
    vault_state.is_paused = true;
    vault_state.touch();
    
    msg!("Emergency pause activated by admin: {}", ctx.accounts.admin.key());
    
    Ok(())
} 