#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::CHEST_SPAWN_SIZE;
use crate::data::{EOByte, EOChar, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChestSpawn {
    pub x: EOChar,
    pub y: EOChar,
    pub key: EOShort,
    pub slot: EOChar,
    pub item_id: EOShort,
    pub respawn_time: EOShort,
    pub amount: EOThree,
}

impl ChestSpawn {
    /// creates a new ChestSpawn with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ChestSpawn {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
        self.key = reader.get_short();
        self.slot = reader.get_char();
        self.item_id = reader.get_short();
        self.respawn_time = reader.get_short();
        self.amount = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(CHEST_SPAWN_SIZE);
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.add_short(self.key);
        builder.add_char(self.slot);
        builder.add_short(self.item_id);
        builder.add_short(self.respawn_time);
        builder.add_three(self.amount);
        builder.get()
    }
}
