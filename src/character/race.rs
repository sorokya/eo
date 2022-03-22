use log::warn;
use num_traits::FromPrimitive;

use crate::data::{EOChar, EOShort};

/// Describes the characters race
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Race {
    White = 0,
    Tan = 1,
    Yellow = 2,
    Orc = 3,
    Skeleton = 4,
    Panda = 5,
    Fish = 6,
}

impl Race {
    pub fn from_char(race_char: EOChar) -> Self {
        match Race::from_u8(race_char) {
            Some(race) => race,
            None => {
                warn!("Invalid race {}", race_char);
                Race::default()
            }
        }
    }

    pub fn from_short(race_short: EOShort) -> Self {
        match Race::from_u16(race_short) {
            Some(race) => race,
            None => {
                warn!("Invalid race {}", race_short);
                Race::default()
            }
        }
    }
}

impl Default for Race {
    fn default() -> Race {
        Race::White
    }
}
