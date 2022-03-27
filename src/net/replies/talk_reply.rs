use log::warn;
use num_traits::FromPrimitive;

use crate::data::EOShort;

/// a reply sent to the client when attempting to send a private chat
///
/// Currently this is only used when the recipient is offline or has
/// blocked the chat
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum TalkReply {
    NotFound = 1,
}

impl TalkReply {
    pub fn from_short(talk_reply_short: EOShort) -> Self {
        match Self::from_u16(talk_reply_short) {
            Some(talk_reply) => talk_reply,
            None => {
                warn!("Invalid talk reply: {}", talk_reply_short);
                Self::default()
            }
        }
    }
}

impl Default for TalkReply {
    fn default() -> TalkReply {
        TalkReply::NotFound
    }
}
