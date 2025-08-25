use crate::error::{DemoError, Result};
use crate::events::{DemoEvents, DemoMetadata, Kill, Headshot, Round, Player, WinCondition, MatchStats};
use crate::parser::protobuf_parser::{ProtobufParser, DemoMessage, DemoHeader, GameEvent, PlayerInfo, RoundInfo};
use crate::parser::event_extractor::EventExtractor;
use crate::utils::validation::validate_demo_file;
use std::path::Path;


/// Options for demo parsing
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// Whether to extract detailed player positions
    pub extract_positions: bool,
    /// Whether to calculate advanced statistics
    pub calculate_stats: bool,
    /// Maximum number of events to parse (0 = unlimited)
    pub max_events: usize,
    /// Whether to validate demo file format
    pub validate_format: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            extract_positions: true,
            calculate_stats: true,
            max_events: 0,
            validate_format: true,
        }
    }
}

/// Main CS2 demo parser
pub struct CS2Parser {
    #[allow(dead_code)]
    options: ParseOptions,
}

impl CS2Parser {
    /// Create a new CS2 parser with default options
    pub fn new() -> Self {
        Self {
            options: ParseOptions::default(),
        }
    }

    /// Create a new CS2 parser with custom options
    pub fn with_options(options: ParseOptions) -> Self {
        Self { options }
    }

    /// Parse a demo file asynchronously
    pub async fn parse_file_async<P: AsRef<Path>>(&self, path: P) -> Result<DemoEvents> {
        let path = path.as_ref();
        
        // Validate file if requested
        if self.options.validate_format {
            validate_demo_file(path)?;
        }

        // Read file data
        let data = tokio::fs::read(path).await
            .map_err(|e| DemoError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to read demo file: {}", e))))?;

        self.parse_bytes_async(data).await
    }

    /// Parse demo data from bytes asynchronously
    pub async fn parse_bytes_async(&self, data: Vec<u8>) -> Result<DemoEvents> {
        // Use tokio::task::spawn_blocking for CPU-intensive parsing
        let options = self.options.clone();
        
        tokio::task::spawn_blocking(move || {
            let parser = CS2Parser::with_options(options);
            parser.parse_bytes_sync(data)
        }).await
            .map_err(|e| DemoError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("Task join error: {}", e))))?
    }

    /// Parse demo data from bytes synchronously
    pub fn parse_bytes_sync(&self, data: Vec<u8>) -> Result<DemoEvents> {
        // Create protobuf parser
        let mut protobuf_parser = ProtobufParser::new(data);
        
        // Parse all messages
        let messages = protobuf_parser.parse_all()?;
        
        // Extract events from messages
        let mut event_extractor = EventExtractor::new();
        let mut events = DemoEvents::default();
        
        for message in messages {
            match message {
                DemoMessage::Header(header) => {
                    events.metadata = self.extract_metadata_from_header(header)?;
                },
                DemoMessage::GameEvent(game_event) => {
                    self.process_game_event(&mut event_extractor, &mut events, game_event)?;
                },
                DemoMessage::PlayerInfo(player_info) => {
                    self.process_player_info(&mut event_extractor, &mut events, player_info)?;
                },
                DemoMessage::RoundInfo(round_info) => {
                    self.process_round_info(&mut event_extractor, &mut events, round_info)?;
                },
                DemoMessage::Unknown { field_id, data } => {
                    // Log unknown fields for debugging
                    tracing::debug!("Unknown protobuf field: {} with {} bytes", field_id, data.len());
                }
            }
        }
        
        // Calculate final statistics
        if self.options.calculate_stats {
            events.stats = self.calculate_match_stats(&events);
        }
        
        Ok(events)
    }

    /// Extract metadata from demo header
    fn extract_metadata_from_header(&self, header: DemoHeader) -> Result<DemoMetadata> {
        Ok(DemoMetadata {
            filename: String::new(),
            version: header.version.to_string(),
            map: header.map_name,
            server: header.server_name,
            duration: header.duration,
            ticks: header.tick_count,
            start_time: None,
        })
    }

    /// Process a game event
    fn process_game_event(&self, _extractor: &mut EventExtractor, events: &mut DemoEvents, game_event: GameEvent) -> Result<()> {
        // Extract kills from game events
        if let Some(kill_data) = game_event.data.get("kill") {
            if let Ok(kill) = self.parse_kill_event(kill_data, game_event.timestamp) {
                events.kills.push(kill.clone());
                
                // Check for headshot
                if let Some(headshot_data) = game_event.data.get("headshot") {
                    if headshot_data == "true" {
                        let headshot = Headshot {
                            shooter: kill.killer.clone(),
                            target: kill.victim.clone(),
                            weapon: kill.weapon.clone(),
                            round: 1, // TODO: Get actual round
                            tick: game_event.timestamp as u32,
                            shooter_pos: None,
                            target_pos: None,
                            distance: Some(0.0), // TODO: Calculate distance
                        };
                        events.headshots.push(headshot);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Process player information
    fn process_player_info(&self, _extractor: &mut EventExtractor, events: &mut DemoEvents, player_info: PlayerInfo) -> Result<()> {
        let player_name = player_info.name.clone();
        let player = Player {
            name: player_name.clone(),
            steam_id: Some(player_info.steam_id.to_string()),
            team: player_info.team.to_string(),
            kills: player_info.kills as u16,
            deaths: player_info.deaths as u16,
            assists: player_info.assists as u16,
            headshot_percentage: 0.0,
            adr: 0.0,
            kdr: 0.0,
        };
        
        events.players.insert(player_name, player);
        Ok(())
    }

    /// Process round information
    fn process_round_info(&self, _extractor: &mut EventExtractor, events: &mut DemoEvents, round_info: RoundInfo) -> Result<()> {
        let round = Round {
            number: round_info.round_number as u8,
            winner: match round_info.winner {
                WinCondition::Elimination => "T".to_string(),
                WinCondition::BombExploded => "T".to_string(),
                WinCondition::BombDefused => "CT".to_string(),
                WinCondition::TimeExpired => "Unknown".to_string(),
                WinCondition::TargetSaved => "CT".to_string(),
                WinCondition::HostageRescued => "CT".to_string(),
                WinCondition::Unknown => "Unknown".to_string(),
            },
            t_score: round_info.t_score as u8,
            ct_score: round_info.ct_score as u8,
            duration: round_info.end_time - round_info.start_time,
            start_tick: round_info.start_time as u32,
            end_tick: round_info.end_time as u32,
            win_condition: round_info.winner,
        };
        
        events.rounds.push(round);
        
        Ok(())
    }

    /// Parse a kill event from game event data
    fn parse_kill_event(&self, _kill_data: &str, timestamp: f32) -> Result<Kill> {
        // TODO: Implement real kill event parsing
        // For now, return a placeholder
        Ok(Kill {
            killer: "Unknown".to_string(),
            victim: "Unknown".to_string(),
            weapon: "Unknown".to_string(),
            headshot: false,
            round: 1,
            tick: timestamp as u32,
            killer_pos: None,
            victim_pos: None,
            distance: Some(0.0),
        })
    }

    /// Calculate match statistics
    fn calculate_match_stats(&self, events: &DemoEvents) -> MatchStats {
        let total_kills = events.kills.len() as u32;
        let total_headshots = events.headshots.len() as u32;
        let total_rounds = events.rounds.len() as u32;
        
        let _headshot_percentage = if total_kills > 0 {
            (total_headshots as f32 / total_kills as f32) * 100.0
        } else {
            0.0
        };
        
        let _avg_round_duration = if total_rounds > 0 {
            events.rounds.iter()
                .map(|r| r.duration)
                .sum::<f32>() / total_rounds as f32
        } else {
            0.0
        };
        
        MatchStats {
            total_rounds: total_rounds as u8,
            final_t_score: events.rounds.last().map(|r| r.t_score as u8).unwrap_or(0),
            final_ct_score: events.rounds.last().map(|r| r.ct_score as u8).unwrap_or(0),
            total_kills: total_kills as u16,
            total_headshots: total_headshots as u16,
            avg_kills_per_round: if total_rounds > 0 { total_kills as f32 / total_rounds as f32 } else { 0.0 },
            duration_minutes: events.metadata.duration as f64 / 60.0,
        }
    }
}
