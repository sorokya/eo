use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader, EO_BREAK_CHAR};

pub const REQUEST_SIZE: usize = 3;

#[derive(Debug, Default)]
pub struct Request {
    pub unknown: EOChar,
    pub npc_index: EOChar,
}

impl Request {
    pub fn new(npc_index: EOChar) -> Self {
        Self {
            npc_index,
            unknown: 1,
        }
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.unknown = reader.get_char();
        reader.seek(1);
        self.npc_index = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(REQUEST_SIZE);
        builder.add_char(self.unknown);
        builder.add_byte(EO_BREAK_CHAR);
        builder.add_char(self.npc_index);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Request, EOByte, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![2, 255, 2];

        let mut packet = Request::default();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.unknown, 1);
        assert_eq!(packet.npc_index, 1);
    }
    #[test]
    fn serialize() {
        let packet = Request::new(1);
        assert_eq!(packet.serialize(), [2, 255, 2]);
    }
}
