use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOChar;

/// used when the server sends the client a file
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum FileType {
    Map = 1,
    Item = 2,
    NPC = 3,
    Spell = 4,
    Class = 5,
}

impl FileType {
    pub fn from_char(file_type_char: EOChar) -> Self {
        match Self::from_u8(file_type_char as u8) {
            Some(file_type) => file_type,
            None => {
                warn!("Invalid file type: {}", file_type_char);
                FileType::default()
            }
        }
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::Map
    }
}
