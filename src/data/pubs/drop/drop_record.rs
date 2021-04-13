#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::drop::EDF_DROP_DATA_SIZE, EOByte, EOShort, EOThree, Serializeable, StreamBuilder,
    StreamReader,
};

/// data structure of a single eid record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropRecord {
    /// links to item id
    pub item_id: EOShort,
    /// minimum amount of items that can drop
    pub min_amount: EOThree,
    /// maximum amount of items that can drop
    pub max_amount: EOThree,
    /// X in 64,000 chance of item being dropped
    pub drop_rate: EOShort,
}

impl DropRecord {
    /// creates a new DropRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for DropRecord {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.item_id = reader.get_short();
        self.min_amount = reader.get_three();
        self.max_amount = reader.get_three();
        self.drop_rate = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(EDF_DROP_DATA_SIZE);
        builder.add_short(self.item_id);
        builder.add_three(self.min_amount);
        builder.add_three(self.max_amount);
        builder.add_short(self.drop_rate);
        builder.get()
    }
}
