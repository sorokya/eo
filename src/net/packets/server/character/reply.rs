use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::{replies::CharacterReply, CharacterList},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: CharacterReply,
    pub session_id: EOShort,
    pub message: String,
    pub character_list: CharacterList,
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

        self.message = reader.get_fixed_string(2);

        if self.reply == CharacterReply::Created || self.reply == CharacterReply::Deleted {
            self.character_list.deserialize(reader)
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        // TODO: calculate capacity
        let mut builder = StreamBuilder::new();
        if self.session_id > 0 {
            builder.add_short(self.session_id);
        } else {
            builder.add_short(self.reply as EOShort);
        }
        builder.add_fixed_string(&self.message, 2);
        if self.reply == CharacterReply::Created || self.reply == CharacterReply::Deleted {
            builder.append(&mut self.character_list.serialize());
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: add better tests

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254, 78, 79];
        let mut packet = Reply::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.reply, CharacterReply::Exists);
        assert_eq!(packet.message, "NO");
    }
    #[test]
    fn serialize() {
        let mut packet = Reply::new();
        packet.reply = CharacterReply::Exists;
        packet.message = "NO".to_string();
        assert_eq!(packet.serialize(), [2, 254, 78, 79]);
    }
}
