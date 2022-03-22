use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// describes the kinds of emotions a character can display
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Emote {
    Happy = 1,
    Depressed = 2,
    Sad = 3,
    Angry = 4,
    Confused = 5,
    Surprised = 6,
    Hearts = 7,
    Moon = 8,
    Suicidal = 9,
    Embarrassed = 10,
    Drunk = 11,
    Trade = 12,
    LevelUp = 13,
    Playful = 14,
}

impl Emote {
    pub fn from_char(emote_char: EOChar) -> Self {
        match Emote::from_u8(emote_char as u8) {
            Some(emote) => emote,
            None => {
                warn!("Invalid emote: {}", emote_char);
                Emote::default()
            }
        }
    }
}

impl Default for Emote {
    fn default() -> Self {
        Emote::Happy
    }
}
