#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{EOChar, EOShort};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gfx {
    pub x: EOChar,
    pub tile: EOShort,
}

impl Gfx {
    pub fn new(x: EOChar, tile: EOShort) -> Self {
        Self { x, tile }
    }
}
