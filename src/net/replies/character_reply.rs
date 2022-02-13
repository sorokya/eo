/// reply when creating a new character
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum CharacterReply {
    Exists = 1,
    Full = 2,
    NotApproved = 3,
    Created = 4,
    Deleted = 5,
}

impl Default for CharacterReply {
    fn default() -> Self {
        Self::Exists
    }
}
