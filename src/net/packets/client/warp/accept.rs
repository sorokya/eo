use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

const ACCEPT_SIZE: usize = 4;

#[derive(Debug, Default)]
pub struct Accept {
    pub map_id: EOShort,
    pub session_id: EOShort,
}

impl Accept {
    pub fn new(map_id: EOShort, session_id: EOShort) -> Self {
        Self { map_id, session_id }
    }
}

impl Serializeable for Accept {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.map_id = reader.get_short();
        self.session_id = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ACCEPT_SIZE);
        builder.add_short(self.map_id);
        builder.add_short(self.session_id);
        builder.get()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let buf = vec![15, 254, 77, 187];
        let reader = StreamReader::new(&buf);
        let mut accept = Accept::default();
        accept.deserialize(&reader);
        assert_eq!(accept.map_id, 14);
        assert_eq!(accept.session_id, 47134);
    }

    #[test]
    fn serialize() {
        let accept = Accept::new(14, 47134);
        assert_eq!(accept.serialize(), [15, 254, 77, 187]);
    }
}
