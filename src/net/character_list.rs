use crate::data::{EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR};

use super::{CharacterInfo, CHARACTER_INFO_SIZE};

pub const CHARACTER_LIST_SIZE: usize = 4;

#[derive(Debug, Default)]
pub struct CharacterList {
    pub length: EOChar,
    pub unknown: EOChar,
    pub characters: Vec<CharacterInfo>,
}

impl CharacterList {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterList {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.length = reader.get_char();
        self.unknown = reader.get_char();
        reader.get_char();
        for _ in 0..self.length {
            let mut character = CharacterInfo::new();
            character.deserialize(reader);
            self.characters.push(character);
            reader.get_char();
        }
    }
    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let length_of_names_with_break_char = self
            .characters
            .iter()
            .fold(0, |acc, character| acc + character.name.len() + 1);
        let mut builder = StreamBuilder::with_capacity(
            length_of_names_with_break_char
                + CHARACTER_LIST_SIZE
                + self.characters.len() * CHARACTER_INFO_SIZE,
        );
        builder.add_char(self.length);
        builder.add_char(self.unknown);
        builder.add_byte(EO_BREAK_CHAR);
        for character in &self.characters {
            builder.append(&mut character.serialize());
            builder.add_byte(EO_BREAK_CHAR);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use character_list::CharacterList;

    use crate::{
        data::{EOByte, Serializeable, StreamReader},
        net::{character_list, CharacterInfo},
    };

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![
            4, 2, 255, 103, 111, 114, 111, 110, 255, 106, 74, 3, 254, 42, 2, 25, 4, 1, 1, 53, 254,
            49, 254, 34, 254, 17, 254, 74, 254, 255, 100, 105, 103, 105, 116, 120, 255, 107, 74, 3,
            254, 1, 1, 2, 1, 1, 1, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 255, 107, 97, 109, 105,
            110, 97, 255, 10, 8, 74, 3, 254, 1, 2, 2, 1, 1, 1, 1, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut character_list = CharacterList::new();
        character_list.deserialize(&reader);
        assert_eq!(character_list.length, 3);
        assert_eq!(character_list.unknown, 1);
        assert_eq!(character_list.characters.len(), 3);
        assert_eq!(character_list.characters[0].name, "goron");
        assert_eq!(character_list.characters[1].name, "digitx");
        assert_eq!(character_list.characters[2].name, "kamina");
    }
    #[test]
    fn serialize() {
        let mut character_list = CharacterList::new();
        character_list.length = 3;
        character_list.unknown = 1;
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[0].name = "goron".to_string();
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[1].name = "digitx".to_string();
        character_list.characters.push(CharacterInfo::new());
        character_list.characters[2].name = "kamina".to_string();
        let buf = character_list.serialize();
        let reader = StreamReader::new(&buf);
        let mut character_list = CharacterList::new();
        character_list.deserialize(&reader);
        assert_eq!(character_list.length, 3);
        assert_eq!(character_list.unknown, 1);
        assert_eq!(character_list.characters.len(), 3);
        assert_eq!(character_list.characters[0].name, "goron");
        assert_eq!(character_list.characters[1].name, "digitx");
        assert_eq!(character_list.characters[2].name, "kamina");
    }
}
