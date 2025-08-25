//! Utility functions for CS2 demo parsing

pub mod time;
pub mod position;
pub mod validation;

use crate::error::{DemoError, Result};
use std::path::Path;

/// Utility functions for demo file operations
pub struct DemoUtils;

impl DemoUtils {
    /// Validate if a file is a valid CS2 demo
    pub fn is_valid_demo_file(path: &Path) -> Result<bool> {
        if !path.exists() {
            return Err(DemoError::file_not_found(path.to_string_lossy()));
        }
        
        if !path.is_file() {
            return Err(DemoError::invalid_format("Path is not a file"));
        }
        
        // Check file extension
        if let Some(extension) = path.extension() {
            if extension != "dem" {
                return Err(DemoError::invalid_format("File is not a .dem file"));
            }
        } else {
            return Err(DemoError::invalid_format("File has no extension"));
        }
        
        // Check file size (minimum size for a valid demo)
        let metadata = std::fs::metadata(path)
            .map_err(|e| DemoError::Io(e))?;
        
        if metadata.len() < 1024 {
            return Err(DemoError::invalid_format("File too small to be a valid demo"));
        }
        
        Ok(true)
    }
    
    /// Get demo file size in bytes
    pub fn get_demo_size(path: &Path) -> Result<u64> {
        let metadata = std::fs::metadata(path)
            .map_err(|e| DemoError::Io(e))?;
        
        Ok(metadata.len())
    }
    
    /// Format file size in human readable format
    pub fn format_file_size(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        match bytes {
            0..KB => format!("{} B", bytes),
            KB..MB => format!("{:.1} KB", bytes as f64 / KB as f64),
            MB..GB => format!("{:.1} MB", bytes as f64 / MB as f64),
            _ => format!("{:.1} GB", bytes as f64 / GB as f64),
        }
    }
    
    /// Calculate demo duration from ticks
    pub fn ticks_to_duration(ticks: u32) -> f64 {
        // CS2 runs at 64 ticks per second
        ticks as f64 / 64.0
    }
    
    /// Calculate ticks from duration
    pub fn duration_to_ticks(duration: f64) -> u32 {
        (duration * 64.0) as u32
    }
    
    /// Format duration in human readable format
    pub fn format_duration(seconds: f64) -> String {
        let hours = (seconds / 3600.0) as u32;
        let minutes = ((seconds % 3600.0) / 60.0) as u32;
        let secs = (seconds % 60.0) as u32;
        
        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{}:{:02}", minutes, secs)
        }
    }
    
    /// Calculate headshot percentage
    pub fn calculate_headshot_percentage(headshots: u16, total_kills: u16) -> f32 {
        if total_kills == 0 {
            0.0
        } else {
            (headshots as f32 / total_kills as f32) * 100.0
        }
    }
    
    /// Calculate K/D ratio
    pub fn calculate_kdr(kills: u16, deaths: u16) -> f32 {
        if deaths == 0 {
            kills as f32
        } else {
            kills as f32 / deaths as f32
        }
    }
    
    /// Calculate average damage per round
    pub fn calculate_adr(total_damage: u32, rounds: u8) -> f32 {
        if rounds == 0 {
            0.0
        } else {
            total_damage as f32 / rounds as f32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_format_file_size() {
        assert_eq!(DemoUtils::format_file_size(1024), "1.0 KB");
        assert_eq!(DemoUtils::format_file_size(1048576), "1.0 MB");
        assert_eq!(DemoUtils::format_file_size(512), "512 B");
    }
    
    #[test]
    fn test_ticks_to_duration() {
        assert_eq!(DemoUtils::ticks_to_duration(64), 1.0);
        assert_eq!(DemoUtils::ticks_to_duration(128), 2.0);
        assert_eq!(DemoUtils::ticks_to_duration(32), 0.5);
    }
    
    #[test]
    fn test_duration_to_ticks() {
        assert_eq!(DemoUtils::duration_to_ticks(1.0), 64);
        assert_eq!(DemoUtils::duration_to_ticks(2.0), 128);
        assert_eq!(DemoUtils::duration_to_ticks(0.5), 32);
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(DemoUtils::format_duration(65.0), "1:05");
        assert_eq!(DemoUtils::format_duration(3665.0), "1:01:05");
        assert_eq!(DemoUtils::format_duration(30.0), "0:30");
    }
    
    #[test]
    fn test_calculate_headshot_percentage() {
        assert_eq!(DemoUtils::calculate_headshot_percentage(5, 10), 50.0);
        assert_eq!(DemoUtils::calculate_headshot_percentage(0, 10), 0.0);
        assert_eq!(DemoUtils::calculate_headshot_percentage(10, 0), 0.0);
    }
    
    #[test]
    fn test_calculate_kdr() {
        assert_eq!(DemoUtils::calculate_kdr(10, 5), 2.0);
        assert_eq!(DemoUtils::calculate_kdr(5, 10), 0.5);
        assert_eq!(DemoUtils::calculate_kdr(10, 0), 10.0);
    }
    
    #[test]
    fn test_calculate_adr() {
        assert_eq!(DemoUtils::calculate_adr(1000, 10), 100.0);
        assert_eq!(DemoUtils::calculate_adr(500, 5), 100.0);
        assert_eq!(DemoUtils::calculate_adr(0, 10), 0.0);
    }
}
