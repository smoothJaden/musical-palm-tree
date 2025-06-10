use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(params: RecordExecutionParams)]
pub struct RecordExecution<'info> {
    #[account(
        mut,
        seeds = [b"prompt", params.prompt_id.as_bytes()],
        bump,
        constraint = prompt_data.is_accessible() @ VaultError::PromptNotAccessible
    )]
    pub prompt_data: Account<'info, PromptData>,
    
    #[account(
        init,
        payer = caller,
        space = ExecutionRecord::SPACE,
        seeds = [
            b"execution",
            params.prompt_id.as_bytes(),
            caller.key().as_ref(),
            &Clock::get()?.unix_timestamp.to_le_bytes()
        ],
        bump
    )]
    pub execution_record: Account<'info, ExecutionRecord>,
    
    #[account(
        seeds = [b"vault_state"],
        bump,
        constraint = vault_state.is_operational() @ VaultError::VaultPaused
    )]
    pub vault_state: Account<'info, VaultState>,
    
    #[account(mut)]
    pub caller: Signer<'info>,
    
    // Token accounts for fee payment and distribution
    #[account(
        mut,
        constraint = caller_token_account.owner == caller.key()
    )]
    pub caller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = author_token_account.owner == prompt_data.author
    )]
    pub author_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == vault_state.treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Validator pool account - validated by seeds
    #[account(
        mut,
        seeds = [b"validator_pool"],
        bump
    )]
    pub validator_pool: AccountInfo<'info>,
    
    /// CHECK: Burn account for deflationary mechanism
    #[account(
        mut,
        constraint = burn_account.key() == spl_token::ID
    )]
    pub burn_account: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RecordExecution>, params: RecordExecutionParams) -> Result<()> {
    // Validate parameters
    params.validate()?;
    
    let prompt_data = &mut ctx.accounts.prompt_data;
    let execution_record = &mut ctx.accounts.execution_record;
    let vault_state = &ctx.accounts.vault_state;
    let clock = Clock::get()?;
    
    // Check access permissions
    let caller_token_balance = ctx.accounts.caller_token_account.amount;
    require!(
        prompt_data.has_access(&ctx.accounts.caller.key(), caller_token_balance),
        VaultError::InsufficientTokenBalance
    );
    
    // Calculate and validate fee payment
    let total_fee = prompt_data.fee_amount;
    if total_fee > 0 {
        require!(
            ctx.accounts.caller_token_account.amount >= total_fee,
            VaultError::InsufficientPayment
        );
        
        // Calculate fee distribution
        let (creator_amount, dao_amount, validator_amount, burn_amount) = 
            prompt_data.calculate_fee_distribution(total_fee);
        
        // Transfer to creator
        if creator_amount > 0 {
            let transfer_to_creator = Transfer {
                from: ctx.accounts.caller_token_account.to_account_info(),
                to: ctx.accounts.author_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            };
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_to_creator,
                ),
                creator_amount,
            )?;
        }
        
        // Transfer to DAO treasury
        if dao_amount > 0 {
            let transfer_to_treasury = Transfer {
                from: ctx.accounts.caller_token_account.to_account_info(),
                to: ctx.accounts.treasury_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            };
            token::transfer(
                CpiContext::new(
                    ctx.accounts.token_program.to_account_info(),
                    transfer_to_treasury,
                ),
                dao_amount,
            )?;
        }
        
        // Transfer to validator pool
        if validator_amount > 0 {
            // Note: In a full implementation, this would transfer to a validator pool account
            // For now, we'll just log the amount
            msg!("Validator reward: {} tokens", validator_amount);
        }
        
        // Burn tokens for deflationary mechanism
        if burn_amount > 0 {
            // Note: In a full implementation, this would burn tokens
            // For now, we'll just log the amount
            msg!("Burned: {} tokens", burn_amount);
        }
        
        msg!("Fee distribution - Creator: {}, DAO: {}, Validators: {}, Burned: {}", 
             creator_amount, dao_amount, validator_amount, burn_amount);
    }
    
    // Create execution record
    *execution_record = ExecutionRecord::new(
        params.prompt_id.clone(),
        ctx.accounts.caller.key(),
        params.version,
        params.input_hash,
        params.output_hash,
        params.signature,
        params.execution_time_ms,
        params.success,
        params.error_message,
    );
    
    // Update prompt statistics
    prompt_data.record_execution(
        params.execution_time_ms as u32,
        params.success,
        total_fee,
    );
    
    msg!("Execution recorded for prompt: {}", params.prompt_id);
    msg!("Caller: {}", ctx.accounts.caller.key());
    msg!("Success: {}", params.success);
    msg!("Execution time: {}ms", params.execution_time_ms);
    
    Ok(())
} 