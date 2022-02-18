/// response from the server for Init Reply
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum InitReply {
    OutOfDate = 1,
    OK = 2,
    Banned = 3,
    FileMap = 4,
    FileItem = 5,
    FileNPC = 6,
    FileSpell = 7,
    Players = 8,
    MapMutation = 9,
    FriendListPlayers = 10,
    FileClass = 11,
}

impl Default for InitReply {
    fn default() -> Self {
        Self::OK
    }
}
