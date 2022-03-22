use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOByte;

/// response from the server for Init Reply
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum InitReply {
    OutOfDate = 1,
    OK = 2,
    Banned = 3,
    WarpMap = 4,
    FileMap = 5,
    FileItem = 6,
    FileNPC = 7,
    FileSpell = 8,
    Players = 9,
    MapMutation = 10,
    FriendListPlayers = 11,
    FileClass = 12,
}

impl InitReply {
    pub fn from_byte(init_reply_byte: EOByte) -> Self {
        match Self::from_u8(init_reply_byte as u8) {
            Some(init_reply) => init_reply,
            None => {
                warn!("Invalid init reply: {}", init_reply_byte);
                InitReply::default()
            }
        }
    }
}

impl Default for InitReply {
    fn default() -> Self {
        Self::OK
    }
}
