//! Position utilities for CS2 demo parsing

use crate::events::Position;

/// Calculate distance between two positions
pub fn calculate_distance(pos1: &Position, pos2: &Position) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Calculate 2D distance (ignoring Z coordinate)
pub fn calculate_distance_2d(pos1: &Position, pos2: &Position) -> f32 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    
    (dx * dx + dy * dy).sqrt()
}

/// Check if two positions are within a certain distance
pub fn is_within_distance(pos1: &Position, pos2: &Position, max_distance: f32) -> bool {
    calculate_distance(pos1, pos2) <= max_distance
}

/// Create a position from coordinates
pub fn create_position(x: f32, y: f32, z: f32) -> Position {
    Position { x, y, z }
}

/// Get the midpoint between two positions
pub fn get_midpoint(pos1: &Position, pos2: &Position) -> Position {
    Position {
        x: (pos1.x + pos2.x) / 2.0,
        y: (pos1.y + pos2.y) / 2.0,
        z: (pos1.z + pos2.z) / 2.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_distance() {
        let pos1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos2 = Position { x: 3.0, y: 4.0, z: 0.0 };
        
        assert_eq!(calculate_distance(&pos1, &pos2), 5.0);
    }
    
    #[test]
    fn test_calculate_distance_2d() {
        let pos1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos2 = Position { x: 3.0, y: 4.0, z: 10.0 };
        
        assert_eq!(calculate_distance_2d(&pos1, &pos2), 5.0);
    }
    
    #[test]
    fn test_is_within_distance() {
        let pos1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos2 = Position { x: 3.0, y: 4.0, z: 0.0 };
        
        assert!(is_within_distance(&pos1, &pos2, 6.0));
        assert!(!is_within_distance(&pos1, &pos2, 4.0));
    }
    
    #[test]
    fn test_get_midpoint() {
        let pos1 = Position { x: 0.0, y: 0.0, z: 0.0 };
        let pos2 = Position { x: 10.0, y: 10.0, z: 10.0 };
        
        let midpoint = get_midpoint(&pos1, &pos2);
        assert_eq!(midpoint.x, 5.0);
        assert_eq!(midpoint.y, 5.0);
        assert_eq!(midpoint.z, 5.0);
    }
}
