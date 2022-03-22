use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    world::{Direction, TinyCoords, TINY_COORDS_SIZE},
};

pub const NPC_MAP_INFO_SIZE: usize = TINY_COORDS_SIZE + 4;
#[derive(Debug, Default)]
pub struct NpcMapInfo {
    pub index: EOChar,
    pub id: EOShort,
    pub coords: TinyCoords,
    pub direction: Direction,
}

impl NpcMapInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NpcMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.index = reader.get_char();
        self.id = reader.get_short();
        self.coords.deserialize(reader);
        self.direction = Direction::from_char(reader.get_char());
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_MAP_INFO_SIZE);
        builder.add_char(self.index);
        builder.add_short(self.id);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction as EOChar);
        builder.get()
    }
}

// TODO: tests
