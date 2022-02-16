use num_traits::FromPrimitive;

use crate::{
    character::AdminLevel,
    data::{EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader},
    net::{
        replies::WelcomeReply, CharacterStats2, PaperdollFull, ServerSettings,
        CHARACTER_STATS_2_SIZE, PAPERDOLL_FULL_SIZE, SERVER_SETTINGS_SIZE,
    },
};

const REPLY_SIZE: usize = CHARACTER_STATS_2_SIZE + PAPERDOLL_FULL_SIZE + SERVER_SETTINGS_SIZE + 41;
#[derive(Debug, Default)]
pub struct ReplySelectCharacter {
    pub reply: WelcomeReply,
    pub player_id: EOShort,
    pub character_id: EOInt,
    pub map_id: EOShort,
    pub map_hash: [EOByte; 4],
    pub map_filename: EOThree,
    pub eif_hash: [EOByte; 4],
    pub enf_hash: [EOByte; 4],
    pub esf_hash: [EOByte; 4],
    pub ecf_hash: [EOByte; 4],
    pub name: String,
    pub title: String,
    pub guild_name: String,
    pub guild_rank_name: String,
    pub class_id: EOChar,
    pub guild_tag: String,
    pub admin_level: AdminLevel,
    pub level: EOChar,
    pub experience: EOInt,
    pub usage: EOInt,
    pub stats: CharacterStats2,
    pub paperdoll: PaperdollFull,
    pub guild_rank: EOChar,
    pub settings: ServerSettings,
    pub login_message: EOChar,
}

impl ReplySelectCharacter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ReplySelectCharacter {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_char = reader.get_char();
        self.reply = match WelcomeReply::from_u8(reply_char) {
            Some(reply) => reply,
            None => panic!("Failed to convert char to WelcomeReply: {}", reply_char),
        };
        self.player_id = reader.get_short();
        self.character_id = reader.get_int();
        self.map_id = reader.get_short();
        self.map_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.map_filename = reader.get_three();
        self.eif_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.enf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.esf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.ecf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.name = reader.get_break_string();
        self.title = reader.get_break_string();
        self.guild_name = reader.get_break_string();
        self.guild_rank_name = reader.get_break_string();
        self.class_id = reader.get_char();
        self.guild_tag = reader.get_fixed_string(3);
        let admin_level_char = reader.get_char();
        self.admin_level = match AdminLevel::from_u8(admin_level_char) {
            Some(admin_level) => admin_level,
            None => panic!("Failed to convert char to AdminLevel: {}", admin_level_char),
        };
        self.level = reader.get_char();
        self.experience = reader.get_int();
        self.usage = reader.get_int();
        self.stats.deserialize(reader);
        self.paperdoll.deserialize(reader);
        self.guild_rank = reader.get_char();
        self.settings.deserialize(reader);
        self.login_message = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REPLY_SIZE);
        builder.add_char(self.reply as EOChar);
        builder.add_short(self.player_id);
        builder.add_int(self.character_id);
        builder.add_short(self.map_id);
        for byte in self.map_hash.iter() {
            builder.add_byte(*byte);
        }
        builder.add_three(self.map_filename);
        for byte in self.eif_hash.iter() {
            builder.add_byte(*byte);
        }
        for byte in self.enf_hash.iter() {
            builder.add_byte(*byte);
        }
        for byte in self.esf_hash.iter() {
            builder.add_byte(*byte);
        }
        for byte in self.ecf_hash.iter() {
            builder.add_byte(*byte);
        }
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.title);
        builder.add_break_string(&self.guild_name);
        builder.add_break_string(&self.guild_rank_name);
        builder.add_char(self.class_id);
        builder.add_string(&self.guild_tag);
        builder.add_char(self.admin_level as EOChar);
        builder.add_char(self.level);
        builder.add_int(self.experience);
        builder.add_int(self.usage);
        builder.append(&mut self.stats.serialize());
        builder.append(&mut self.paperdoll.serialize());
        builder.add_char(self.guild_rank);
        builder.append(&mut self.settings.serialize());
        builder.add_char(self.login_message);
        builder.get()
    }
}
