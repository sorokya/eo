use num_traits::FromPrimitive;

use crate::{data::{EOByte, Serializeable, StreamBuilder, StreamReader, EOShort, EO_BREAK_CHAR}, character::{Gender, Race}};

const CREATE_SIZE: usize = 11;

#[derive(Debug, Default)]
pub struct Create {
    pub session_id: EOShort,
    pub gender: Gender,
    pub hair_style: EOShort,
    pub hair_color: EOShort,
    pub race: Race,
    pub name: String,
}

impl Create {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Create {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.session_id = reader.get_short();
        let gender_short = reader.get_short();
        self.gender = match Gender::from_u16(gender_short) {
            Some(gender) => gender,
            None => panic!("Failed to convert short to Gender: {}", gender_short),
        };
        self.hair_style = reader.get_short();
        self.hair_color = reader.get_short();
        let race_short = reader.get_short();
        self.race = match Race::from_u16(race_short) {
            Some(race) => race,
            None => panic!("Failed to convert short to Race: {}", race_short),
        };
        reader.get_byte();
        self.name = reader.get_break_string().to_lowercase();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(CREATE_SIZE + self.name.len() + 1);
        builder.add_short(self.session_id);
        builder.add_short(self.gender as EOShort);
        builder.add_short(self.hair_style);
        builder.add_short(self.hair_color);
        builder.add_short(self.race as EOShort);
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_break_string(&self.name.to_lowercase());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{EOByte, StreamReader};

    #[test]
    fn serialize() {
        let mut packet = Create::new();
        packet.session_id = 1000;
        packet.gender = Gender::Male;
        packet.hair_style = 8;
        packet.hair_color = 3;
        packet.race = Race::White;
        packet.name = "goron".to_string();
        assert_eq!(packet.serialize(), [242, 4, 2, 254, 9, 254, 4, 254, 1, 254, 255, 103, 111, 114, 111, 110, 255])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![242, 4, 2, 254, 9, 254, 4, 254, 1, 254, 255, 103, 111, 114, 111, 110, 255];
        let mut packet = Create::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.session_id, 1000);
        assert_eq!(packet.gender, Gender::Male);
        assert_eq!(packet.hair_style, 8);
        assert_eq!(packet.hair_color, 3);
        assert_eq!(packet.race, Race::White);
        assert_eq!(packet.name, "goron".to_string());
    }
}
