# CS2 Demo Core - API Reference

Complete API documentation for the CS2 Demo Core library.

## Table of Contents

- [Core Types](#core-types)
- [Main API](#main-api)
- [Event Types](#event-types)
- [Error Handling](#error-handling)
- [Utilities](#utilities)

## Core Types

### `CS2DemoCore`

The main entry point for parsing CS2 demo files.

```rust
pub struct CS2DemoCore {
    parser: CS2Parser,
}
```

#### Methods

##### `new() -> Self`

Creates a new CS2 Demo Core instance.

```rust
let demo_core = CS2DemoCore::new();
```

##### `parse_file(path: &str) -> Result<DemoEvents>`

Parses a demo file and extracts all events.

```rust
let events = demo_core.parse_file("match.dem").await?;
```

**Parameters:**
- `path` - Path to the demo file (.dem)

**Returns:**
- `Result<DemoEvents>` - Parsed events or error

**Errors:**
- `DemoError::FileNotFound` - File doesn't exist
- `DemoError::InvalidFormat` - Invalid demo format
- `DemoError::Corrupted` - Demo file is corrupted
- `DemoError::Io` - I/O error

##### `parse_bytes(data: &[u8]) -> Result<DemoEvents>`

Parses demo data from bytes.

```rust
let demo_data = fs::read("match.dem").await?;
let events = demo_core.parse_bytes(&demo_data).await?;
```

**Parameters:**
- `data` - Raw demo file bytes

**Returns:**
- `Result<DemoEvents>` - Parsed events or error

### `DemoEvents`

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

## Event Types

### `DemoMetadata`

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

### `Kill`

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

### `Headshot`

Information about a headshot kill.

```rust
pub struct Headshot {
    pub killer: String,
    pub victim: String,
    pub weapon: String,
    pub tick: u32,
    pub position: Position,
}
```

### `Clutch`

Information about a clutch situation.

```rust
pub struct Clutch {
    pub player: String,
    pub enemies: u8,
    pub successful: bool,
    pub round: u8,
    pub start_tick: u32,
    pub end_tick: u32,
    pub duration: f32,
}
```

### `Round`

Information about a game round.

```rust
pub struct Round {
    pub number: u8,
    pub winner: WinCondition,
    pub t_score: u8,
    pub ct_score: u8,
    pub duration: f32,
    pub start_tick: u32,
    pub end_tick: u32,
    pub total_kills: u16,
}
```

### `Player`

Information about a player.

```rust
pub struct Player {
    pub name: String,
    pub steam_id: String,
    pub team: String,
    pub kills: u16,
    pub deaths: u16,
    pub assists: u16,
    pub damage_dealt: u32,
    pub damage_taken: u32,
    pub mvps: u8,
    pub score: u32,
}
```

### `MatchStats`

Overall match statistics.

```rust
pub struct MatchStats {
    pub duration_minutes: f64,
    pub total_kills: u16,
    pub total_headshots: u16,
    pub total_rounds: u8,
    pub final_t_score: u8,
    pub final_ct_score: u8,
    pub avg_kills_per_round: f32,
    pub headshot_percentage: f32,
}
```

### `Position`

3D position in the game world.

```rust
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```

### `WinCondition`

Enum for round win conditions.

```rust
pub enum WinCondition {
    Terrorist,
    CounterTerrorist,
    Draw,
}
```

## Error Handling

### `DemoError`

Custom error type for demo parsing errors.

```rust
pub enum DemoError {
    FileNotFound { path: String },
    InvalidFormat { message: String },
    Corrupted { message: String },
    UnsupportedVersion { version: String },
    Io(std::io::Error),
    Protobuf(protobuf::Error),
    Timeout { message: String },
}
```

### Error Handling Examples

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};

async fn handle_demo_errors() -> Result<(), DemoError> {
    let demo_core = CS2DemoCore::new();
    
    match demo_core.parse_file("match.dem").await {
        Ok(events) => {
            println!("Successfully parsed demo");
            Ok(())
        }
        Err(DemoError::FileNotFound { path }) => {
            eprintln!("Demo file not found: {}", path);
            Err(DemoError::FileNotFound { path })
        }
        Err(DemoError::InvalidFormat { message }) => {
            eprintln!("Invalid demo format: {}", message);
            Err(DemoError::InvalidFormat { message })
        }
        Err(e) => {
            eprintln!("Unexpected error: {:?}", e);
            Err(e)
        }
    }
}
```

## Utilities

### `CS2Parser`

Low-level parser for demo files.

```rust
pub struct CS2Parser {
    options: ParseOptions,
}
```

#### Methods

##### `new() -> Self`

Creates a new parser instance.

##### `parse_file_async(path: &str) -> Result<DemoEvents>`

Parses a demo file asynchronously.

##### `parse_bytes_async(data: &[u8]) -> Result<DemoEvents>`

Parses demo bytes asynchronously.

### `ParseOptions`

Configuration options for parsing.

```rust
pub struct ParseOptions {
    pub extract_kills: bool,
    pub extract_headshots: bool,
    pub extract_clutches: bool,
    pub extract_rounds: bool,
    pub extract_players: bool,
    pub max_file_size: Option<usize>,
    pub timeout_seconds: Option<u64>,
}
```

### `EventExtractor`

Extracts events from parsed demo data.

```rust
pub struct EventExtractor {
    players: HashMap<u32, Player>,
    round_kills: Vec<Kill>,
    round_headshots: Vec<Headshot>,
}
```

## Usage Patterns

### Basic Parsing

```rust
use cs2_demo_core::CS2DemoCore;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file("match.dem").await?;
    
    println!("Map: {}", events.metadata.map);
    println!("Kills: {}", events.kills.len());
    println!("Headshots: {}", events.headshots.len());
    
    Ok(())
}
```

### Advanced Analysis

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

async fn analyze_demo(path: &str) -> Result<DemoAnalysis, Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file(path).await?;
    
    let analysis = DemoAnalysis {
        map: events.metadata.map,
        duration: events.stats.duration_minutes,
        total_kills: events.stats.total_kills,
        headshot_percentage: events.stats.headshot_percentage,
        top_fragger: find_top_fragger(&events),
        weapon_stats: analyze_weapons(&events),
    };
    
    Ok(analysis)
}

fn find_top_fragger(events: &DemoEvents) -> Option<String> {
    events.players.iter()
        .max_by_key(|(_, player)| player.kills)
        .map(|(_, player)| player.name.clone())
}

fn analyze_weapons(events: &DemoEvents) -> HashMap<String, u16> {
    let mut weapon_kills = HashMap::new();
    for kill in &events.kills {
        *weapon_kills.entry(kill.weapon.clone()).or_insert(0) += 1;
    }
    weapon_kills
}
```

### Error Recovery

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};
use tokio::time::{sleep, Duration};

async fn parse_with_retry(path: &str, max_retries: u32) -> Result<DemoEvents, DemoError> {
    let demo_core = CS2DemoCore::new();
    let mut attempts = 0;
    
    loop {
        match demo_core.parse_file(path).await {
            Ok(events) => return Ok(events),
            Err(e) => {
                attempts += 1;
                if attempts >= max_retries {
                    return Err(e);
                }
                
                eprintln!("Attempt {} failed, retrying...", attempts);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
```

## Performance Considerations

### Memory Usage

- **Small demos** (< 100MB): ~10-20MB memory usage
- **Medium demos** (100-500MB): ~20-50MB memory usage
- **Large demos** (> 500MB): ~50-100MB memory usage

### Processing Speed

- **Small demos**: ~0.1-0.5 seconds
- **Medium demos**: ~0.5-2.0 seconds
- **Large demos**: ~2.0-10.0 seconds

### Optimization Tips

1. **Use async parsing** for non-blocking operations
2. **Process events in batches** for large demos
3. **Filter events early** to reduce memory usage
4. **Use streaming** for very large demos (future feature)

## Thread Safety

All types in the library are `Send` and `Sync`, making them safe to use across threads:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let demo_core = Arc::new(Mutex::new(CS2DemoCore::new()));

// Safe to share across threads
let demo_core_clone = demo_core.clone();
tokio::spawn(async move {
    let core = demo_core_clone.lock().await;
    let events = core.parse_file("match.dem").await.unwrap();
    // Process events...
});
```

## Version Compatibility

The library is designed to be backward compatible within major versions. Breaking changes will be clearly documented in the changelog.

### Minimum Requirements

- **Rust**: 1.70+
- **CS2 Demo Version**: All supported versions
- **Platform**: All platforms supported by Rust

For more examples and advanced usage patterns, see the [Usage Guide](USAGE.md) and the `examples/` directory in the repository.
