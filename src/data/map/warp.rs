#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{EOChar, EOShort};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Warp {
    pub x: EOChar,
    pub warp_map: EOShort,
    pub warp_x: EOChar,
    pub warp_y: EOChar,
    pub level_req: EOChar,
    pub door: bool,
}

impl Warp {
    pub fn new() -> Self {
        Self::default()
    }
}
