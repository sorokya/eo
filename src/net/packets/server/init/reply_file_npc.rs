use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader},
    net::replies::InitReply,
};

#[derive(Debug, Default)]
pub struct ReplyFileNPC {
    pub id: EOChar,
    pub data: Vec<EOByte>,
}

impl ReplyFileNPC {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ReplyFileNPC {
    fn deserialize(&mut self, reader: &StreamReader) {
        reader.get_char(); // reply code
        self.id = reader.get_char();
        self.data = reader.get_vec(reader.remaining());
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(2 + self.data.len());
        builder.add_char(InitReply::FileNPC as EOChar);
        builder.add_char(self.id);
        builder.append(&mut self.data.clone());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, ReplyFileNPC, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let npc_bytes: Vec<EOByte> = vec![
            0x07, 0x02, 0x45, 0x4E, 0x46, 0x02, 0xFE, 0xFE, 0xFE, 0x02, 0xFE, 0x01, 0x04, 0x65,
            0x6F, 0x66, 0x01, 0xFE, 0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01,
            0xFE, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
            0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0x01, 0xFE, 0xFE,
        ];

        let mut init_file_npc = ReplyFileNPC::new();
        let reader = StreamReader::new(&npc_bytes);
        init_file_npc.deserialize(&reader);
        assert_eq!(init_file_npc.id, 1);
        assert_eq!(init_file_npc.data, npc_bytes[2..]);
    }

    #[test]
    fn serialize() {
        let npc_bytes: Vec<EOByte> = vec![
            0x07, 0x02, 0x45, 0x4E, 0x46, 0x02, 0xFE, 0xFE, 0xFE, 0x02, 0xFE, 0x01, 0x04, 0x65,
            0x6F, 0x66, 0x01, 0xFE, 0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01,
            0xFE, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
            0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0x01, 0xFE, 0xFE,
        ];
        let mut init_file_npc = ReplyFileNPC::new();
        init_file_npc.id = 1;
        init_file_npc.data = npc_bytes[2..].to_vec();

        assert_eq!(init_file_npc.serialize(), npc_bytes);
    }
}
