use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader},
    net::replies::InitReply,
};

#[derive(Debug, Default)]
pub struct InitFileClass {
    pub id: EOChar,
    pub data: Vec<EOByte>,
}

impl InitFileClass {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for InitFileClass {
    fn deserialize(&mut self, reader: &StreamReader) {
        reader.get_char(); // reply code
        self.id = reader.get_char();
        self.data = reader.get_vec(reader.remaining());
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(2 + self.data.len());
        builder.add_char(InitReply::FileClass as EOChar);
        builder.add_char(self.id);
        builder.append(&mut self.data.clone());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InitFileClass, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let class_bytes: Vec<EOByte> = vec![
            0x12, 0x02, 0x45, 0x43, 0x46, 0x02, 0xFE, 0xFE, 0xFE, 0x02, 0xFE, 0x01, 0x08, 0x50,
            0x65, 0x61, 0x73, 0x61, 0x6E, 0x74, 0x01, 0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
            0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
        ];

        let mut init_file_class = InitFileClass::new();
        let reader = StreamReader::new(&class_bytes);
        init_file_class.deserialize(&reader);
        assert_eq!(init_file_class.id, 1);
        assert_eq!(init_file_class.data, class_bytes[2..]);
    }

    #[test]
    fn serialize() {
        let class_bytes: Vec<EOByte> = vec![
            0x0C, 0x02, 0x45, 0x43, 0x46, 0x02, 0xFE, 0xFE, 0xFE, 0x02, 0xFE, 0x01, 0x08, 0x50,
            0x65, 0x61, 0x73, 0x61, 0x6E, 0x74, 0x01, 0x01, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
            0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
        ];
        let mut init_file_class = InitFileClass::new();
        init_file_class.id = 1;
        init_file_class.data = class_bytes[2..].to_vec();

        assert_eq!(init_file_class.serialize(), class_bytes);
    }
}
