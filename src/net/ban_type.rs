use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOByte;

/// describes players ban type during init sequence
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum BanType {
    Temporary = 0,
    Permanent = 2,
}

impl BanType {
    pub fn from_byte(ban_type_byte: EOByte) -> Self {
        match Self::from_u8(ban_type_byte as u8) {
            Some(ban_type) => ban_type,
            None => {
                warn!("Invalid ban type: {}", ban_type_byte);
                BanType::default()
            }
        }
    }
}

impl Default for BanType {
    fn default() -> Self {
        Self::Temporary
    }
}
