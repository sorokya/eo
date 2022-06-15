use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

use super::Coords;

pub const TINY_COORDS_SIZE: usize = 2;

/// x/y coordinates of entity in the game (represented with chars)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TinyCoords {
    pub x: EOChar,
    pub y: EOChar,
}

impl TinyCoords {
    pub fn new(x: EOChar, y: EOChar) -> Self {
        Self { x, y }
    }

    pub fn to_coords(&self) -> Coords {
        Coords {
            x: self.x as EOShort,
            y: self.y as EOShort,
        }
    }
}

impl Serializeable for TinyCoords {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(TINY_COORDS_SIZE);
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.get()
    }
}
