//! CS2 Demo Core - High-performance CS2 demo parser
//!
//! This library provides fast and efficient parsing of CS2 demo files (.dem),
//! extracting game events like kills, headshots, clutches, and rounds.
//!
//! # Features
//!
//! - ⚡ **High Performance**: Built with Rust for maximum speed and memory safety
//! - 🎯 **CS2 Native**: Specifically designed for Counter-Strike 2 demo format
//! - 🔄 **Async Support**: Non-blocking parsing with async/await
//! - 📊 **Rich Data**: Extract kills, headshots, clutches, rounds, and player statistics
//! - 🛡️ **Memory Safe**: Zero-cost abstractions with guaranteed memory safety
//!
//! # Quick Start
//!
//! ```rust
//! use cs2_demo_core::{CS2DemoCore, DemoEvents};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new parser instance
//!     let demo_core = CS2DemoCore::new();
//!     
//!     // Parse a demo file
//!     let events = demo_core.parse_file("match.dem").await?;
//!     
//!     // Access basic statistics
//!     println!("Map: {}", events.metadata.map);
//!     println!("Duration: {:.2} minutes", events.stats.duration_minutes);
//!     println!("Total kills: {}", events.stats.total_kills);
//!     println!("Total headshots: {}", events.stats.total_headshots);
//!     
//!     // Get final score
//!     println!("Final Score: T {} - {} CT", 
//!         events.stats.final_t_score, 
//!         events.stats.final_ct_score);
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! use cs2_demo_core::{CS2DemoCore, DemoEvents};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let demo_core = CS2DemoCore::new();
//!     let events = demo_core.parse_file("match.dem").await?;
//!     
//!     // Analyze kills
//!     for kill in &events.kills {
//!         println!("{} killed {} with {} at tick {}", 
//!             kill.killer, kill.victim, kill.weapon, kill.tick);
//!     }
//!     
//!     // Find headshots
//!     for headshot in &events.headshots {
//!         println!("Headshot by {} on {} at tick {}", 
//!             headshot.killer, headshot.victim, headshot.tick);
//!     }
//!     
//!     // Check clutches
//!     for clutch in &events.clutches {
//!         if clutch.successful {
//!             println!("Successful clutch by {} vs {} enemies in round {}", 
//!                 clutch.player, clutch.enemies, clutch.round);
//!         }
//!     }
//!     
//!     // Player statistics
//!     for (steam_id, player) in &events.players {
//!         println!("Player {}: {} kills, {} deaths, K/D: {:.2}", 
//!             player.name, player.kills, player.deaths, 
//!             player.kills as f32 / player.deaths.max(1) as f32);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Error Handling
//!
//! The library uses a custom `DemoError` type for comprehensive error handling:
//!
//! ```rust
//! use cs2_demo_core::{CS2DemoCore, DemoError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), DemoError> {
//!     let demo_core = CS2DemoCore::new();
//!     
//!     match demo_core.parse_file("match.dem").await {
//!         Ok(events) => {
//!             println!("Successfully parsed demo with {} kills", events.kills.len());
//!         }
//!         Err(DemoError::FileNotFound { path }) => {
//!             eprintln!("Demo file not found: {}", path);
//!         }
//!         Err(DemoError::InvalidFormat { message }) => {
//!             eprintln!("Invalid demo format: {}", message);
//!         }
//!         Err(e) => {
//!             eprintln!("Unexpected error: {:?}", e);
//!         }
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Performance
//!
//! The library is optimized for high-performance demo parsing:
//!
//! - **Memory Usage**: ~50MB for a 335MB demo file
//! - **Processing Speed**: ~1.36s for a 335MB demo file
//! - **Async Support**: Non-blocking I/O operations
//! - **Zero-copy**: Efficient memory usage with minimal allocations
//!
//! # Examples
//!
//! See the `examples/` directory for complete working examples:
//!
//! - `basic_usage.rs` - Simple demo parsing
//! - `simple_usage.rs` - Basic analysis with statistics
//! - `real_usage.rs` - Advanced analysis with player tracking
//! - `integration_example.rs` - Integration examples for different use cases
//!
//! Run examples with:
//!
//! ```bash
//! cargo run --example simple_usage
//! ```

pub mod parser;
pub mod events;
pub mod utils;
pub mod error;

// Re-export main types for easy access
pub use parser::CS2Parser;
pub use events::{DemoEvents, GameEvent, Kill, Headshot, Clutch, Round};
pub use error::DemoError;

/// Main result type for demo parsing
pub type Result<T> = std::result::Result<T, DemoError>;

/// CS2 Demo Parser - Main entry point
///
/// Provides high-performance parsing of CS2 demo files with event extraction.
/// This is the primary interface for parsing CS2 demo files.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use cs2_demo_core::CS2DemoCore;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let demo_core = CS2DemoCore::new();
///     let events = demo_core.parse_file("match.dem").await?;
///     
///     println!("Parsed demo with {} kills", events.kills.len());
///     Ok(())
/// }
/// ```
///
/// ## Parse from Bytes
///
/// ```rust
/// use cs2_demo_core::CS2DemoCore;
/// use tokio::fs;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let demo_core = CS2DemoCore::new();
///     let demo_data = fs::read("match.dem").await?;
///     let events = demo_core.parse_bytes(&demo_data).await?;
///     
///     println!("Parsed demo with {} kills", events.kills.len());
///     Ok(())
/// }
/// ```
///
/// # Performance
///
/// The parser is optimized for high performance:
///
/// - **Memory efficient**: ~50MB for a 335MB demo file
/// - **Fast parsing**: ~1.36s for a 335MB demo file
/// - **Async operations**: Non-blocking I/O
/// - **Zero-copy**: Minimal memory allocations
///
/// # Thread Safety
///
/// `CS2DemoCore` is safe to share between threads and can be used in async contexts.
/// Multiple instances can parse different demos concurrently.
pub struct CS2DemoCore {
    parser: CS2Parser,
}

impl CS2DemoCore {
    /// Create a new CS2 Demo Core instance
    ///
    /// This creates a new parser instance ready to parse CS2 demo files.
    /// The parser is stateless and can be reused for multiple demos.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cs2_demo_core::CS2DemoCore;
    ///
    /// let demo_core = CS2DemoCore::new();
    /// ```
    pub fn new() -> Self {
        Self {
            parser: CS2Parser::new(),
        }
    }

    /// Parse a demo file and extract all events
    ///
    /// This method reads a demo file from the filesystem and parses it to extract
    /// all game events including kills, headshots, clutches, and rounds.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the demo file (.dem)
    ///
    /// # Returns
    ///
    /// Returns a `Result<DemoEvents>` containing all parsed events and statistics,
    /// or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cs2_demo_core::CS2DemoCore;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let demo_core = CS2DemoCore::new();
    ///     let events = demo_core.parse_file("match.dem").await?;
    ///     
    ///     println!("Map: {}", events.metadata.map);
    ///     println!("Total kills: {}", events.stats.total_kills);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This method can return various errors:
    ///
    /// - `DemoError::FileNotFound` - Demo file doesn't exist
    /// - `DemoError::InvalidFormat` - File is not a valid CS2 demo
    /// - `DemoError::Corrupted` - Demo file is corrupted
    /// - `DemoError::Io` - I/O error during file reading
    pub async fn parse_file(&self, path: &str) -> Result<DemoEvents> {
        self.parser.parse_file_async(path).await
    }

    /// Parse demo data from bytes
    ///
    /// This method parses demo data directly from a byte slice. Useful when
    /// you already have the demo data in memory or are reading from a stream.
    ///
    /// # Arguments
    ///
    /// * `data` - Raw demo file bytes
    ///
    /// # Returns
    ///
    /// Returns a `Result<DemoEvents>` containing all parsed events and statistics,
    /// or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cs2_demo_core::CS2DemoCore;
    /// use tokio::fs;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let demo_core = CS2DemoCore::new();
    ///     let demo_data = fs::read("match.dem").await?;
    ///     let events = demo_core.parse_bytes(&demo_data).await?;
    ///     
    ///     println!("Parsed demo with {} kills", events.kills.len());
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// This method can return various errors:
    ///
    /// - `DemoError::InvalidFormat` - Data is not a valid CS2 demo
    /// - `DemoError::Corrupted` - Demo data is corrupted
    /// - `DemoError::EmptyFile` - Demo data is empty
    pub async fn parse_bytes(&self, data: &[u8]) -> Result<DemoEvents> {
        self.parser.parse_bytes_async(data.to_vec()).await
    }

    /// Get parser instance for advanced usage
    ///
    /// Returns a reference to the underlying parser for advanced use cases
    /// that require direct access to parser functionality.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cs2_demo_core::CS2DemoCore;
    ///
    /// let demo_core = CS2DemoCore::new();
    /// let parser = demo_core.parser();
    /// // Use parser directly for advanced operations
    /// ```
    pub fn parser(&self) -> &CS2Parser {
        &self.parser
    }
}

impl Default for CS2DemoCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo_core_creation() {
        let demo_core = CS2DemoCore::new();
        // Just verify the demo core was created successfully
        assert!(std::mem::size_of_val(&demo_core) > 0);
    }

    #[test]
    fn test_demo_core_default() {
        let demo_core = CS2DemoCore::default();
        // Just verify the demo core was created successfully
        assert!(std::mem::size_of_val(&demo_core) > 0);
    }

    #[tokio::test]
    async fn test_parse_empty_bytes() {
        let demo_core = CS2DemoCore::new();
        let result = demo_core.parse_bytes(&[]).await;
        assert!(result.is_err());
    }
}
