use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::{replies::LoginReply, CharacterList},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: LoginReply,
    pub character_list: Option<CharacterList>,
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
            let mut character_list = CharacterList::new();
            character_list.deserialize(reader);
            self.character_list = Some(character_list);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        // TODO: calculate capacity
        let mut builder = StreamBuilder::new();
        builder.add_short(self.reply as EOShort);

        if self.reply == LoginReply::OK {
            builder.append(&mut self.character_list.as_ref().expect("Reply is OK but character_list is not set").serialize());
        }

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Reply;
    use crate::{
        data::{EOByte, Serializeable, StreamReader},
        net::{replies::LoginReply, CharacterInfo, CharacterList},
    };

    #[test]
    fn deserialize_ok() {
        let buf: Vec<EOByte> = vec![
            4, 254, 4, 2, 255, 103, 111, 114, 111, 110, 255, 106, 74, 3, 254, 42, 2, 25, 4, 1, 1,
            53, 254, 49, 254, 34, 254, 17, 254, 74, 254, 255, 100, 105, 103, 105, 116, 120, 255,
            107, 74, 3, 254, 1, 1, 2, 1, 1, 1, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 255, 107,
            97, 109, 105, 110, 97, 255, 10, 8, 74, 3, 254, 1, 2, 2, 1, 1, 1, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::OK);
        let character_list = reply.character_list.unwrap();
        assert_eq!(character_list.length, 3);
        assert_eq!(character_list.unknown, 1);
        assert_eq!(character_list.characters.len(), 3);
        assert_eq!(character_list.characters[0].name, "goron");
        assert_eq!(character_list.characters[1].name, "digitx");
        assert_eq!(character_list.characters[2].name, "kamina");
    }
    #[test]
    fn serialize_ok() {
        let mut reply = Reply::new();
        reply.reply = LoginReply::OK;
        let mut character_list = CharacterList::new();
        character_list.length = 3;
        character_list.unknown = 1;
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[0].name = "goron".to_string();
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[1].name = "digitx".to_string();
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[2].name = "kamina".to_string();
        reply.character_list = Some(character_list);
        let buf = reply.serialize();
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::OK);
        let character_list = reply.character_list.unwrap();
        assert_eq!(character_list.length, 3);
        assert_eq!(character_list.unknown, 1);
        assert_eq!(character_list.characters.len(), 3);
        assert_eq!(character_list.characters[0].name, "goron");
        assert_eq!(character_list.characters[1].name, "digitx");
        assert_eq!(character_list.characters[2].name, "kamina");
    }
    #[test]
    fn deserialize_error() {
        let buf: Vec<EOByte> = vec![2, 254];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, LoginReply::WrongUsername);
        assert!(reply.character_list.is_none());
    }
    #[test]
    fn serialize_error() {
        let mut reply = Reply::new();
        reply.reply = LoginReply::WrongUsername;

        assert_eq!(reply.serialize(), [2, 254]);
    }
}
