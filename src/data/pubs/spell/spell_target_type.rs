#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// specifies the target type of a spell
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
