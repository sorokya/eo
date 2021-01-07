#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MapType {
    Default = 0,
    Unknown1 = 1,
    Unknown2 = 2,
    PlayerKill = 3,
}

impl Default for MapType {
    fn default() -> Self {
        Self::Default
    }
}
