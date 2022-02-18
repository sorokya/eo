use num_traits::FromPrimitive;

use crate::{
    character::AdminLevel,
    data::{
        EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader,
        EO_BREAK_CHAR,
    },
    net::{
        replies::WelcomeReply, CharacterStats2, Item, NearbyInfo, PaperdollFull, ServerSettings,
        Spell, Weight, CHARACTER_STATS_2_SIZE, ITEM_SIZE, NEARBY_INFO_SIZE, PAPERDOLL_FULL_SIZE,
        SERVER_SETTINGS_SIZE, SPELL_SIZE, WEIGHT_SIZE,
    },
};

const REPLY_SIZE: usize = 2;
#[derive(Debug, Default)]
pub struct Reply {
    pub reply: WelcomeReply,
    pub select_character: Option<SelectCharacter>,
    pub enter_game: Option<EnterGame>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_short = reader.get_short();
        self.reply = match WelcomeReply::from_u16(reply_short) {
            Some(reply) => reply,
            None => panic!("Failed to convert short to WelcomeReply: {}", reply_short),
        };

        if self.reply == WelcomeReply::SelectCharacter {
            let mut select_character = SelectCharacter::new();
            select_character.deserialize(reader);
            self.select_character = Some(select_character);
        } else if self.reply == WelcomeReply::EnterGame {
            let mut enter_game = EnterGame::new();
            enter_game.deserialize(reader);
            self.enter_game = Some(enter_game);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        match self.reply {
            WelcomeReply::SelectCharacter => {
                let mut select_character_data = match &self.select_character {
                    Some(select_character) => select_character.serialize(),
                    None => panic!("Reply is SelectCharacter but no SelectCharacter data"),
                };
                let mut builder =
                    StreamBuilder::with_capacity(REPLY_SIZE + select_character_data.len());
                builder.add_short(self.reply as EOShort);
                builder.append(&mut select_character_data);
                builder.get()
            }
            WelcomeReply::EnterGame => {
                let mut enter_game_data = match &self.enter_game {
                    Some(enter_game) => enter_game.serialize(),
                    None => panic!("Reply is EnterGame but no EnterGame data"),
                };
                let mut builder = StreamBuilder::with_capacity(REPLY_SIZE + enter_game_data.len());
                builder.add_short(self.reply as EOShort);
                builder.append(&mut enter_game_data);
                builder.get()
            }
            _ => {
                let mut builder = StreamBuilder::with_capacity(REPLY_SIZE);
                builder.add_short(self.reply as EOShort);
                builder.get()
            }
        }
    }
}

const SELECT_CHARACTER_SIZE: usize =
    CHARACTER_STATS_2_SIZE + PAPERDOLL_FULL_SIZE + SERVER_SETTINGS_SIZE + 42;
#[derive(Debug, Default)]
pub struct SelectCharacter {
    pub player_id: EOShort,
    pub character_id: EOInt,
    pub map_id: EOShort,
    pub map_hash: [EOByte; 4],
    pub map_filesize: EOThree,
    pub eif_hash: [EOByte; 4],
    pub eif_length: EOShort,
    pub enf_hash: [EOByte; 4],
    pub enf_length: EOShort,
    pub esf_hash: [EOByte; 4],
    pub esf_length: EOShort,
    pub ecf_hash: [EOByte; 4],
    pub ecf_length: EOShort,
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

impl SelectCharacter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for SelectCharacter {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.player_id = reader.get_short();
        self.character_id = reader.get_int();
        self.map_id = reader.get_short();
        self.map_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.map_filesize = reader.get_three();
        self.eif_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.eif_length = reader.get_short();
        self.enf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.enf_length = reader.get_short();
        self.esf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.esf_length = reader.get_short();
        self.ecf_hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.ecf_length = reader.get_short();
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
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SELECT_CHARACTER_SIZE);
        builder.add_short(self.player_id);
        builder.add_int(self.character_id);
        builder.add_short(self.map_id);
        builder.append(&mut self.map_hash.to_vec());
        builder.add_three(self.map_filesize);
        builder.append(&mut self.eif_hash.to_vec());
        builder.add_short(self.eif_length);
        builder.append(&mut self.enf_hash.to_vec());
        builder.add_short(self.enf_length);
        builder.append(&mut self.esf_hash.to_vec());
        builder.add_short(self.esf_length);
        builder.append(&mut self.ecf_hash.to_vec());
        builder.add_short(self.ecf_length);
        builder.add_break_string(&self.name);
        builder.add_break_string(&self.title);
        builder.add_break_string(&self.guild_name);
        builder.add_break_string(&self.guild_rank_name);
        builder.add_char(self.class_id);
        builder.add_fixed_string(&self.guild_tag, 3);
        builder.add_char(self.admin_level as EOChar);
        builder.add_char(self.level);
        builder.add_int(self.experience);
        builder.add_int(self.usage);
        builder.append(&mut self.stats.serialize());
        builder.append(&mut self.paperdoll.serialize());
        builder.add_char(self.guild_rank);
        builder.append(&mut self.settings.serialize());
        builder.add_char(self.login_message);
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

const ENTER_GAME_SIZE: usize = WEIGHT_SIZE + NEARBY_INFO_SIZE + 3;
#[derive(Debug, Default)]
pub struct EnterGame {
    pub news: [String; 9],
    pub weight: Weight,
    pub items: Vec<Item>,
    pub spells: Vec<Spell>,
    pub nearby_info: NearbyInfo,
}

impl EnterGame {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for EnterGame {
    fn deserialize(&mut self, reader: &StreamReader) {
        reader.get_byte();
        for i in 0..9 {
            self.news[i] = reader.get_break_string();
        }
        self.weight.deserialize(reader);
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut item = Item::new();
            item.deserialize(reader);
            self.items.push(item);
        }
        reader.get_byte();
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut spell = Spell::new();
            spell.deserialize(reader);
            self.spells.push(spell);
        }
        reader.get_byte();
        self.nearby_info.deserialize(reader);
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity({
            let news_size = self.news.iter().fold(0, |acc, news| acc + news.len());
            ENTER_GAME_SIZE
                + news_size
                + 9
                + self.items.len() * ITEM_SIZE
                + self.spells.len() * SPELL_SIZE
        });

        builder.add_byte(EO_BREAK_CHAR);
        for i in 0..9 {
            builder.add_break_string(&self.news[i]);
        }
        builder.append(&mut self.weight.serialize());
        for item in &self.items {
            builder.append(&mut item.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        for spell in &self.spells {
            builder.append(&mut spell.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        builder.append(&mut self.nearby_info.serialize());
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        character::{Gender, Race, SitState},
        net::{CharacterMapInfo, NpcMapInfo},
        world::Direction,
    };

    use super::*;

    #[test]
    fn deserialize_enter_game() {
        let data: Vec<EOByte> = vec![
            0x03, 0xFE, 0xFF, 0x57, 0x65, 0x6C, 0x63, 0x6F, 0x6D, 0x65, 0x20, 0x74, 0x6F, 0x20,
            0x65, 0x6E, 0x64, 0x6C, 0x65, 0x73, 0x73, 0x20, 0x76, 0x2E, 0x32, 0x35, 0xFF, 0x5B,
            0x41, 0x55, 0x47, 0x20, 0x32, 0x30, 0x31, 0x33, 0x5D, 0x20, 0x62, 0x65, 0x63, 0x61,
            0x75, 0x73, 0x65, 0x20, 0x6F, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x64, 0x6F,
            0x73, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6B, 0x73, 0x20, 0x61, 0x6E, 0x64, 0x20,
            0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x61, 0x63, 0x74,
            0x69, 0x6F, 0x6E, 0x73, 0x20, 0x6F, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72,
            0x65, 0x76, 0x69, 0x6F, 0x75, 0x73, 0x20, 0x68, 0x6F, 0x73, 0x74, 0x20, 0x28, 0x78,
            0x73, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x65, 0x75, 0x29, 0x20, 0x61, 0x6C,
            0x6C, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x63, 0x6F,
            0x72, 0x72, 0x75, 0x70, 0x74, 0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x72, 0x6F,
            0x6C, 0x6C, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6B, 0x20, 0x74, 0x6F, 0x20, 0x6F,
            0x63, 0x74, 0x6F, 0x62, 0x65, 0x72, 0x20, 0x32, 0x30, 0x31, 0x32, 0x3B, 0x20, 0x61,
            0x6E, 0x64, 0x20, 0x65, 0x6E, 0x64, 0x6C, 0x65, 0x73, 0x73, 0x20, 0x6F, 0x6E, 0x6C,
            0x69, 0x6E, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x66, 0x6F, 0x72, 0x63, 0x65, 0x64,
            0x20, 0x69, 0x6E, 0x74, 0x6F, 0x20, 0x61, 0x20, 0x6E, 0x65, 0x77, 0x20, 0x64, 0x61,
            0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x2E, 0x20,
            0x50, 0x6C, 0x65, 0x61, 0x73, 0x65, 0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x63, 0x74,
            0x20, 0x78, 0x73, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x65, 0x75, 0x20, 0x69,
            0x66, 0x20, 0x79, 0x6F, 0x75, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x63, 0x6F, 0x6D,
            0x6D, 0x65, 0x6E, 0x74, 0x73, 0x20, 0x6F, 0x72, 0x20, 0x71, 0x75, 0x65, 0x73, 0x74,
            0x69, 0x6F, 0x6E, 0x73, 0x20, 0x61, 0x62, 0x6F, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69,
            0x73, 0x20, 0x6D, 0x61, 0x74, 0x74, 0x65, 0x72, 0x2E, 0xFF, 0x5B, 0x41, 0x55, 0x47,
            0x20, 0x32, 0x30, 0x31, 0x33, 0x5D, 0x20, 0x6F, 0x6C, 0x64, 0x20, 0x69, 0x6E, 0x61,
            0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65,
            0x72, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x72,
            0x65, 0x6C, 0x65, 0x61, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x61, 0x72,
            0x65, 0x20, 0x66, 0x72, 0x65, 0x65, 0x20, 0x74, 0x6F, 0x20, 0x72, 0x65, 0x2D, 0x72,
            0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x2C, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
            0x66, 0x75, 0x6E, 0x2E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x47, 0xFF,
            0xFF, 0x02, 0xFF, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72, 0xFF, 0x38, 0x02, 0xC1, 0xFE,
            0x07, 0xFE, 0x07, 0xFE, 0x01, 0x02, 0x20, 0x20, 0x20, 0x01, 0x01, 0x02, 0x01, 0x01,
            0x0B, 0xFE, 0x0B, 0xFE, 0x0B, 0xFE, 0x0B, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
            0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0x01,
            0xFF, 0x02, 0x10, 0x02, 0x02, 0x0C, 0x01, 0xFF,
        ];

        let mut reply = Reply::new();
        let reader = StreamReader::new(&data);
        reply.deserialize(&reader);

        assert_eq!(reply.reply, WelcomeReply::EnterGame);
        assert!(reply.enter_game.is_some());
        assert!(reply.select_character.is_none());

        let enter_game = reply.enter_game.unwrap();
        assert_eq!(enter_game.news.len(), 9);
        assert_eq!(enter_game.news[0], "Welcome to endless v.25");
        assert_eq!(enter_game.news[1], "[AUG 2013] because of the ddos attacks and the direct actions of the previous host (xsserver.eu) all data been corrupted and rolled back to october 2012; and endless online was forced into a new database start. Please contact xsserver.eu if you have comments or questions about this matter.");
        assert_eq!(enter_game.news[2], "[AUG 2013] old inactive character names been released and are free to re-register, have fun.");
        for i in 3..9 {
            assert_eq!(enter_game.news[i], "");
        }
        assert_eq!(enter_game.weight.current, 0);
        assert_eq!(enter_game.weight.max, 70);
        assert_eq!(enter_game.items.len(), 0);
        assert_eq!(enter_game.spells.len(), 0);
        assert_eq!(enter_game.nearby_info.characters.len(), 1);
        assert_eq!(enter_game.nearby_info.characters[0].name, "player");
        assert_eq!(enter_game.nearby_info.characters[0].id, 308);
        assert_eq!(enter_game.nearby_info.characters[0].map_id, 192);
        assert_eq!(enter_game.nearby_info.characters[0].coords.x, 6);
        assert_eq!(enter_game.nearby_info.characters[0].coords.y, 6);
        assert_eq!(
            enter_game.nearby_info.characters[0].direction,
            Direction::Down
        );
        assert_eq!(enter_game.nearby_info.characters[0].class_id, 1);
        assert_eq!(enter_game.nearby_info.characters[0].guild_tag, "   ");
        assert_eq!(enter_game.nearby_info.characters[0].level, 0);
        assert_eq!(enter_game.nearby_info.characters[0].gender, Gender::Female);
        assert_eq!(enter_game.nearby_info.characters[0].hair_style, 1);
        assert_eq!(enter_game.nearby_info.characters[0].hair_color, 0);
        assert_eq!(enter_game.nearby_info.characters[0].race, Race::White);
        assert_eq!(enter_game.nearby_info.characters[0].max_hp, 10);
        assert_eq!(enter_game.nearby_info.characters[0].hp, 10);
        assert_eq!(enter_game.nearby_info.characters[0].max_tp, 10);
        assert_eq!(enter_game.nearby_info.characters[0].tp, 10);
        assert_eq!(enter_game.nearby_info.characters[0].paperdoll.boots, 0);
        assert_eq!(enter_game.nearby_info.characters[0].paperdoll.armor, 0);
        assert_eq!(enter_game.nearby_info.characters[0].paperdoll.hat, 0);
        assert_eq!(enter_game.nearby_info.characters[0].paperdoll.shield, 0);
        assert_eq!(enter_game.nearby_info.characters[0].paperdoll.weapon, 0);
        assert_eq!(
            enter_game.nearby_info.characters[0].sit_state,
            SitState::Standing
        );
        assert_eq!(enter_game.nearby_info.characters[0].invisible, false);
        assert_eq!(enter_game.nearby_info.npcs.len(), 1);
        assert_eq!(enter_game.nearby_info.npcs[0].index, 1);
        assert_eq!(enter_game.nearby_info.npcs[0].id, 268);
        assert_eq!(enter_game.nearby_info.npcs[0].coords.x, 1);
        assert_eq!(enter_game.nearby_info.npcs[0].coords.y, 11);
        assert_eq!(enter_game.nearby_info.npcs[0].direction, Direction::Down);
        assert_eq!(enter_game.nearby_info.items.len(), 0);
    }

    #[test]
    fn serialize_enter_game() {
        let mut reply = Reply::new();
        reply.reply = WelcomeReply::EnterGame;

        let mut enter_game = EnterGame::new();
        enter_game.news[0] = "Welcome to endless v.25".to_string();
        enter_game.news[1] = "[AUG 2013] because of the ddos attacks and the direct actions of the previous host (xsserver.eu) all data been corrupted and rolled back to october 2012; and endless online was forced into a new database start. Please contact xsserver.eu if you have comments or questions about this matter.".to_string();
        enter_game.news[2] = "[AUG 2013] old inactive character names been released and are free to re-register, have fun.".to_string();
        for i in 3..9 {
            enter_game.news[i] = String::new();
        }
        enter_game.weight.current = 0;
        enter_game.weight.max = 70;
        enter_game
            .nearby_info
            .characters
            .push(CharacterMapInfo::new());
        enter_game.nearby_info.characters[0].name = "player".to_string();
        enter_game.nearby_info.characters[0].id = 308;
        enter_game.nearby_info.characters[0].map_id = 192;
        enter_game.nearby_info.characters[0].coords.x = 6;
        enter_game.nearby_info.characters[0].coords.y = 6;
        enter_game.nearby_info.characters[0].direction = Direction::Down;
        enter_game.nearby_info.characters[0].class_id = 1;
        enter_game.nearby_info.characters[0].guild_tag = "   ".to_string();
        enter_game.nearby_info.characters[0].level = 0;
        enter_game.nearby_info.characters[0].gender = Gender::Female;
        enter_game.nearby_info.characters[0].hair_style = 1;
        enter_game.nearby_info.characters[0].hair_color = 0;
        enter_game.nearby_info.characters[0].race = Race::White;
        enter_game.nearby_info.characters[0].max_hp = 10;
        enter_game.nearby_info.characters[0].hp = 10;
        enter_game.nearby_info.characters[0].max_tp = 10;
        enter_game.nearby_info.characters[0].tp = 10;
        enter_game.nearby_info.characters[0].paperdoll.boots = 0;
        enter_game.nearby_info.characters[0].paperdoll.armor = 0;
        enter_game.nearby_info.characters[0].paperdoll.hat = 0;
        enter_game.nearby_info.characters[0].paperdoll.shield = 0;
        enter_game.nearby_info.characters[0].paperdoll.weapon = 0;
        enter_game.nearby_info.characters[0].sit_state = SitState::Standing;
        enter_game.nearby_info.characters[0].invisible = false;
        enter_game.nearby_info.npcs.push(NpcMapInfo::new());
        enter_game.nearby_info.npcs[0].index = 1;
        enter_game.nearby_info.npcs[0].id = 268;
        enter_game.nearby_info.npcs[0].coords.x = 1;
        enter_game.nearby_info.npcs[0].coords.y = 11;
        enter_game.nearby_info.npcs[0].direction = Direction::Down;

        reply.enter_game = Some(enter_game);

        assert_eq!(
            reply.serialize(),
            vec![
                0x03, 0xFE, 0xFF, 0x57, 0x65, 0x6C, 0x63, 0x6F, 0x6D, 0x65, 0x20, 0x74, 0x6F, 0x20,
                0x65, 0x6E, 0x64, 0x6C, 0x65, 0x73, 0x73, 0x20, 0x76, 0x2E, 0x32, 0x35, 0xFF, 0x5B,
                0x41, 0x55, 0x47, 0x20, 0x32, 0x30, 0x31, 0x33, 0x5D, 0x20, 0x62, 0x65, 0x63, 0x61,
                0x75, 0x73, 0x65, 0x20, 0x6F, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x64, 0x6F,
                0x73, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x6B, 0x73, 0x20, 0x61, 0x6E, 0x64, 0x20,
                0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x20, 0x61, 0x63, 0x74,
                0x69, 0x6F, 0x6E, 0x73, 0x20, 0x6F, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72,
                0x65, 0x76, 0x69, 0x6F, 0x75, 0x73, 0x20, 0x68, 0x6F, 0x73, 0x74, 0x20, 0x28, 0x78,
                0x73, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x65, 0x75, 0x29, 0x20, 0x61, 0x6C,
                0x6C, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x63, 0x6F,
                0x72, 0x72, 0x75, 0x70, 0x74, 0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x72, 0x6F,
                0x6C, 0x6C, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6B, 0x20, 0x74, 0x6F, 0x20, 0x6F,
                0x63, 0x74, 0x6F, 0x62, 0x65, 0x72, 0x20, 0x32, 0x30, 0x31, 0x32, 0x3B, 0x20, 0x61,
                0x6E, 0x64, 0x20, 0x65, 0x6E, 0x64, 0x6C, 0x65, 0x73, 0x73, 0x20, 0x6F, 0x6E, 0x6C,
                0x69, 0x6E, 0x65, 0x20, 0x77, 0x61, 0x73, 0x20, 0x66, 0x6F, 0x72, 0x63, 0x65, 0x64,
                0x20, 0x69, 0x6E, 0x74, 0x6F, 0x20, 0x61, 0x20, 0x6E, 0x65, 0x77, 0x20, 0x64, 0x61,
                0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x2E, 0x20,
                0x50, 0x6C, 0x65, 0x61, 0x73, 0x65, 0x20, 0x63, 0x6F, 0x6E, 0x74, 0x61, 0x63, 0x74,
                0x20, 0x78, 0x73, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x65, 0x75, 0x20, 0x69,
                0x66, 0x20, 0x79, 0x6F, 0x75, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x63, 0x6F, 0x6D,
                0x6D, 0x65, 0x6E, 0x74, 0x73, 0x20, 0x6F, 0x72, 0x20, 0x71, 0x75, 0x65, 0x73, 0x74,
                0x69, 0x6F, 0x6E, 0x73, 0x20, 0x61, 0x62, 0x6F, 0x75, 0x74, 0x20, 0x74, 0x68, 0x69,
                0x73, 0x20, 0x6D, 0x61, 0x74, 0x74, 0x65, 0x72, 0x2E, 0xFF, 0x5B, 0x41, 0x55, 0x47,
                0x20, 0x32, 0x30, 0x31, 0x33, 0x5D, 0x20, 0x6F, 0x6C, 0x64, 0x20, 0x69, 0x6E, 0x61,
                0x63, 0x74, 0x69, 0x76, 0x65, 0x20, 0x63, 0x68, 0x61, 0x72, 0x61, 0x63, 0x74, 0x65,
                0x72, 0x20, 0x6E, 0x61, 0x6D, 0x65, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6E, 0x20, 0x72,
                0x65, 0x6C, 0x65, 0x61, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x61, 0x72,
                0x65, 0x20, 0x66, 0x72, 0x65, 0x65, 0x20, 0x74, 0x6F, 0x20, 0x72, 0x65, 0x2D, 0x72,
                0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x2C, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20,
                0x66, 0x75, 0x6E, 0x2E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x47, 0xFF,
                0xFF, 0x02, 0xFF, 0x70, 0x6C, 0x61, 0x79, 0x65, 0x72, 0xFF, 0x38, 0x02, 0xC1, 0xFE,
                0x07, 0xFE, 0x07, 0xFE, 0x01, 0x02, 0x20, 0x20, 0x20, 0x01, 0x01, 0x02, 0x01, 0x01,
                0x0B, 0xFE, 0x0B, 0xFE, 0x0B, 0xFE, 0x0B, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE,
                0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0xFE, 0x01, 0x01,
                0xFF, 0x02, 0x10, 0x02, 0x02, 0x0C, 0x01, 0xFF,
            ]
        );
    }
}
