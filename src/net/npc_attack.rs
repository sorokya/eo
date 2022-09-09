use log::warn;
use num_traits::FromPrimitive;

use crate::{
    data::{EOChar, EOShort, EOThree, Serializeable, StreamBuilder},
    world::Direction,
};

pub const NPC_ATTACK_SIZE: usize = 9;

#[derive(Debug, Default)]
pub struct NPCAttack {
    pub index: EOChar,
    pub target_dead: bool,
    pub direction: Direction,
    pub target_player_id: EOShort,
    pub target_damage: EOThree,
    pub target_health_percent: EOChar,
}

impl Serializeable for NPCAttack {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        self.index = reader.get_char();
        self.target_dead = reader.get_char() == 2;

        let direction_char = reader.get_char();
        self.direction = match Direction::from_u8(direction_char) {
            Some(direction) => direction,
            _ => {
                warn!("Failed to parse direction: {}", direction_char);
                Direction::Down
            }
        };

        self.target_player_id = reader.get_short();
        self.target_damage = reader.get_three();
        self.target_health_percent = reader.get_char();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(NPC_ATTACK_SIZE);
        builder.add_char(self.index);
        builder.add_char(if self.target_dead { 2 } else { 1 });
        builder.add_char(self.direction as EOChar);
        builder.add_short(self.target_player_id);
        builder.add_three(self.target_damage);
        builder.add_char(self.target_health_percent);
        builder.get()
    }
}
