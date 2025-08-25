//! Time utilities for CS2 demo parsing

/// Convert ticks to seconds
pub fn ticks_to_seconds(ticks: u32) -> f64 {
    ticks as f64 / 64.0
}

/// Convert seconds to ticks
pub fn seconds_to_ticks(seconds: f64) -> u32 {
    (seconds * 64.0) as u32
}

/// Format duration in MM:SS format
pub fn format_duration_mm_ss(seconds: f64) -> String {
    let minutes = (seconds / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:02}:{:02}", minutes, secs)
}

/// Format duration in HH:MM:SS format
pub fn format_duration_hh_mm_ss(seconds: f64) -> String {
    let hours = (seconds / 3600.0) as u32;
    let minutes = ((seconds % 3600.0) / 60.0) as u32;
    let secs = (seconds % 60.0) as u32;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ticks_to_seconds() {
        assert_eq!(ticks_to_seconds(64), 1.0);
        assert_eq!(ticks_to_seconds(128), 2.0);
    }
    
    #[test]
    fn test_seconds_to_ticks() {
        assert_eq!(seconds_to_ticks(1.0), 64);
        assert_eq!(seconds_to_ticks(2.0), 128);
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration_mm_ss(65.0), "01:05");
        assert_eq!(format_duration_hh_mm_ss(3665.0), "01:01:05");
    }
}
