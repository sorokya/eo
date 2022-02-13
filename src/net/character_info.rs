use num_traits::FromPrimitive;

use crate::{
    character::{AdminLevel, Gender, Race},
    data::{EOChar, EOInt, Serializeable, StreamBuilder, StreamReader},
};

use super::{PaperdollBAHSW, PAPERDOLL_BAHSW_SIZE};

pub const CHARACTER_INFO_SIZE: usize = 10 + PAPERDOLL_BAHSW_SIZE;

#[derive(Debug, Default)]
pub struct CharacterInfo {
    pub name: String,
    pub id: EOInt,
    pub level: EOChar,
    pub gender: Gender,
    pub hair_style: EOChar,
    pub hair_color: EOChar,
    pub race: Race,
    pub admin_level: AdminLevel,
    pub paperdoll: PaperdollBAHSW,
}

impl CharacterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for CharacterInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.id = reader.get_int();
        self.level = reader.get_char();
        let gender_char = reader.get_char();
        self.gender = match Gender::from_u8(gender_char) {
            Some(gender) => gender,
            None => panic!("Failed to convert char to Gender: {}", gender_char),
        };
        self.hair_style = reader.get_char();
        self.hair_color = reader.get_char();
        let race_char = reader.get_char();
        self.race = match Race::from_u8(race_char) {
            Some(race) => race,
            None => panic!("Failed to convert char to Race: {}", race_char),
        };
        let admin_level_char = reader.get_char();
        self.admin_level = match AdminLevel::from_u8(admin_level_char) {
            Some(admin_level) => admin_level,
            None => panic!("Failed to convert char to AdminLevel: {}", admin_level_char),
        };
        self.paperdoll.deserialize(reader);
    }
    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_INFO_SIZE + self.name.len() + 1);
        builder.add_break_string(&self.name);
        builder.add_int(self.id);
        builder.add_char(self.level);
        builder.add_char(self.gender as EOChar);
        builder.add_char(self.hair_style);
        builder.add_char(self.hair_color);
        builder.add_char(self.race as EOChar);
        builder.add_char(self.admin_level as EOChar);
        builder.append(&mut self.paperdoll.serialize());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        character::{AdminLevel, Gender, Race},
        data::{EOByte, Serializeable, StreamReader},
        net::PaperdollBAHSW,
    };

    use super::CharacterInfo;

    #[test]
    fn deserialize() {
        let buf: Vec<EOByte> = vec![
            103, 111, 114, 111, 110, 255, 182, 73, 3, 254, 42, 2, 25, 4, 1, 1, 53, 254, 49, 254,
            34, 254, 17, 254, 74, 254,
        ];
        let reader = StreamReader::new(&buf);
        let mut character_info = CharacterInfo::new();
        character_info.deserialize(&reader);
        assert_eq!(character_info.name, "goron");
        assert_eq!(character_info.id, 146415);
        assert_eq!(character_info.level, 41);
        assert_eq!(character_info.gender, Gender::Male);
        assert_eq!(character_info.hair_style, 24);
        assert_eq!(character_info.hair_color, 3);
        assert_eq!(character_info.race, Race::White);
        assert_eq!(character_info.admin_level, AdminLevel::Player);
        assert_eq!(character_info.paperdoll.boots, 52);
        assert_eq!(character_info.paperdoll.armor, 48);
        assert_eq!(character_info.paperdoll.hat, 33);
        assert_eq!(character_info.paperdoll.shield, 16);
        assert_eq!(character_info.paperdoll.weapon, 73);
    }
    #[test]
    fn serialize() {
        let mut character_info = CharacterInfo::new();
        character_info.name = "goron".to_string();
        character_info.id = 146415;
        character_info.level = 41;
        character_info.gender = Gender::Male;
        character_info.hair_style = 24;
        character_info.hair_color = 3;
        character_info.race = Race::White;
        character_info.admin_level = AdminLevel::Player;
        let mut paperdoll_bahsw = PaperdollBAHSW::new();
        paperdoll_bahsw.boots = 52;
        paperdoll_bahsw.armor = 48;
        paperdoll_bahsw.hat = 33;
        paperdoll_bahsw.shield = 16;
        paperdoll_bahsw.weapon = 73;
        character_info.paperdoll = paperdoll_bahsw;

        assert_eq!(
            character_info.serialize(),
            [
                103, 111, 114, 111, 110, 255, 182, 73, 3, 254, 42, 2, 25, 4, 1, 1, 53, 254, 49,
                254, 34, 254, 17, 254, 74, 254
            ]
        );
    }
}
