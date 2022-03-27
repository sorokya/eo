use log::warn;
use num_traits::FromPrimitive;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::EOChar;

/// describes a characters admin level
///
/// new characters default as Player
///
/// The Guide, Guardian, GameMaster, HighGameMaster options
/// are usually used as a rank system for admins within the
/// game.
///
/// You could use these levels to restrict access to certain
/// commands and features within the game to only specified
/// admin levels.
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AdminLevel {
    Player = 0,
    LightGuide = 1,
    Guide = 2,
    Guardian = 3,
    GameMaster = 4,
    HighGameMaster = 5,
}

impl AdminLevel {
    pub fn from_char(admin_level_char: EOChar) -> Self {
        match AdminLevel::from_u8(admin_level_char) {
            Some(admin_level) => admin_level,
            None => {
                warn!("Invalid admin level: {}", admin_level_char);
                AdminLevel::default()
            }
        }
    }
}

impl Default for AdminLevel {
    fn default() -> AdminLevel {
        AdminLevel::Player
    }
}
