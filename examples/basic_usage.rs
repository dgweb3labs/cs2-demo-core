use cs2_demo_core::{CS2DemoCore, DemoEvents};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("CS2 Demo Core - Basic Usage Example");
    println!("===================================");
    
    // Create a new CS2 Demo Core instance
    let demo_core = CS2DemoCore::new();
    
    // Example demo file path (you would replace this with an actual demo file)
    let demo_path = "example.dem";
    
    // Check if demo file exists
    if !Path::new(demo_path).exists() {
        println!("Demo file '{}' not found. Please provide a valid CS2 demo file.", demo_path);
        println!("You can download demo files from:");
        println!("- Steam: steam://rungame/730/76561202255233023/+csgo_download_match");
        println!("- CSStats.gg: https://csstats.gg/");
        println!("- HLTV: https://www.hltv.org/");
        return Ok(());
    }
    
    println!("Parsing demo file: {}", demo_path);
    
    // Parse the demo file
    match demo_core.parse_file(demo_path).await {
        Ok(events) => {
            println!("\nDemo parsed successfully!");
            print_demo_summary(&events);
        }
        Err(e) => {
            eprintln!("Error parsing demo: {:?}", e);
        }
    }
    
    Ok(())
}

fn print_demo_summary(events: &DemoEvents) {
    println!("\nDemo Summary:");
    println!("=============");
    
    // Basic metadata
    println!("Map: {}", events.metadata.map);
    println!("Server: {}", events.metadata.server);
    println!("Duration: {:.2} minutes", events.metadata.duration / 60.0);
    println!("Ticks: {}", events.metadata.ticks);
    
    // Statistics
    println!("\nMatch Statistics:");
    println!("Total Rounds: {}", events.stats.total_rounds);
    println!("Total Kills: {}", events.stats.total_kills);
    println!("Total Headshots: {}", events.stats.total_headshots);
    println!("Average Kills per Round: {:.2}", events.stats.avg_kills_per_round);
    println!("Final Score - T: {} | CT: {}", events.stats.final_t_score, events.stats.final_ct_score);
    
    // Player statistics
    if !events.players.is_empty() {
        println!("\nTop Players:");
        let top_fraggers = events.top_fraggers(5);
        for (i, (name, kills)) in top_fraggers.iter().enumerate() {
            if let Some(player) = events.get_player_stats(name) {
                println!("{}. {} - Kills: {}, Deaths: {}, K/D: {:.2}, HS%: {:.1}%", 
                    i + 1, name, kills, player.deaths, player.kdr, player.headshot_percentage);
            }
        }
    }
    
    // Recent events
    if !events.kills.is_empty() {
        println!("\nRecent Kills:");
        let recent_kills = events.kills.iter().rev().take(5);
        for kill in recent_kills {
            println!("{} killed {} with {} ({})", 
                kill.killer, kill.victim, kill.weapon, 
                if kill.headshot { "Headshot" } else { "Body" });
        }
    }
    
    // Clutches
    if !events.clutches.is_empty() {
        println!("\nClutches:");
        for clutch in &events.clutches {
            let result = if clutch.successful { "WON" } else { "LOST" };
            println!("{} {} a {}-v-{} clutch in round {}", 
                clutch.player, result, clutch.enemies, clutch.enemies, clutch.round);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_demo_core_creation() {
        let demo_core = CS2DemoCore::new();
        assert!(demo_core.parser().options.extract_positions);
    }
}
