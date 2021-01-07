#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::TileSpec;
use crate::data::EOChar;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tile {
    pub x: EOChar,
    pub spec: TileSpec,
}

impl Tile {
    pub fn new(x: EOChar, spec: TileSpec) -> Self {
        Self { x, spec }
    }
}
