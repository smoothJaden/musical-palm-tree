use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(
        constraint = current_owner.key() == prompt_data.author @ VaultError::UnauthorizedAuthor
    )]
    pub current_owner: Signer<'info>,
    
    /// CHECK: New owner account
    pub new_owner: AccountInfo<'info>,
}

pub fn handler(ctx: Context<TransferOwnership>, new_owner: Pubkey) -> Result<()> {
    let prompt_data = &mut ctx.accounts.prompt_data;
    
    require!(
        prompt_data.author != new_owner,
        VaultError::CannotTransferToSameOwner
    );
    
    let old_owner = prompt_data.author;
    prompt_data.author = new_owner;
    prompt_data.touch();
    
    msg!("Ownership transferred from {} to {}", old_owner, new_owner);
    
    Ok(())
} 