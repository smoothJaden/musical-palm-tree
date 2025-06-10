use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Invalid fee distribution - total must not exceed 100%")]
    InvalidFeeDistribution,
    
    #[msg("Invalid treasury address")]
    InvalidTreasury,
    
    #[msg("Prompt ID cannot be empty")]
    EmptyPromptId,
    
    #[msg("Prompt ID is too long (max 64 characters)")]
    PromptIdTooLong,
    
    #[msg("Metadata URI cannot be empty")]
    EmptyMetadataUri,
    
    #[msg("Metadata URI is too long (max 256 characters)")]
    MetadataUriTooLong,
    
    #[msg("Version string cannot be empty")]
    EmptyVersion,
    
    #[msg("Version string is too long (max 32 characters)")]
    VersionTooLong,
    
    #[msg("Too many tags (max 5 allowed)")]
    TooManyTags,
    
    #[msg("Invalid royalty distribution - must sum to 100%")]
    InvalidRoyaltyDistribution,
    
    #[msg("Prompt already exists")]
    PromptAlreadyExists,
    
    #[msg("Prompt not found")]
    PromptNotFound,
    
    #[msg("Unauthorized - only prompt author can perform this action")]
    UnauthorizedAuthor,
    
    #[msg("Unauthorized - only admin can perform this action")]
    UnauthorizedAdmin,
    
    #[msg("Prompt is not active")]
    PromptNotActive,
    
    #[msg("Prompt is not accessible")]
    PromptNotAccessible,
    
    #[msg("Access denied - insufficient token balance")]
    InsufficientTokenBalance,
    
    #[msg("Access denied - required NFT not owned")]
    RequiredNftNotOwned,
    
    #[msg("Access denied - not on whitelist")]
    NotOnWhitelist,
    
    #[msg("Daily usage limit exceeded")]
    DailyUsageLimitExceeded,
    
    #[msg("Insufficient payment for prompt execution")]
    InsufficientPayment,
    
    #[msg("Invalid execution signature")]
    InvalidExecutionSignature,
    
    #[msg("Error message is too long (max 256 characters)")]
    ErrorMessageTooLong,
    
    #[msg("Duplicate tag name")]
    DuplicateTag,
    
    #[msg("Tag not found")]
    TagNotFound,
    
    #[msg("Insufficient stake amount")]
    InsufficientStake,
    
    #[msg("Stake not found")]
    StakeNotFound,
    
    #[msg("Minimum stake duration not met")]
    MinimumStakeDurationNotMet,
    
    #[msg("No rewards available to claim")]
    NoRewardsAvailable,
    
    #[msg("Vault is currently paused")]
    VaultPaused,
    
    #[msg("Invalid version format")]
    InvalidVersionFormat,
    
    #[msg("Version already exists")]
    VersionAlreadyExists,
    
    #[msg("Cannot fork own prompt")]
    CannotForkOwnPrompt,
    
    #[msg("Fork not allowed for this license type")]
    ForkNotAllowed,
    
    #[msg("Invalid content hash")]
    InvalidContentHash,
    
    #[msg("Execution record already exists")]
    ExecutionRecordAlreadyExists,
    
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    
    #[msg("Invalid account data")]
    InvalidAccountData,
    
    #[msg("Account not initialized")]
    AccountNotInitialized,
    
    #[msg("Invalid program state")]
    InvalidProgramState,
    
    #[msg("Feature not implemented")]
    FeatureNotImplemented,
    
    #[msg("Invalid license type")]
    InvalidLicenseType,
    
    #[msg("License expired")]
    LicenseExpired,
    
    #[msg("Maximum execution limit reached")]
    MaxExecutionLimitReached,
    
    #[msg("Invalid reward calculation")]
    InvalidRewardCalculation,
    
    #[msg("Cooldown period not elapsed")]
    CooldownPeriodNotElapsed,
    
    #[msg("Invalid stake amount - below minimum")]
    StakeAmountBelowMinimum,
    
    #[msg("Invalid stake amount - above maximum")]
    StakeAmountAboveMaximum,
    
    #[msg("Prompt is deprecated")]
    PromptDeprecated,
    
    #[msg("Prompt is suspended")]
    PromptSuspended,
    
    #[msg("Prompt is removed")]
    PromptRemoved,
    
    #[msg("Invalid status transition")]
    InvalidStatusTransition,
    
    #[msg("Cannot transfer ownership to same owner")]
    CannotTransferToSameOwner,
    
    #[msg("Transfer not allowed")]
    TransferNotAllowed,
    
    #[msg("Invalid access control configuration")]
    InvalidAccessControl,
    
    #[msg("Whitelist is full")]
    WhitelistFull,
    
    #[msg("Address already on whitelist")]
    AddressAlreadyOnWhitelist,
    
    #[msg("Address not on whitelist")]
    AddressNotOnWhitelist,
} 