use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main events container for a CS2 demo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoEvents {
    /// Demo metadata
    pub metadata: DemoMetadata,
    /// All kills in the demo
    pub kills: Vec<Kill>,
    /// All headshots in the demo
    pub headshots: Vec<Headshot>,
    /// All clutches in the demo
    pub clutches: Vec<Clutch>,
    /// All rounds in the demo
    pub rounds: Vec<Round>,
    /// All players in the demo
    pub players: HashMap<String, Player>,
    /// Match statistics
    pub stats: MatchStats,
}

/// Demo metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoMetadata {
    /// Demo file name
    pub filename: String,
    /// Demo version
    pub version: String,
    /// Map name
    pub map: String,
    /// Server name
    pub server: String,
    /// Demo duration in seconds
    pub duration: f32,
    /// Number of ticks
    pub ticks: u32,
    /// Demo start time
    pub start_time: Option<String>,
}

/// Kill event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kill {
    /// Killer player name
    pub killer: String,
    /// Victim player name
    pub victim: String,
    /// Weapon used
    pub weapon: String,
    /// Whether it was a headshot
    pub headshot: bool,
    /// Round number
    pub round: u8,
    /// Tick when kill occurred
    pub tick: u32,
    /// Position of killer
    pub killer_pos: Option<Position>,
    /// Position of victim
    pub victim_pos: Option<Position>,
    /// Distance of the kill
    pub distance: Option<f32>,
}

/// Headshot event (subset of kills)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Headshot {
    /// Shooter player name
    pub shooter: String,
    /// Target player name
    pub target: String,
    /// Weapon used
    pub weapon: String,
    /// Round number
    pub round: u8,
    /// Tick when headshot occurred
    pub tick: u32,
    /// Position of shooter
    pub shooter_pos: Option<Position>,
    /// Position of target
    pub target_pos: Option<Position>,
    /// Distance of the headshot
    pub distance: Option<f32>,
}

/// Clutch event (1vX situations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clutch {
    /// Player who clutched
    pub player: String,
    /// Number of enemies in the clutch
    pub enemies: u8,
    /// Whether clutch was successful
    pub successful: bool,
    /// Round number
    pub round: u8,
    /// Start tick of clutch
    pub start_tick: u32,
    /// End tick of clutch
    pub end_tick: u32,
    /// Duration in seconds
    pub duration: f32,
}

/// Round information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    /// Round number
    pub number: u8,
    /// Winner team (T or CT)
    pub winner: String,
    /// Score for terrorist team
    pub t_score: u8,
    /// Score for counter-terrorist team
    pub ct_score: u8,
    /// Round duration in seconds
    pub duration: f32,
    /// Start tick
    pub start_tick: u32,
    /// End tick
    pub end_tick: u32,
    /// Win condition
    pub win_condition: WinCondition,
}

/// Win condition types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WinCondition {
    /// Team eliminated
    Elimination,
    /// Bomb exploded
    BombExploded,
    /// Bomb defused
    BombDefused,
    /// Time ran out
    TimeExpired,
    /// Target saved
    TargetSaved,
    /// Hostage rescued
    HostageRescued,
    /// Unknown condition
    Unknown,
}

/// Player information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// Player name
    pub name: String,
    /// Steam ID
    pub steam_id: Option<String>,
    /// Team (T or CT)
    pub team: String,
    /// Total kills
    pub kills: u16,
    /// Total deaths
    pub deaths: u16,
    /// Total assists
    pub assists: u16,
    /// Headshot percentage
    pub headshot_percentage: f32,
    /// Average damage per round
    pub adr: f32,
    /// Kill/death ratio
    pub kdr: f32,
}

/// 3D position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Match statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStats {
    /// Total rounds played
    pub total_rounds: u8,
    /// Final T score
    pub final_t_score: u8,
    /// Final CT score
    pub final_ct_score: u8,
    /// Total kills in match
    pub total_kills: u16,
    /// Total headshots in match
    pub total_headshots: u16,
    /// Average kills per round
    pub avg_kills_per_round: f32,
    /// Match duration in minutes
    pub duration_minutes: f64,
}

/// Game event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    /// Kill event
    Kill(Kill),
    /// Headshot event
    Headshot(Headshot),
    /// Clutch event
    Clutch(Clutch),
    /// Round event
    Round(Round),
}

impl DemoEvents {
    /// Create a new empty DemoEvents
    pub fn new() -> Self {
        Self {
            metadata: DemoMetadata {
                filename: String::new(),
                version: String::new(),
                map: String::new(),
                server: String::new(),
                duration: 0.0,
                ticks: 0,
                start_time: None,
            },
            kills: Vec::new(),
            headshots: Vec::new(),
            clutches: Vec::new(),
            rounds: Vec::new(),
            players: HashMap::new(),
            stats: MatchStats {
                total_rounds: 0,
                final_t_score: 0,
                final_ct_score: 0,
                total_kills: 0,
                total_headshots: 0,
                avg_kills_per_round: 0.0,
                duration_minutes: 0.0,
            },
        }
    }
    
    /// Get all events in chronological order
    pub fn all_events(&self) -> Vec<GameEvent> {
        let mut events = Vec::new();
        
        // Add kills
        for kill in &self.kills {
            events.push(GameEvent::Kill(kill.clone()));
        }
        
        // Add headshots
        for hs in &self.headshots {
            events.push(GameEvent::Headshot(hs.clone()));
        }
        
        // Add clutches
        for clutch in &self.clutches {
            events.push(GameEvent::Clutch(clutch.clone()));
        }
        
        // Add rounds
        for round in &self.rounds {
            events.push(GameEvent::Round(round.clone()));
        }
        
        // Sort by tick
        events.sort_by(|a, b| {
            let tick_a = match a {
                GameEvent::Kill(k) => k.tick,
                GameEvent::Headshot(hs) => hs.tick,
                GameEvent::Clutch(c) => c.start_tick,
                GameEvent::Round(r) => r.start_tick,
            };
            let tick_b = match b {
                GameEvent::Kill(k) => k.tick,
                GameEvent::Headshot(hs) => hs.tick,
                GameEvent::Clutch(c) => c.start_tick,
                GameEvent::Round(r) => r.start_tick,
            };
            tick_a.cmp(&tick_b)
        });
        
        events
    }
    
    /// Get events for a specific round
    pub fn events_for_round(&self, round_number: u8) -> Vec<GameEvent> {
        self.all_events()
            .into_iter()
            .filter(|event| {
                match event {
                    GameEvent::Kill(k) => k.round == round_number,
                    GameEvent::Headshot(hs) => hs.round == round_number,
                    GameEvent::Clutch(c) => c.round == round_number,
                    GameEvent::Round(r) => r.number == round_number,
                }
            })
            .collect()
    }
    
    /// Get player statistics
    pub fn get_player_stats(&self, player_name: &str) -> Option<&Player> {
        self.players.get(player_name)
    }
    
    /// Get top fraggers (players with most kills)
    pub fn top_fraggers(&self, limit: usize) -> Vec<(&String, u16)> {
        let mut players: Vec<_> = self.players.iter()
            .map(|(name, player)| (name, player.kills))
            .collect();
        
        players.sort_by(|a, b| b.1.cmp(&a.1));
        players.truncate(limit);
        players
    }
}

impl Default for DemoEvents {
    fn default() -> Self {
        Self::new()
    }
}
