#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::Warp;
use crate::data::EOChar;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WarpRow {
    pub y: EOChar,
    pub tiles: Vec<Warp>,
}

impl WarpRow {
    pub fn new(y: EOChar, tiles_length: usize) -> Self {
        Self {
            y,
            tiles: Vec::with_capacity(tiles_length),
        }
    }
}
