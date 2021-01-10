#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{pubs::talk::TalkRecord, EOByte, EOShort, Serializeable, StreamReader};

/// represents etf (npc talk) files
///
/// The talk file contains a list of NPCs and the messages they can say
/// See [super::TalkRecord] for details on the data in each record
///
/// The file layout is:
///```text
/// "ETF" (fixed string)
/// Record*
/// {
///     id (2 bytes)
///     rate (1 byte)
///     messages_length (1 byte)
///     String*trades_length
///     {
///         message (prefixed string)
///     }
/// }
///```
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TalkFile {
    pub records: Vec<TalkRecord>,
}

impl TalkFile {
    /// creates a new TalkFile with no records
    pub fn new() -> Self {
        Self {
            records: Vec::default(),
        }
    }
    /// returns the number of records in the TalkFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the ShopFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the TalkFile
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
        let mut record = TalkRecord::new();
        record.deserialize(reader);
        self.records.push(record);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{prelude::Write, Cursor};

    use super::{EOByte, Serializeable, TalkFile, TalkRecord};
    use crate::data::EOChar;

    #[test]
    fn read_valid_etf() -> std::io::Result<()> {
        let mut etf = TalkFile::new();

        let mut records: Vec<TalkRecord> = Vec::with_capacity(1);
        {
            let mut record = TalkRecord::new();
            record.npc_id = 1;
            record.rate = 10;
            record.messages = Vec::with_capacity(2);
            record.messages.push("Hello".to_string());
            record.messages.push("Hi".to_string());
            record.messages_length = record.messages.len() as EOChar;
            records.push(record);
        }

        let mut buf = build_fake_etf(records).unwrap();
        etf.read(&mut buf)?;

        assert_eq!(etf.len(), 1);
        assert_eq!(etf.records[0].npc_id, 1);
        assert_eq!(etf.records[0].rate, 10);
        assert_eq!(etf.records[0].messages_length, 2);
        assert_eq!(etf.records[0].messages[0], "Hello");
        assert_eq!(etf.records[0].messages[1], "Hi");

        Ok(())
    }

    fn build_fake_etf(records: Vec<TalkRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"ETF")?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
