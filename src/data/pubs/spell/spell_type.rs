#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// represents the type of magic a spell does
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
