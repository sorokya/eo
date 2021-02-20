use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader},
    net::InitReply,
};

#[derive(Debug, Default)]
pub struct InitInit {
    pub reply_code: InitReply,
}

impl InitInit {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitInit {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        let reply_code_char = reader.get_char();
        self.reply_code = match InitReply::from_u8(reply_code_char) {
            Some(reply_code) => reply_code,
            None => panic!("Failed to convert char to InitReply: {}", reply_code_char),
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_char(self.reply_code as EOChar);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitInit, InitReply, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![0x03];
        let mut reader = StreamReader::new(&buf);
        let mut init = InitInit::new();
        init.deserialize(&mut reader);
        assert_eq!(init.reply_code, InitReply::OK);
    }
    #[test]
    fn serialize() {
        let mut init = InitInit::new();
        init.reply_code = InitReply::OK;
        assert_eq!(init.serialize(), [0x03]);
    }
}
