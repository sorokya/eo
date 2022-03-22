use crate::{
    data::{EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    world::Direction,
};

const PLAYER_SIZE: usize = 3;

#[derive(Debug, Default)]
pub struct Player {
    pub player_id: EOShort,
    pub direction: Direction,
}

impl Player {
    pub fn new(player_id: EOShort, direction: Direction) -> Self {
        Self {
            player_id,
            direction,
        }
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.direction = Direction::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(PLAYER_SIZE);
        builder.add_short(self.player_id);
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
        let packet = Player::new(1, Direction::Down);
        assert_eq!(packet.serialize(), [2, 254, 1])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254, 1];
        let mut packet = Player::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_id, 1);
        assert_eq!(packet.direction, Direction::Down);
    }
}
