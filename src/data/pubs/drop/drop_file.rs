#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::{DropNPCRecord, PubRecord},
    {EOByte, EOShort, StreamReader},
};

/// represents edf files
///
/// The drop file contains a list of NPCs and items they can drop when killed
/// See [DropNPCRecord] and [DropRecord] for details on the data in each record
///
/// The file layout is:
///```text
/// "EDF" (fixed string)
/// Record*
/// {
///     id (2 bytes)
///     length (2 bytes)
///     DropRecord*length
///     {
///         item_id (2 bytes)
///         min_amount (3 bytes)
///         max_amount (3 bytes)
///         drop_rate (2 bytes)
///     }
/// }
///```
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DropFile {
    pub records: Vec<DropNPCRecord>,
}

impl DropFile {
    /// creates a new DropFile with no records
    pub fn new() -> Self {
        Self {
            records: Vec::default(),
        }
    }
    /// returns the number of records in the DropFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the DropFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the DropFile
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
        let mut record = DropNPCRecord::new();
        record.deserialize(reader);
        self.records.push(record);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{prelude::Write, Cursor};

    use super::{DropFile, DropNPCRecord, EOByte, EOShort, PubRecord};
    use crate::data::pubs::DropRecord;

    #[test]
    fn read_valid_edf() -> std::io::Result<()> {
        let mut edf = DropFile::new();
        let mut records: Vec<DropNPCRecord> = Vec::new();

        {
            let mut record = DropNPCRecord::new();
            record.npc_id = 1;
            record.drops = Vec::with_capacity(1);

            let mut drop_record = DropRecord::new();
            drop_record.item_id = 1;
            drop_record.min_amount = 1;
            drop_record.max_amount = 15;
            drop_record.drop_rate = 50;
            record.drops.push(drop_record);
            record.length = record.drops.len() as EOShort;
            records.push(record);
        }

        let mut buf = build_fake_edf(records).unwrap();
        edf.read(&mut buf)?;

        assert_eq!(edf.records.len(), 1);
        assert_eq!(edf.records[0].npc_id, 1);
        assert_eq!(edf.records[0].drops.len(), 1);
        assert_eq!(edf.records[0].drops[0].item_id, 1);
        assert_eq!(edf.records[0].drops[0].min_amount, 1);
        assert_eq!(edf.records[0].drops[0].max_amount, 15);
        assert_eq!(edf.records[0].drops[0].drop_rate, 50);

        Ok(())
    }

    fn build_fake_edf(records: Vec<DropNPCRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"EDF")?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
