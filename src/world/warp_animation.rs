use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// describes the animation used when warping
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum WarpAnimation {
    Scroll = 1,
    Admin = 2,
}

impl WarpAnimation {
    pub fn from_char(warp_animation_char: EOChar) -> Option<Self> {
        match Self::from_u8(warp_animation_char as u8) {
            Some(warp_animation) => Some(warp_animation),
            None => {
                if warp_animation_char != 0 {
                    warn!("Invalid warp animation: {}", warp_animation_char);
                }
                None
            }
        }
    }
}

impl Default for WarpAnimation {
    fn default() -> Self {
        WarpAnimation::Scroll
    }
}
