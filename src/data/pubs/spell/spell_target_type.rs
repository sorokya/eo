#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SpellTargetType {
    Normal = 0,
    SELF = 1,
    Unknown1 = 2,
    Group = 3,
}

impl Default for SpellTargetType {
    fn default() -> Self {
        Self::Normal
    }
}
