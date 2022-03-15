use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    world::{Direction, TinyCoords, TINY_COORDS_SIZE},
};

const PLAYER_SIZE: usize = 3 + TINY_COORDS_SIZE;

#[derive(Debug, Default)]
pub struct Player {
    pub player_id: EOShort,
    pub direction: Direction,
    pub coords: TinyCoords,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        let direction_char = reader.get_char();
        self.direction = match Direction::from_u8(direction_char) {
            Some(direction) => direction,
            None => panic!("Invalid direction: {}", direction_char),
        };
        self.coords.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PLAYER_SIZE);
        builder.add_short(self.player_id);
        builder.add_char(self.direction as EOChar);
        builder.append(&mut self.coords.serialize());
        builder.get()
    }
    // TODO: tests
}
