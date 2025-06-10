use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: CreateVersionParams)]
pub struct CreateVersion<'info> {
    #[account(
        mut,
        seeds = [b"prompt", params.prompt_id.as_bytes()],
        bump,
        constraint = prompt_data.author == author.key() @ VaultError::UnauthorizedAuthor
    )]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.is_operational() @ VaultError::VaultPaused
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub author: Signer<'info>,
}

pub fn handler(ctx: Context<CreateVersion>, params: CreateVersionParams) -> Result<()> {
    let prompt_data = &mut ctx.accounts.prompt_data;
    let clock = Clock::get()?;
    
    // Create new version entry
    let new_version = VersionEntry {
        version: params.version.clone(),
        metadata_uri: params.metadata_uri,
        timestamp: clock.unix_timestamp,
        content_hash: params.content_hash,
    };
    
    // Add version to prompt
    prompt_data.add_version(new_version)?;
    
    msg!("New version created for prompt: {}", params.prompt_id);
    msg!("Version: {}", params.version);
    
    Ok(())
} 