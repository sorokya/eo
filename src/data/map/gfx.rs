#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::GFX_SIZE;
use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Gfx {
    pub x: EOChar,
    pub tile: EOShort,
}

impl Gfx {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Gfx {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.x = reader.get_char();
        self.tile = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(GFX_SIZE);
        builder.add_char(self.x);
        builder.add_short(self.tile);
        builder.get()
    }
}
