use crate::data::{Serializeable, StreamBuilder, EOChar};

pub const NPC_CHAT_SIZE: usize = 1;

#[derive(Debug, Default, Clone)]
pub struct NPCChat {
    pub index: EOChar,
    pub message: String,
}

impl Serializeable for NPCChat {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.index = reader.get_char();
        self.message = reader.get_prefix_string();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_CHAT_SIZE + 1 + self.message.len());
        builder.add_char(self.index);
        builder.add_prefix_string(&self.message);
        builder.get()
    }
}
