#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::io::{
    prelude::{Read, Seek},
    SeekFrom,
};

use crate::data::{
    pubs::{MasterRecord, PubRecord},
    {EOByte, EOShort, StreamReader},
};

/// represents emf (skill master) files
///
/// The master file contains a list of vendor NPCs and skills they teach
/// See [super::MasterRecord], [super::MasterSkillRecord]
/// for details on the data in each record
///
/// The file layout is:
///```text
/// "EMF" (fixed string)
/// Record*
/// {
///     id (2 bytes)
///     name (prefixed string)
///     Unknown (3 bytes)
///     skills_length (2 bytes)
///     MasterSkillRecord*trades_length
///     {
///         skill_id (2 bytes)
///         level_req (1 byte)
///         class_req (1 byte)
///         price (4 bytes)
///         skill_req1 (2 bytes)
///         skill_req2 (2 bytes)
///         skill_req3 (2 bytes)
///         skill_req4 (2 bytes)
///         strength_req (2 bytes)
///         intelligence_req (2 bytes)
///         wisdom_req (2 bytes)
///         agility_req (2 bytes)
///         constitution_req (2 bytes)
///         charisma_req (2 bytes)
///     }
/// }
///```
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MasterFile {
    pub records: Vec<MasterRecord>,
}

impl MasterFile {
    /// creates a new MasterFile with no records
    pub fn new() -> Self {
        Self {
            records: Vec::default(),
        }
    }
    /// returns the number of records in the MasterFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the ShopFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the MasterFile
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
        let mut record = MasterRecord::new();
        record.deserialize(reader);
        self.records.push(record);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{prelude::Write, Cursor};

    use super::{EOByte, EOShort, MasterFile, MasterRecord, PubRecord};

    use crate::data::pubs::MasterSkillRecord;

    #[test]
    fn read_valid_emf() -> std::io::Result<()> {
        let mut emf = MasterFile::new();

        let mut records: Vec<MasterRecord> = Vec::with_capacity(1);

        {
            let mut record = MasterRecord::new();
            record.vendor_id = 1;
            record.name = "Aeven Skill Master".to_string();
            record.skills = Vec::with_capacity(1);

            let mut skill = MasterSkillRecord::new();
            skill.skill_id = 5;
            skill.level_req = 8;
            skill.class_req = 3;
            skill.price = 1000;
            skill.skill_req1 = 1;
            skill.skill_req2 = 2;
            skill.skill_req3 = 3;
            skill.skill_req4 = 4;
            skill.strength_req = 1;
            skill.intelligence_req = 2;
            skill.wisdom_req = 3;
            skill.agility_req = 4;
            skill.constitution_req = 5;
            skill.charisma_req = 6;
            record.skills.push(skill);
            record.skills_length = record.skills.len() as EOShort;
            records.push(record);
        }

        let mut buf = build_fake_emf(records).unwrap();
        emf.read(&mut buf)?;

        assert_eq!(emf.len(), 1);
        assert_eq!(emf.records[0].vendor_id, 1);
        assert_eq!(emf.records[0].name, "Aeven Skill Master");
        assert_eq!(emf.records[0].skills_length, 1);
        assert_eq!(emf.records[0].skills[0].skill_id, 5);
        assert_eq!(emf.records[0].skills[0].level_req, 8);
        assert_eq!(emf.records[0].skills[0].class_req, 3);
        assert_eq!(emf.records[0].skills[0].price, 1000);
        assert_eq!(emf.records[0].skills[0].skill_req1, 1);
        assert_eq!(emf.records[0].skills[0].skill_req2, 2);
        assert_eq!(emf.records[0].skills[0].skill_req3, 3);
        assert_eq!(emf.records[0].skills[0].skill_req4, 4);
        assert_eq!(emf.records[0].skills[0].strength_req, 1);
        assert_eq!(emf.records[0].skills[0].intelligence_req, 2);
        assert_eq!(emf.records[0].skills[0].wisdom_req, 3);
        assert_eq!(emf.records[0].skills[0].agility_req, 4);
        assert_eq!(emf.records[0].skills[0].constitution_req, 5);
        assert_eq!(emf.records[0].skills[0].charisma_req, 6);

        Ok(())
    }

    fn build_fake_emf(records: Vec<MasterRecord>) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"EMF")?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
