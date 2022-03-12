#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::npc::NPCRecord, EOByte, EOChar, EOInt, EOShort, Serializeable, StreamBuilder,
    StreamReader,
};

/// represents enf files
///
/// The npc file contains a list of every npc
/// in the game. See [NPCRecord] for details on the data
/// in each record.
///
/// The file layout is:
///```text
/// "ENF" (fixed string)
/// hash (4 bytes)
/// Length (2 bytes)
/// Unknown (1 bytes) (always 0)
/// Record*Length
/// {
///     name (prefixed string)
///     graphic_id (2 bytes)
///     unknown (1 byte)
///     boss (2 bytes)
///     child (2 bytes)
///     type (2 bytes)
///     vendor_id (2 bytes)
///     hp (3 bytes)
///     unknown (2 bytes)
///     min_damage (2 bytes)
///     max_damage (2 bytes)
///     accuracy (2 bytes)
///     evade (2 bytes)
///     armor (2 bytes)
///     unknown (2 bytes)
///     unknown (1 byte)
///     unknown (1 byte)
///     element_weak (2 bytes)
///     element_weak_power (2 bytes)
///     unknown (1 byte)
///     experience (3 bytes)
/// }
///```
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NPCFile {
    pub hash: [EOByte; 4],
    length: usize,
    unknown: EOChar,
    pub records: Vec<NPCRecord>,
}

impl NPCFile {
    /// creates a new NPCFile with no records
    pub fn new() -> Self {
        Self {
            hash: [0, 0, 0, 0],
            length: 0,
            unknown: 0,
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
        let reader = StreamReader::new(&data_buf);
        self.deserialize(&reader);
        Ok(())
    }

    fn read_record(&mut self, id: usize, reader: &StreamReader) {
        let mut record = NPCRecord::new(id as EOInt);
        record.deserialize(reader);
        self.records.push(record);
    }
}

impl Serializeable for NPCFile {
    fn deserialize(&mut self, reader: &StreamReader) {
        reader.seek(3);
        self.hash = [
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
            reader.get_byte(),
        ];
        self.length = reader.get_short() as usize;
        self.unknown = reader.get_char();
        self.records = Vec::with_capacity(self.length + 1);
        for id in 1..self.length + 1 {
            self.read_record(id, reader);
        }
    }

    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_string("ENF");
        builder.append(&mut self.hash.to_vec());
        builder.add_short(self.length as EOShort);
        builder.add_char(self.unknown);
        for record in &self.records {
            builder.append(&mut record.serialize());
        }
        builder.get()
    }
}

#[cfg(test)]
mod tests {
    use super::{EOByte, EOInt, NPCFile, NPCRecord, Serializeable};
    use crate::data::{encode_number, pubs::NPCType};
    use std::io::{prelude::Write, Cursor};

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
            record.npc_specific_param = 7;
            record.boss = true;
            record.child = false;
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

        assert_eq!(enf.records.len(), 2);
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
        assert_eq!(enf.records[0].npc_specific_param, 7);
        assert_eq!(enf.records[0].boss, true);
        assert_eq!(enf.records[0].child, false);
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
