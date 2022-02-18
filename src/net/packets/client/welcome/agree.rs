use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader},
    net::FileType,
};

#[derive(Debug, Default)]
pub struct Agree {
    pub file_type: FileType,
}

impl Agree {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Agree {
    fn deserialize(&mut self, reader: &StreamReader) {
        let file_type_char = reader.get_char();
        self.file_type = match FileType::from_u8(file_type_char) {
            Some(file_type) => file_type,
            _ => panic!("Failed to convert char to FileType: {}", file_type_char),
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(1);
        builder.add_char(self.file_type as EOChar);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Agree, EOByte, FileType, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2];
        let mut packet = Agree::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.file_type, FileType::Map);
    }
    #[test]
    fn serialize() {
        let mut packet = Agree::new();
        packet.file_type = FileType::Map;
        assert_eq!(packet.serialize(), [2])
    }
}
