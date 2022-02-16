use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

pub const COORDS_SIZE: usize = 2;

/// describes a facing direction for entities in the game
#[derive(Debug, Default)]
pub struct Coords {
    pub x: EOChar,
    pub y: EOChar,
}

impl Serializeable for Coords {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(COORDS_SIZE);
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.get()
    }
}
