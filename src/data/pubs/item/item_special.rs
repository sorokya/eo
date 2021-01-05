#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// an item's "Special" stat.
///
/// It's sort of a mix between rarity and cursed.
///
/// The rarity changes the item's text color in the client.
///
/// Lore makes an item impossible to trade or drop.
///
/// Cursed makes an item impossible to unequip without destroying the item
/// with a "CureCurse" item.
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
