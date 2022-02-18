use crate::data::{EOShort, Serializeable, StreamBuilder};

pub const CHARACTER_BASE_STATS_SIZE: usize = 12;
#[derive(Debug, Default, Clone)]
pub struct CharacterBaseStats {
    pub strength: EOShort,
    pub intelligence: EOShort,
    pub wisdom: EOShort,
    pub agility: EOShort,
    pub constitution: EOShort,
    pub charisma: EOShort,
}

impl Serializeable for CharacterBaseStats {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.strength = reader.get_short();
        self.intelligence = reader.get_short();
        self.wisdom = reader.get_short();
        self.agility = reader.get_short();
        self.constitution = reader.get_short();
        self.charisma = reader.get_short();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_BASE_STATS_SIZE);
        builder.add_short(self.strength);
        builder.add_short(self.intelligence);
        builder.add_short(self.wisdom);
        builder.add_short(self.agility);
        builder.add_short(self.constitution);
        builder.add_short(self.charisma);
        builder.get()
    }
}

pub const CHARACTER_SECONDARY_STATS_SIZE: usize = 10;
#[derive(Debug, Default, Clone)]
pub struct CharacterSecondaryStats {
    pub min_damage: EOShort,
    pub max_damage: EOShort,
    pub accuracy: EOShort,
    pub evasion: EOShort,
    pub armor: EOShort,
}

impl Serializeable for CharacterSecondaryStats {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.min_damage = reader.get_short();
        self.max_damage = reader.get_short();
        self.accuracy = reader.get_short();
        self.evasion = reader.get_short();
        self.armor = reader.get_short();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_SECONDARY_STATS_SIZE);
        builder.add_short(self.min_damage);
        builder.add_short(self.max_damage);
        builder.add_short(self.accuracy);
        builder.add_short(self.evasion);
        builder.add_short(self.armor);
        builder.get()
    }
}

pub const CHARACTER_STATS_1_SIZE: usize =
    CHARACTER_BASE_STATS_SIZE + CHARACTER_SECONDARY_STATS_SIZE + 14;
#[derive(Debug, Default)]
pub struct CharacterStats1 {
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub base: CharacterBaseStats,
    pub secondary: CharacterSecondaryStats,
}

impl Serializeable for CharacterStats1 {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.stat_points = reader.get_short();
        self.skill_points = reader.get_short();
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.base.deserialize(reader);
        self.secondary.deserialize(reader);
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_STATS_1_SIZE);
        builder.add_short(self.stat_points);
        builder.add_short(self.skill_points);
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.append(&mut self.base.serialize());
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

pub const CHARACTER_STATS_2_SIZE: usize =
    CHARACTER_BASE_STATS_SIZE + CHARACTER_SECONDARY_STATS_SIZE + 16;
#[derive(Debug, Default, Clone)]
pub struct CharacterStats2 {
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub stat_points: EOShort,
    pub skill_points: EOShort,
    pub karma: EOShort,
    pub secondary: CharacterSecondaryStats,
    pub base: CharacterBaseStats,
}

impl Serializeable for CharacterStats2 {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.stat_points = reader.get_short();
        self.skill_points = reader.get_short();
        self.karma = reader.get_short();
        self.secondary.deserialize(reader);
        self.base.deserialize(reader);
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_STATS_2_SIZE);
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.add_short(self.stat_points);
        builder.add_short(self.skill_points);
        builder.add_short(self.karma);
        builder.append(&mut self.secondary.serialize());
        builder.append(&mut self.base.serialize());
        builder.get()
    }
}

pub const CHARACTER_STATS_3_SIZE: usize =
    CHARACTER_BASE_STATS_SIZE + CHARACTER_SECONDARY_STATS_SIZE + 8;
#[derive(Debug, Default)]
pub struct CharacterStats3 {
    pub base: CharacterBaseStats,
    pub max_hp: EOShort,
    pub max_tp: EOShort,
    pub max_sp: EOShort,
    pub max_weight: EOShort,
    pub secondary: CharacterSecondaryStats,
}

impl Serializeable for CharacterStats3 {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.base.deserialize(reader);
        self.max_hp = reader.get_short();
        self.max_tp = reader.get_short();
        self.max_sp = reader.get_short();
        self.max_weight = reader.get_short();
        self.secondary.deserialize(reader);
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_STATS_3_SIZE);
        builder.append(&mut self.base.serialize());
        builder.add_short(self.max_hp);
        builder.add_short(self.max_tp);
        builder.add_short(self.max_sp);
        builder.add_short(self.max_weight);
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

pub const CHARACTER_STATS_4_SIZE: usize =
    CHARACTER_BASE_STATS_SIZE + CHARACTER_SECONDARY_STATS_SIZE + 8;
#[derive(Debug, Default)]
pub struct CharacterStats4 {
    pub hp: EOShort,
    pub max_hp: EOShort,
    pub tp: EOShort,
    pub max_tp: EOShort,
    pub base: CharacterBaseStats,
    pub secondary: CharacterSecondaryStats,
}

impl Serializeable for CharacterStats4 {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.hp = reader.get_short();
        self.max_hp = reader.get_short();
        self.tp = reader.get_short();
        self.max_tp = reader.get_short();
        self.base.deserialize(reader);
        self.secondary.deserialize(reader);
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHARACTER_STATS_4_SIZE);
        builder.add_short(self.hp);
        builder.add_short(self.max_hp);
        builder.add_short(self.tp);
        builder.add_short(self.max_tp);
        builder.append(&mut self.base.serialize());
        builder.append(&mut self.secondary.serialize());
        builder.get()
    }
}

// TODO: tests
