//! CS2 Demo Parser Module
//! 
//! This module contains the core parsing logic for CS2 demo files.

mod demo_parser;
mod protobuf_parser;
mod event_extractor;

pub use demo_parser::CS2Parser;
pub use event_extractor::EventExtractor;

use crate::error::Result;
use crate::events::DemoEvents;


/// Main parser trait for CS2 demos
pub trait DemoParser {
    /// Parse a demo file from path
    fn parse_file(&self, path: &str) -> Result<DemoEvents>;
    
    /// Parse demo data from bytes
    fn parse_bytes(&self, data: &[u8]) -> Result<DemoEvents>;
    
    /// Parse demo file with custom options
    fn parse_file_with_options(&self, path: &str, options: ParseOptions) -> Result<DemoEvents>;
}

/// Parser options for customization
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Whether to extract player positions
    pub extract_positions: bool,
    /// Whether to extract weapon information
    pub extract_weapons: bool,
    /// Whether to extract round information
    pub extract_rounds: bool,
    /// Maximum number of events to extract (0 = unlimited)
    pub max_events: usize,
    /// Whether to validate demo integrity
    pub validate_integrity: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            extract_positions: true,
            extract_weapons: true,
            extract_rounds: true,
            max_events: 0,
            validate_integrity: true,
        }
    }
}

impl ParseOptions {
    /// Create minimal parsing options (kills only)
    pub fn minimal() -> Self {
        Self {
            extract_positions: false,
            extract_weapons: false,
            extract_rounds: false,
            max_events: 0,
            validate_integrity: false,
        }
    }
    
    /// Create comprehensive parsing options
    pub fn comprehensive() -> Self {
        Self {
            extract_positions: true,
            extract_weapons: true,
            extract_rounds: true,
            max_events: 0,
            validate_integrity: true,
        }
    }
}
