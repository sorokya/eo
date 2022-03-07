use crate::{
    data::{EOByte, Serializeable, StreamBuilder, StreamReader},
};

#[derive(Debug, Default)]
pub struct ReplyFileMap {
    pub data: Vec<EOByte>,
}

impl ReplyFileMap {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ReplyFileMap {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.data = reader.get_vec(reader.remaining());
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(self.data.len() + 1);
        builder.append(&mut self.data.clone()); // <- euuw
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, ReplyFileMap, Serializeable, StreamReader};

    #[test]
    fn deserialize() {
        let map_bytes: Vec<EOByte> = vec![
            0x45, 0x4D, 0x46, 0x81, 0x69, 0xDD, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x59, 0x2C, 0x68, 0x4B, 0x01, 0x01, 0x01, 0x01, 0x01, 0xFE, 0x03, 0x03, 0x01, 0xFE,
            0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x04, 0x02,
            0x97, 0xFE, 0x01, 0x96, 0xFE, 0x03, 0x96, 0xFE, 0x03, 0x04, 0x02, 0x96, 0xFE, 0x01,
            0x96, 0xFE, 0x03, 0x96, 0xFE, 0x01, 0x04, 0x01, 0x96, 0xFE, 0x02, 0x96, 0xFE, 0x03,
            0x96, 0xFE, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
        ];

        let mut init_file_map = ReplyFileMap::new();
        let reader = StreamReader::new(&map_bytes);
        init_file_map.deserialize(&reader);
        assert_eq!(init_file_map.data, map_bytes[1..].to_vec());
    }

    #[test]
    fn serialize() {
        let map_bytes = vec![
            0x45, 0x4D, 0x46, 0x81, 0x69, 0xDD, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x59,
            0x2C, 0x68, 0x4B, 0x01, 0x01, 0x01, 0x01, 0x01, 0xFE, 0x03, 0x03, 0x01, 0xFE, 0x02,
            0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x04, 0x02, 0x97,
            0xFE, 0x01, 0x96, 0xFE, 0x03, 0x96, 0xFE, 0x03, 0x04, 0x02, 0x96, 0xFE, 0x01, 0x96,
            0xFE, 0x03, 0x96, 0xFE, 0x01, 0x04, 0x01, 0x96, 0xFE, 0x02, 0x96, 0xFE, 0x03, 0x96,
            0xFE, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
        ];

        let mut init_file_map = ReplyFileMap::new();
        init_file_map.data = map_bytes;
        assert_eq!(
            init_file_map.serialize(),
            vec![
                0x45, 0x4D, 0x46, 0x81, 0x69, 0xDD, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0x59, 0x2C, 0x68, 0x4B, 0x01, 0x01, 0x01, 0x01, 0x01, 0xFE, 0x03, 0x03, 0x01, 0xFE,
                0x02, 0x02, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x04, 0x02, 0x04, 0x02,
                0x97, 0xFE, 0x01, 0x96, 0xFE, 0x03, 0x96, 0xFE, 0x03, 0x04, 0x02, 0x96, 0xFE, 0x01,
                0x96, 0xFE, 0x03, 0x96, 0xFE, 0x01, 0x04, 0x01, 0x96, 0xFE, 0x02, 0x96, 0xFE, 0x03,
                0x96, 0xFE, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
            ]
        );
    }
}
