use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::replies::CharacterReply,
};

const REPLY_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: CharacterReply,
    pub session_id: EOShort,
    pub message: String,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_or_session_id = reader.get_short();
        if reply_code_or_session_id > 6 {
            self.session_id = reply_code_or_session_id;
        } else {
            self.reply = match CharacterReply::from_u16(reply_code_or_session_id) {
                Some(reply) => reply,
                None => panic!(
                    "Failed to convert short to CharacterReply: {}",
                    reply_code_or_session_id
                ),
            };
        }
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REPLY_SIZE + self.message.len());
        if self.session_id > 0 {
            builder.add_short(self.session_id);
        } else {
            builder.add_short(self.reply as EOShort);
        }
        builder.add_string(&self.message);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: add tests for session id

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![6, 254, 79, 75];
        let mut packet = Reply::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.reply, CharacterReply::Created);
        assert_eq!(packet.message, "OK");
    }
    #[test]
    fn serialize() {
        let mut packet = Reply::new();
        packet.reply = CharacterReply::Created;
        packet.message = "OK".to_string();
        assert_eq!(packet.serialize(), [6, 254, 79, 75]);
    }
}
