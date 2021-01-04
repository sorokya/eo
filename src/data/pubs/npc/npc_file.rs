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
        let mut record = NPCRecord::new(id as EOInt);
        record.deserialize(reader);
        if record.name != "eof" {
            self.records.push(record);
        }

        Ok(())
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
