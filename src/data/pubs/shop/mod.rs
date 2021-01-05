const ESF_DATA_SIZE: usize = 8;
const ESF_TRADE_DATA_SIZE: usize = 9;
const ESF_CRAFT_DATA_SIZE: usize = 14;

mod shop_record;
pub use shop_record::ShopRecord;
mod shop_trade_record;
pub use shop_trade_record::ShopTradeRecord;
mod shop_craft_record;
pub use shop_craft_record::ShopCraftRecord;
mod shop_file;
pub use shop_file::ShopFile;
