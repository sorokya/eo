#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::class::ClassRecord,
    {EOByte, EOInt, EOShort, Serializeable, StreamReader},
};

/// represents ecf files
///
/// The class file contains a list of every player class
/// in the game. See [ClassRecord] for details on the data
/// in each record.
///
/// The file layout is:
///```text
/// "ECF" (fixed string)
/// RID (4 bytes)
/// Length (2 bytes)
/// Record*Length
/// {
///     name (prefixed string)
///     base (1 byte)
///     class_type (1 byte)
///     strength (2 bytes)
///     intelligence (2 bytes)
///     wisdom (2 bytes)
///     agility (2 bytes)
///     constitution (2 bytes)
///     charisma (2 bytes)
/// }
///```
/// RID is the files "revision" number. It's used to signal the client
/// that a new version is available and needs to be downloaded.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassFile {
    pub revision: EOInt,
    length: usize,
    pub records: Vec<ClassRecord>,
}

impl ClassFile {
    /// creates a new ClassFile with no records
    pub fn new() -> Self {
        Self {
            revision: 0,
            length: 0,
            records: Vec::default(),
        }
    }
    /// returns the number of records in the ClassFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the ClassFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the ClassFile
    pub fn read<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut data_buf: Vec<EOByte> = Vec::new();
        buf.seek(SeekFrom::Start(0))?;
        buf.read_to_end(&mut data_buf)?;
        let mut reader = StreamReader::new(&data_buf);
        reader.seek(3);
        self.revision = reader.get_int();
        self.length = reader.get_short() as usize;
        reader.get_char();
        self.records = Vec::with_capacity(self.length);
        for id in 1..self.length {
            self.read_record(id, &mut reader)?;
        }

        Ok(())
    }

    fn read_record(&mut self, id: usize, reader: &mut StreamReader) -> std::io::Result<()> {
        let mut record = ClassRecord::new(id as EOInt);
        record.deserialize(reader);
        if record.name != "eof" {
            self.records.push(record);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ClassFile, ClassRecord, EOByte, EOInt, Serializeable};
    use crate::data::encode_number;
    use std::io::{prelude::Write, Cursor};

    #[test]
    fn read_valid_ecf() -> std::io::Result<()> {
        let mut ecf = ClassFile::new();
        let mut records: Vec<ClassRecord> = Vec::new();

        {
            let mut record = ClassRecord::new(1);
            record.name = "Peasant".to_string();
            records.push(record);
        }

        {
            let mut record = ClassRecord::new(2);
            record.name = "Warrior".to_string();
            record.class_type = 0;
            record.strength = 2;
            records.push(record);
        }

        {
            let mut record = ClassRecord::new(3);
            record.name = "eof".to_string();
            records.push(record);
        }

        let mut buf = build_fake_ecf(19283, records).unwrap();
        ecf.read(&mut buf)?;

        assert_eq!(ecf.records[0].name, "Peasant");
        assert_eq!(ecf.records[1].name, "Warrior");
        assert_eq!(ecf.records[1].class_type, 0);
        assert_eq!(ecf.records[1].strength, 2);
        assert_eq!(ecf.records.len(), 2);

        Ok(())
    }

    fn build_fake_ecf(
        rid: EOInt,
        records: Vec<ClassRecord>,
    ) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"ECF")?;
        buf.write_all(&encode_number(rid))?;
        buf.write_all(&encode_number(records.len() as EOInt)[0..2])?;
        buf.write_all(&encode_number(1)[0..1])?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
