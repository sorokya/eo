use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// describes a facing direction for entities in the game
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Direction {
    Down = 0,
    Left = 1,
    Up = 2,
    Right = 3,
}

impl Direction {
    pub fn from_char(direction_char: EOChar) -> Self {
        match Self::from_u8(direction_char as u8) {
            Some(direction) => direction,
            None => {
                warn!("Invalid direction: {}", direction_char);
                Direction::default()
            }
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Down
    }
}
