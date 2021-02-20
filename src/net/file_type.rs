/// used when the server sends the client a file
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum FileType {
    Map = 1,
    Item = 2,
    NPC = 3,
    Spell = 4,
    Class = 5,
}

impl Default for FileType {
    fn default() -> Self {
        Self::Map
    }
}
