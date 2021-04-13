use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const CONNECTION_ACCEPT_SIZE: usize = 7;

#[derive(Debug, Default)]
pub struct Accept {
    pub encoding_multiples: [EOByte; 2],
    pub player_id: EOShort,
}

impl Accept {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Accept {
    fn deserialize(&mut self, reader: &StreamReader) {
        for byte in self.encoding_multiples.iter_mut() {
            *byte = reader.get_short() as EOByte;
        }
        self.player_id = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(CONNECTION_ACCEPT_SIZE);
        for byte in self.encoding_multiples.iter() {
            builder.add_short(*byte as EOShort);
        }
        builder.add_short(self.player_id);
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{Accept, EOByte, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let data: Vec<EOByte> = vec![11, 254, 6, 254, 2, 254];

        let mut packet = Accept::new();
        let reader = StreamReader::new(&data);
        packet.deserialize(&reader);
        assert_eq!(packet.encoding_multiples, [10, 5]);
        assert_eq!(packet.player_id, 1);
    }
    #[test]
    fn serialize() {
        let mut packet = Accept::new();
        packet.encoding_multiples = [10, 5];
        packet.player_id = 1;
        assert_eq!(packet.serialize(), [11, 254, 6, 254, 2, 254]);
    }
}
