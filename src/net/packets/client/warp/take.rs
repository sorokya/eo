use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

const ACCEPT_SIZE: usize = 4;

#[derive(Debug, Default)]
pub struct Take {
    pub map_id: EOShort,
    pub warp_id: EOShort,
}

impl Take {
    pub fn new(warp_id: EOShort, map_id: EOShort) -> Self {
        Self { map_id, warp_id }
    }
}

impl Serializeable for Take {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.map_id = reader.get_short();
        self.warp_id = reader.get_short();
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ACCEPT_SIZE);
        builder.add_short(self.map_id);
        builder.add_short(self.warp_id);
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
        let mut take = Take::default();
        take.deserialize(&reader);
        assert_eq!(take.map_id, 14);
        assert_eq!(take.warp_id, 47134);
    }

    #[test]
    fn serialize() {
        let take = Take::new(14, 47134);
        assert_eq!(take.serialize(), [15, 254, 77, 187]);
    }
}
