//! Validation utilities for CS2 demo parsing

use crate::error::{DemoError, Result};
use std::path::Path;
use tracing::debug;


/// Validate a demo file format
pub fn validate_demo_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    
    // Check if file exists
    if !path.exists() {
        return Err(DemoError::file_not_found(path.to_string_lossy()));
    }
    
    // Check file extension
    if let Some(extension) = path.extension() {
        if extension != "dem" {
            return Err(DemoError::invalid_format(format!(
                "Invalid file extension: {}. Expected .dem", 
                extension.to_string_lossy()
            )));
        }
    } else {
        return Err(DemoError::invalid_format("No file extension found. Expected .dem"));
    }
    
    // Check file size
    let metadata = std::fs::metadata(path)
        .map_err(|e| DemoError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read file metadata: {}", e))))?;
    
    if metadata.len() < 1024 {
        return Err(DemoError::invalid_format("File too small to be a valid demo"));
    }
    
    // Read and validate header
    let mut file = std::fs::File::open(path)
        .map_err(|e| DemoError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to open file: {}", e))))?;
    
    let mut header = [0u8; 1024];
    let bytes_read = std::io::Read::read(&mut file, &mut header)
        .map_err(|e| DemoError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read file header: {}", e))))?;
    
    if bytes_read < 8 {
        return Err(DemoError::invalid_format("File too small to read header"));
    }
    
    validate_demo_header(&header[..bytes_read])?;
    
    debug!("Demo file validation passed: {}", path.display());
    Ok(())
}

/// Validate demo file header
pub fn validate_demo_header(data: &[u8]) -> Result<()> {
    if data.len() < 8 {
        return Err(DemoError::invalid_format("Header too small"));
    }
    
    // Check for PBDEMS2 signature
    let signature = &data[0..8];
    let expected_signature = b"PBDEMS2\0";
    
    if signature != expected_signature {
        return Err(DemoError::invalid_format(format!(
            "Invalid demo signature: {:?}. Expected PBDEMS2", 
            String::from_utf8_lossy(signature)
        )));
    }
    
    // Check for additional header validation
    if data.len() >= 11 {
        let version = u32::from_le_bytes([data[7], data[8], data[9], data[10]]);
        debug!("Demo version: {}", version);
        
        // CS2 demos typically have version 2
        if version != 2 {
            debug!("Warning: Unexpected demo version: {}", version);
        }
    }
    
    // Look for common CS2 strings in header
    let header_str = String::from_utf8_lossy(data);
    let cs2_indicators = [
        "Counter-Strike",
        "SourceTV", 
        "DValve",
        "south_america",
        "Server"
    ];
    
    let has_cs2_indicators = cs2_indicators.iter()
        .any(|indicator| header_str.contains(indicator));
    
    if !has_cs2_indicators {
        debug!("Warning: No CS2-specific indicators found in header");
    }
    
    Ok(())
}

/// Validate demo data bytes
pub fn validate_demo_data(data: &[u8]) -> Result<()> {
    if data.is_empty() {
        return Err(DemoError::EmptyFile);
    }
    
    if data.len() < 1024 {
        return Err(DemoError::invalid_format("Demo data too small"));
    }
    
    validate_demo_header(data)?;
    
    Ok(())
}

/// Check if data contains protobuf messages
pub fn has_protobuf_messages(data: &[u8]) -> bool {
    if data.len() < 8 {
        return false;
    }
    
    // Skip header and look for protobuf field headers
    for i in 8..data.len().saturating_sub(1) {
        let byte = data[i];
        
        // Check for protobuf wire format patterns
        // Field headers typically have field_id in upper bits and wire_type in lower 3 bits
        let wire_type = byte & 0x07;
        
        // Common wire types: 0 (varint), 1 (64-bit), 2 (length-delimited), 5 (32-bit)
        if wire_type == 0 || wire_type == 1 || wire_type == 2 || wire_type == 5 {
            // Check if next few bytes look like valid protobuf data
            if i + 4 < data.len() {
                // Look for reasonable field IDs (typically 1-100)
                let field_id = byte >> 3;
                if field_id > 0 && field_id <= 100 {
                    return true;
                }
            }
        }
    }
    
    false
}

/// Extract basic demo information from header
pub fn extract_demo_info(data: &[u8]) -> Result<DemoInfo> {
    if data.len() < 1024 {
        return Err(DemoError::invalid_format("Data too small for info extraction"));
    }
    
    let header_str = String::from_utf8_lossy(&data[0..1024]);
    
    // Extract map name
    let map_name = extract_map_name(&header_str);
    
    // Extract server info
    let server_info = extract_server_info(&header_str);
    
    // Extract version
    let version = if data.len() >= 11 {
        u32::from_le_bytes([data[7], data[8], data[9], data[10]])
    } else {
        0
    };
    
    Ok(DemoInfo {
        signature: "PBDEMS2".to_string(),
        version,
        map_name,
        server_info,
        has_protobuf: has_protobuf_messages(data),
    })
}

/// Extract map name from header string
fn extract_map_name(header_str: &str) -> String {
    // Look for common map patterns
    let map_patterns = [
        "de_ancient",
        "de_anubis", 
        "de_inferno",
        "de_mirage",
        "de_nuke",
        "de_overpass",
        "de_vertigo",
        "de_dust2",
        "de_cache",
        "de_cobblestone",
        "de_train",
    ];
    
    for pattern in &map_patterns {
        if header_str.contains(pattern) {
            return pattern.to_string();
        }
    }
    
    "unknown".to_string()
}

/// Extract server information from header string
fn extract_server_info(header_str: &str) -> String {
    // Look for server patterns
    if header_str.contains("SourceTV") {
        return "SourceTV".to_string();
    }
    
    if header_str.contains("Server") {
        // Try to extract server name
        if let Some(start) = header_str.find("Server") {
            let after_server = &header_str[start..];
            if let Some(end) = after_server.find('\0') {
                return after_server[..end].to_string();
            }
        }
    }
    
    "unknown".to_string()
}

/// Basic demo information
#[derive(Debug, Clone)]
pub struct DemoInfo {
    pub signature: String,
    pub version: u32,
    pub map_name: String,
    pub server_info: String,
    pub has_protobuf: bool,
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

    
        #[test]
    fn test_validate_demo_header() {
        let valid_data = b"PBDEMS2\0test data";
        assert!(validate_demo_header(valid_data).is_ok());
        
        let invalid_data = b"INVALID\0test data";
        assert!(validate_demo_header(invalid_data).is_err());
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
