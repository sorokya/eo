use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const COORDS_SIZE: usize = 4;

/// x/y coordinates of entity in the game
#[derive(Debug, Clone, Copy, Default)]
pub struct Coords {
    pub x: EOShort,
    pub y: EOShort,
}

impl Serializeable for Coords {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.x = reader.get_short();
        self.y = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(COORDS_SIZE);
        builder.add_short(self.x);
        builder.add_short(self.y);
        builder.get()
    }
}
