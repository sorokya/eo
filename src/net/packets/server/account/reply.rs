use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::replies::AccountReply,
};

const REPLY_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: Option<AccountReply>,
    pub session_id: EOShort,
    pub message: String,
    pub sequence: EOChar,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn no(reply: AccountReply) -> Self {
        Self {
            reply: Some(reply),
            session_id: 0,
            message: "NO".to_string(),
            sequence: 0,
        }
    }

    pub fn ok(reply: AccountReply) -> Self {
        Self {
            reply: Some(reply),
            session_id: 0,
            message: "OK".to_string(),
            sequence: 0,
        }
    }

    pub fn r#continue(session_id: EOShort, sequence: EOChar) -> Self {
        Self {
            reply: None,
            session_id,
            message: "OK".to_string(),
            sequence,
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_code_or_session_id = reader.get_short();
        if reply_code_or_session_id > 6 {
            self.session_id = reply_code_or_session_id;
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
            REPLY_SIZE + if self.session_id > 0 { 1 } else { 0 } + self.message.len(),
        );
        if self.session_id > 0 {
            builder.add_short(self.session_id);
            builder.add_char(self.sequence);
        } else {
            builder.add_short(self.reply.expect("No session id or reply code") as EOShort);
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
