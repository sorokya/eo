use num_traits::FromPrimitive;

use crate::{
    data::{EOChar, EOThree, Serializeable, StreamBuilder, StreamReader},
    world::{Direction, TinyCoords, TINY_COORDS_SIZE},
};

const PLAYER_SIZE: usize = 4 + TINY_COORDS_SIZE;

#[derive(Debug, Default)]
pub struct Player {
    pub direction: Direction,
    pub timestamp: EOThree,
    pub coords: TinyCoords,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        let direction_char = reader.get_char();
        self.direction = match Direction::from_u8(direction_char) {
            Some(direction) => direction,
            None => panic!("Invalid direction: {}", direction_char),
        };
        self.timestamp = reader.get_three();
        self.coords.deserialize(&reader);
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(PLAYER_SIZE);
        builder.add_char(self.direction as EOChar);
        builder.add_three(self.timestamp);
        builder.append(&mut self.coords.serialize());
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use crate::data::EOByte;

    use super::*;

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![1, 244, 159, 75, 7, 12];
        let mut reader = StreamReader::new(&buf);
        let mut player = Player::new();
        player.deserialize(&mut reader);
        assert_eq!(player.direction, Direction::Down);
        assert_eq!(player.timestamp, 4776883);
        assert_eq!(player.coords.x, 6);
        assert_eq!(player.coords.y, 11);
    }

    #[test]
    fn serialize() {
        let player = Player {
            direction: Direction::Down,
            timestamp: 4776883,
            coords: TinyCoords { x: 6, y: 11 },
        };
        assert_eq!(player.serialize(), [1, 244, 159, 75, 7, 12]);
    }
}
