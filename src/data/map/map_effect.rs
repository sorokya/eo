#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MapEffect {
    r#None = 0,
    HPDrain = 1,
    TPDrain = 2,
    Quake1 = 3,
    Quake2 = 4,
    Quake3 = 5,
    Quake4 = 6,
}

impl Default for MapEffect {
    fn default() -> Self {
        Self::None
    }
}
