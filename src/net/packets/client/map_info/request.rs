use crate::data::{
    EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR,
};

#[derive(Debug, Default)]
pub struct Request {
    pub player_ids: Option<Vec<EOShort>>,
    pub npc_indexes: Option<Vec<EOChar>>,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn players(player_ids: Vec<EOShort>) -> Self {
        Self {
            player_ids: Some(player_ids),
            ..Self::default()
        }
    }
    pub fn npcs(npc_indexes: Vec<EOChar>) -> Self {
        Self {
            npc_indexes: Some(npc_indexes),
            ..Self::default()
        }
    }
    pub fn both(player_ids: Vec<EOShort>, npc_indexes: Vec<EOChar>) -> Self {
        Self {
            player_ids: Some(player_ids),
            npc_indexes: Some(npc_indexes),
        }
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        if !reader.has_break() {
            let mut player_ids = Vec::new();
            while !reader.eof() {
                player_ids.push(reader.get_short());
            }
            self.player_ids = Some(player_ids);
            self.npc_indexes = None;
        } else {
            let mut player_ids = Vec::new();
            while reader.peek_byte() != EO_BREAK_CHAR {
                player_ids.push(reader.get_short());
            }
            if player_ids.len() > 0 {
                self.player_ids = Some(player_ids);
            } else {
                self.player_ids = None;
            }
            let mut npc_indexes = Vec::new();
            reader.seek(1);
            while !reader.eof() {
                npc_indexes.push(reader.get_char());
            }
            if npc_indexes.len() > 0 {
                self.npc_indexes = Some(npc_indexes);
            }
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let data_size: usize = {
            let mut size = 0;

            if let Some(player_ids) = &self.player_ids {
                size += player_ids.len() * 2;
            }

            if let Some(npc_indexes) = &self.npc_indexes {
                size += 1 + npc_indexes.len();
            }

            size
        };

        let mut builder = StreamBuilder::with_capacity(data_size);
        if let Some(player_ids) = &self.player_ids {
            for player_id in player_ids {
                builder.add_short(*player_id);
            }
        }
        if let Some(npc_indexes) = &self.npc_indexes {
            builder.add_byte(EO_BREAK_CHAR);
            for npc_index in npc_indexes {
                builder.add_char(*npc_index);
            }
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_ids.as_ref().unwrap().len(), 1);
        assert_eq!(packet.player_ids.as_ref().unwrap()[0], 1);
        assert_eq!(packet.npc_indexes, None);

        let data: Vec<EOByte> = vec![255, 2];
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_ids, None);
        assert_eq!(packet.npc_indexes.as_ref().unwrap().len(), 1);
        assert_eq!(packet.npc_indexes.as_ref().unwrap()[0], 1);

        let data: Vec<EOByte> = vec![2, 254, 255, 2];
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_ids.as_ref().unwrap().len(), 1);
        assert_eq!(packet.player_ids.as_ref().unwrap()[0], 1);
        assert_eq!(packet.npc_indexes.as_ref().unwrap().len(), 1);
        assert_eq!(packet.npc_indexes.as_ref().unwrap()[0], 1);
    }
    #[test]
    fn serialize() {
        let packet = Request::players(vec![1]);
        assert_eq!(packet.serialize(), [2, 254]);
        let packet = Request::npcs(vec![1]);
        assert_eq!(packet.serialize(), [255, 2]);
        let packet = Request::both(vec![1], vec![1]);
        assert_eq!(packet.serialize(), [2, 254, 255, 2]);
    }
}
