#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{pubs::item::ItemRecord, EOByte, EOInt, EOShort, Serializeable, StreamReader};

/// represents eif files
///
/// The item file contains a list of every item
/// in the game. See [ItemRecord] for details on the data
/// in each record.
///
/// The file layout is:
///```text
/// "EIF" (fixed string)
/// hash (4 bytes)
/// Length (2 bytes)
/// Record*Length
/// {
///     name (prefixed string)
///     graphic (2 bytes)
///     type (1 byte)
///     sub_type (1 byte)
///     special (1 byte)
///     hp (2 bytes)
///     tp (2 bytes)
///     min_damage (2 bytes)
///     max_damage (2 bytes)
///     accuracy (2 bytes)
///     evade (2 bytes)
///     armor (2 bytes)
///     unknown (1 byte)
///     intelligence (1 byte)
///     wisdom (1 byte)
///     agility (1 byte)
///     constitution (1 byte)
///     charisma (1 byte)
///     light (1 byte) - unused
///     dark (1 byte) - unused
///     earth (1 byte) - unused
///     air (1 byte) - unused
///     water (1 byte) - unused
///     fire (1 byte) - unused
///     item_specific_param_1 (3 bytes)
///     item_specific_param_2 (1 byte)
///     item_specific_param_3 (1 byte)
///     level_req (2 bytes)
///     class_req (2 bytes)
///     strength_req (2 bytes)
///     intelligence_req (2 bytes)
///     wisdom_req (2 bytes)
///     agility_req (2 bytes)
///     constitution_req (2 bytes)
///     charisma_req (2 bytes)
///     element (1 byte)
///     element_power (1 byte)
///     weight (1 byte)
///     unknown (1 byte)
///     item_size (1 byte)
/// }
///```
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ItemFile {
    pub hash: EOInt,
    length: usize,
    pub records: Vec<ItemRecord>,
}

impl ItemFile {
    /// creates a new ItemFile with no records
    pub fn new() -> Self {
        Self {
            hash: 0,
            length: 0,
            records: Vec::default(),
        }
    }
    /// returns the number of records in the ItemFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the ItemFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the ItemFile
    pub fn read<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut data_buf: Vec<EOByte> = Vec::new();
        buf.seek(SeekFrom::Start(0))?;
        buf.read_to_end(&mut data_buf)?;
        let reader = StreamReader::new(&data_buf);
        reader.seek(3);
        self.hash = reader.get_int();
        self.length = reader.get_short() as usize;
        reader.get_char();
        self.records = Vec::with_capacity(self.length);
        for id in 1..self.length {
            self.read_record(id, &reader)?;
        }

        Ok(())
    }

    fn read_record(&mut self, id: usize, reader: &StreamReader) -> std::io::Result<()> {
        let mut record = ItemRecord::new(id as EOInt);
        record.deserialize(reader);
        if record.name != "eof" {
            self.records.push(record);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, EOInt, ItemFile, ItemRecord, Serializeable};
    use crate::data::encode_number;
    use std::io::{prelude::Write, Cursor};

    #[test]
    fn read_valid_eif() -> std::io::Result<()> {
        let mut eif = ItemFile::new();
        let mut records: Vec<ItemRecord> = Vec::new();

        {
            let mut record = ItemRecord::new(1);
            record.name = "Gold".to_string();
            records.push(record);
        }

        {
            let mut record = ItemRecord::new(2);
            record.name = "eof".to_string();
            records.push(record);
        }

        let mut buf = build_fake_eif(19283, records).unwrap();
        eif.read(&mut buf)?;

        assert_eq!(eif.records[0].name, "Gold");
        assert_eq!(eif.records.len(), 1);

        Ok(())
    }

    fn build_fake_eif(
        rid: EOInt,
        records: Vec<ItemRecord>,
    ) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"EIF")?;
        buf.write_all(&encode_number(rid))?;
        buf.write_all(&encode_number(records.len() as EOInt)[0..2])?;
        buf.write_all(&encode_number(1)[0..1])?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
