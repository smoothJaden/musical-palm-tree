use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: ForkPromptParams)]
pub struct ForkPrompt<'info> {
    #[account(
        seeds = [b"prompt", params.original_prompt_id.as_bytes()],
        bump,
        constraint = original_prompt.is_accessible() @ VaultError::PromptNotAccessible
    )]
    pub original_prompt: Account<'info, PromptData>,
    
    #[account(
        init,
        payer = forker,
        space = PromptData::SPACE,
        seeds = [b"prompt", params.new_prompt_id.as_bytes()],
        bump
    )]
    pub new_prompt: Account<'info, PromptData>,
    
    #[account(
        mut,
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.is_operational() @ VaultError::VaultPaused
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(
        mut,
        constraint = forker.key() != original_prompt.author @ VaultError::CannotForkOwnPrompt
    )]
    pub forker: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<ForkPrompt>, params: ForkPromptParams) -> Result<()> {
    let original_prompt = &ctx.accounts.original_prompt;
    let new_prompt = &mut ctx.accounts.new_prompt;
    let vault_state = &mut ctx.accounts.vault_state;
    let clock = Clock::get()?;
    
    // Check if forking is allowed for this license type
    match original_prompt.license_type {
        LicenseType::Private => return Err(VaultError::ForkNotAllowed.into()),
        _ => {} // Other license types allow forking
    }
    
    // Create initial version entry for the fork
    let initial_version = VersionEntry {
        version: params.version.clone(),
        metadata_uri: params.metadata_uri.clone(),
        timestamp: clock.unix_timestamp,
        content_hash: params.content_hash,
    };
    
    // Initialize the forked prompt
    new_prompt.id = params.new_prompt_id.clone();
    new_prompt.author = ctx.accounts.forker.key();
    new_prompt.metadata_uri = params.metadata_uri;
    new_prompt.current_version = params.version;
    new_prompt.license_type = params.license_type;
    new_prompt.fee_amount = params.fee_amount;
    new_prompt.token_gate = params.token_gate;
    new_prompt.execution_count = 0;
    new_prompt.status = PromptStatus::Active;
    new_prompt.created_at = clock.unix_timestamp;
    new_prompt.last_updated = clock.unix_timestamp;
    new_prompt.version_count = 1;
    new_prompt.recent_versions = vec![initial_version];
    new_prompt.execution_stats = ExecutionStats::default();
    new_prompt.royalty_config = RoyaltyConfig::default();
    new_prompt.tags = params.tags;
    new_prompt.access_control = params.access_control;
    
    // Update vault state
    vault_state.increment_prompt_count();
    
    msg!("Prompt forked: {} -> {}", params.original_prompt_id, params.new_prompt_id);
    msg!("Forker: {}", ctx.accounts.forker.key());
    
    Ok(())
} 