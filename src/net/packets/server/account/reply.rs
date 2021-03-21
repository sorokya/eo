use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::AccountReply,
};

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: AccountReply,
    pub message: String,
    pub sequence: EOChar,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        let reply_code = reader.get_short();
        self.reply = match AccountReply::from_u16(reply_code) {
            Some(reply) => reply,
            None => panic!("Failed to convert short to AccountReply: {}", reply_code),
        };
        self.message = reader.get_end_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(3 + self.message.len());
        builder.add_short(self.reply as EOShort);
        if self.reply == AccountReply::Continue {
            builder.add_char(self.sequence);
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
        let mut reader = StreamReader::new(&data);
        packet.deserialize(&mut reader);
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
