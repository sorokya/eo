/// a reply sent to the client when attempting to send a private chat
///
/// Currently this is only used when the recipient is offline or has
/// blocked the chat
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum TalkReply {
    NotFound = 1,
}
