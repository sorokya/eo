/// represents the type of magic a spell does
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SpellType {
    Heal = 0,
    Damage = 1,
    Bard = 2,
}

impl Default for SpellType {
    fn default() -> Self {
        Self::Heal
    }
}
