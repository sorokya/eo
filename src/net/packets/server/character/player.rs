use crate::data::{EOByte, EOInt, EOShort, Serializeable, StreamBuilder, StreamReader};

const PLAYER_SIZE: usize = 6;

#[derive(Debug, Default)]
pub struct Player {
    pub session_id: EOShort,
    pub character_id: EOInt,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.session_id = reader.get_short();
        self.character_id = reader.get_int();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(PLAYER_SIZE);
        builder.add_short(self.session_id);
        builder.add_int(self.character_id);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![242, 4, 6, 254, 254, 254];
        let mut player = Player::new();
        let reader = StreamReader::new(&data);
        player.deserialize(&reader);
        assert_eq!(player.session_id, 1000);
        assert_eq!(player.character_id, 5);
    }
    #[test]
    fn serialize() {
        let mut player = Player::new();
        player.session_id = 1000;
        player.character_id = 5;
        assert_eq!(player.serialize(), [242, 4, 6, 254, 254, 254]);
    }
}
