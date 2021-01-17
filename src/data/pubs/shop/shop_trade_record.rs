#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::shop::ESF_TRADE_DATA_SIZE, EOByte, EOChar, EOShort, EOThree, Serializeable,
    StreamBuilder, StreamReader,
};

/// data structure of an item a shop can buy or sell
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShopTradeRecord {
    /// links to item id of the item to be bought/sold
    pub item_id: EOShort,
    /// price to buy the item
    pub buy_price: EOThree,
    /// price shop will pay for item
    pub sell_price: EOThree,
    /// max amount of items player can buy per trade
    pub max_amount: EOChar,
}

impl ShopTradeRecord {
    /// creates a new ShopTradeRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShopTradeRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.item_id = reader.get_short();
        self.buy_price = reader.get_three();
        self.sell_price = reader.get_three();
        self.max_amount = reader.get_char();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ESF_TRADE_DATA_SIZE);
        builder.add_short(self.item_id);
        builder.add_three(self.buy_price);
        builder.add_three(self.sell_price);
        builder.add_char(self.max_amount);
        builder.get()
    }
}
