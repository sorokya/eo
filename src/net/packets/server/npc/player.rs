use crate::{
    data::{Serializeable, StreamBuilder, EO_BREAK_CHAR},
    net::{
        npc_attack::NPCAttack, NPCChat, NPCPosition, NPC_ATTACK_SIZE, NPC_CHAT_SIZE,
        NPC_POSITION_SIZE,
    },
};

pub const NPC_PLAYER_SIZE: usize = 3;

#[derive(Default, Debug)]
pub struct Player {
    pub positions: Vec<NPCPosition>,
    pub attacks: Vec<NPCAttack>,
    pub chats: Vec<NPCChat>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            attacks: Vec::new(),
            chats: Vec::new(),
        }
    }
}

impl Serializeable for Player {
    fn deserialize(&mut self, reader: &crate::data::StreamReader) {
        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut npc_position = NPCPosition::default();
            npc_position.deserialize(&reader);
        }
        reader.get_byte();

        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut npc_attack = NPCAttack::default();
            npc_attack.deserialize(&reader);
        }
        reader.get_byte();

        while reader.peek_byte() != EO_BREAK_CHAR {
            let mut npc_chat = NPCChat::default();
            npc_chat.deserialize(&reader);
        }
        reader.get_byte();
    }

    fn serialize(&self) -> Vec<crate::data::EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            NPC_PLAYER_SIZE
                + self.positions.len() * NPC_POSITION_SIZE
                + self.attacks.len() * NPC_ATTACK_SIZE
                + self.chats.len() * NPC_CHAT_SIZE
                + self.chats.len()
                + self
                    .chats
                    .iter()
                    .map(|chat| chat.message.len())
                    .sum::<usize>(),
        );

        for position in &self.positions {
            builder.append(&mut position.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);

        for position in &self.positions {
            builder.append(&mut position.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);

        for position in &self.positions {
            builder.append(&mut position.serialize());
        }
        builder.add_byte(EO_BREAK_CHAR);

        builder.get()
    }
}
