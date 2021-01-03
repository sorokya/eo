use std::io::{prelude::*, SeekFrom};

use crate::data::{pubs::class::*, pubs::*, *};

/// represents an EO class file
///
/// The class file contains a list of every class in the game world.
/// It is saved on the server and send to clients on login.
///
/// It contains a revision number. If the server revision differs
/// from the client's revision number the file is re-downloaded.
#[derive(Debug, Default)]
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
        buf.seek(SeekFrom::Start(3))?;
        self.read_revision(buf)?;
        self.read_length(buf)?;
        buf.seek(SeekFrom::Current(1))?;
        self.records = Vec::with_capacity(self.length);
        for id in 1..self.length {
            self.read_record(id, buf)?;
        }

        Ok(())
    }

    fn read_revision<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut revision_bytes: Vec<EOByte> = vec![0; 4];
        buf.read_exact(&mut revision_bytes)?;
        self.revision = decode_number(revision_bytes.as_slice());
        Ok(())
    }

    fn read_length<R: Read + Seek>(&mut self, buf: &mut R) -> std::io::Result<()> {
        let mut length_bytes: Vec<EOByte> = vec![0; 2];
        buf.read_exact(&mut length_bytes)?;
        self.length = decode_number(length_bytes.as_slice()) as usize;
        Ok(())
    }

    fn read_record<R: Read + Seek>(&mut self, id: usize, buf: &mut R) -> std::io::Result<()> {
        let name_length = self.get_record_name_length(buf)?;
        let mut data_buf: Vec<EOByte> = vec![0; ECF_DATA_SIZE + name_length + 1];
        buf.read_exact(&mut data_buf)?;

        let mut record = ClassRecord::new(id as EOInt);
        record.deserialize(&data_buf);
        if record.name != "eof" {
            self.records.push(record);
        }

        Ok(())
    }

    fn get_record_name_length<R: Read + Seek>(&self, buf: &mut R) -> std::io::Result<usize> {
        let mut size_of_name_bytes: Vec<EOByte> = vec![0; 1];
        buf.read_exact(&mut size_of_name_bytes)?;
        buf.seek(SeekFrom::Current(-1))?;

        Ok(decode_number(&size_of_name_bytes) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::encode_number;
    use std::io::Cursor;

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
            record.class_type = ClassType::Melee;
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
        assert_eq!(ecf.records[1].class_type, ClassType::Melee);
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
