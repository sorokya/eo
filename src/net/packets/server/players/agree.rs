use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::{CharacterMapInfo, CHARACTER_MAP_INFO_SIZE},
};

const REPLY_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Agree {
    pub character: CharacterMapInfo,
    pub unknown: EOChar,
}

impl Agree {
    pub fn new(character: CharacterMapInfo) -> Self {
        Self {
            character,
            unknown: 1,
        }
    }
}

impl Serializeable for Agree {
    fn deserialize(&mut self, reader: &StreamReader) {
        reader.seek(1);
        self.character.deserialize(&reader);
        self.unknown = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REPLY_SIZE + CHARACTER_MAP_INFO_SIZE);
        builder.add_byte(EO_BREAK_CHAR);
        builder.append(&mut self.character.serialize());
        builder.add_char(self.unknown);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![
            255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255, 2,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(agree.character.name, "player".to_string());
        assert_eq!(agree.unknown, 1);
    }
    #[test]
    fn serialize() {
        let buf: Vec<EOByte> = vec![
            255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255, 2,
        ];
        let reader = StreamReader::new(&buf);
        let mut agree = Agree::default();
        agree.deserialize(&reader);
        assert_eq!(agree.serialize(), buf);
    }
}
