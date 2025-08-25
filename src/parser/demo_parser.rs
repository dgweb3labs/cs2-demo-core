use crate::error::{DemoError, Result};
use crate::events::DemoEvents;
use crate::parser::{DemoParser, ParseOptions};
use std::fs;
use std::path::Path;
use tracing::{debug, info};

/// Main CS2 Demo Parser implementation
pub struct CS2Parser {
    options: ParseOptions,
}

impl CS2Parser {
    /// Create a new CS2 parser with default options
    pub fn new() -> Self {
        Self {
            options: ParseOptions::default(),
        }
    }
    
    /// Create a new CS2 parser with custom options
    pub fn with_options(options: ParseOptions) -> Self {
        Self { options }
    }
    
    /// Parse demo file asynchronously
    pub async fn parse_file_async(&self, path: &str) -> Result<DemoEvents> {
        let path = Path::new(path);
        
        if !path.exists() {
            return Err(DemoError::file_not_found(path.to_string_lossy()));
        }
        
        info!("Starting to parse demo file: {}", path.display());
        
        // Read file asynchronously
        let data = tokio::fs::read(path).await
            .map_err(|e| DemoError::Io(e))?;
        
        if data.is_empty() {
            return Err(DemoError::EmptyFile);
        }
        
        debug!("Read {} bytes from demo file", data.len());
        
        // Parse the demo data
        self.parse_bytes_async(&data).await
    }
    
    /// Parse demo bytes asynchronously
    pub async fn parse_bytes_async(&self, data: &[u8]) -> Result<DemoEvents> {
        info!("Parsing demo data of {} bytes", data.len());
        
        // Validate demo header
        self.validate_demo_header(data)?;
        
        // Parse protobuf messages
        let events = DemoEvents::new();
        
        // TODO: Implement actual protobuf parsing
        // For now, return empty events structure
        debug!("Demo parsing completed successfully");
        
        Ok(events)
    }
    
    /// Validate demo file header
    fn validate_demo_header(&self, data: &[u8]) -> Result<()> {
        if data.len() < 11 {
            return Err(DemoError::invalid_format("Demo file too small"));
        }
        
        // Check for CS2 demo signature (PBDEMS2)
        let signature = &data[0..7];
        if signature != b"PBDEMS2" {
            return Err(DemoError::invalid_format("Invalid demo signature"));
        }
        
        // Check demo version (CS2 uses different versioning)
        let version = u32::from_le_bytes([data[7], data[8], data[9], data[10]]);
        debug!("Demo header validated - version: {}", version);
        
        // TODO: Implement proper CS2 version validation
        // For now, accept any version
        Ok(())
    }
    
    /// Extract metadata from demo
    fn extract_metadata(&self, _data: &[u8]) -> Result<crate::events::DemoMetadata> {
        // TODO: Implement metadata extraction
        Ok(crate::events::DemoMetadata {
            filename: String::new(),
            version: "4".to_string(),
            map: String::new(),
            server: String::new(),
            duration: 0.0,
            ticks: 0,
            start_time: None,
        })
    }
}

impl DemoParser for CS2Parser {
    fn parse_file(&self, path: &str) -> Result<DemoEvents> {
        // For now, use blocking version
        let path = Path::new(path);
        
        if !path.exists() {
            return Err(DemoError::file_not_found(path.to_string_lossy()));
        }
        
        info!("Starting to parse demo file: {}", path.display());
        
        let data = fs::read(path)
            .map_err(|e| DemoError::Io(e))?;
        
        if data.is_empty() {
            return Err(DemoError::EmptyFile);
        }
        
        debug!("Read {} bytes from demo file", data.len());
        
        self.parse_bytes(&data)
    }
    
    fn parse_bytes(&self, data: &[u8]) -> Result<DemoEvents> {
        info!("Parsing demo data of {} bytes", data.len());
        
        // Validate demo header
        self.validate_demo_header(data)?;
        
        // Parse protobuf messages
        let events = DemoEvents::new();
        
        // TODO: Implement actual protobuf parsing
        // For now, return empty events structure
        debug!("Demo parsing completed successfully");
        
        Ok(events)
    }
    
    fn parse_file_with_options(&self, path: &str, options: ParseOptions) -> Result<DemoEvents> {
        let parser = CS2Parser::with_options(options);
        parser.parse_file(path)
    }
}

impl Default for CS2Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser_creation() {
        let parser = CS2Parser::new();
        assert!(parser.options.extract_positions);
        assert!(parser.options.extract_weapons);
        assert!(parser.options.extract_rounds);
    }
    
    #[test]
    fn test_parser_with_options() {
        let options = ParseOptions::minimal();
        let parser = CS2Parser::with_options(options);
        assert!(!parser.options.extract_positions);
        assert!(!parser.options.extract_weapons);
        assert!(!parser.options.extract_rounds);
    }
    
    #[test]
    fn test_validate_demo_header() {
        let parser = CS2Parser::new();
        
        // Valid header
        let valid_data = b"HL2DEMO\x04\x00\x00\x00";
        assert!(parser.validate_demo_header(valid_data).is_ok());
        
        // Invalid signature
        let invalid_data = b"INVALID\x04\x00\x00\x00";
        assert!(parser.validate_demo_header(invalid_data).is_err());
        
        // Too small
        let small_data = b"HL2";
        assert!(parser.validate_demo_header(small_data).is_err());
    }
}
