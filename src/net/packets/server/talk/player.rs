use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub player_id: EOShort,
    pub message: String,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.message.len() + 2);
        builder.add_short(self.player_id);
        builder.add_string(&self.message);
        builder.get()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn deserialize() {
//         todo!();
//         let data: Vec<EOByte> = vec![];
//         let mut open = Player::new();
//         let reader = StreamReader::new(&data);
//         open.deserialize(&reader);
//     }
//     #[test]
//     fn serialize() {
//         todo!();
//         let mut open = Player::new();
//         assert_eq!(open.serialize(), [])
//     }
// }
