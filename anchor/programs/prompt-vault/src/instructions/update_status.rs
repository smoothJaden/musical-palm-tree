use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: UpdateStatusParams)]
pub struct UpdateStatus<'info> {
    #[account(
        mut,
        seeds = [b"prompt", params.prompt_id.as_bytes()],
        bump,
        constraint = prompt_data.author == author.key() @ VaultError::UnauthorizedAuthor
    )]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(mut)]
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateStatus>, params: UpdateStatusParams) -> Result<()> {
    let prompt_data = &mut ctx.accounts.prompt_data;
    
    prompt_data.status = params.status;
    prompt_data.touch();
    
    msg!("Status updated for prompt: {} to {:?}", params.prompt_id, params.status);
    
    Ok(())
} 