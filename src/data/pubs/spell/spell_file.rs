use std::io::{prelude::*, SeekFrom};

use crate::data::{pubs::spell::*, pubs::*, *};

/// represents an EO spell file
///
/// The spell file contains a list of every spell in the game world.
/// It is saved on the server and send to clients on login.
///
/// It contains a revision number. If the server revision differs
/// from the client's revision number the file is re-downloaded.
#[derive(Debug, Default)]
pub struct SpellFile {
    pub revision: EOInt,
    length: usize,
    pub records: Vec<SpellRecord>,
}

impl SpellFile {
    /// creates a new SpellFile with no records
    pub fn new() -> Self {
        Self {
            revision: 0,
            length: 0,
            records: Vec::default(),
        }
    }
    /// returns the number of records in the SpellFile
    pub fn len(&self) -> EOShort {
        self.records.len() as EOShort
    }
    /// returns True if the SpellFile contains no records
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// reads the content of a [Read]+[Seek] implemented struct into the SpellFile
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
        let mut record = SpellRecord::new(id as EOInt);
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
    fn read_valid_esf() -> std::io::Result<()> {
        let mut esf = SpellFile::new();
        let mut records: Vec<SpellRecord> = Vec::new();

        {
            let mut record = SpellRecord::new(1);
            record.name = "Holy Light".to_string();
            record.shout = "holy light purge this darkness".to_string();
            record.min_damage = 5;
            record.max_damage = 10;
            record.spell_type = SpellType::Damage;
            record.target_restrict = SpellTargetRestrict::Opponent;
            record.target_type = SpellTargetType::Normal;
            record.tp_cost = 2;
            record.sp_cost = 4;
            record.accuracy = 10;
            record.cast_time = 3;
            record.icon_id = 1;
            record.graphic_id = 1;
            records.push(record);
        }

        {
            let mut record = SpellRecord::new(2);
            record.name = "Love Tap".to_string();
            record.shout = "have some love".to_string();
            record.hp = 100;
            record.spell_type = SpellType::Heal;
            record.target_restrict = SpellTargetRestrict::Friendly;
            record.target_type = SpellTargetType::Normal;
            record.tp_cost = 10;
            record.sp_cost = 4;
            record.cast_time = 5;
            record.icon_id = 2;
            record.graphic_id = 2;
            records.push(record);
        }

        {
            let mut record = SpellRecord::new(3);
            record.name = "eof".to_string();
            records.push(record);
        }

        let mut buf = build_fake_esf(19283, records).unwrap();
        esf.read(&mut buf)?;

        assert_eq!(esf.records[0].name, "Holy Light");
        assert_eq!(esf.records[0].shout, "holy light purge this darkness");
        assert_eq!(esf.records[0].min_damage, 5);
        assert_eq!(esf.records[0].max_damage, 10);
        assert_eq!(esf.records[0].spell_type, SpellType::Damage);
        assert_eq!(
            esf.records[0].target_restrict,
            SpellTargetRestrict::Opponent
        );
        assert_eq!(esf.records[0].target_type, SpellTargetType::Normal);
        assert_eq!(esf.records[0].tp_cost, 2);
        assert_eq!(esf.records[0].sp_cost, 4);
        assert_eq!(esf.records[0].accuracy, 10);
        assert_eq!(esf.records[0].cast_time, 3);
        assert_eq!(esf.records[0].icon_id, 1);
        assert_eq!(esf.records[0].graphic_id, 1);
        assert_eq!(esf.records[0].hp, 0);
        assert_eq!(esf.records[1].name, "Love Tap");
        assert_eq!(esf.records[1].shout, "have some love");
        assert_eq!(esf.records[1].min_damage, 0);
        assert_eq!(esf.records[1].max_damage, 0);
        assert_eq!(esf.records[1].spell_type, SpellType::Heal);
        assert_eq!(
            esf.records[1].target_restrict,
            SpellTargetRestrict::Friendly
        );
        assert_eq!(esf.records[1].target_type, SpellTargetType::Normal);
        assert_eq!(esf.records[1].hp, 100);
        assert_eq!(esf.records[1].tp_cost, 10);
        assert_eq!(esf.records[1].sp_cost, 4);
        assert_eq!(esf.records[1].accuracy, 0);
        assert_eq!(esf.records[1].cast_time, 5);
        assert_eq!(esf.records[1].icon_id, 2);
        assert_eq!(esf.records[1].graphic_id, 2);

        assert_eq!(esf.records.len(), 2);
        Ok(())
    }

    fn build_fake_esf(
        rid: EOInt,
        records: Vec<SpellRecord>,
    ) -> std::io::Result<Cursor<Vec<EOByte>>> {
        let mut buf: Cursor<Vec<EOByte>> = Cursor::new(Vec::new());
        buf.write_all(b"ESF")?;
        buf.write_all(&encode_number(rid))?;
        buf.write_all(&encode_number(records.len() as EOInt)[0..2])?;
        buf.write_all(&encode_number(1)[0..1])?;
        for record in records {
            buf.write_all(&record.serialize())?;
        }

        Ok(buf)
    }
}
