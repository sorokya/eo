use log::warn;
use num_traits::FromPrimitive;

use crate::{
    data::{EOChar, Serializeable, StreamBuilder},
    world::{Direction, TinyCoords, TINY_COORDS_SIZE},
};

pub const NPC_POSITION_SIZE: usize = 2 + TINY_COORDS_SIZE;

#[derive(Debug, Default, Clone)]
pub struct NPCPosition {
    pub index: EOChar,
    pub coords: TinyCoords,
    pub direction: Direction,
}

impl Serializeable for NPCPosition {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.index = reader.get_char();
        self.coords.deserialize(&reader);

        let direction_char = reader.get_char();
        self.direction = match Direction::from_u8(direction_char) {
            Some(direction) => direction,
            _ => {
                warn!("Failed to parse direction: {}", direction_char);
                Direction::Down
            }
        };
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_POSITION_SIZE);
        builder.add_char(self.index);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction as EOChar);
        builder.get()
    }
}
