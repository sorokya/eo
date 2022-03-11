use crate::{
    data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::{ItemMapInfo, ITEM_MAP_INFO_SIZE},
};

const REPLY_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Reply {
    pub player_ids: Vec<EOShort>,
    pub npc_indexes: Vec<EOChar>,
    pub items: Vec<ItemMapInfo>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        while reader.peek_byte() != EO_BREAK_CHAR {
            self.player_ids.push(reader.get_short());
        }
        reader.seek(1);
        while reader.peek_byte() != EO_BREAK_CHAR {
            self.npc_indexes.push(reader.get_char());
        }
        reader.seek(1);
        while !reader.eof() {
            let mut item = ItemMapInfo::new();
            item.deserialize(&reader);
            self.items.push(item);
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            REPLY_SIZE
                + (self.player_ids.len() * 2)
                + self.npc_indexes.len()
                + (self.items.len() * ITEM_MAP_INFO_SIZE),
        );
        for player_id in &self.player_ids {
            builder.add_short(*player_id);
        }
        builder.add_byte(EO_BREAK_CHAR);
        for npc_index in &self.npc_indexes {
            builder.add_char(*npc_index);
        }
        builder.add_byte(EO_BREAK_CHAR);
        for item in &self.items {
            builder.append(&mut item.serialize());
        }
        builder.get()
    }
    // 92_2_255_2_255_4_254_205_2_10_3_2_254_254
    // TODO: tests
}
