use crate::data::{
    pubs::{inn::*, PubRecord},
    *,
};

#[derive(Debug, Clone, Default)]
pub struct InnRecord {
    pub id: EOShort,
    pub name: String,
    pub spawn_map: EOShort,
    pub spawn_x: EOChar,
    pub spawn_y: EOChar,
    pub inn_sleep_map: EOShort,
    pub inn_sleep_x: EOChar,
    pub inn_sleep_y: EOChar,
    pub alt_spawn_enabled: bool,
    pub alt_spawn_map: EOShort,
    pub alt_spawn_x: EOChar,
    pub alt_spawn_y: EOChar,
    pub question1: String,
    pub answer1: String,
    pub question2: String,
    pub answer2: String,
    pub question3: String,
    pub answer3: String,
}

impl InnRecord {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PubRecord for InnRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.id = reader.get_short();
        self.name = reader.get_prefix_string();
        self.spawn_map = reader.get_short();
        self.spawn_x = reader.get_char();
        self.spawn_y = reader.get_char();
        self.inn_sleep_map = reader.get_short();
        self.inn_sleep_x = reader.get_char();
        self.inn_sleep_y = reader.get_char();
        self.alt_spawn_enabled = reader.get_char() == 1;
        self.alt_spawn_map = reader.get_short();
        self.alt_spawn_x = reader.get_char();
        self.alt_spawn_y = reader.get_char();
        self.question1 = reader.get_prefix_string();
        self.answer1 = reader.get_prefix_string();
        self.question2 = reader.get_prefix_string();
        self.answer2 = reader.get_prefix_string();
        self.question3 = reader.get_prefix_string();
        self.answer3 = reader.get_prefix_string();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            EID_DATA_SIZE
                + self.name.len()
                + self.question1.len()
                + self.answer1.len()
                + self.question2.len()
                + self.answer2.len()
                + self.question3.len()
                + self.answer3.len()
                + 7,
        );

        builder.add_short(self.id);
        builder.add_prefix_string(&self.name);
        builder.add_short(self.spawn_map);
        builder.add_char(self.spawn_x);
        builder.add_char(self.spawn_y);
        builder.add_short(self.inn_sleep_map);
        builder.add_char(self.inn_sleep_x);
        builder.add_char(self.inn_sleep_y);
        builder.add_char(self.alt_spawn_enabled as EOChar);
        builder.add_short(self.alt_spawn_map);
        builder.add_char(self.alt_spawn_x);
        builder.add_char(self.alt_spawn_y);
        builder.add_prefix_string(&self.question1);
        builder.add_prefix_string(&self.answer1);
        builder.add_prefix_string(&self.question2);
        builder.add_prefix_string(&self.answer2);
        builder.add_prefix_string(&self.question3);
        builder.add_prefix_string(&self.answer3);
        builder.get()
    }
}
