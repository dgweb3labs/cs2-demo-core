# CS2 Demo Core - Usage Guide

This guide provides detailed examples and use cases for the CS2 Demo Core library.

## Table of Contents

- [Installation](#installation)
- [Basic Usage](#basic-usage)
- [Advanced Usage](#advanced-usage)
- [Error Handling](#error-handling)
- [Performance Tips](#performance-tips)
- [Common Patterns](#common-patterns)
- [Integration Examples](#integration-examples)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cs2-demo-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Basic Usage

### Simple Demo Parsing

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file("match.dem").await?;
    
    println!("Demo Analysis:");
    println!("Map: {}", events.metadata.map);
    println!("Duration: {:.2} minutes", events.stats.duration_minutes);
    println!("Total kills: {}", events.stats.total_kills);
    println!("Total headshots: {}", events.stats.total_headshots);
    println!("Final Score: T {} - {} CT", 
        events.stats.final_t_score, 
        events.stats.final_ct_score);
    
    Ok(())
}
```

### Parse from Bytes

```rust
use cs2_demo_core::CS2DemoCore;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    
    // Read demo file as bytes
    let demo_data = fs::read("match.dem").await?;
    
    // Parse from bytes
    let events = demo_core.parse_bytes(&demo_data).await?;
    
    println!("Parsed demo with {} kills", events.kills.len());
    Ok(())
}
```

## Advanced Usage

### Kill Analysis

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

struct KillAnalyzer {
    demo_core: CS2DemoCore,
}

impl KillAnalyzer {
    pub fn new() -> Self {
        Self {
            demo_core: CS2DemoCore::new(),
        }
    }
    
    pub async fn analyze_kills(&self, path: &str) -> Result<KillStats, Box<dyn std::error::Error>> {
        let events = self.demo_core.parse_file(path).await?;
        
        let mut weapon_kills = std::collections::HashMap::new();
        let mut player_kills = std::collections::HashMap::new();
        let mut headshot_count = 0;
        
        for kill in &events.kills {
            // Count weapon usage
            *weapon_kills.entry(&kill.weapon).or_insert(0) += 1;
            
            // Count player kills
            *player_kills.entry(&kill.killer).or_insert(0) += 1;
            
            // Count headshots
            if kill.headshot {
                headshot_count += 1;
            }
        }
        
        let top_weapon = weapon_kills.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(weapon, _)| weapon.to_string());
            
        let top_fragger = player_kills.iter()
            .max_by_key(|(_, &count)| count)
            .map(|(player, _)| player.to_string());
        
        Ok(KillStats {
            total_kills: events.kills.len(),
            headshot_count,
            headshot_percentage: (headshot_count as f32 / events.kills.len() as f32) * 100.0,
            top_weapon,
            top_fragger,
        })
    }
}

struct KillStats {
    total_kills: usize,
    headshot_count: usize,
    headshot_percentage: f32,
    top_weapon: Option<String>,
    top_fragger: Option<String>,
}
```

### Player Statistics

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

struct PlayerAnalyzer {
    demo_core: CS2DemoCore,
}

impl PlayerAnalyzer {
    pub fn new() -> Self {
        Self {
            demo_core: CS2DemoCore::new(),
        }
    }
    
    pub async fn get_player_stats(&self, path: &str) -> Result<Vec<PlayerStats>, Box<dyn std::error::Error>> {
        let events = self.demo_core.parse_file(path).await?;
        
        let mut player_stats = Vec::new();
        
        for (steam_id, player) in &events.players {
            let kdr = if player.deaths > 0 {
                player.kills as f32 / player.deaths as f32
            } else {
                player.kills as f32
            };
            
            let adr = if player.deaths > 0 {
                player.damage_dealt as f32 / player.deaths as f32
            } else {
                player.damage_dealt as f32
            };
            
            player_stats.push(PlayerStats {
                name: player.name.clone(),
                steam_id: steam_id.clone(),
                team: player.team.clone(),
                kills: player.kills,
                deaths: player.deaths,
                assists: player.assists,
                damage_dealt: player.damage_dealt,
                kdr,
                adr,
            });
        }
        
        // Sort by K/D ratio
        player_stats.sort_by(|a, b| b.kdr.partial_cmp(&a.kdr).unwrap());
        
        Ok(player_stats)
    }
}

struct PlayerStats {
    name: String,
    steam_id: String,
    team: String,
    kills: u16,
    deaths: u16,
    assists: u16,
    damage_dealt: u32,
    kdr: f32,
    adr: f32,
}
```

### Round Analysis

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

struct RoundAnalyzer {
    demo_core: CS2DemoCore,
}

impl RoundAnalyzer {
    pub fn new() -> Self {
        Self {
            demo_core: CS2DemoCore::new(),
        }
    }
    
    pub async fn analyze_rounds(&self, path: &str) -> Result<RoundAnalysis, Box<dyn std::error::Error>> {
        let events = self.demo_core.parse_file(path).await?;
        
        let mut t_wins = 0;
        let mut ct_wins = 0;
        let mut round_durations = Vec::new();
        
        for round in &events.rounds {
            match round.winner {
                crate::events::WinCondition::Terrorist => t_wins += 1,
                crate::events::WinCondition::CounterTerrorist => ct_wins += 1,
                crate::events::WinCondition::Draw => {},
            }
            
            round_durations.push(round.duration);
        }
        
        let avg_round_duration = round_durations.iter().sum::<f32>() / round_durations.len() as f32;
        let longest_round = round_durations.iter().fold(0.0, |a, &b| a.max(b));
        
        Ok(RoundAnalysis {
            total_rounds: events.rounds.len(),
            t_wins,
            ct_wins,
            avg_round_duration,
            longest_round,
        })
    }
}

struct RoundAnalysis {
    total_rounds: usize,
    t_wins: usize,
    ct_wins: usize,
    avg_round_duration: f32,
    longest_round: f32,
}
```

## Error Handling

### Comprehensive Error Handling

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};

async fn parse_demo_with_error_handling(path: &str) -> Result<(), DemoError> {
    let demo_core = CS2DemoCore::new();
    
    match demo_core.parse_file(path).await {
        Ok(events) => {
            println!("✅ Successfully parsed demo");
            println!("   Map: {}", events.metadata.map);
            println!("   Kills: {}", events.kills.len());
            println!("   Headshots: {}", events.headshots.len());
            println!("   Rounds: {}", events.rounds.len());
        }
        Err(DemoError::FileNotFound { path }) => {
            eprintln!("❌ Demo file not found: {}", path);
            eprintln!("   Please check the file path and try again");
        }
        Err(DemoError::InvalidFormat { message }) => {
            eprintln!("❌ Invalid demo format: {}", message);
            eprintln!("   This file is not a valid CS2 demo");
        }
        Err(DemoError::Corrupted { message }) => {
            eprintln!("❌ Demo file corrupted: {}", message);
            eprintln!("   The demo file may be incomplete or damaged");
        }
        Err(DemoError::UnsupportedVersion { version }) => {
            eprintln!("❌ Unsupported demo version: {}", version);
            eprintln!("   This demo version is not supported by the parser");
        }
        Err(DemoError::Io(e)) => {
            eprintln!("❌ I/O error: {}", e);
            eprintln!("   Check file permissions and disk space");
        }
        Err(e) => {
            eprintln!("❌ Unexpected error: {:?}", e);
        }
    }
    
    Ok(())
}
```

### Retry Logic

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};
use tokio::time::{sleep, Duration};

async fn parse_demo_with_retry(path: &str, max_retries: u32) -> Result<DemoEvents, DemoError> {
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
                
                eprintln!("Attempt {} failed: {:?}, retrying in 1 second...", attempts, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
```

## Performance Tips

### Batch Processing

```rust
use cs2_demo_core::CS2DemoCore;
use tokio::fs;
use std::path::Path;

async fn process_multiple_demos(demo_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let mut entries = fs::read_dir(demo_dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("dem") {
            println!("Processing: {}", path.display());
            
            match demo_core.parse_file(path.to_str().unwrap()).await {
                Ok(events) => {
                    println!("  ✅ {} kills, {} headshots", 
                        events.kills.len(), events.headshots.len());
                }
                Err(e) => {
                    println!("  ❌ Error: {:?}", e);
                }
            }
        }
    }
    
    Ok(())
}
```

### Memory Efficient Processing

```rust
use cs2_demo_core::CS2DemoCore;
use tokio::fs;

async fn process_large_demo(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    
    // Read file in chunks for large demos
    let file = fs::File::open(path).await?;
    let metadata = file.metadata().await?;
    
    if metadata.len() > 500 * 1024 * 1024 { // 500MB
        println!("Large demo detected ({} MB), processing in chunks...", 
            metadata.len() / 1024 / 1024);
    }
    
    let events = demo_core.parse_file(path).await?;
    
    // Process events in batches to avoid memory issues
    let batch_size = 1000;
    for (i, kill_batch) in events.kills.chunks(batch_size).enumerate() {
        println!("Processing kill batch {} ({} kills)", i + 1, kill_batch.len());
        
        for kill in kill_batch {
            // Process individual kill
            if kill.headshot {
                println!("  Headshot: {} killed {} with {}", 
                    kill.killer, kill.victim, kill.weapon);
            }
        }
    }
    
    Ok(())
}
```

## Common Patterns

### Demo Validation

```rust
use cs2_demo_core::{CS2DemoCore, DemoError};
use std::path::Path;

async fn validate_demo(path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Check if file exists
    if !Path::new(path).exists() {
        return Ok(false);
    }
    
    // Check file extension
    if Path::new(path).extension().and_then(|s| s.to_str()) != Some("dem") {
        return Ok(false);
    }
    
    // Try to parse the demo
    let demo_core = CS2DemoCore::new();
    match demo_core.parse_file(path).await {
        Ok(events) => {
            // Additional validation
            if events.kills.is_empty() && events.rounds.is_empty() {
                println!("Warning: Demo appears to be empty");
                return Ok(false);
            }
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}
```

### Demo Comparison

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};

struct DemoComparison {
    demo1_stats: DemoStats,
    demo2_stats: DemoStats,
    differences: Vec<String>,
}

struct DemoStats {
    total_kills: usize,
    total_headshots: usize,
    avg_kills_per_round: f32,
    top_fragger: String,
}

async fn compare_demos(path1: &str, path2: &str) -> Result<DemoComparison, Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    
    let events1 = demo_core.parse_file(path1).await?;
    let events2 = demo_core.parse_file(path2).await?;
    
    let stats1 = extract_demo_stats(&events1);
    let stats2 = extract_demo_stats(&events2);
    
    let mut differences = Vec::new();
    
    if stats1.total_kills != stats2.total_kills {
        differences.push(format!("Total kills: {} vs {}", 
            stats1.total_kills, stats2.total_kills));
    }
    
    if stats1.total_headshots != stats2.total_headshots {
        differences.push(format!("Total headshots: {} vs {}", 
            stats1.total_headshots, stats2.total_headshots));
    }
    
    Ok(DemoComparison {
        demo1_stats: stats1,
        demo2_stats: stats2,
        differences,
    })
}

fn extract_demo_stats(events: &DemoEvents) -> DemoStats {
    let total_kills = events.kills.len();
    let total_headshots = events.headshots.len();
    let avg_kills_per_round = if !events.rounds.is_empty() {
        total_kills as f32 / events.rounds.len() as f32
    } else {
        0.0
    };
    
    let top_fragger = events.players.iter()
        .max_by_key(|(_, player)| player.kills)
        .map(|(_, player)| player.name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    
    DemoStats {
        total_kills,
        total_headshots,
        avg_kills_per_round,
        top_fragger,
    }
}
```

## Integration Examples

### Web API Integration

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};
use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct DemoAnalysisResponse {
    success: bool,
    data: Option<DemoAnalysis>,
    error: Option<String>,
}

#[derive(Serialize)]
struct DemoAnalysis {
    map: String,
    duration: f64,
    total_kills: u16,
    total_headshots: u16,
    final_score: String,
}

async fn analyze_demo_api(Path(demo_path): Path<String>) -> Json<DemoAnalysisResponse> {
    let demo_core = CS2DemoCore::new();
    
    match demo_core.parse_file(&demo_path).await {
        Ok(events) => {
            let analysis = DemoAnalysis {
                map: events.metadata.map,
                duration: events.stats.duration_minutes,
                total_kills: events.stats.total_kills,
                total_headshots: events.stats.total_headshots,
                final_score: format!("T {} - {} CT", 
                    events.stats.final_t_score, 
                    events.stats.final_ct_score),
            };
            
            Json(DemoAnalysisResponse {
                success: true,
                data: Some(analysis),
                error: None,
            })
        }
        Err(e) => {
            Json(DemoAnalysisResponse {
                success: false,
                data: None,
                error: Some(format!("Failed to parse demo: {:?}", e)),
            })
        }
    }
}
```

### Database Integration

```rust
use cs2_demo_core::{CS2DemoCore, DemoEvents};
use sqlx::{PgPool, Row};

async fn save_demo_to_database(pool: &PgPool, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let demo_core = CS2DemoCore::new();
    let events = demo_core.parse_file(path).await?;
    
    // Save demo metadata
    sqlx::query!(
        "INSERT INTO demos (filename, map, duration, total_kills, total_headshots, final_t_score, final_ct_score) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        events.metadata.filename,
        events.metadata.map,
        events.stats.duration_minutes,
        events.stats.total_kills as i32,
        events.stats.total_headshots as i32,
        events.stats.final_t_score as i32,
        events.stats.final_ct_score as i32,
    )
    .execute(pool)
    .await?;
    
    // Save kills
    for kill in &events.kills {
        sqlx::query!(
            "INSERT INTO kills (demo_filename, killer, victim, weapon, tick, headshot) 
             VALUES ($1, $2, $3, $4, $5, $6)",
            events.metadata.filename,
            kill.killer,
            kill.victim,
            kill.weapon,
            kill.tick as i32,
            kill.headshot,
        )
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
```

This usage guide provides comprehensive examples for integrating the CS2 Demo Core library into your applications. For more examples, see the `examples/` directory in the repository.
