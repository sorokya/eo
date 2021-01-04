#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SpellTargetRestrict {
    NPCOnly = 0,
    Friendly = 1,
    Opponent = 2,
}

impl Default for SpellTargetRestrict {
    fn default() -> Self {
        Self::NPCOnly
    }
}
