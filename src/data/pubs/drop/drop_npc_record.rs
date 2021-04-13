#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::drop::{DropRecord, EDF_DROP_DATA_SIZE, EDF_NPC_DATA_SIZE},
    {EOByte, EOShort, Serializeable, StreamBuilder, StreamReader},
};

/// data structure of a single edf record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropNPCRecord {
    /// links to an npc id
    pub npc_id: EOShort,
    /// number of drops the npc has
    pub length: EOShort,
    /// list of drop records
    pub drops: Vec<DropRecord>,
}

impl DropNPCRecord {
    /// creates a new DropRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for DropNPCRecord {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.npc_id = reader.get_short();
        self.length = reader.get_short();
        self.drops = Vec::with_capacity(self.length as usize);
        for _ in 0..self.length {
            let mut drop = DropRecord::new();
            drop.deserialize(reader);
            self.drops.push(drop);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            EDF_NPC_DATA_SIZE + self.length as usize * EDF_DROP_DATA_SIZE,
        );

        builder.add_short(self.npc_id);
        builder.add_short(self.length);
        for drop in &self.drops {
            builder.append(&mut Serializeable::serialize(drop));
        }
        builder.get()
    }
}
