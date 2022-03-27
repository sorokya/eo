use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const SERVER_SETTINGS_SIZE: usize = 14;
#[derive(Debug, Default)]
pub struct ServerSettings {
    pub jail_map_id: EOShort,
    pub recover_map: EOShort,
    pub recover_x: EOChar,
    pub recover_y: EOChar,
    pub light_guide_flood_rate: EOShort,
    pub guardian_flood_rate: EOShort,
    pub game_master_flood_rate: EOShort,
    pub high_game_master_flood_rate: EOShort,
}

impl ServerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ServerSettings {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.jail_map_id = reader.get_short();
        self.recover_map = reader.get_short();
        self.recover_x = reader.get_char();
        self.recover_y = reader.get_char();
        self.light_guide_flood_rate = reader.get_short();
        self.guardian_flood_rate = reader.get_short();
        self.game_master_flood_rate = reader.get_short();
        self.high_game_master_flood_rate = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SERVER_SETTINGS_SIZE);
        builder.add_short(self.jail_map_id);
        builder.add_short(self.recover_map);
        builder.add_char(self.recover_x);
        builder.add_char(self.recover_y);
        builder.add_short(self.light_guide_flood_rate);
        builder.add_short(self.guardian_flood_rate);
        builder.add_short(self.game_master_flood_rate);
        builder.add_short(self.high_game_master_flood_rate);
        builder.get()
    }
}

// TODO: tests
