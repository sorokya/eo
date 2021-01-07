#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use super::{NPCSpeed, NPC_SPAWN_SIZE};
use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NPCSpawn {
    pub x: EOChar,
    pub y: EOChar,
    pub npc_id: EOShort,
    pub speed: NPCSpeed,
    pub respawn_time: EOShort,
    pub amount: EOChar,
}

impl NPCSpawn {
    /// creates a new NPCSpawn with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for NPCSpawn {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.x = reader.get_char();
        self.y = reader.get_char();
        self.npc_id = reader.get_short();
        let speed_char = reader.get_char();
        self.speed = match NPCSpeed::from_u8(speed_char) {
            Some(speed) => speed,
            None => panic!("Failed to convert char to NPCSpeed: {}", speed_char),
        };
        self.respawn_time = reader.get_short();
        self.amount = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_SPAWN_SIZE);
        builder.add_char(self.x);
        builder.add_char(self.y);
        builder.add_short(self.npc_id);
        builder.add_char(self.speed as EOChar);
        builder.add_short(self.respawn_time);
        builder.add_char(self.amount);
        builder.get()
    }
}
