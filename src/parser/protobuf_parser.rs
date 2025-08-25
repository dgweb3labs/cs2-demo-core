use crate::error::{DemoError, Result};
use crate::events::{DemoMetadata, Kill, Headshot, Clutch, Round, Player, Position, WinCondition};
use std::collections::HashMap;

/// Protocol Buffer message types for CS2 demo parsing
#[derive(Debug, Clone)]
pub enum DemoMessage {
    Header(DemoHeader),
    GameEvent(GameEvent),
    PlayerInfo(PlayerInfo),
    RoundInfo(RoundInfo),
    Unknown { field_id: u32, data: Vec<u8> },
}

/// Demo file header information
#[derive(Debug, Clone)]
pub struct DemoHeader {
    pub signature: String,
    pub version: u32,
    pub map_name: String,
    pub server_name: String,
    pub player_count: u32,
    pub tick_count: u32,
    pub duration: f32,
}

/// Game event information
#[derive(Debug, Clone)]
pub struct GameEvent {
    pub event_type: u32,
    pub timestamp: f32,
    pub data: HashMap<String, String>,
}

/// Player information
#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub steam_id: u64,
    pub name: String,
    pub team: u32,
    pub position: Position,
    pub health: u32,
    pub armor: u32,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
}

/// Round information
#[derive(Debug, Clone)]
pub struct RoundInfo {
    pub round_number: u32,
    pub start_time: f32,
    pub end_time: f32,
    pub winner: WinCondition,
    pub t_score: u32,
    pub ct_score: u32,
}

/// Protocol Buffer parser for CS2 demo files
pub struct ProtobufParser {
    data: Vec<u8>,
    position: usize,
}

impl ProtobufParser {
    /// Create a new protobuf parser
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            position: 0,
        }
    }

    /// Parse all messages in the demo file
    pub fn parse_all(&mut self) -> Result<Vec<DemoMessage>> {
        let mut messages = Vec::new();
        
        // Check for PBDEMS2 signature
        if !self.check_signature()? {
            return Err(DemoError::invalid_format("Missing PBDEMS2 signature"));
        }

        // Skip header and parse messages
        self.skip_header()?;
        
        while self.position < self.data.len() {
            if let Some(message) = self.parse_next_message()? {
                messages.push(message);
            } else {
                break;
            }
        }

        Ok(messages)
    }

    /// Parse the next message in the stream
    pub fn parse_next_message(&mut self) -> Result<Option<DemoMessage>> {
        if self.position >= self.data.len() {
            return Ok(None);
        }

        // Read field header (protobuf wire format)
        let field_header = self.read_varint()?;
        let field_id = field_header >> 3;
        let wire_type = field_header & 0x07;

        match wire_type {
            0 => { // Varint
                let value = self.read_varint()?;
                Ok(Some(self.create_message_from_field(field_id, value)?))
            },
            1 => { // 64-bit
                let value = self.read_u64()?;
                Ok(Some(self.create_message_from_field(field_id, value)?))
            },
            2 => { // Length-delimited
                let length = self.read_varint()? as usize;
                let data = self.read_bytes(length)?;
                Ok(Some(self.create_message_from_field(field_id, data)?))
            },
            5 => { // 32-bit
                let value = self.read_u32()?;
                Ok(Some(self.create_message_from_field(field_id, value)?))
            },
            _ => {
                // Skip unknown wire types
                self.position += 1;
                Ok(None)
            }
        }
    }

    /// Check if the file has the correct PBDEMS2 signature
    fn check_signature(&self) -> Result<bool> {
        if self.data.len() < 8 {
            return Ok(false);
        }
        
        let signature = &self.data[0..8];
        let expected = b"PBDEMS2\0";
        
        Ok(signature == expected)
    }

    /// Skip the demo header section
    fn skip_header(&mut self) -> Result<()> {
        // Skip signature (8 bytes)
        self.position = 8;
        
        // Skip version and other header fields
        // Look for the first protobuf message
        while self.position < self.data.len() {
            if self.data[self.position] & 0x07 == 2 { // Length-delimited field
                break;
            }
            self.position += 1;
        }
        
        Ok(())
    }

    /// Create a message from a protobuf field
    fn create_message_from_field(&self, field_id: u32, value: impl std::fmt::Debug) -> Result<DemoMessage> {
        match field_id {
            1 => Ok(DemoMessage::Header(self.parse_header_field(value)?)),
            2 => Ok(DemoMessage::GameEvent(self.parse_game_event_field(value)?)),
            3 => Ok(DemoMessage::PlayerInfo(self.parse_player_info_field(value)?)),
            4 => Ok(DemoMessage::RoundInfo(self.parse_round_info_field(value)?)),
            _ => Ok(DemoMessage::Unknown { 
                field_id, 
                data: format!("{:?}", value).into_bytes() 
            }),
        }
    }

    /// Parse header field
    fn parse_header_field(&self, _value: impl std::fmt::Debug) -> Result<DemoHeader> {
        // TODO: Implement real header parsing
        Ok(DemoHeader {
            signature: "PBDEMS2".to_string(),
            version: 2,
            map_name: "de_ancient".to_string(),
            server_name: "SourceTV".to_string(),
            player_count: 10,
            tick_count: 0,
            duration: 0.0,
        })
    }

    /// Parse game event field
    fn parse_game_event_field(&self, _value: impl std::fmt::Debug) -> Result<GameEvent> {
        // TODO: Implement real game event parsing
        Ok(GameEvent {
            event_type: 0,
            timestamp: 0.0,
            data: HashMap::new(),
        })
    }

    /// Parse player info field
    fn parse_player_info_field(&self, _value: impl std::fmt::Debug) -> Result<PlayerInfo> {
        // TODO: Implement real player info parsing
        Ok(PlayerInfo {
            steam_id: 0,
            name: "Player".to_string(),
            team: 0,
            position: Position { x: 0.0, y: 0.0, z: 0.0 },
            health: 100,
            armor: 0,
            kills: 0,
            deaths: 0,
            assists: 0,
        })
    }

    /// Parse round info field
    fn parse_round_info_field(&self, _value: impl std::fmt::Debug) -> Result<RoundInfo> {
        // TODO: Implement real round info parsing
        Ok(RoundInfo {
            round_number: 1,
            start_time: 0.0,
            end_time: 0.0,
            winner: WinCondition::Unknown,
            t_score: 0,
            ct_score: 0,
        })
    }

    /// Read a varint from the current position
    fn read_varint(&mut self) -> Result<u32> {
        let mut result = 0u32;
        let mut shift = 0;
        
        loop {
            if self.position >= self.data.len() {
                return Err(DemoError::corrupted("Unexpected end of data"));
            }
            
            let byte = self.data[self.position];
            self.position += 1;
            
            result |= ((byte & 0x7F) as u32) << shift;
            
            if (byte & 0x80) == 0 {
                break;
            }
            
            shift += 7;
            if shift >= 32 {
                return Err(DemoError::invalid_format("Varint too large"));
            }
        }
        
        Ok(result)
    }

    /// Read a u32 from the current position
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

    /// Read a u64 from the current position
    fn read_u64(&mut self) -> Result<u64> {
        if self.position + 8 > self.data.len() {
            return Err(DemoError::corrupted("Unexpected end of data"));
        }
        
        let value = u64::from_le_bytes([
            self.data[self.position],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ]);
        
        self.position += 8;
        Ok(value)
    }

    /// Read bytes from the current position
    fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>> {
        if self.position + length > self.data.len() {
            return Err(DemoError::corrupted("Unexpected end of data"));
        }
        
        let data = self.data[self.position..self.position + length].to_vec();
        self.position += length;
        Ok(data)
    }

    /// Get current position in the data
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
