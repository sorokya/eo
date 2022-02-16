use crate::{
    data::{EOByte, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader},
    world::{Coords, COORDS_SIZE},
};

pub const ITEM_MAP_INFO_SIZE: usize = COORDS_SIZE + 7;
#[derive(Debug, Default)]
pub struct ItemMapInfo {
    pub uid: EOShort,
    pub id: EOShort,
    pub coords: Coords,
    pub amount: EOThree,
}

impl ItemMapInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ItemMapInfo {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.uid = reader.get_short();
        self.id = reader.get_short();
        self.coords.deserialize(reader);
        self.amount = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ITEM_MAP_INFO_SIZE);
        builder.add_short(self.uid);
        builder.add_short(self.id);
        builder.append(&mut self.coords.serialize());
        builder.add_three(self.amount);
        builder.get()
    }
}

// TODO: tests
