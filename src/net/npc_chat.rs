use crate::data::{EOShort, Serializeable, StreamBuilder};

pub const NPC_CHAT_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct NPCChat {
    pub index: EOShort,
    pub message: String,
}

impl Serializeable for NPCChat {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.index = reader.get_short();
        self.message = reader.get_prefix_string();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_CHAT_SIZE + 1 + self.message.len());
        builder.add_short(self.index);
        builder.add_prefix_string(&self.message);
        builder.get()
    }
}
