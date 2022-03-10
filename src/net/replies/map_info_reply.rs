/// reply when player requests MapInfo for NPC or Character
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum MapInfoReply {
    NPC = 0,
    Character = 1,
}

impl Default for MapInfoReply {
    fn default() -> Self {
        Self::Character
    }
}
