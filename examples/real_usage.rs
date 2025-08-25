use cs2_demo_core::{CS2DemoCore, DemoEvents};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("CS2 Demo Core - Real Usage Example");
    println!("===================================");

    // Create a new CS2 Demo Core instance
    let demo_core = CS2DemoCore::new();

    // Demo file path - replace with your actual demo file
    let demo_path = "examples/data/sample.dem";

    // Check if demo file exists
    if !Path::new(demo_path).exists() {
        println!("Demo file '{}' not found.", demo_path);
        println!("Please update the path to your actual demo file.");
        println!("Common demo locations:");
        println!("- Steam: steam://rungame/730/76561202255233023/+csgo_download_match");
        println!("- CSStats.gg: https://csstats.gg/");
        println!("- HLTV: https://www.hltv.org/");
        println!("- Local: ~/.steam/steam/userdata/*/730/local/cfg/replays/");
        return Ok(());
    }

    println!("Parsing demo file: {}", demo_path);

    // Parse the demo file
    match demo_core.parse_file(demo_path).await {
        Ok(events) => {
            println!("\n✅ Demo parsed successfully!");
            print_demo_summary(&events);
        }
        Err(e) => {
            eprintln!("❌ Error parsing demo: {:?}", e);
        }
    }

    Ok(())
}

fn print_demo_summary(events: &DemoEvents) {
    println!("\n📊 Demo Summary:");
    println!("=================");
    
    // Basic info
    println!("Map: {}", events.metadata.map);
    println!("Server: {}", events.metadata.server);
    println!("Duration: {:.2} minutes", events.stats.duration_minutes);
    println!("Total Rounds: {}", events.stats.total_rounds);
    
    // Scores
    println!("\n🏆 Final Score:");
    println!("T: {} | CT: {}", events.stats.final_t_score, events.stats.final_ct_score);
    
    // Statistics
    println!("\n📈 Match Statistics:");
    println!("Total Kills: {}", events.stats.total_kills);
    println!("Total Headshots: {}", events.stats.total_headshots);
    println!("Average Kills per Round: {:.2}", events.stats.avg_kills_per_round);
    
    // Events breakdown
    println!("\n🎯 Events Breakdown:");
    println!("Kills: {}", events.kills.len());
    println!("Headshots: {}", events.headshots.len());
    println!("Clutches: {}", events.clutches.len());
    println!("Rounds: {}", events.rounds.len());
    
    // Players
    println!("\n👥 Players ({})", events.players.len());
    for (steam_id, player) in &events.players {
        println!("  {}: {} kills, {} deaths", player.name, player.kills, player.deaths);
    }
    
    // Top fraggers
    if !events.kills.is_empty() {
        println!("\n🔥 Top Fraggers:");
        let mut player_kills: std::collections::HashMap<String, u16> = std::collections::HashMap::new();
        
        for kill in &events.kills {
            *player_kills.entry(kill.killer.clone()).or_insert(0) += 1;
        }
        
        let mut sorted_players: Vec<_> = player_kills.iter().collect();
        sorted_players.sort_by(|a, b| b.1.cmp(a.1));
        
        for (i, (player, kills)) in sorted_players.iter().take(5).enumerate() {
            println!("  {}. {}: {} kills", i + 1, player, kills);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_demo_core_creation() {
        let demo_core = CS2DemoCore::new();
        assert!(demo_core.parser().is_some());
    }

    #[tokio::test]
    async fn test_empty_events() {
        let events = DemoEvents::new();
        assert_eq!(events.kills.len(), 0);
        assert_eq!(events.headshots.len(), 0);
        assert_eq!(events.clutches.len(), 0);
    }
}
