/// reply when creating a new character
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum CharacterReply {
    Exists = 1,
    Full = 2,
    NotApproved = 3,
    OK = 4,
    Deleted = 5,
}
