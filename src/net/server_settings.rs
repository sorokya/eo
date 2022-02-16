use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const SERVER_SETTINGS_SIZE: usize = 14;
#[derive(Debug, Default)]
pub struct ServerSettings {
    pub jail_map_id: EOShort,
    pub unknown_1: EOShort,
    pub unknown_2: EOChar,
    pub unknown_3: EOChar,
    pub light_guide_flood_rate: EOShort,
    pub guardian_flood_rate: EOShort,
    pub game_master_flood_rate: EOShort,
    pub unknown_4: EOShort,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ServerSettings {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.jail_map_id = reader.get_short();
        self.unknown_1 = reader.get_short();
        self.unknown_2 = reader.get_char();
        self.unknown_3 = reader.get_char();
        self.light_guide_flood_rate = reader.get_short();
        self.guardian_flood_rate = reader.get_short();
        self.game_master_flood_rate = reader.get_short();
        self.unknown_4 = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SERVER_SETTINGS_SIZE);
        builder.add_short(self.jail_map_id);
        builder.add_short(self.unknown_1);
        builder.add_char(self.unknown_2);
        builder.add_char(self.unknown_3);
        builder.add_short(self.light_guide_flood_rate);
        builder.add_short(self.guardian_flood_rate);
        builder.add_short(self.game_master_flood_rate);
        builder.add_short(self.unknown_4);
        builder.get()
    }
}

// TODO: tests
