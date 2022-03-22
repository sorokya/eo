use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// this is the icon that shows up when you view a players paperdoll
///
/// it describes wether the player is in a party or not,
/// and if they're a Game Master
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum PaperdollIcon {
    Normal = 0,
    GameMaster = 4,
    HighGameMaster = 5,
    Party = 6,
    GameMasterParty = 9,
    HighGameMasterParty = 10,
}

impl PaperdollIcon {
    pub fn from_char(paperdoll_icon_char: EOChar) -> Self {
        match Self::from_u8(paperdoll_icon_char as u8) {
            Some(paperdoll_icon) => paperdoll_icon,
            None => {
                warn!("Invalid paperdoll icon: {}", paperdoll_icon_char);
                PaperdollIcon::default()
            }
        }
    }
}

impl Default for PaperdollIcon {
    fn default() -> Self {
        Self::Normal
    }
}
