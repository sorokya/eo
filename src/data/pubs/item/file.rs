use std::io::{prelude::*, SeekFrom};

use crate::data::{pubs::item::*, pubs::*, *};

/// represents an EO item file
///
/// The item file contains a list of every item in the game world.
/// It is saved on the server and send to clients on login.
///
/// It contains a revision number. If the server revision differs
/// from the client's revision number the file is re-downloaded.
#[derive(Debug, Default)]
pub struct ItemFile {
    pub revision: EOInt,
    length: usize,
    pub records: Vec<ItemRecord>,
}

impl ItemFile {
    /// creates a new ItemFile with no records
    pub fn new() -> Self {
        Self {
            revision: 0,
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
    ///
    ///
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
        let name = self.get_record_name(buf)?;
        if name != "eof" {
            let mut data_buf: Vec<EOByte> = vec![0; EIF_DATA_SIZE];
            buf.read_exact(&mut data_buf)?;
            let mut record = ItemRecord::new(id as EOInt, &name);
            record.deserialize(&data_buf);
            self.records.push(record);
        }

        Ok(())
    }

    fn get_record_name<R: Read + Seek>(&self, buf: &mut R) -> std::io::Result<String> {
        let mut size_of_name_bytes: Vec<EOByte> = vec![0; 1];
        buf.read_exact(&mut size_of_name_bytes)?;

        let size_of_name = decode_number(&size_of_name_bytes) as usize;
        let mut name_bytes: Vec<EOByte> = vec![0; size_of_name];
        buf.read_exact(&mut name_bytes)?;

        let name =
            String::from_utf8(name_bytes.to_vec()).expect("Failed to convert byte array to string");

        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::encode_number;
    use std::io::Cursor;

    #[test]
    fn read_valid_eif() -> std::io::Result<()> {
        let mut eif = ItemFile::new();
        let mut records: Vec<ItemRecord> = Vec::new();
        records.push(ItemRecord::new(1, "Gold"));
        records.push(ItemRecord::new(1, "eof"));
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
