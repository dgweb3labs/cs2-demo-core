//! Validation utilities for CS2 demo parsing

use crate::error::{DemoError, Result};
use std::path::Path;
use tracing::debug;

/// Validate demo file extension
pub fn validate_demo_extension(path: &Path) -> Result<()> {
    if let Some(extension) = path.extension() {
        if extension != "dem" {
            return Err(DemoError::invalid_format("File must have .dem extension"));
        }
    } else {
        return Err(DemoError::invalid_format("File must have an extension"));
    }
    Ok(())
}

/// Validate demo file size
pub fn validate_demo_size(path: &Path) -> Result<()> {
    let metadata = std::fs::metadata(path)
        .map_err(|e| DemoError::Io(e))?;
    
    let size = metadata.len();
    
    // Minimum size for a valid demo file
    if size < 1024 {
        return Err(DemoError::invalid_format("Demo file too small"));
    }
    
    // Maximum reasonable size (1GB)
    if size > 1024 * 1024 * 1024 {
        return Err(DemoError::invalid_format("Demo file too large"));
    }
    
    Ok(())
}

/// Validate demo header signature
pub fn validate_demo_signature(data: &[u8]) -> Result<()> {
    if data.len() < 11 {
        return Err(DemoError::invalid_format("Demo file too small for header"));
    }
    
    let signature = &data[0..7];
    if signature != b"PBDEMS2" {
        return Err(DemoError::invalid_format("Invalid demo signature"));
    }
    
    Ok(())
}

/// Validate demo version
pub fn validate_demo_version(data: &[u8]) -> Result<()> {
    if data.len() < 11 {
        return Err(DemoError::invalid_format("Demo file too small for version"));
    }
    
    let version = u32::from_le_bytes([data[7], data[8], data[9], data[10]]);
    
    // TODO: Implement proper CS2 version validation
    // For now, accept any version
    
    Ok(())
}

/// Validate player name
pub fn validate_player_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(DemoError::invalid_event("Player name cannot be empty"));
    }
    
    if name.len() > 32 {
        return Err(DemoError::invalid_event("Player name too long"));
    }
    
    // Check for invalid characters
    if name.chars().any(|c| c.is_control()) {
        return Err(DemoError::invalid_event("Player name contains invalid characters"));
    }
    
    Ok(())
}

/// Validate weapon name
pub fn validate_weapon_name(weapon: &str) -> Result<()> {
    if weapon.is_empty() {
        return Err(DemoError::invalid_event("Weapon name cannot be empty"));
    }
    
    if weapon.len() > 64 {
        return Err(DemoError::invalid_event("Weapon name too long"));
    }
    
    Ok(())
}

/// Validate round number
pub fn validate_round_number(round: u8) -> Result<()> {
    // CS2 matches typically have 30 rounds max (15-15)
    if round > 30 {
        return Err(DemoError::invalid_event("Round number too high"));
    }
    
    Ok(())
}

/// Validate tick number
pub fn validate_tick_number(tick: u32) -> Result<()> {
    // Reasonable maximum for a demo tick
    if tick > 1000000 {
        return Err(DemoError::invalid_event("Tick number too high"));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_validate_demo_extension() {
        let valid_path = PathBuf::from("test.dem");
        assert!(validate_demo_extension(&valid_path).is_ok());
        
        let invalid_path = PathBuf::from("test.txt");
        assert!(validate_demo_extension(&invalid_path).is_err());
    }
    
    #[test]
    fn test_validate_demo_signature() {
        let valid_data = b"HL2DEMO\x04\x00\x00\x00";
        assert!(validate_demo_signature(valid_data).is_ok());
        
        let invalid_data = b"INVALID\x04\x00\x00\x00";
        assert!(validate_demo_signature(invalid_data).is_err());
    }
    
    #[test]
    fn test_validate_demo_version() {
        let valid_data = b"HL2DEMO\x04\x00\x00\x00";
        assert!(validate_demo_version(valid_data).is_ok());
        
        let invalid_data = b"HL2DEMO\x03\x00\x00\x00";
        assert!(validate_demo_version(invalid_data).is_err());
    }
    
    #[test]
    fn test_validate_player_name() {
        assert!(validate_player_name("Player1").is_ok());
        assert!(validate_player_name("").is_err());
        assert!(validate_player_name(&"a".repeat(33)).is_err());
    }
    
    #[test]
    fn test_validate_weapon_name() {
        assert!(validate_weapon_name("ak47").is_ok());
        assert!(validate_weapon_name("").is_err());
        assert!(validate_weapon_name(&"a".repeat(65)).is_err());
    }
    
    #[test]
    fn test_validate_round_number() {
        assert!(validate_round_number(15).is_ok());
        assert!(validate_round_number(31).is_err());
    }
    
    #[test]
    fn test_validate_tick_number() {
        assert!(validate_tick_number(64000).is_ok());
        assert!(validate_tick_number(2000000).is_err());
    }
}
