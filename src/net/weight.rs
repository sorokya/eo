use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

pub const WEIGHT_SIZE: usize = 2;
#[derive(Debug, Default)]
pub struct Weight {
    pub current: EOChar,
    pub max: EOChar,
}

impl Weight {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Weight {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.current = reader.get_char();
        self.max = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(WEIGHT_SIZE);
        builder.add_char(self.current);
        builder.add_char(self.max);
        builder.get()
    }
}

// TODO: tests
