#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::shop::ESF_TRADE_DATA_SIZE, EOByte, EOChar, EOShort, Serializeable, StreamBuilder,
    StreamReader,
};

/// data structure of an item a shop can craft
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShopCraftRecord {
    /// links to item id of the item to be crafted
    pub item_id: EOShort,
    /// ingredient 1 item id
    pub ingredient1_id: EOShort,
    /// ingredient 1 amount of items needed
    pub ingredient1_amount: EOChar,
    /// ingredient 2 item id
    pub ingredient2_id: EOShort,
    /// ingredient 2 amount of items needed
    pub ingredient2_amount: EOChar,
    /// ingredient 3 item id
    pub ingredient3_id: EOShort,
    /// ingredient 3 amount of items needed
    pub ingredient3_amount: EOChar,
    /// ingredient 4 item id
    pub ingredient4_id: EOShort,
    /// ingredient 4 amount of items needed
    pub ingredient4_amount: EOChar,
}

impl ShopCraftRecord {
    /// creates a new ShopCraftRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShopCraftRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.item_id = reader.get_short();
        self.ingredient1_id = reader.get_short();
        self.ingredient1_amount = reader.get_char();
        self.ingredient2_id = reader.get_short();
        self.ingredient2_amount = reader.get_char();
        self.ingredient3_id = reader.get_short();
        self.ingredient3_amount = reader.get_char();
        self.ingredient4_id = reader.get_short();
        self.ingredient4_amount = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ESF_TRADE_DATA_SIZE);
        builder.add_short(self.item_id);
        builder.add_short(self.ingredient1_id);
        builder.add_char(self.ingredient1_amount);
        builder.add_short(self.ingredient2_id);
        builder.add_char(self.ingredient2_amount);
        builder.add_short(self.ingredient3_id);
        builder.add_char(self.ingredient3_amount);
        builder.add_short(self.ingredient4_id);
        builder.add_char(self.ingredient4_amount);
        builder.get()
    }
}
