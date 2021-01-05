use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::{PubRecord, ShopRecord},
    {EOByte, EOShort, StreamReader},
};

/// represents esf (shop) files
///
/// The shop file contains a list of vendor NPCs and items they trade/craft
/// See [super::ShopRecord], [super::ShopTradeRecord] and
/// [super::ShopCraftRecord] for details on the data in each record
///
/// The file layout is:
///```text
/// "ESF" (fixed string)
/// Record*
/// {
///     id (2 bytes)
///     name (prefixed string)
///     Unknown (3 bytes)
///     trades_length (2 bytes)
///     crafts_length (1 byte)
///     ShopTradeRecord*trades_length
///     {
///         item_id (2 bytes)
///         buy_price (3 bytes)
///         sell_price (3 bytes)
///         max_amount (1 byte)
///     }
///     ShopCraftRecord*crafts_length
///     {
///         item_id (2 bytes)
///         ingredient1_id (2 bytes)
///         ingredient1_amount (1 byte)
///         ingredient2_id (2 bytes)
///         ingredient2_amount (1 byte)
///         ingredient3_id (2 bytes)
///         ingredient3_amount (1 byte)
///         ingredient4_id (2 bytes)
///         ingredient4_amount (1 byte)
///     }
/// }
///```
#[derive(Debug, Default)]
pub struct ShopFile {
    pub records: Vec<ShopRecord>,
}

impl ShopFile {
    /// creates a new ShopFile with no records
    pub fn new() -> Self {
        Self {
            records: Vec::default(),
        }
    }
    /// returns the number of records in the ShopFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the ShopFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the ShopFile
    pub fn read<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut data_buf: Vec<EOByte> = Vec::new();
        buf.seek(SeekFrom::Start(0))?;
        buf.read_to_end(&mut data_buf)?;
        let mut reader = StreamReader::new(&data_buf);
        reader.seek(3);
        self.records = Vec::new();
        while !reader.eof() {
            self.read_record(&mut reader)?;
        }

        Ok(())
    }

    fn read_record(&mut self, reader: &mut StreamReader) -> std::io::Result<()> {
        let mut record = ShopRecord::new();
        record.deserialize(reader);
        self.records.push(record);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{prelude::Write, Cursor};

    use super::{EOByte, EOShort, PubRecord, ShopFile, ShopRecord};

    use crate::data::{
        pubs::{ShopCraftRecord, ShopTradeRecord},
        EOChar,
    };

    #[test]
    fn read_valid_esf() -> std::io::Result<()> {
        let mut esf = ShopFile::new();

        let mut records: Vec<ShopRecord> = Vec::with_capacity(1);

        {
            let mut record = ShopRecord::new();
            record.vendor_id = 1;

            record.trades = Vec::with_capacity(1);
            let mut trade = ShopTradeRecord::new();
            trade.item_id = 50;
            trade.buy_price = 100;
            trade.max_amount = 4;
            record.trades.push(trade);
            record.trades_length = record.trades.len() as EOShort;

            record.crafts = Vec::with_capacity(1);
            let mut craft = ShopCraftRecord::new();
            craft.item_id = 123;
            craft.ingredient1_id = 10;
            craft.ingredient1_amount = 100;
            craft.ingredient2_id = 16;
            craft.ingredient2_amount = 25;
            record.crafts.push(craft);
            record.crafts_length = record.crafts.len() as EOChar;

            records.push(record);
        }

        let mut buf = build_fake_esf(records).unwrap();
        esf.read(&mut buf)?;

        assert_eq!(esf.records.len(), 1);
        assert_eq!(esf.records[0].vendor_id, 1);
        assert_eq!(esf.records[0].trades_length, 1);
        assert_eq!(esf.records[0].trades[0].item_id, 50);
        assert_eq!(esf.records[0].trades[0].buy_price, 100);
        assert_eq!(esf.records[0].trades[0].sell_price, 0);
        assert_eq!(esf.records[0].trades[0].max_amount, 4);
        assert_eq!(esf.records[0].crafts_length, 1);
        assert_eq!(esf.records[0].crafts[0].item_id, 123);
        assert_eq!(esf.records[0].crafts[0].ingredient1_id, 10);
        assert_eq!(esf.records[0].crafts[0].ingredient1_amount, 100);
        assert_eq!(esf.records[0].crafts[0].ingredient2_id, 16);
        assert_eq!(esf.records[0].crafts[0].ingredient2_amount, 25);
        assert_eq!(esf.records[0].crafts[0].ingredient3_id, 0);
        assert_eq!(esf.records[0].crafts[0].ingredient3_amount, 0);
        assert_eq!(esf.records[0].crafts[0].ingredient4_id, 0);
        assert_eq!(esf.records[0].crafts[0].ingredient4_amount, 0);

        Ok(())
    }

    fn build_fake_esf(records: Vec<ShopRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"ESF")?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
