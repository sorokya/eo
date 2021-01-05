#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::inn::InnRecord,
    pubs::PubRecord,
    {EOByte, EOShort, StreamReader},
};

/// represents eid files
///
/// The inn file contains a list of homes a player can be
/// registered to in the game. See [InnRecord] for details
/// on the data in each record.
///
/// The file layout is:
///```text
/// "EID" (fixed string)
/// Record*
/// {
///     id (2 bytes)
///     name (prefixed string)
///     spawn_map (2 bytes)
///     spawn_x (1 byte)
///     spawn_y (1 byte)
///     inn_sleep_map (2 bytes)
///     inn_sleep_x (1 byte)
///     inn_sleep_y (1 byte)
///     alt_spawn_enabled (1 byte)
///     alt_spawn_map (2 bytes)
///     alt_spawn_x (1 byte)
///     alt_spawn_y (1 byte)
///     question1 (prefixed string)
///     answer1 (prefixed string)
///     question2 (prefixed string)
///     answer2 (prefixed string)
///     question3 (prefixed string)
///     answer3 (prefixed string)
/// }
///```
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InnFile {
    pub records: Vec<InnRecord>,
}

impl InnFile {
    /// creates a new InnFile with no records
    pub fn new() -> Self {
        Self {
            records: Vec::default(),
        }
    }
    /// returns the number of records in the InnFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the InnFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the InnFile
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
        let mut record = InnRecord::new();
        record.deserialize(reader);
        self.records.push(record);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, InnFile, InnRecord, PubRecord};
    use std::io::{prelude::Write, Cursor};

    #[test]
    fn read_valid_eid() -> std::io::Result<()> {
        let mut eid = InnFile::new();
        let mut records: Vec<InnRecord> = Vec::new();

        {
            let mut record = InnRecord::new();
            record.name = "Wanderer".to_string();
            record.spawn_map = 173;
            record.spawn_x = 8;
            record.spawn_y = 8;
            records.push(record);
        }

        {
            let mut record = InnRecord::new();
            record.name = "Aeven".to_string();
            record.spawn_map = 5;
            record.spawn_x = 50;
            record.spawn_y = 50;
            record.question1 = "What's 2+2?".to_string();
            record.answer1 = "4".to_string();
            record.question2 = "Are you sure?".to_string();
            record.answer2 = "yes".to_string();
            record.question3 = "Say 'I love aeven'".to_string();
            record.answer3 = "i love aeven".to_string();
            records.push(record);
        }

        let mut buf = build_fake_eid(records).unwrap();
        eid.read(&mut buf)?;

        assert_eq!(eid.records.len(), 2);
        assert_eq!(eid.records[0].name, "Wanderer");
        assert_eq!(eid.records[0].spawn_map, 173);
        assert_eq!(eid.records[0].spawn_x, 8);
        assert_eq!(eid.records[0].spawn_y, 8);
        assert_eq!(eid.records[1].name, "Aeven");
        assert_eq!(eid.records[1].spawn_map, 5);
        assert_eq!(eid.records[1].spawn_x, 50);
        assert_eq!(eid.records[1].spawn_y, 50);
        assert_eq!(eid.records[1].question1, "What's 2+2?");
        assert_eq!(eid.records[1].answer1, "4");
        assert_eq!(eid.records[1].question2, "Are you sure?");
        assert_eq!(eid.records[1].answer2, "yes");
        assert_eq!(eid.records[1].question3, "Say 'I love aeven'");
        assert_eq!(eid.records[1].answer3, "i love aeven");

        Ok(())
    }

    fn build_fake_eid(records: Vec<InnRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"EID")?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
