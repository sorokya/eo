use log::warn;
use num_traits::FromPrimitive;

use crate::data::{EOChar, EOShort};

/// describes the character gender
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Gender {
    Female = 0,
    Male = 1,
}

impl Gender {
    pub fn from_char(gender_char: EOChar) -> Self {
        match Self::from_u8(gender_char) {
            Some(gender) => gender,
            None => {
                warn!("Invalid gender: {}", gender_char);
                Gender::default()
            }
        }
    }

    pub fn from_short(gender_short: EOShort) -> Self {
        match Self::from_u16(gender_short) {
            Some(gender) => gender,
            None => {
                warn!("Invalid gender: {}", gender_short);
                Gender::default()
            }
        }
    }
}

impl Default for Gender {
    fn default() -> Gender {
        Gender::Female
    }
}
