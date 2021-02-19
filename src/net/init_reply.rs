/// response from the server for Init Reply
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum InitReply {
    OutOfDate = 1,
    OK = 2,
    Banned = 3,
    FileMap = 5,
    FileItem = 6,
    FileNPC = 7,
    FileSpell = 8,
    Players = 9,
    MapMutation = 10,
    FriendListPlayers = 11,
    FileClass = 12,
}

impl Default for InitReply {
    fn default() -> Self {
        Self::OK
    }
}
