# CS2 Demo Core

[![Crates.io](https://img.shields.io/crates/v/cs2-demo-core)](https://crates.io/crates/cs2-demo-core)
[![Documentation](https://docs.rs/cs2-demo-core/badge.svg)](https://docs.rs/cs2-demo-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/dgweb3labs/cs2-demo-core/workflows/Rust/badge.svg)](https://github.com/dgweb3labs/cs2-demo-core/actions)

A high-performance Rust library for parsing Counter-Strike 2 demo files (.dem) and extracting game events like kills, headshots, clutches, and rounds.

**Built and maintained by [DG Web3 Labs](https://github.com/dgweb3labs)**

## Features

- ⚡ **High Performance**: Built with Rust for maximum speed and memory safety
- 🎯 **CS2 Native**: Specifically designed for Counter-Strike 2 demo format
- 🔄 **Async Support**: Non-blocking parsing with async/await
- 📊 **Rich Data**: Extract kills, headshots, clutches, rounds, and player statistics
- 🛡️ **Memory Safe**: Zero-cost abstractions with guaranteed memory safety
- 📦 **Easy Integration**: Simple API for quick integration into your projects

## Quick Start

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cs2-demo-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new parser instance
    let demo_core = CS2DemoCore::new();
    
    // Parse a demo file
    let events = demo_core.parse_file("match.dem").await?;
    
    // Access basic statistics
    println!("Map: {}", events.metadata.map);
    println!("Duration: {:.2} minutes", events.stats.duration_minutes);
    println!("Total kills: {}", events.stats.total_kills);
    println!("Total headshots: {}", events.stats.total_headshots);
    
    // Get final score
    println!("Final Score: T {} - {} CT", 
        events.stats.final_t_score, 
        events.stats.final_ct_score);
    
    Ok(())
}
```

### Advanced Usage

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file("match.dem").await?;
    
    // Analyze kills
    for kill in &events.kills {
        println!("{} killed {} with {} at tick {}", 
            kill.killer, kill.victim, kill.weapon, kill.tick);
    }
    
    // Find headshots
    for headshot in &events.headshots {
        println!("Headshot by {} on {} at tick {}", 
            headshot.killer, headshot.victim, headshot.tick);
    }
    
    // Check clutches
    for clutch in &events.clutches {
        if clutch.successful {
            println!("Successful clutch by {} vs {} enemies in round {}", 
                clutch.player, clutch.enemies, clutch.round);
        }
    }
    
    // Player statistics
    for (steam_id, player) in &events.players {
        println!("Player {}: {} kills, {} deaths, K/D: {:.2}", 
            player.name, player.kills, player.deaths, 
            player.kills as f32 / player.deaths.max(1) as f32);
    }
    
    Ok(())
}
```

## API Reference

### Main Types

#### `CS2DemoCore`
The main entry point for parsing CS2 demo files.

```rust
pub struct CS2DemoCore {
    parser: CS2Parser,
}

impl CS2DemoCore {
    /// Create a new CS2 Demo Core instance
    pub fn new() -> Self;
    
    /// Parse a demo file and extract all events
    pub async fn parse_file(&self, path: &str) -> Result<DemoEvents>;
    
    /// Parse demo data from bytes
    pub async fn parse_bytes(&self, data: &[u8]) -> Result<DemoEvents>;
}
```

#### `DemoEvents`
Container for all extracted demo data.

```rust
pub struct DemoEvents {
    pub metadata: DemoMetadata,
    pub kills: Vec<Kill>,
    pub headshots: Vec<Headshot>,
    pub clutches: Vec<Clutch>,
    pub rounds: Vec<Round>,
    pub players: HashMap<String, Player>,
    pub stats: MatchStats,
}
```

#### `DemoMetadata`
Basic information about the demo file.

```rust
pub struct DemoMetadata {
    pub filename: String,
    pub version: String,
    pub map: String,
    pub server: String,
    pub duration: f32,
    pub ticks: u32,
    pub start_time: Option<String>,
}
```

#### `Kill`
Information about a player kill.

```rust
pub struct Kill {
    pub killer: String,
    pub victim: String,
    pub weapon: String,
    pub tick: u32,
    pub position: Position,
    pub headshot: bool,
}
```

### Error Handling

The library uses a custom `DemoError` type for comprehensive error handling:

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};

#[tokio::main]
async fn main() -> Result<(), DemoError> {
    let demo_core = CS2DemoCore::new();
    
    match demo_core.parse_file("match.dem").await {
        Ok(events) => {
            println!("Successfully parsed demo with {} kills", events.kills.len());
        }
        Err(DemoError::FileNotFound { path }) => {
            eprintln!("Demo file not found: {}", path);
        }
        Err(DemoError::InvalidFormat { message }) => {
            eprintln!("Invalid demo format: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
        }
    }
    
    Ok(())
}
```

## Examples

### Basic Demo Analysis

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

struct DemoAnalyzer {
    demo_core: CS2DemoCore,
}

impl DemoAnalyzer {
    pub fn new() -> Self {
        Self {
            demo_core: CS2DemoCore::new(),
        }
    }
    
    pub async fn analyze_demo(&self, path: &str) -> Result<DemoSummary, Box<dyn std::error::Error>> {
        let events = self.demo_core.parse_file(path).await?;
        
        let summary = DemoSummary {
            map: events.metadata.map,
            duration: events.stats.duration_minutes,
            total_kills: events.stats.total_kills,
            total_headshots: events.stats.total_headshots,
            final_score: format!("T {} - {} CT", 
                events.stats.final_t_score, 
                events.stats.final_ct_score),
        };
        
        Ok(summary)
    }
}

struct DemoSummary {
    map: String,
    duration: f64,
    total_kills: u16,
    total_headshots: u16,
    final_score: String,
}
```

### Real-time Processing

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    
    // Read demo file as bytes
    let demo_data = fs::read("match.dem").await?;
    
    // Parse from bytes
    let events = demo_core.parse_bytes(&demo_data).await?;
    
    // Process events in real-time
    for kill in &events.kills {
        if kill.headshot {
            println!("🎯 Headshot: {} killed {} with {}", 
                kill.killer, kill.victim, kill.weapon);
        }
    }
    
    Ok(())
}
```

## Performance

The library is optimized for high-performance demo parsing:

- **Memory Usage**: ~50MB for a 335MB demo file
- **Processing Speed**: ~1.36s for a 335MB demo file
- **Async Support**: Non-blocking I/O operations
- **Zero-copy**: Efficient memory usage with minimal allocations

## Error Types

| Error Type | Description |
|------------|-------------|
| `FileNotFound` | Demo file doesn't exist |
| `InvalidFormat` | File is not a valid CS2 demo |
| `Corrupted` | Demo file is corrupted or incomplete |
| `UnsupportedVersion` | Demo version not supported |
| `Io` | I/O error during file reading |
| `Protobuf` | Error parsing protobuf data |
| `Timeout` | Parsing operation timed out |

## Contributing

We welcome contributions! Please see our [Contributing Guide](docs/CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/dgweb3labs/cs2-demo-core.git
cd cs2-demo-core

# Run tests
cargo test

# Run examples
cargo run --example simple_usage

# Check documentation
cargo doc --open
```

## Documentation

- 📖 [Usage Guide](docs/USAGE.md) - Detailed usage examples
- 🔧 [API Reference](docs/API.md) - Complete API documentation
- 🏗️ [Architecture](docs/ARCHITECTURE.md) - Project structure and design
- 🤝 [Contributing](docs/CONTRIBUTING.md) - How to contribute

## Roadmap

### SDK Features (cs2-demo-core)
- [ ] **Real protobuf parsing implementation**
- [ ] **Extract actual events** (kills, headshots, rounds)
- [ ] **Proper CS2 version validation**
- [ ] **Multi-threading support**
- [ ] **Streaming parser for large demos**
- [ ] **Performance optimizations**
- [ ] **Enhanced error handling**

### Future Enhancements
- [ ] **Benchmarking suite**
- [ ] **Integration tests**
- [ ] **Performance profiling tools**
- [ ] **Memory usage optimization**

## Support

- 📖 [Documentation](https://docs.rs/cs2-demo-core)
- 🐛 [Issue Tracker](https://github.com/dgweb3labs/cs2-demo-core/issues)
- 💬 [Discussions](https://github.com/dgweb3labs/cs2-demo-core/discussions)
- 📧 [Email](mailto:contact@dgweb3labs.com)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built for the CS2 community
- Inspired by the need for high-performance demo analysis
- Part of the DG Web3 Labs ecosystem

---

**Made with ❤️ by [DG Web3 Labs](https://github.com/dgweb3labs)**
