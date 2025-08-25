use crate::error::Result;
use crate::events::{DemoEvents, Kill, Headshot, Clutch, Round, Player, Position};
use crate::parser::protobuf_parser::{DemoMessage, GameEvent, PlayerInfo, RoundInfo};
use tracing::{debug, info};

/// Event extractor for CS2 demo events
pub struct EventExtractor {
    /// Current round number
    current_round: u8,
    /// Current tick
    current_tick: u32,
    /// Players in the current round
    players: std::collections::HashMap<u32, Player>,
    /// Kills in current round
    round_kills: Vec<Kill>,
    /// Headshots in current round
    round_headshots: Vec<Headshot>,
}

impl EventExtractor {
    /// Create a new event extractor
    pub fn new() -> Self {
        Self {
            current_round: 0,
            current_tick: 0,
            players: std::collections::HashMap::new(),
            round_kills: Vec::new(),
            round_headshots: Vec::new(),
        }
    }
    
    /// Extract events from protobuf messages
    pub fn extract_events(&mut self, messages: Vec<DemoMessage>) -> Result<DemoEvents> {
        let mut events = DemoEvents::new();
        
        info!("Extracting events from {} messages", messages.len());
        
        for message in messages {
            match message {
                DemoMessage::Header(header) => {
                    self.extract_metadata(&header, &mut events)?;
                }
                DemoMessage::GameEvent(game_event) => {
                    self.extract_game_event(&game_event, &mut events)?;
                }
                DemoMessage::Player(player_info) => {
                    self.extract_player_info(&player_info, &mut events)?;
                }
                DemoMessage::Round(round_info) => {
                    self.extract_round_info(&round_info, &mut events)?;
                }
                DemoMessage::Unknown(data) => {
                    debug!("Skipping unknown message of {} bytes", data.len());
                }
            }
        }
        
        // Process any remaining events
        self.finalize_events(&mut events)?;
        
        info!("Extracted {} kills, {} headshots, {} rounds", 
              events.kills.len(), events.headshots.len(), events.rounds.len());
        
        Ok(events)
    }
    
    /// Extract metadata from demo header
    fn extract_metadata(&self, header: &crate::parser::protobuf_parser::DemoHeader, events: &mut DemoEvents) -> Result<()> {
        events.metadata.version = header.version.to_string();
        events.metadata.map = header.map_name.clone();
        events.metadata.server = header.server_name.clone();
        events.metadata.duration = header.playback_time as f32;
        events.metadata.ticks = header.playback_ticks;
        
        debug!("Extracted metadata: map={}, duration={}s, ticks={}", 
               events.metadata.map, events.metadata.duration, events.metadata.ticks);
        
        Ok(())
    }
    
    /// Extract game events
    fn extract_game_event(&mut self, game_event: &GameEvent, _events: &mut DemoEvents) -> Result<()> {
        self.current_tick = game_event.tick;
        
        // TODO: Implement actual game event parsing
        // This would involve parsing the protobuf data to extract:
        // - Kill events
        // - Headshot events  
        // - Clutch situations
        // - Round events
        
        debug!("Processing game event at tick {}", self.current_tick);
        
        Ok(())
    }
    
    /// Extract player information
    fn extract_player_info(&self, player_info: &PlayerInfo, events: &mut DemoEvents) -> Result<()> {
        let player = Player {
            name: player_info.name.clone(),
            steam_id: Some(player_info.guid.clone()),
            team: String::new(), // Will be determined from game events
            kills: 0,
            deaths: 0,
            assists: 0,
            headshot_percentage: 0.0,
            adr: 0.0,
            kdr: 0.0,
        };
        
        events.players.insert(player_info.name.clone(), player);
        
        debug!("Extracted player: {}", player_info.name);
        
        Ok(())
    }
    
    /// Extract round information
    fn extract_round_info(&mut self, round_info: &RoundInfo, events: &mut DemoEvents) -> Result<()> {
        self.current_round = round_info.round_number;
        
        let round = Round {
            number: round_info.round_number,
            winner: match round_info.winner {
                2 => "T".to_string(),
                3 => "CT".to_string(),
                _ => "Unknown".to_string(),
            },
            t_score: 0, // Will be calculated from kills
            ct_score: 0, // Will be calculated from kills
            duration: round_info.duration,
            start_tick: self.current_tick,
            end_tick: self.current_tick,
            win_condition: self.determine_win_condition(round_info.reason),
        };
        
        events.rounds.push(round.clone());
        
        debug!("Extracted round {}: winner={}, duration={}s", 
               round_info.round_number, round.winner, round_info.duration);
        
        Ok(())
    }
    
    /// Determine win condition from reason code
    fn determine_win_condition(&self, reason: u8) -> crate::events::WinCondition {
        match reason {
            1 => crate::events::WinCondition::Elimination,
            2 => crate::events::WinCondition::BombExploded,
            3 => crate::events::WinCondition::BombDefused,
            4 => crate::events::WinCondition::TimeExpired,
            5 => crate::events::WinCondition::TargetSaved,
            6 => crate::events::WinCondition::HostageRescued,
            _ => crate::events::WinCondition::Unknown,
        }
    }
    
    /// Finalize events and calculate statistics
    fn finalize_events(&mut self, events: &mut DemoEvents) -> Result<()> {
        // Calculate match statistics
        events.stats.total_rounds = events.rounds.len() as u8;
        events.stats.total_kills = events.kills.len() as u16;
        events.stats.total_headshots = events.headshots.len() as u16;
        
        if events.stats.total_rounds > 0 {
            events.stats.avg_kills_per_round = events.stats.total_kills as f32 / events.stats.total_rounds as f32;
        }
        
        if events.metadata.duration > 0.0 {
            events.stats.duration_minutes = events.metadata.duration as f64 / 60.0;
        }
        
        // Calculate player statistics
        for player in events.players.values_mut() {
            if player.deaths > 0 {
                player.kdr = player.kills as f32 / player.deaths as f32;
            }
            
            if player.kills > 0 {
                player.headshot_percentage = (player.kills as f32 / player.kills as f32) * 100.0;
            }
        }
        
        // Calculate final scores
        if let Some(last_round) = events.rounds.last() {
            events.stats.final_t_score = last_round.t_score;
            events.stats.final_ct_score = last_round.ct_score;
        }
        
        debug!("Finalized events: {} rounds, {} kills, {} headshots", 
               events.stats.total_rounds, events.stats.total_kills, events.stats.total_headshots);
        
        Ok(())
    }
    
    /// Detect clutch situations (1vX)
    fn detect_clutches(&self, _kills: &[Kill], _round: u8) -> Vec<Clutch> {
        let clutches = Vec::new();
        
        // TODO: Implement clutch detection logic
        // This would involve:
        // 1. Tracking alive players per team
        // 2. Detecting when one player is left vs multiple enemies
        // 3. Determining if the clutch was successful
        
        clutches
    }
    
    /// Calculate distance between two positions
    fn calculate_distance(&self, pos1: &Position, pos2: &Position) -> f32 {
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        let dz = pos1.z - pos2.z;
        
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for EventExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_extractor_creation() {
        let extractor = EventExtractor::new();
        assert_eq!(extractor.current_round, 0);
        assert_eq!(extractor.current_tick, 0);
    }
    
    #[test]
    fn test_determine_win_condition() {
        let extractor = EventExtractor::new();
        
        assert!(matches!(extractor.determine_win_condition(1), crate::events::WinCondition::Elimination));
        assert!(matches!(extractor.determine_win_condition(2), crate::events::WinCondition::BombExploded));
        assert!(matches!(extractor.determine_win_condition(3), crate::events::WinCondition::BombDefused));
        assert!(matches!(extractor.determine_win_condition(99), crate::events::WinCondition::Unknown));
    }
    
    #[test]
    fn test_calculate_distance() {
        let extractor = EventExtractor::new();
        
        let pos1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos2 = Position { x: 3.0, y: 4.0, z: 0.0 };
        
        let distance = extractor.calculate_distance(&pos1, &pos2);
        assert_eq!(distance, 5.0);
    }
}
