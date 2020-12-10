#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum ItemSpecial {
    Normal = 0,
    Uncommon = 1,
    Rare = 2,
    Unique = 3,
    Lore = 4,
    Cursed = 5,
}

impl Default for ItemSpecial {
    fn default() -> Self {
        Self::Normal
    }
}
