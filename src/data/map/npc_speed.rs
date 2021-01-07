#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum NPCSpeed {
    Speed1 = 0,
    Speed2 = 1,
    Speed3 = 2,
    Speed4 = 3,
    Speed5 = 4,
    Speed6 = 5,
    Speed7 = 6,
    Frozen = 7,
}

impl Default for NPCSpeed {
    fn default() -> Self {
        Self::Speed1
    }
}
