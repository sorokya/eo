use crate::data::{EOByte, EOInt, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Request {
    pub character_id: EOInt,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Request {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.character_id = reader.get_int();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(4);
        builder.add_int(self.character_id);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, Request, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![20, 254, 254, 254];
        let mut packet = Request::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.character_id, 19);
    }
    #[test]
    fn serialize() {
        let mut packet = Request::new();
        packet.character_id = 19;
        assert_eq!(packet.serialize(), [20, 254, 254, 254])
    }
}
