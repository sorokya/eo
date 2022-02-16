use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR};

use super::{
    CharacterMapInfo, ItemMapInfo, NpcMapInfo, CHARACTER_MAP_INFO_SIZE, ITEM_MAP_INFO_SIZE,
    NPC_MAP_INFO_SIZE,
};

pub const NEARBY_INFO_SIZE: usize =
    CHARACTER_MAP_INFO_SIZE + NPC_MAP_INFO_SIZE + ITEM_MAP_INFO_SIZE + 3;
#[derive(Debug, Default)]
pub struct NearbyInfo {
    pub characters: Vec<CharacterMapInfo>,
    pub npcs: Vec<NpcMapInfo>,
    pub items: Vec<ItemMapInfo>,
}

impl NearbyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NearbyInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        let num_of_characters = reader.get_char();
        reader.get_byte();
        for _ in 0..num_of_characters {
            let mut character = CharacterMapInfo::new();
            character.deserialize(reader);
            self.characters.push(character);
        }
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut npc = NpcMapInfo::new();
            npc.deserialize(reader);
            self.npcs.push(npc);
        }
        reader.get_byte();
        while !reader.eof() {
            let mut item = ItemMapInfo::new();
            item.deserialize(reader);
            self.items.push(item);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(NEARBY_INFO_SIZE);
        builder.add_char(self.characters.len() as EOChar);
        builder.add_byte(EO_BREAK_CHAR);
        for character in &self.characters {
            builder.append(&mut character.serialize());
        }
        for npc in &self.npcs {
            builder.append(&mut npc.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);
        for item in &self.items {
            builder.append(&mut item.serialize());
        }
        builder.get()
    }
}

// TODO: tests
