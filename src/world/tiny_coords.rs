use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

pub const TINY_COORDS_SIZE: usize = 2;

/// x/y coordinates of entity in the game (represented with chars)
#[derive(Debug, Clone, Copy, Default)]
pub struct TinyCoords {
    pub x: EOChar,
    pub y: EOChar,
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
