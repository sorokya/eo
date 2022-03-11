use crate::{
    data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR},
    net::{CharacterMapInfo, NpcMapInfo, CHARACTER_MAP_INFO_SIZE, NPC_MAP_INFO_SIZE},
};

#[derive(Debug, Default)]
pub struct Reply {
    pub characters: Option<Vec<CharacterMapInfo>>,
    pub npcs: Option<Vec<NpcMapInfo>>,
}

impl Reply {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn characters(characters: Vec<CharacterMapInfo>) -> Self {
        Self {
            characters: Some(characters),
            npcs: None,
        }
    }
    pub fn npcs(npcs: Vec<NpcMapInfo>) -> Self {
        Self {
            characters: None,
            npcs: Some(npcs),
        }
    }
    pub fn both(characters: Vec<CharacterMapInfo>, npcs: Vec<NpcMapInfo>) -> Self {
        Self {
            characters: Some(characters),
            npcs: Some(npcs),
        }
    }
}

impl Serializeable for Reply {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.characters = None;
        self.npcs = None;
        let num_entities = reader.get_char();
        if reader.has_break() {
            reader.seek(1);
            let mut characters = Vec::with_capacity(num_entities as usize);
            for _ in 0..num_entities {
                let mut character = CharacterMapInfo::new();
                character.deserialize(reader);
                characters.push(character);
            }
            if characters.len() > 0 {
                self.characters = Some(characters);
            }
        }

        let mut npcs = Vec::with_capacity(if self.characters.is_none() {
            num_entities as usize
        } else {
            3
        });

        while !reader.eof() {
            let mut npc = NpcMapInfo::new();
            npc.deserialize(reader);
            npcs.push(npc);
        }

        if npcs.len() > 0 {
            self.npcs = Some(npcs);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity({
            let mut size = 1;
            if let Some(characters) = &self.characters {
                size += 1 + characters.len() * CHARACTER_MAP_INFO_SIZE;
            }
            if let Some(npcs) = &self.npcs {
                size += npcs.len() * NPC_MAP_INFO_SIZE;
            }
            size
        });
        builder.add_char({
            if let Some(characters) = &self.characters {
                characters.len() as EOChar
            } else if let Some(npcs) = &self.npcs {
                npcs.len() as EOChar
            } else {
                0
            }
        });
        if let Some(characters) = &self.characters {
            builder.add_byte(EO_BREAK_CHAR);
            for character in characters {
                builder.append(&mut character.serialize());
            }
        }
        if let Some(npcs) = &self.npcs {
            for npc in npcs {
                builder.append(&mut npc.serialize());
            }
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Reply;
    use crate::data::{EOByte, Serializeable, StreamReader};

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
        assert!(reply.characters.is_some());
        assert!(reply.npcs.is_none());

        let buf: Vec<EOByte> = vec![2, 50, 2, 254, 41, 29, 1];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert!(reply.characters.is_none());
        assert!(reply.npcs.is_some());

        let buf: Vec<EOByte> = vec![
            2, 255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255, 50, 2, 254, 41, 29, 1,
        ];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert!(reply.characters.is_some());
        assert!(reply.npcs.is_some());
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

        let buf: Vec<EOByte> = vec![2, 50, 2, 254, 41, 29, 1];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert_eq!(reply.serialize(), buf);

        let buf: Vec<EOByte> = vec![
            2, 255, 112, 108, 97, 121, 101, 114, 255, 96, 2, 23, 254, 11, 254, 16, 254, 2, 2, 32,
            32, 32, 7, 1, 2, 1, 1, 11, 254, 11, 254, 11, 254, 11, 254, 1, 254, 1, 254, 1, 254, 1,
            254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 254, 1, 1, 255, 50, 2, 254, 41, 29, 1,
        ];
        let reader = StreamReader::new(&buf);
        reply.deserialize(&reader);
        assert_eq!(reply.serialize(), buf);
    }
}
