use crate::data::{EOByte, EOInt, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Default)]
pub struct Take {
    pub character_id: EOInt,
}

impl Take {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Take {
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
    use super::*;
    use crate::data::{EOByte, StreamReader};

    #[test]
    fn serialize() {
        let mut packet = Take::new();
        packet.character_id = 16;
        assert_eq!(packet.serialize(), [17, 254, 254, 254])
    }

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![17, 254, 254, 254];
        let mut packet = Take::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.character_id, 16);
    }
}
