use crate::{
    data::{EOChar, Serializeable, StreamBuilder, StreamReader},
    world::Direction,
};

const PLAYER_SIZE: usize = 1;

#[derive(Debug, Default)]
pub struct Player {
    pub direction: Direction,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.direction = Direction::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(PLAYER_SIZE);
        builder.add_char(self.direction as EOChar);
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use crate::data::EOByte;

    use super::*;

    #[test]
    fn serialize() {
        let mut packet = Player::new();
        packet.direction = Direction::Down;
        assert_eq!(packet.serialize(), [1])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![1];
        let mut packet = Player::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.direction, Direction::Down);
    }
}
