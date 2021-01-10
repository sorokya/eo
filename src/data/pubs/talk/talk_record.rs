#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::talk::ETF_DATA_SIZE, EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader,
};

/// data structure of a single etf record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TalkRecord {
    /// linked to npc id
    pub npc_id: EOShort,
    /// rate at which messages are spoken
    pub rate: EOChar,
    /// number of messages for the npc
    pub messages_length: EOChar,
    /// list of messages for the npc
    pub messages: Vec<String>,
}

impl TalkRecord {
    /// creates a new TalkRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for TalkRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.npc_id = reader.get_short();
        self.rate = reader.get_char();
        self.messages_length = reader.get_char();
        self.messages = Vec::with_capacity(self.messages_length as usize);
        for _ in 0..self.messages_length {
            self.messages.push(reader.get_prefix_string());
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            ETF_DATA_SIZE
                + (self.messages_length as usize)
                + self
                    .messages
                    .iter()
                    .map(|s: &String| s.len())
                    .sum::<usize>() as usize,
        );

        builder.add_short(self.npc_id);
        builder.add_char(self.rate);
        builder.add_char(self.messages_length);

        for message in &self.messages {
            builder.add_prefix_string(message);
        }

        builder.get()
    }
}
