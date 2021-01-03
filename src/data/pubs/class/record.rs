use num_traits::FromPrimitive;

use crate::data::{
    pubs::{class::*, PubRecord},
    *,
};

#[derive(Debug, Clone, Default)]
pub struct ClassRecord {
    pub id: EOInt,
    pub name: String,
    pub base: EOChar,
    pub class_type: ClassType,
    pub strength: EOShort,
    pub intelligence: EOShort,
    pub wisdom: EOShort,
    pub agility: EOShort,
    pub constitution: EOShort,
    pub charisma: EOShort,
}

impl ClassRecord {
    pub fn new(id: EOInt) -> Self {
        let mut record = Self::default();
        record.id = id;
        record
    }
}

impl PubRecord for ClassRecord {
    fn deserialize(&mut self, buf: &[EOByte]) {
        let mut reader = StreamReader::new(&buf);
        self.name = reader.get_prefix_string();
        self.base = reader.get_char();
        let type_char = reader.get_char();
        self.class_type = match ClassType::from_u8(type_char) {
            Some(b) => b,
            None => panic!("Failed to convert char to ClassType: {}", type_char),
        };
        self.strength = reader.get_short();
        self.intelligence = reader.get_short();
        self.wisdom = reader.get_short();
        self.agility = reader.get_short();
        self.constitution = reader.get_short();
        self.charisma = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ECF_DATA_SIZE + self.name.len() + 1);
        builder.add_prefix_string(&self.name);
        builder.add_char(self.base);
        builder.add_char(self.class_type as u8);
        builder.add_short(self.strength);
        builder.add_short(self.intelligence);
        builder.add_short(self.wisdom);
        builder.add_short(self.agility);
        builder.add_short(self.constitution);
        builder.add_short(self.charisma);
        builder.get()
    }
}
