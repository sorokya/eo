use crate::data::{EOByte, EOShort, Serializeable, StreamBuilder, StreamReader};

pub const SPELL_SIZE: usize = 4;
#[derive(Debug, Clone, Default)]
pub struct Spell {
    pub id: EOShort,
    pub level: EOShort,
}

impl Spell {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Spell {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.id = reader.get_short();
        self.level = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(SPELL_SIZE);
        builder.add_short(self.id);
        builder.add_short(self.level);
        builder.get()
    }
}

// TODO: tests
