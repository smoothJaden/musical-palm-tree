use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

/// Utility functions for the PromptVault program

/// Generate a content hash from prompt data
pub fn generate_content_hash(content: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hasher.finalize().into()
}

/// Validate semantic version format (basic validation)
pub fn validate_version_format(version: &str) -> bool {
    // Basic semver validation: x.y.z format
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    
    for part in parts {
        if part.parse::<u32>().is_err() {
            return false;
        }
    }
    
    true
}

/// Calculate percentage from basis points
pub fn bps_to_percentage(bps: u16) -> f64 {
    (bps as f64) / 100.0
}

/// Calculate amount from basis points
pub fn calculate_bps_amount(total: u64, bps: u16) -> u64 {
    (total * bps as u64) / 10000
}

/// Validate IPFS URI format
pub fn validate_ipfs_uri(uri: &str) -> bool {
    uri.starts_with("ipfs://") || uri.starts_with("https://ipfs.io/ipfs/")
}

/// Generate a unique execution ID
pub fn generate_execution_id(prompt_id: &str, caller: &Pubkey, timestamp: i64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prompt_id.as_bytes());
    hasher.update(caller.as_ref());
    hasher.update(&timestamp.to_le_bytes());
    
    let hash = hasher.finalize();
    bs58::encode(&hash[..16]).into_string() // Use first 16 bytes for shorter ID
}

/// Validate prompt ID format
pub fn validate_prompt_id(id: &str) -> bool {
    // Check length
    if id.is_empty() || id.len() > 64 {
        return false;
    }
    
    // Check characters (alphanumeric, underscore, hyphen only)
    id.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Calculate compound interest for staking rewards
pub fn calculate_compound_interest(
    principal: u64,
    rate_per_second: u64,
    time_seconds: u64,
) -> u64 {
    if time_seconds == 0 {
        return 0;
    }
    
    // Simple interest calculation for now
    // In a real implementation, this would use compound interest
    (principal * rate_per_second * time_seconds) / 1_000_000
}

/// Validate tag name format
pub fn validate_tag_name(tag: &str) -> bool {
    !tag.is_empty() && 
    tag.len() <= 32 && 
    tag.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

/// Check if two version strings are compatible
pub fn are_versions_compatible(v1: &str, v2: &str) -> bool {
    let v1_parts: Vec<&str> = v1.split('.').collect();
    let v2_parts: Vec<&str> = v2.split('.').collect();
    
    if v1_parts.len() != 3 || v2_parts.len() != 3 {
        return false;
    }
    
    // Same major version means compatible
    v1_parts[0] == v2_parts[0]
}

/// Generate a deterministic seed for PDAs
pub fn generate_pda_seed(base: &str, identifier: &str) -> Vec<u8> {
    let mut seed = base.as_bytes().to_vec();
    seed.extend_from_slice(identifier.as_bytes());
    seed
}

/// Validate access control configuration
pub fn validate_access_control(access_control: &crate::state::AccessControl) -> bool {
    // Check whitelist size limit
    if access_control.whitelist.len() > 100 {
        return false;
    }
    
    // Check daily usage limit is reasonable
    if let Some(limit) = access_control.daily_usage_limit {
        if limit == 0 || limit > 10000 {
            return false;
        }
    }
    
    true
}

/// Calculate time-weighted average for execution statistics
pub fn calculate_time_weighted_average(
    current_avg: u32,
    current_count: u64,
    new_value: u32,
) -> u32 {
    if current_count == 0 {
        return new_value;
    }
    
    let total_value = (current_avg as u64) * current_count + (new_value as u64);
    (total_value / (current_count + 1)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_version_format() {
        assert!(validate_version_format("1.0.0"));
        assert!(validate_version_format("10.20.30"));
        assert!(!validate_version_format("1.0"));
        assert!(!validate_version_format("1.0.0.1"));
        assert!(!validate_version_format("1.a.0"));
    }
    
    #[test]
    fn test_validate_prompt_id() {
        assert!(validate_prompt_id("my_prompt_v1"));
        assert!(validate_prompt_id("prompt-123"));
        assert!(!validate_prompt_id(""));
        assert!(!validate_prompt_id("prompt with spaces"));
        assert!(!validate_prompt_id("prompt@special"));
    }
    
    #[test]
    fn test_calculate_bps_amount() {
        assert_eq!(calculate_bps_amount(1000, 1000), 100); // 10%
        assert_eq!(calculate_bps_amount(1000, 5000), 500); // 50%
        assert_eq!(calculate_bps_amount(1000, 10000), 1000); // 100%
    }
    
    #[test]
    fn test_are_versions_compatible() {
        assert!(are_versions_compatible("1.0.0", "1.1.0"));
        assert!(are_versions_compatible("1.5.2", "1.0.0"));
        assert!(!are_versions_compatible("1.0.0", "2.0.0"));
        assert!(!are_versions_compatible("2.1.0", "1.9.9"));
    }
} 