use crate::{
    character::{Gender, Race, SitState},
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    world::{Coords, Direction, WarpAnimation, COORDS_SIZE},
};

use super::{PaperdollB000A0HSW, PAPERDOLL_B000A0HSW_SIZE};

const CHARACTER_MAP_INFO_SIZE: usize = COORDS_SIZE + PAPERDOLL_B000A0HSW_SIZE + 26;
#[derive(Debug, Default)]
pub struct CharacterMapInfo {
    pub name: String,
    pub id: EOShort,
    pub map_id: EOShort,
    pub coords: Coords,
    pub direction: Direction,
    pub class_id: EOChar,
    pub guild_tag: String,
    pub level: EOChar,
    pub gender: Gender,
    pub hair_style: EOChar,
    pub hair_color: EOChar,
    pub race: Race,
    pub max_hp: EOShort,
    pub hp: EOShort,
    pub max_tp: EOShort,
    pub tp: EOShort,
    pub paperdoll: PaperdollB000A0HSW,
    pub sit_state: SitState,
    pub invisible: bool,
    pub warp_animation: Option<WarpAnimation>,
}

impl CharacterMapInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_size(&self) -> usize {
        CHARACTER_MAP_INFO_SIZE + self.name.len()
    }
}

impl Serializeable for CharacterMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_break_string();
        self.id = reader.get_short();
        self.map_id = reader.get_short();
        self.coords.deserialize(reader);
        self.direction = Direction::from_char(reader.get_char());
        self.class_id = reader.get_char();
        self.guild_tag = reader.get_fixed_string(3);
        self.level = reader.get_char();
        self.gender = Gender::from_char(reader.get_char());
        self.hair_style = reader.get_char();
        self.hair_color = reader.get_char();
        self.race = Race::from_char(reader.get_char());
        self.max_hp = reader.get_short();
        self.hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.tp = reader.get_short();
        self.paperdoll.deserialize(reader);
        self.sit_state = SitState::from_char(reader.get_char());
        self.invisible = reader.get_char() == 1;
        if reader.peek_byte() != EO_BREAK_CHAR {
            self.warp_animation = WarpAnimation::from_char(reader.get_char());
        }
        reader.get_byte();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            CHARACTER_MAP_INFO_SIZE + if self.warp_animation.is_some() { 1 } else { 0 },
        );
        builder.add_break_string(&self.name);
        builder.add_short(self.id);
        builder.add_short(self.map_id);
        builder.append(&mut self.coords.serialize());
        builder.add_char(self.direction as u8);
        builder.add_char(self.class_id);
        builder.add_fixed_string(&self.guild_tag, 3);
        builder.add_char(self.level);
        builder.add_char(self.gender as EOChar);
        builder.add_char(self.hair_style);
        builder.add_char(self.hair_color);
        builder.add_char(self.race as EOChar);
        builder.add_short(self.max_hp);
        builder.add_short(self.hp);
        builder.add_short(self.max_tp);
        builder.add_short(self.tp);
        builder.append(&mut self.paperdoll.serialize());
        builder.add_char(self.sit_state as EOChar);
        builder.add_char(if self.invisible { 1 } else { 0 });
        if let Some(warp_animation) = &self.warp_animation {
            builder.add_char(*warp_animation as EOChar);
        }
        builder.add_byte(EO_BREAK_CHAR);
        builder.get()
    }
}

// TODO: tests
