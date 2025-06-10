use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: UpdateLicenseParams)]
pub struct UpdateLicense<'info> {
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

pub fn handler(ctx: Context<UpdateLicense>, params: UpdateLicenseParams) -> Result<()> {
    let prompt_data = &mut ctx.accounts.prompt_data;
    
    // Update license type if provided
    if let Some(license_type) = params.license_type {
        prompt_data.license_type = license_type;
    }
    
    // Update fee amount if provided
    if let Some(fee_amount) = params.fee_amount {
        prompt_data.fee_amount = fee_amount;
    }
    
    // Update token gate if provided
    if let Some(token_gate) = params.token_gate {
        prompt_data.token_gate = token_gate;
    }
    
    // Update access control if provided
    if let Some(access_control) = params.access_control {
        prompt_data.access_control = access_control;
    }
    
    // Update royalty config if provided
    if let Some(royalty_config) = params.royalty_config {
        prompt_data.royalty_config = royalty_config;
    }
    
    prompt_data.touch();
    
    msg!("License updated for prompt: {}", params.prompt_id);
    
    Ok(())
} 