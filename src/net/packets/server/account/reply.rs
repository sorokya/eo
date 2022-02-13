use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::replies::AccountReply,
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: AccountReply,
    pub session_id: EOShort,
    pub message: String,
    pub sequence: EOChar,
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
            self.reply = match AccountReply::from_u16(reply_code_or_session_id) {
                Some(reply) => reply,
                None => panic!("Failed to convert short to AccountReply: {}", reply_code_or_session_id),
            };
        }
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(3 + self.message.len());
        if self.session_id > 0 {
            builder.add_short(self.session_id);
            builder.add_char(self.sequence);
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

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![4, 254, 79, 75];
        let mut packet = Reply::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.reply, AccountReply::Created);
        assert_eq!(packet.message, "OK");
    }
    #[test]
    fn serialize() {
        let mut packet = Reply::new();
        packet.reply = AccountReply::Created;
        packet.message = "OK".to_string();
        assert_eq!(packet.serialize(), [4, 254, 79, 75]);
    }
}
