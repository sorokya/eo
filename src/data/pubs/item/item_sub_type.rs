#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// an item's sub-type
///
/// used to further categorize an item's type.
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ItemSubType {
    None = 0,
    Ranged = 1,
    Arrows = 2,
    Wings = 3,
    TwoHanded = 4,
    Unknown1 = 5,
}

impl Default for ItemSubType {
    fn default() -> Self {
        Self::None
    }
}
