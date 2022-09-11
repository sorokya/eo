use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader},
    net::{NpcMapInfo, NPC_MAP_INFO_SIZE},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub npcs: Vec<NpcMapInfo>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        while !reader.eof() {
            let mut npc = NpcMapInfo::new();
            npc.deserialize(reader);
            self.npcs.push(npc);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(1 + self.npcs.len() * NPC_MAP_INFO_SIZE);
        builder.add_char(self.npcs.len() as EOChar);
        for npc in &self.npcs {
            builder.append(&mut npc.serialize());
        }
        builder.get()
    }
}

// TODO: tests
