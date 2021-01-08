#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Gfx, GFX_ROW_SIZE, GFX_SIZE};
use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GfxRow {
    pub y: EOChar,
    pub tiles: Vec<Gfx>,
}

impl GfxRow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for GfxRow {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.y = reader.get_char();
        let inner_length = reader.get_char();
        self.tiles = Vec::with_capacity(inner_length as usize);
        for _ in 0..inner_length {
            let mut tile = Gfx::new();
            tile.deserialize(reader);
            self.tiles.push(tile);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(GFX_ROW_SIZE + self.tiles.len() * GFX_SIZE);
        builder.add_char(self.y);
        for tile in &self.tiles {
            builder.append(&mut Serializeable::serialize(tile));
        }
        builder.get()
    }
}
