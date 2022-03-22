use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// describes a characters sitting state in the game
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SitState {
    Standing = 0,
    Chair = 1,
    Floor = 2,
}

impl SitState {
    pub fn from_char(sit_state_char: EOChar) -> Self {
        match Self::from_u8(sit_state_char as u8) {
            Some(sit_state) => sit_state,
            None => {
                warn!("Invalid sit state: {}", sit_state_char);
                SitState::default()
            }
        }
    }
}

impl Default for SitState {
    fn default() -> Self {
        SitState::Standing
    }
}
