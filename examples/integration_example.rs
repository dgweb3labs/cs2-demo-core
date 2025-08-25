//! Integration example showing how to use cs2-demo-core in different scenarios
//!
//! This example demonstrates:
//! 1. Web API integration
//! 2. Desktop application
//! 3. Mobile app backend
//! 4. Anti-cheat system

use cs2_demo_core::{CS2DemoCore, DemoEvents};
use std::path::Path;
use std::collections::HashMap;

// 1. Web API Integration (NestJS/Node.js equivalent)
pub struct DemoAnalysisAPI {
    core: CS2DemoCore,
}

impl DemoAnalysisAPI {
    pub fn new() -> Self {
        Self {
            core: CS2DemoCore::new(),
        }
    }

    // Endpoint: POST /api/demo/analyze
    pub async fn analyze_demo(&self, demo_path: &str) -> Result<DemoAnalysisResult, String> {
        match self.core.parse_file(demo_path).await {
            Ok(events) => {
                let result = DemoAnalysisResult {
                    success: true,
                    metadata: events.metadata.clone(),
                    stats: events.stats.clone(),
                    highlights: self.extract_highlights(&events),
                    suspicious_activity: self.detect_suspicious_activity(&events),
                };
                Ok(result)
            }
            Err(e) => Err(format!("Failed to parse demo: {:?}", e)),
        }
    }

    fn extract_highlights(&self, events: &DemoEvents) -> Vec<Highlight> {
        let mut highlights = Vec::new();
        
        // Find clutches (1vX situations)
        for clutch in &events.clutches {
            highlights.push(Highlight {
                event_type: "clutch".to_string(),
                timestamp: clutch.timestamp,
                description: format!("{} vs {} players", clutch.player, clutch.opponents),
                importance: 9,
            });
        }
        
        // Find ace rounds (5 kills in one round)
        for round in &events.rounds {
            if round.kills.len() >= 5 {
                highlights.push(Highlight {
                    event_type: "ace".to_string(),
                    timestamp: round.start_time,
                    description: "Ace round!".to_string(),
                    importance: 10,
                });
            }
        }
        
        highlights
    }

    fn detect_suspicious_activity(&self, events: &DemoEvents) -> Vec<SuspiciousActivity> {
        let mut suspicious = Vec::new();
        
        // Check for unrealistic headshot percentages
        for (steam_id, player) in &events.players {
            if player.kills > 10 {
                let hs_percentage = (player.headshots as f32 / player.kills as f32) * 100.0;
                if hs_percentage > 80.0 {
                    suspicious.push(SuspiciousActivity {
                        player: player.name.clone(),
                        activity_type: "high_headshot_percentage".to_string(),
                        confidence: hs_percentage / 100.0,
                        description: format!("{}% headshot rate", hs_percentage),
                    });
                }
            }
        }
        
        suspicious
    }
}

// 2. Desktop Application (Tauri/Electron equivalent)
pub struct DesktopDemoAnalyzer {
    core: CS2DemoCore,
    recent_demos: Vec<String>,
}

impl DesktopDemoAnalyzer {
    pub fn new() -> Self {
        Self {
            core: CS2DemoCore::new(),
            recent_demos: Vec::new(),
        }
    }

    pub async fn analyze_demo_file(&mut self, file_path: &str) -> Result<DemoEvents, String> {
        // Add to recent demos
        if !self.recent_demos.contains(&file_path.to_string()) {
            self.recent_demos.push(file_path.to_string());
            if self.recent_demos.len() > 10 {
                self.recent_demos.remove(0);
            }
        }

        self.core.parse_file(file_path).await.map_err(|e| format!("{:?}", e))
    }

    pub fn get_recent_demos(&self) -> &Vec<String> {
        &self.recent_demos
    }

    pub fn export_to_json(&self, events: &DemoEvents) -> Result<String, String> {
        serde_json::to_string_pretty(events).map_err(|e| format!("JSON error: {:?}", e))
    }
}

// 3. Mobile App Backend (Flutter/Dart equivalent)
pub struct MobileDemoService {
    core: CS2DemoCore,
    cache: HashMap<String, DemoEvents>,
}

impl MobileDemoService {
    pub fn new() -> Self {
        Self {
            core: CS2DemoCore::new(),
            cache: HashMap::new(),
        }
    }

    pub async fn get_demo_summary(&mut self, demo_path: &str) -> Result<DemoSummary, String> {
        // Check cache first
        if let Some(events) = self.cache.get(demo_path) {
            return Ok(self.create_summary(events));
        }

        // Parse demo
        let events = self.core.parse_file(demo_path).await.map_err(|e| format!("{:?}", e))?;
        
        // Cache result
        self.cache.insert(demo_path.to_string(), events.clone());
        
        Ok(self.create_summary(&events))
    }

    fn create_summary(&self, events: &DemoEvents) -> DemoSummary {
        DemoSummary {
            map: events.metadata.map.clone(),
            duration: events.stats.duration_minutes,
            total_kills: events.stats.total_kills,
            total_headshots: events.stats.total_headshots,
            final_score: format!("{} - {}", events.stats.final_t_score, events.stats.final_ct_score),
            top_player: self.get_top_player(events),
        }
    }

    fn get_top_player(&self, events: &DemoEvents) -> Option<String> {
        events.players.iter()
            .max_by_key(|(_, player)| player.kills)
            .map(|(_, player)| player.name.clone())
    }
}

// 4. Anti-Cheat System (VAC/Faceit equivalent)
pub struct AntiCheatAnalyzer {
    core: CS2DemoCore,
    detection_rules: Vec<DetectionRule>,
}

impl AntiCheatAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            core: CS2DemoCore::new(),
            detection_rules: Vec::new(),
        };
        
        // Add detection rules
        analyzer.add_detection_rule(DetectionRule {
            name: "high_headshot_percentage".to_string(),
            threshold: 0.8,
            description: "Player has >80% headshot rate".to_string(),
        });
        
        analyzer.add_detection_rule(DetectionRule {
            name: "impossible_angles".to_string(),
            threshold: 0.9,
            description: "Player making impossible shots".to_string(),
        });
        
        analyzer
    }

    pub fn add_detection_rule(&mut self, rule: DetectionRule) {
        self.detection_rules.push(rule);
    }

    pub async fn analyze_for_cheats(&self, demo_path: &str) -> Result<CheatAnalysis, String> {
        let events = self.core.parse_file(demo_path).await.map_err(|e| format!("{:?}", e))?;
        
        let mut detections = Vec::new();
        let mut overall_risk = 0.0;
        
        for rule in &self.detection_rules {
            let risk_score = self.evaluate_rule(rule, &events);
            if risk_score > rule.threshold {
                detections.push(Detection {
                    rule_name: rule.name.clone(),
                    risk_score,
                    description: rule.description.clone(),
                });
                overall_risk = overall_risk.max(risk_score);
            }
        }
        
        Ok(CheatAnalysis {
            demo_path: demo_path.to_string(),
            overall_risk,
            detections,
            total_players: events.players.len(),
            suspicious_players: detections.len(),
        })
    }

    fn evaluate_rule(&self, rule: &DetectionRule, events: &DemoEvents) -> f32 {
        match rule.name.as_str() {
            "high_headshot_percentage" => {
                let mut max_hs_rate = 0.0;
                for (_, player) in &events.players {
                    if player.kills > 5 {
                        let hs_rate = player.headshots as f32 / player.kills as f32;
                        max_hs_rate = max_hs_rate.max(hs_rate);
                    }
                }
                max_hs_rate
            }
            "impossible_angles" => {
                // Placeholder for angle analysis
                0.0
            }
            _ => 0.0
        }
    }
}

// Data structures
#[derive(Debug, Clone)]
pub struct DemoAnalysisResult {
    pub success: bool,
    pub metadata: crate::events::DemoMetadata,
    pub stats: crate::events::MatchStats,
    pub highlights: Vec<Highlight>,
    pub suspicious_activity: Vec<SuspiciousActivity>,
}

#[derive(Debug, Clone)]
pub struct Highlight {
    pub event_type: String,
    pub timestamp: f32,
    pub description: String,
    pub importance: u8,
}

#[derive(Debug, Clone)]
pub struct SuspiciousActivity {
    pub player: String,
    pub activity_type: String,
    pub confidence: f32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DemoSummary {
    pub map: String,
    pub duration: f64,
    pub total_kills: u16,
    pub total_headshots: u16,
    pub final_score: String,
    pub top_player: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub name: String,
    pub threshold: f32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub rule_name: String,
    pub risk_score: f32,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct CheatAnalysis {
    pub demo_path: String,
    pub overall_risk: f32,
    pub detections: Vec<Detection>,
    pub total_players: usize,
    pub suspicious_players: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CS2 Demo Core - Integration Examples");
    println!("====================================");

    let demo_path = "examples/data/sample.dem";

    if !Path::new(demo_path).exists() {
        println!("Demo file not found. Running examples with empty data...");
        
        // Test with empty data
        let api = DemoAnalysisAPI::new();
        let desktop = DesktopDemoAnalyzer::new();
        let mobile = MobileDemoService::new();
        let anticheat = AntiCheatAnalyzer::new();
        
        println!("✅ All components initialized successfully!");
        return Ok(());
    }

    println!("Testing with demo: {}", demo_path);

    // 1. Web API Example
    println!("\n🌐 Web API Example:");
    let api = DemoAnalysisAPI::new();
    match api.analyze_demo(demo_path).await {
        Ok(result) => println!("✅ API Analysis: {} highlights found", result.highlights.len()),
        Err(e) => println!("❌ API Error: {}", e),
    }

    // 2. Desktop App Example
    println!("\n🖥️ Desktop App Example:");
    let mut desktop = DesktopDemoAnalyzer::new();
    match desktop.analyze_demo_file(demo_path).await {
        Ok(events) => println!("✅ Desktop Analysis: {} kills found", events.kills.len()),
        Err(e) => println!("❌ Desktop Error: {}", e),
    }

    // 3. Mobile Service Example
    println!("\n📱 Mobile Service Example:");
    let mut mobile = MobileDemoService::new();
    match mobile.get_demo_summary(demo_path).await {
        Ok(summary) => println!("✅ Mobile Summary: {} - {}", summary.map, summary.final_score),
        Err(e) => println!("❌ Mobile Error: {}", e),
    }

    // 4. Anti-Cheat Example
    println!("\n🛡️ Anti-Cheat Example:");
    let anticheat = AntiCheatAnalyzer::new();
    match anticheat.analyze_for_cheats(demo_path).await {
        Ok(analysis) => println!("✅ Anti-Cheat: {} suspicious players", analysis.suspicious_players),
        Err(e) => println!("❌ Anti-Cheat Error: {}", e),
    }

    println!("\n🎉 All integration examples completed!");
    Ok(())
}
