#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Gfx;
use crate::data::EOChar;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GfxRow {
    pub y: EOChar,
    pub tiles: Vec<Gfx>,
}

impl GfxRow {
    pub fn new(y: EOChar, tiles_length: usize) -> Self {
        Self {
            y,
            tiles: Vec::with_capacity(tiles_length),
        }
    }
}
