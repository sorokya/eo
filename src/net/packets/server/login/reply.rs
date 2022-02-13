use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, Serializeable, StreamBuilder, StreamReader, EOShort},
    net::{replies::LoginReply, CharacterList},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: LoginReply,
    pub character_list: CharacterList,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_short = reader.get_short();
        self.reply = match LoginReply::from_u16(reply_short) {
            Some(reply) => reply,
            None => panic!("Failed to convert short to LoginReply: {}", reply_short),
        };

        if self.reply == LoginReply::OK {
            self.character_list.deserialize(reader);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(0);
        builder.add_short(self.reply as EOShort);

        if self.reply == LoginReply::OK {
            builder.append(&mut self.character_list.serialize());
        }

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::{EOByte, StreamReader, Serializeable}, net::{replies::LoginReply, CharacterInfo}};
    use super::Reply;

    #[test]
    fn deserialize_ok() {
        let buf: Vec<EOByte> = vec![
            4, 254, 4, 2, 255, 103, 111, 114, 111, 110, 255, 106, 74, 3, 254, 42, 2, 25,
            4, 1, 1, 53, 254, 49, 254, 34, 254, 17, 254, 74, 254, 255, 100, 105, 103, 105, 116, 120, 255, 107, 74,
            3, 254, 1, 1, 2, 1, 1, 1, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 255, 107, 97, 109, 105, 110, 97, 255, 10,
            8, 74, 3, 254, 1, 2, 2, 1, 1, 1, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::OK);
        assert_eq!(reply.character_list.length, 3);
        assert_eq!(reply.character_list.unknown, 1);
        assert_eq!(reply.character_list.characters.len(), 3);
        assert_eq!(reply.character_list.characters[0].name, "goron");
        assert_eq!(reply.character_list.characters[1].name, "digitx");
        assert_eq!(reply.character_list.characters[2].name, "kamina");
    }
    #[test]
    fn serialize_ok() {
        let mut reply = Reply::new();
        reply.reply = LoginReply::OK;
        reply.character_list.length = 3;
        reply.character_list.unknown = 1;
        reply.character_list.characters.push(CharacterInfo::new());
        reply.character_list.characters[0].name = "goron".to_string();
        reply.character_list.characters.push(CharacterInfo::new());
        reply.character_list.characters[1].name = "digitx".to_string();
        reply.character_list.characters.push(CharacterInfo::new());
        reply.character_list.characters[2].name = "kamina".to_string();
        let buf = reply.serialize();
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::OK);
        assert_eq!(reply.character_list.length, 3);
        assert_eq!(reply.character_list.unknown, 1);
        assert_eq!(reply.character_list.characters.len(), 3);
        assert_eq!(reply.character_list.characters[0].name, "goron");
        assert_eq!(reply.character_list.characters[1].name, "digitx");
        assert_eq!(reply.character_list.characters[2].name, "kamina");
    }
    #[test]
    fn deserialize_error() {
        let buf: Vec<EOByte> = vec![
            2, 254
        ];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::WrongUsername);
        assert_eq!(reply.character_list.length, 0);
    }
    #[test]
    fn serialize_error() {
        let mut reply = Reply::new();
        reply.reply = LoginReply::WrongUsername;

        assert_eq!(
            reply.serialize(),
            [2, 254]
        );
    }
}
