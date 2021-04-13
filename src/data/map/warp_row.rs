#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Warp, WARP_ROW_SIZE, WARP_SIZE};
use crate::data::{EOByte, EOChar, Serializeable, StreamBuilder, StreamReader};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WarpRow {
    pub y: EOChar,
    pub tiles: Vec<Warp>,
}

impl WarpRow {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for WarpRow {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.y = reader.get_char();
        let inner_length = reader.get_char();
        self.tiles = Vec::with_capacity(inner_length as usize);
        for _ in 0..inner_length {
            let mut warp = Warp::new();
            warp.deserialize(reader);
            self.tiles.push(warp);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder =
            StreamBuilder::with_capacity(WARP_ROW_SIZE + self.tiles.len() * WARP_SIZE);
        builder.add_char(self.y);
        for tile in &self.tiles {
            builder.append(&mut Serializeable::serialize(tile));
        }
        builder.get()
    }
}
