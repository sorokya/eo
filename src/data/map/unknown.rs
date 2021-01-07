#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::UNKNOWN_SIZE;
use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Unknown {
    pub unknown1: EOChar,
    pub unknown2: EOChar,
    pub unknown3: EOChar,
    pub unknown4: EOChar,
}

impl Unknown {
    /// creates a new Unknown with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Unknown {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.unknown1 = reader.get_char();
        self.unknown2 = reader.get_char();
        self.unknown3 = reader.get_char();
        self.unknown4 = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(UNKNOWN_SIZE);
        builder.add_char(self.unknown1);
        builder.add_char(self.unknown2);
        builder.add_char(self.unknown3);
        builder.add_char(self.unknown4);
        builder.get()
    }
}
