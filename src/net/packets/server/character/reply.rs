use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::{replies::CharacterReply, CharacterList},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: Option<CharacterReply>,
    pub session_id: Option<EOShort>,
    pub message: Option<String>,
    pub character_list: Option<CharacterList>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no(reply: CharacterReply) -> Self {
        Self {
            reply: Some(reply),
            session_id: None,
            message: Some("NO".to_string()),
            character_list: None,
        }
    }

    pub fn r#continue(session_id: EOShort) -> Self {
        Self {
            reply: None,
            session_id: Some(session_id),
            message: Some("OK".to_string()),
            character_list: None,
        }
    }

    pub fn created(character_list: CharacterList) -> Self {
        Self {
            reply: Some(CharacterReply::Created),
            session_id: None,
            message: None,
            character_list: Some(character_list),
        }
    }

    pub fn deleted(character_list: CharacterList) -> Self {
        Self {
            reply: Some(CharacterReply::Deleted),
            session_id: None,
            message: None,
            character_list: Some(character_list),
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_or_session_id = reader.get_short();
        if reply_code_or_session_id >= 10 {
            self.session_id = Some(reply_code_or_session_id);
        } else {
            self.reply = match CharacterReply::from_u16(reply_code_or_session_id) {
                Some(reply) => Some(reply),
                None => panic!(
                    "Failed to convert short to CharacterReply: {}",
                    reply_code_or_session_id
                ),
            };
        }
        if self.session_id.is_some()
            || !matches!(
                self.reply.as_ref().unwrap(),
                CharacterReply::Created | CharacterReply::Deleted
            )
        {
            self.message = Some(reader.get_break_string());
        }

        if self.reply == Some(CharacterReply::Created)
            || self.reply == Some(CharacterReply::Deleted)
        {
            let mut character_list = CharacterList::new();
            character_list.deserialize(reader);
            self.character_list = Some(character_list);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        // TODO: calculate capacity
        let mut builder = StreamBuilder::new();
        if self.session_id.is_some() {
            builder.add_short(self.session_id.unwrap());
        } else {
            builder.add_short(self.reply.unwrap() as EOShort);
        }

        if self.session_id.is_some()
            || !matches!(
                self.reply.as_ref().unwrap(),
                CharacterReply::Created | CharacterReply::Deleted
            )
        {
            builder.add_fixed_string(
                self.message
                    .as_ref()
                    .expect("Message should be something here"),
                2,
            );
        }

        if self.character_list.is_some() {
            builder.append(&mut self.character_list.as_ref().unwrap().serialize());
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
        let mut reply = Reply::new();
        let reader = StreamReader::new(&data);
        reply.deserialize(&reader);
        assert_eq!(reply.reply, Some(CharacterReply::Exists));
        assert_eq!(reply.message, Some("NO".to_string()));
    }
    #[test]
    fn serialize() {
        let mut reply = Reply::new();
        reply.reply = Some(CharacterReply::Exists);
        reply.message = Some("NO".to_string());
        assert_eq!(reply.serialize(), [2, 254, 78, 79]);
    }
}
