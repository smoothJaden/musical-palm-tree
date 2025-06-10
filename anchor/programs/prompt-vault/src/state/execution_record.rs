use anchor_lang::prelude::*;
use super::*;

/// Execution record for PromptSig verification
#[account]
pub struct ExecutionRecord {
    /// Prompt ID that was executed
    pub prompt_id: String,
    /// Caller who executed the prompt
    pub caller: Pubkey,
    /// Version of the prompt that was executed
    pub version: String,
    /// Hash of the input data
    pub input_hash: [u8; 32],
    /// Hash of the output data
    pub output_hash: [u8; 32],
    /// Execution timestamp
    pub timestamp: i64,
    /// Cryptographic signature of the execution
    pub signature: [u8; 64],
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Whether the execution was successful
    pub success: bool,
    /// Error message if execution failed
    pub error_message: Option<String>,
}

impl ExecutionRecord {
    pub const SPACE: usize = EXECUTION_RECORD_SIZE;

    /// Create a new execution record
    pub fn new(
        prompt_id: String,
        caller: Pubkey,
        version: String,
        input_hash: [u8; 32],
        output_hash: [u8; 32],
        signature: [u8; 64],
        execution_time_ms: u64,
        success: bool,
        error_message: Option<String>,
    ) -> Self {
        Self {
            prompt_id,
            caller,
            version,
            input_hash,
            output_hash,
            timestamp: Clock::get().unwrap().unix_timestamp,
            signature,
            execution_time_ms,
            success,
            error_message,
        }
    }

    /// Verify the execution signature
    pub fn verify_signature(&self, expected_signer: &Pubkey) -> bool {
        // In a real implementation, this would verify the cryptographic signature
        // For now, we'll do a basic check
        self.caller == *expected_signer
    }

    /// Get a hash of the execution data for verification
    pub fn get_execution_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        
        let mut hasher = Sha256::new();
        hasher.update(self.prompt_id.as_bytes());
        hasher.update(self.caller.as_ref());
        hasher.update(self.version.as_bytes());
        hasher.update(&self.input_hash);
        hasher.update(&self.output_hash);
        hasher.update(&self.timestamp.to_le_bytes());
        
        hasher.finalize().into()
    }

    /// Check if the execution was recent (within last 24 hours)
    pub fn is_recent(&self) -> bool {
        let current_time = Clock::get().unwrap().unix_timestamp;
        current_time - self.timestamp < 86400 // 24 hours in seconds
    }
}

/// Parameters for recording an execution
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecordExecutionParams {
    /// Prompt ID
    pub prompt_id: String,
    /// Version executed
    pub version: String,
    /// Hash of input data
    pub input_hash: [u8; 32],
    /// Hash of output data
    pub output_hash: [u8; 32],
    /// Execution signature
    pub signature: [u8; 64],
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Whether execution was successful
    pub success: bool,
    /// Error message if failed
    pub error_message: Option<String>,
}

impl RecordExecutionParams {
    /// Validate the execution parameters
    pub fn validate(&self) -> Result<()> {
        require!(!self.prompt_id.is_empty(), crate::errors::VaultError::EmptyPromptId);
        require!(!self.version.is_empty(), crate::errors::VaultError::EmptyVersion);
        
        // Validate error message length if present
        if let Some(ref error_msg) = self.error_message {
            require!(error_msg.len() <= 256, crate::errors::VaultError::ErrorMessageTooLong);
        }
        
        Ok(())
    }

    /// Generate a unique seed for the execution record PDA
    pub fn generate_execution_seed(&self, caller: &Pubkey) -> String {
        format!("execution_{}_{}_{}_{}", 
                self.prompt_id, 
                caller.to_string()[..8].to_string(),
                self.version,
                Clock::get().unwrap().unix_timestamp)
    }
} 