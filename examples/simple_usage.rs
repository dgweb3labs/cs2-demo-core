//! Simple usage example for cs2-demo-core
//!
//! This example shows how to use the SDK in your own projects

use cs2_demo_core::{CS2DemoCore, DemoEvents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CS2 Demo Core - Simple Usage");
    println!("============================");

    // 1. Create a new instance
    let demo_core = CS2DemoCore::new();

    // 2. Parse a demo file (replace with your actual demo path)
    let demo_path = "examples/data/sample.dem";
    
    // Check if file exists
    if !std::path::Path::new(demo_path).exists() {
        println!("Demo file not found at: {}", demo_path);
        println!("Please update the path to your actual demo file.");
        println!("Example paths:");
        println!("- Windows: C:\\Steam\\steamapps\\common\\Counter-Strike Global Offensive\\csgo\\replays\\match.dem");
        println!("- Linux: ~/.steam/steam/userdata/*/730/local/cfg/replays/match.dem");
        println!("- Mac: ~/Library/Application Support/Steam/userdata/*/730/local/cfg/replays/match.dem");
        
        // Create empty events for demonstration
        let events = DemoEvents::new();
        print_basic_info(&events);
        return Ok(());
    }

    // 3. Parse the demo
    match demo_core.parse_file(demo_path).await {
        Ok(events) => {
            println!("✅ Demo parsed successfully!");
            print_basic_info(&events);
        }
        Err(e) => {
            eprintln!("❌ Error: {:?}", e);
        }
    }

    Ok(())
}

fn print_basic_info(events: &DemoEvents) {
    println!("\n📊 Basic Info:");
    println!("Map: {}", events.metadata.map);
    println!("Duration: {:.2} minutes", events.stats.duration_minutes);
    println!("Total Kills: {}", events.stats.total_kills);
    println!("Total Headshots: {}", events.stats.total_headshots);
    println!("Final Score: T {} - {} CT", 
             events.stats.final_t_score, 
             events.stats.final_ct_score);
}

// Example of how to use in a library
pub struct DemoAnalyzer {
    core: CS2DemoCore,
}

impl DemoAnalyzer {
    pub fn new() -> Self {
        Self {
            core: CS2DemoCore::new(),
        }
    }

    pub async fn analyze_demo(&self, path: &str) -> Result<DemoEvents, Box<dyn std::error::Error>> {
        self.core.parse_file(path).await.map_err(|e| e.into())
    }

    pub fn get_top_fragger(&self, events: &DemoEvents) -> Option<(String, u16)> {
        let mut player_kills = std::collections::HashMap::new();
        
        for kill in &events.kills {
            *player_kills.entry(kill.killer.clone()).or_insert(0) += 1;
        }
        
        player_kills.into_iter().max_by_key(|(_, kills)| *kills)
    }

    pub fn calculate_headshot_percentage(&self, events: &DemoEvents) -> f32 {
        if events.stats.total_kills == 0 {
            return 0.0;
        }
        (events.stats.total_headshots as f32 / events.stats.total_kills as f32) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyzer() {
        let analyzer = DemoAnalyzer::new();
        let events = DemoEvents::new();
        
        assert_eq!(analyzer.calculate_headshot_percentage(&events), 0.0);
        assert_eq!(analyzer.get_top_fragger(&events), None);
    }
}
