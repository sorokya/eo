use crate::{
    character::Emote,
    data::{EOChar, Serializeable, StreamBuilder},
};

#[derive(Default, Debug)]
pub struct Report {
    pub emote: Emote,
}

impl Serializeable for Report {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.emote = Emote::from_char(reader.get_char());
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_char(self.emote as EOChar);
        builder.get()
    }
}
