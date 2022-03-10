use crate::data::{
    EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR,
};

pub const REQUEST_SIZE: usize = 1;

#[derive(Debug, Default)]
pub struct Request {
    pub player_id: Option<EOShort>,
    pub npc_index: Option<EOChar>,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn player(player_id: EOShort) -> Self {
        Self {
            player_id: Some(player_id),
            ..Self::default()
        }
    }
    pub fn npc(npc_index: EOChar) -> Self {
        Self {
            npc_index: Some(npc_index),
            ..Self::default()
        }
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        if reader.peek_byte() == EO_BREAK_CHAR {
            reader.seek(1);
            self.npc_index = Some(reader.get_char());
            self.player_id = None;
        } else {
            self.player_id = Some(reader.get_short());
            self.npc_index = None;
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let data_size: usize = if self.player_id.is_some() { 2 } else { 1 };

        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE + data_size);
        if self.player_id.is_some() {
            builder.add_short(self.player_id.unwrap());
            builder.add_byte(EO_BREAK_CHAR);
        } else {
            builder.add_byte(EO_BREAK_CHAR);
            builder.add_char(self.npc_index.unwrap());
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 254, 255];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_id, Some(1));
        assert_eq!(packet.npc_index, None);

        let data: Vec<EOByte> = vec![255, 2];
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.player_id, None);
        assert_eq!(packet.npc_index, Some(1));
    }
    #[test]
    fn serialize() {
        let packet = Request::player(1);
        assert_eq!(packet.serialize(), [2, 254, 255]);
        let packet = Request::npc(1);
        assert_eq!(packet.serialize(), [255, 2]);
    }
}
