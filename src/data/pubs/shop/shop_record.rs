#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::shop::{
        ShopCraftRecord, ShopTradeRecord, ESF_CRAFT_DATA_SIZE, ESF_DATA_SIZE, ESF_TRADE_DATA_SIZE,
    },
    EOByte, EOChar, EOShort, Serializeable, StreamBuilder, StreamReader,
};

/// data structure of a single edf record
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ShopRecord {
    /// links to a vendor id
    pub vendor_id: EOShort,
    /// name of the shop
    pub name: String,
    /// number of trades the shop offers
    pub trades_length: EOShort,
    /// number of crafts the shop offers
    pub crafts_length: EOChar,
    /// trades the shop offers
    pub trades: Vec<ShopTradeRecord>,
    /// crafts the shop offers
    pub crafts: Vec<ShopCraftRecord>,
}

impl ShopRecord {
    /// creates a new ShopRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for ShopRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.vendor_id = reader.get_short();
        self.name = reader.get_prefix_string();
        reader.get_byte();
        reader.get_byte();
        reader.get_byte();
        self.trades_length = reader.get_short();
        self.crafts_length = reader.get_char();

        self.trades = Vec::with_capacity(self.trades_length as usize);
        for _ in 0..self.trades_length {
            let mut trade = ShopTradeRecord::new();
            trade.deserialize(reader);
            self.trades.push(trade);
        }

        self.crafts = Vec::with_capacity(self.crafts_length as usize);
        for _ in 0..self.crafts_length {
            let mut craft = ShopCraftRecord::new();
            craft.deserialize(reader);
            self.crafts.push(craft);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            ESF_DATA_SIZE
                + self.name.len()
                + 1
                + ESF_TRADE_DATA_SIZE * (self.trades_length as usize)
                + ESF_CRAFT_DATA_SIZE * (self.crafts_length as usize),
        );

        builder.add_short(self.vendor_id);
        builder.add_prefix_string(&self.name);
        builder.add_byte(0x01);
        builder.add_byte(0xFB);
        builder.add_byte(0x01);
        builder.add_short(self.trades_length);
        builder.add_char(self.crafts_length);

        for trade in &self.trades {
            builder.append(&mut Serializeable::serialize(trade));
        }

        for craft in &self.crafts {
            builder.append(&mut Serializeable::serialize(craft));
        }

        builder.get()
    }
}
