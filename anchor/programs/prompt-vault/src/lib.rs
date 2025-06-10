use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use sha2::{Digest, Sha256};

declare_id!("PromptVau1tProgramId11111111111111111111111");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use errors::*;
use instructions::*;
use state::*;

#[program]
pub mod prompt_vault {
    use super::*;

    /// Initialize the global vault state
    pub fn initialize(
        ctx: Context<Initialize>,
        admin: Pubkey,
        fee_bps: u16,
        min_stake_amount: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, admin, fee_bps, min_stake_amount)
    }

    /// Register a new prompt
    pub fn register_prompt(
        ctx: Context<RegisterPrompt>,
        id: String,
        title: String,
        description: String,
        content_uri: String,
        tags: Vec<String>,
        license_type: state::LicenseType,
        price: u64,
    ) -> Result<()> {
        instructions::register_prompt::handler(
            ctx, id, title, description, content_uri, tags, license_type, price
        )
    }

    /// Create a new version of an existing prompt
    pub fn create_version(
        ctx: Context<CreateVersion>,
        version: String,
        content_uri: String,
        changelog: String,
    ) -> Result<()> {
        instructions::create_version::handler(ctx, version, content_uri, changelog)
    }

    /// Update prompt metadata
    pub fn update_metadata(
        ctx: Context<UpdateMetadata>,
        title: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<()> {
        instructions::update_metadata::handler(ctx, title, description, tags)
    }

    /// Update license terms
    pub fn update_license(
        ctx: Context<UpdateLicense>,
        license_type: state::LicenseType,
        price: Option<u64>,
        token_requirement: Option<u64>,
        whitelist: Option<Vec<Pubkey>>,
    ) -> Result<()> {
        instructions::update_license::handler(ctx, license_type, price, token_requirement, whitelist)
    }

    /// Record prompt execution and handle payments
    pub fn record_execution(
        ctx: Context<RecordExecution>,
        execution_id: String,
        success: bool,
        execution_time_ms: u32,
        signature: Vec<u8>,
    ) -> Result<()> {
        instructions::record_execution::handler(
            ctx, execution_id, success, execution_time_ms, signature
        )
    }

    /// Update prompt status (active, deprecated, etc.)
    pub fn update_status(
        ctx: Context<UpdateStatus>,
        status: state::PromptStatus,
    ) -> Result<()> {
        instructions::update_status::handler(ctx, status)
    }

    /// Transfer prompt ownership
    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        new_owner: Pubkey,
    ) -> Result<()> {
        instructions::transfer_ownership::handler(ctx, new_owner)
    }

    /// Fork an existing prompt
    pub fn fork_prompt(
        ctx: Context<ForkPrompt>,
        new_id: String,
        title: String,
        description: String,
    ) -> Result<()> {
        instructions::fork_prompt::handler(ctx, new_id, title, description)
    }

    /// Stake tokens for prompt ranking
    pub fn stake_for_ranking(
        ctx: Context<StakeForRanking>,
        amount: u64,
    ) -> Result<()> {
        instructions::stake_for_ranking::handler(ctx, amount)
    }

    /// Unstake tokens
    pub fn unstake(
        ctx: Context<Unstake>,
        amount: u64,
    ) -> Result<()> {
        instructions::unstake::handler(ctx, amount)
    }

    /// Claim accumulated rewards
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        instructions::claim_rewards::handler(ctx)
    }

    /// Emergency pause (admin only)
    pub fn emergency_pause(
        ctx: Context<EmergencyPause>,
    ) -> Result<()> {
        instructions::emergency_pause::handler(ctx)
    }

    /// Resume operations (admin only)
    pub fn resume_operations(
        ctx: Context<ResumeOperations>,
    ) -> Result<()> {
        instructions::resume_operations::handler(ctx)
    }
} 