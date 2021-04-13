#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use super::{TileSpec, TILE_SIZE};
use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tile {
    pub x: EOChar,
    pub spec: TileSpec,
}

impl Tile {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Tile {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_char();
        let spec_char = reader.get_char();
        self.spec = match TileSpec::from_u8(spec_char) {
            Some(spec) => spec,
            None => panic!("Failed to convert char to TileSpec: {}", spec_char),
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(TILE_SIZE);
        builder.add_char(self.x);
        builder.add_char(self.spec as EOChar);
        builder.get()
    }
}
