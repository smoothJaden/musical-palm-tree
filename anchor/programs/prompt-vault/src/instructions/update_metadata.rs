use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: UpdateMetadataParams)]
pub struct UpdateMetadata<'info> {
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

pub fn handler(ctx: Context<UpdateMetadata>, params: UpdateMetadataParams) -> Result<()> {
    let prompt_data = &mut ctx.accounts.prompt_data;
    
    // Update metadata URI if provided
    if let Some(new_uri) = params.metadata_uri {
        prompt_data.metadata_uri = new_uri;
    }
    
    // Add new tags
    for tag in params.add_tags {
        prompt_data.add_tag(tag)?;
    }
    
    // Remove specified tags
    for tag_name in params.remove_tags {
        prompt_data.remove_tag(&tag_name)?;
    }
    
    prompt_data.touch();
    
    msg!("Metadata updated for prompt: {}", params.prompt_id);
    
    Ok(())
} 