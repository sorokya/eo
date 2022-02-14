/// reply when creating a new character
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum CharacterReply {
    Exists = 1,
    Full = 2,
    InvalidRequest = 3,
    NotApproved = 4,
    Created = 5,
    Deleted = 6,
}

impl Default for CharacterReply {
    fn default() -> Self {
        Self::Exists
    }
}
