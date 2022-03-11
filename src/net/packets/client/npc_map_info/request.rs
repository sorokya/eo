use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR};

pub const REQUEST_SIZE: usize = 2;

#[derive(Debug, Default)]
pub struct Request {
    pub npc_indexes: Vec<EOChar>,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        let num_npcs = reader.get_char();
        reader.seek(1);
        self.npc_indexes = Vec::with_capacity(num_npcs as usize);
        while !reader.eof() {
            self.npc_indexes.push(reader.get_char());
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE + self.npc_indexes.len());
        builder.add_char(self.npc_indexes.len() as EOChar);
        builder.add_byte(EO_BREAK_CHAR);
        for npc_index in &self.npc_indexes {
            builder.add_char(*npc_index);
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 255, 2];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.npc_indexes.len(), 1);
        assert_eq!(packet.npc_indexes[0], 1);
    }
    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.npc_indexes.push(1);
        assert_eq!(packet.serialize(), [2, 255, 2]);
    }
}
