use crate::{
    character::Emote,
    data::{EOChar, EOShort, Serializeable, StreamBuilder},
};

#[derive(Default, Debug)]
pub struct Player {
    pub player_id: EOShort,
    pub emote: Emote,
}

impl Player {
    pub fn new(player_id: EOShort, emote: Emote) -> Self {
        Self { player_id, emote }
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.player_id = reader.get_short();
        self.emote = Emote::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(3);
        builder.add_short(self.player_id);
        builder.add_char(self.emote as EOChar);
        builder.get()
    }
}
