use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::replies::AccountReply,
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: Option<AccountReply>,
    pub session_id: Option<EOShort>,
    pub message: String,
    pub sequence: Option<EOChar>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no(reply: AccountReply) -> Self {
        Self {
            reply: Some(reply),
            session_id: None,
            message: "NO".to_string(),
            sequence: None,
        }
    }

    pub fn ok(reply: AccountReply) -> Self {
        Self {
            reply: Some(reply),
            session_id: None,
            message: "GO".to_string(),
            sequence: None,
        }
    }

    pub fn r#continue(session_id: EOShort, sequence: EOChar) -> Self {
        Self {
            reply: None,
            session_id: Some(session_id),
            message: "OK".to_string(),
            sequence: Some(sequence),
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_or_session_id = reader.get_short();
        if reply_code_or_session_id >= 10 {
            self.session_id = Some(reply_code_or_session_id);
            self.sequence = Some(reader.get_char());
        } else {
            self.reply = match AccountReply::from_u16(reply_code_or_session_id) {
                Some(reply) => Some(reply),
                None => panic!(
                    "Failed to convert short to AccountReply: {}",
                    reply_code_or_session_id
                ),
            };
        }
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            if self.session_id.is_some() { 3 } else { 2 } + self.message.len(),
        );

        if let Some(session_id) = self.session_id {
            builder.add_short(session_id);
        }
        if let Some(sequence) = self.sequence {
            builder.add_char(sequence);
        }
        if let Some(reply) = self.reply {
            builder.add_short(reply as EOShort);
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
        let data: Vec<EOByte> = vec![4, 254, 79, 75];
        let mut reply = Reply::new();
        let reader = StreamReader::new(&data);
        reply.deserialize(&reader);
        assert_eq!(reply.reply.unwrap(), AccountReply::Created);
        assert_eq!(reply.message, "OK");
    }
    #[test]
    fn serialize() {
        let mut reply = Reply::new();
        reply.reply = Some(AccountReply::Created);
        reply.message = "OK".to_string();
        assert_eq!(reply.serialize(), [4, 254, 79, 75]);
    }
}
