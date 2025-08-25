use crate::error::{DemoError, Result};

use tracing::{debug, warn};

/// Protobuf message types for CS2 demos
#[derive(Debug)]
pub enum DemoMessage {
    /// Demo header information
    Header(DemoHeader),
    /// Game event
    GameEvent(GameEvent),
    /// Player information
    Player(PlayerInfo),
    /// Round information
    Round(RoundInfo),
    /// Unknown message type
    Unknown(Vec<u8>),
}

/// Demo header structure
#[derive(Debug, Clone)]
pub struct DemoHeader {
    pub version: u32,
    pub protocol: u32,
    pub server_name: String,
    pub client_name: String,
    pub map_name: String,
    pub game_directory: String,
    pub playback_time: f32,
    pub playback_ticks: u32,
    pub playback_frames: u32,
    pub signon_length: u32,
}

/// Game event structure
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub event_id: u32,
    pub tick: u32,
    pub data: Vec<u8>,
}

/// Player information
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub user_id: u32,
    pub name: String,
    pub guid: String,
    pub friends_id: u32,
    pub friends_name: String,
    pub fake_player: bool,
    pub hltv: bool,
    pub custom_files: Vec<u32>,
    pub files_downloaded: u8,
}

/// Round information
#[derive(Debug, Clone)]
pub struct RoundInfo {
    pub round_number: u8,
    pub winner: u8,
    pub reason: u8,
    pub duration: f32,
}

/// Protobuf parser for CS2 demo files
pub struct ProtobufParser {
    /// Current position in the demo file
    position: usize,
    /// Demo data
    data: Vec<u8>,
}

impl ProtobufParser {
    /// Create a new protobuf parser
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            position: 0,
            data,
        }
    }
    
    /// Parse all messages in the demo
    pub fn parse_all(&mut self) -> Result<Vec<DemoMessage>> {
        let mut messages = Vec::new();
        
        while self.position < self.data.len() {
            match self.parse_next_message() {
                Ok(Some(message)) => {
                    messages.push(message);
                }
                Ok(None) => {
                    // End of messages
                    break;
                }
                Err(e) => {
                    warn!("Error parsing message at position {}: {:?}", self.position, e);
                    // Continue parsing other messages
                    continue;
                }
            }
        }
        
        debug!("Parsed {} messages from demo", messages.len());
        Ok(messages)
    }
    
    /// Parse the next message in the demo
    pub fn parse_next_message(&mut self) -> Result<Option<DemoMessage>> {
        if self.position >= self.data.len() {
            return Ok(None);
        }
        
        // Read message type
        let message_type = self.read_u8()?;
        
        // Read message length
        let message_length = self.read_u32()?;
        
        if message_length == 0 {
            return Ok(None);
        }
        
        // Check if we have enough data
        if self.position + message_length as usize > self.data.len() {
            return Err(DemoError::corrupted("Message length exceeds available data"));
        }
        
        // Read message data
        let message_data = &self.data[self.position..self.position + message_length as usize];
        self.position += message_length as usize;
        
        // Parse message based on type
        let message = match message_type {
            1 => self.parse_header(message_data)?,
            2 => self.parse_game_event(message_data)?,
            3 => self.parse_player_info(message_data)?,
            4 => self.parse_round_info(message_data)?,
            _ => {
                debug!("Unknown message type: {}", message_type);
                DemoMessage::Unknown(message_data.to_vec())
            }
        };
        
        Ok(Some(message))
    }
    
    /// Parse demo header
    fn parse_header(&self, _data: &[u8]) -> Result<DemoMessage> {
        // TODO: Implement actual protobuf parsing for header
        // For now, return a placeholder
        let header = DemoHeader {
            version: 4,
            protocol: 0,
            server_name: String::new(),
            client_name: String::new(),
            map_name: String::new(),
            game_directory: String::new(),
            playback_time: 0.0,
            playback_ticks: 0,
            playback_frames: 0,
            signon_length: 0,
        };
        
        Ok(DemoMessage::Header(header))
    }
    
    /// Parse game event
    fn parse_game_event(&self, data: &[u8]) -> Result<DemoMessage> {
        // TODO: Implement actual protobuf parsing for game events
        let event = GameEvent {
            event_id: 0,
            tick: 0,
            data: data.to_vec(),
        };
        
        Ok(DemoMessage::GameEvent(event))
    }
    
    /// Parse player info
    fn parse_player_info(&self, _data: &[u8]) -> Result<DemoMessage> {
        // TODO: Implement actual protobuf parsing for player info
        let player = PlayerInfo {
            user_id: 0,
            name: String::new(),
            guid: String::new(),
            friends_id: 0,
            friends_name: String::new(),
            fake_player: false,
            hltv: false,
            custom_files: Vec::new(),
            files_downloaded: 0,
        };
        
        Ok(DemoMessage::Player(player))
    }
    
    /// Parse round info
    fn parse_round_info(&self, _data: &[u8]) -> Result<DemoMessage> {
        // TODO: Implement actual protobuf parsing for round info
        let round = RoundInfo {
            round_number: 0,
            winner: 0,
            reason: 0,
            duration: 0.0,
        };
        
        Ok(DemoMessage::Round(round))
    }
    
    /// Read a single byte
    fn read_u8(&mut self) -> Result<u8> {
        if self.position >= self.data.len() {
            return Err(DemoError::corrupted("Unexpected end of data"));
        }
        
        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }
    
    /// Read a 32-bit integer
    fn read_u32(&mut self) -> Result<u32> {
        if self.position + 4 > self.data.len() {
            return Err(DemoError::corrupted("Unexpected end of data"));
        }
        
        let value = u32::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ]);
        
        self.position += 4;
        Ok(value)
    }
    
    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }
    
    /// Get total data length
    pub fn data_len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_protobuf_parser_creation() {
        let data = vec![1, 2, 3, 4];
        let parser = ProtobufParser::new(data);
        assert_eq!(parser.position(), 0);
        assert_eq!(parser.data_len(), 4);
    }
    
    #[test]
    fn test_read_u8() {
        let data = vec![1, 2, 3, 4];
        let mut parser = ProtobufParser::new(data);
        
        assert_eq!(parser.read_u8().unwrap(), 1);
        assert_eq!(parser.read_u8().unwrap(), 2);
        assert_eq!(parser.position(), 2);
    }
    
    #[test]
    fn test_read_u32() {
        let data = vec![1, 0, 0, 0, 2, 0, 0, 0];
        let mut parser = ProtobufParser::new(data);
        
        assert_eq!(parser.read_u32().unwrap(), 1);
        assert_eq!(parser.read_u32().unwrap(), 2);
        assert_eq!(parser.position(), 8);
    }
}
