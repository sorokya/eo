#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Tile;
use crate::data::EOChar;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TileRow {
    pub y: EOChar,
    pub tiles: Vec<Tile>,
}

impl TileRow {
    pub fn new(y: EOChar, tiles_length: usize) -> Self {
        Self {
            y,
            tiles: Vec::with_capacity(tiles_length),
        }
    }
}
