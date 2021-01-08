#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::WARP_SIZE;
use crate::data::{EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Warp {
    pub x: EOChar,
    pub warp_map: EOShort,
    pub warp_x: EOChar,
    pub warp_y: EOChar,
    pub level_req: EOChar,
    pub door: bool,
}

impl Warp {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for Warp {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.x = reader.get_char();
        self.warp_map = reader.get_short();
        self.warp_x = reader.get_char();
        self.warp_y = reader.get_char();
        self.level_req = reader.get_char();
        self.door = reader.get_short() == 1;
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(WARP_SIZE);
        builder.add_char(self.x);
        builder.add_short(self.warp_map);
        builder.add_char(self.warp_x);
        builder.add_char(self.warp_y);
        builder.add_char(self.level_req);
        builder.add_short(self.door as EOShort);
        builder.get()
    }
}
