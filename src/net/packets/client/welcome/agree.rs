use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader},
    net::FileType,
};

#[derive(Debug, Default)]
pub struct Agree {
    pub file_type: FileType,
    pub session_id: EOShort,
    pub character_id: Option<EOShort>,
    pub file_id: Option<EOChar>,
}

impl Agree {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Agree {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.file_type = FileType::from_char(reader.get_char());
        self.session_id = reader.get_short();
        match self.file_type {
            FileType::Map => {
                self.character_id = Some(reader.get_short());
            }
            _ => {
                self.file_id = Some(reader.get_char());
            }
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            3 + match self.file_type {
                FileType::Map => 2,
                _ => 1,
            },
        );
        builder.add_char(self.file_type as EOChar);
        builder.add_short(self.session_id);
        if let Some(character_id) = self.character_id {
            builder.add_short(character_id);
        }
        if let Some(file_id) = self.file_id {
            builder.add_char(file_id);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Agree, EOByte, FileType, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![4, 53, 111, 2];
        let mut packet = Agree::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.file_type, FileType::NPC);
        assert_eq!(packet.session_id, 27882);
        assert_eq!(packet.file_id, Some(1));
    }
    #[test]
    fn serialize() {
        let mut packet = Agree::new();
        packet.file_type = FileType::NPC;
        packet.session_id = 27882;
        packet.file_id = Some(1);
        assert_eq!(packet.serialize(), [4, 53, 111, 2])
    }
}
