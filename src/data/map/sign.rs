#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::EOChar;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Sign {
    pub x: EOChar,
    pub y: EOChar,
    pub title: String,
    pub message: String,
}

impl Sign {
    pub fn new() -> Self {
        Self::default()
    }
}
