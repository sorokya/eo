use crate::data::{EOByte, EOInt, EOThree, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Message {
    pub player_id: EOThree,
    pub character_id: EOInt,
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Message {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_three();
        self.character_id = reader.get_int();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(7);
        builder.add_three(self.player_id);
        builder.add_int(self.character_id);
        builder.get()
    }
}

// TODO: tests
