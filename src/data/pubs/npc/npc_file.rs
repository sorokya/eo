use std::io::{prelude::*, SeekFrom};

use crate::data::{pubs::npc::*, pubs::*, *};

/// represents an EO npc file
///
/// The npc file contains a list of every npc in the game world.
/// It is saved on the server and send to clients on login.
///
/// It contains a revision number. If the server revision differs
/// from the client's revision number the file is re-downloaded.
#[derive(Debug, Default)]
pub struct NPCFile {
    pub revision: EOInt,
    length: usize,
    pub records: Vec<NPCRecord>,
}

impl NPCFile {
    /// creates a new NPCFile with no records
    pub fn new() -> Self {
        Self {
            revision: 0,
            length: 0,
            records: Vec::default(),
        }
    }
    /// returns the number of records in the NPCFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the NPCFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the NPCFile
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
        let name_length = self.get_name_length(buf)?;
        let mut data_buf: Vec<EOByte> = vec![0; ENF_DATA_SIZE + name_length + 1];
        buf.read_exact(&mut data_buf)?;

        let mut record = NPCRecord::new(id as EOInt);
        record.deserialize(&data_buf);
        if record.name != "eof" {
            self.records.push(record);
        }

        Ok(())
    }

    fn get_name_length<R: Read + Seek>(&self, buf: &mut R) -> std::io::Result<usize> {
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
    fn read_valid_enf() -> std::io::Result<()> {
        let mut enf = NPCFile::new();
        let mut records: Vec<NPCRecord> = Vec::new();

        {
            let mut record = NPCRecord::new(1);
            record.name = "Goat".to_string();
            record.experience = 8;
            record.hp = 6;
            record.min_damage = 0;
            record.max_damage = 2;
            record.accuracy = 1;
            record.evade = 2;
            record.armor = 3;
            record.element_weak = 4;
            record.element_weak_power = 5;
            record.graphic_id = 6;
            record.vendor_id = 7;
            record.boss = 8;
            record.child = 9;
            record.npc_type = NPCType::Passive;
            records.push(record);
        }

        {
            let mut record = NPCRecord::new(2);
            record.name = "eof".to_string();
            records.push(record);
        }

        let mut buf = build_fake_enf(19283, records).unwrap();
        enf.read(&mut buf)?;

        assert_eq!(enf.records.len(), 1);
        assert_eq!(enf.records[0].name, "Goat");
        assert_eq!(enf.records[0].experience, 8);
        assert_eq!(enf.records[0].hp, 6);
        assert_eq!(enf.records[0].min_damage, 0);
        assert_eq!(enf.records[0].max_damage, 2);
        assert_eq!(enf.records[0].accuracy, 1);
        assert_eq!(enf.records[0].evade, 2);
        assert_eq!(enf.records[0].armor, 3);
        assert_eq!(enf.records[0].element_weak, 4);
        assert_eq!(enf.records[0].element_weak_power, 5);
        assert_eq!(enf.records[0].graphic_id, 6);
        assert_eq!(enf.records[0].vendor_id, 7);
        assert_eq!(enf.records[0].boss, 8);
        assert_eq!(enf.records[0].child, 9);
        assert_eq!(enf.records[0].npc_type, NPCType::Passive);
        Ok(())
    }

    fn build_fake_enf(rid: EOInt, records: Vec<NPCRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"ENF")?;
        buf.write_all(&encode_number(rid))?;
        buf.write_all(&encode_number(records.len() as EOInt)[0..2])?;
        buf.write_all(&encode_number(1)[0..1])?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
