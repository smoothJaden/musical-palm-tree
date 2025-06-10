use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: RegisterPromptParams)]
pub struct RegisterPrompt<'info> {
    #[account(
        init,
        payer = author,
        space = PromptData::SPACE,
        seeds = [b"prompt", params.id.as_bytes()],
        bump
    )]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(
        mut,
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.is_operational() @ VaultError::VaultPaused
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterPrompt>, params: RegisterPromptParams) -> Result<()> {
    // Validate parameters
    params.validate()?;
    
    let prompt_data = &mut ctx.accounts.prompt_data;
    let vault_state = &mut ctx.accounts.vault_state;
    let clock = Clock::get()?;
    
    // Create initial version entry
    let initial_version = VersionEntry {
        version: params.version.clone(),
        metadata_uri: params.metadata_uri.clone(),
        timestamp: clock.unix_timestamp,
        content_hash: params.content_hash,
    };
    
    // Initialize prompt data
    prompt_data.id = params.id.clone();
    prompt_data.author = ctx.accounts.author.key();
    prompt_data.metadata_uri = params.metadata_uri;
    prompt_data.current_version = params.version;
    prompt_data.license_type = params.license_type;
    prompt_data.fee_amount = params.fee_amount;
    prompt_data.token_gate = params.token_gate;
    prompt_data.execution_count = 0;
    prompt_data.status = PromptStatus::Active;
    prompt_data.created_at = clock.unix_timestamp;
    prompt_data.last_updated = clock.unix_timestamp;
    prompt_data.version_count = 1;
    prompt_data.recent_versions = vec![initial_version];
    prompt_data.execution_stats = ExecutionStats::default();
    prompt_data.royalty_config = params.royalty_config.unwrap_or_default();
    prompt_data.tags = params.tags;
    prompt_data.access_control = params.access_control;
    
    // Update vault state
    vault_state.increment_prompt_count();
    
    msg!("Prompt registered: {}", params.id);
    msg!("Author: {}", ctx.accounts.author.key());
    msg!("License: {:?}", params.license_type);
    msg!("Fee: {} lamports", params.fee_amount);
    
    Ok(())
} 