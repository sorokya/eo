use num_traits::FromPrimitive;

use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::{
        replies::MapInfoReply, CharacterMapInfo, NpcMapInfo, CHARACTER_MAP_INFO_SIZE,
        NPC_MAP_INFO_SIZE,
    },
};

const REPLY_SIZE: usize = 1;

#[derive(Debug, Default)]
pub struct Reply {
    pub reply: MapInfoReply,
    pub character: Option<CharacterMapInfo>,
    pub npc: Option<NpcMapInfo>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn character(character: CharacterMapInfo) -> Self {
        Self {
            reply: MapInfoReply::Character,
            character: Some(character),
            npc: None,
        }
    }
    pub fn npc(npc: NpcMapInfo) -> Self {
        Self {
            reply: MapInfoReply::NPC,
            character: None,
            npc: Some(npc),
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        let reply_char = reader.get_char();
        self.reply = match MapInfoReply::from_u8(reply_char) {
            Some(reply) => reply,
            None => panic!("Failed to convert char to MapInfoReply: {}", reply_char),
        };

        reader.seek(1);

        match self.reply {
            MapInfoReply::NPC => {
                let mut npc = NpcMapInfo::new();
                npc.deserialize(&reader);
                self.npc = Some(npc);
                self.character = None;
            }
            MapInfoReply::Character => {
                let mut character = CharacterMapInfo::new();
                character.deserialize(&reader);
                self.character = Some(character);
                self.npc = None;
            }
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let data_size = match self.reply {
            MapInfoReply::NPC => NPC_MAP_INFO_SIZE,
            MapInfoReply::Character => CHARACTER_MAP_INFO_SIZE,
        };
        let mut builder = StreamBuilder::with_capacity(REPLY_SIZE + data_size);
        builder.add_char(self.reply as EOChar);
        builder.add_byte(EO_BREAK_CHAR);

        match self.reply {
            MapInfoReply::NPC => {
                let npc = self.npc.as_ref().unwrap();
                builder.append(&mut npc.serialize());
            }
            MapInfoReply::Character => {
                let character = self.character.as_ref().unwrap();
                builder.append(&mut character.serialize());
            }
        }

        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Reply;
    use crate::{
        data::{EOByte, Serializeable, StreamReader},
        net::replies::MapInfoReply,
    };

    #[test]
    fn deserialize_ok() {
        let buf: Vec<EOByte> = vec![
            2, 255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.reply, MapInfoReply::Character);
        assert!(reply.character.is_some());
        assert!(reply.npc.is_none());

        let buf: Vec<EOByte> = vec![
            1, 255, 50, 2, 254, 41, 29, 1
        ];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert_eq!(reply.reply, MapInfoReply::NPC);
        assert!(reply.character.is_none());
        assert!(reply.npc.is_some());
    }
    #[test]
    fn serialize_ok() {
        let buf: Vec<EOByte> = vec![
            2, 255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255,
        ];
        let reader = StreamReader::new(&buf);
        let mut reply = Reply::new();
        reply.deserialize(&reader);
        assert_eq!(reply.serialize(), buf);

        let buf: Vec<EOByte> = vec![
            1, 255, 50, 2, 254, 41, 29, 1
        ];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert_eq!(reply.serialize(), buf);
    }
}
