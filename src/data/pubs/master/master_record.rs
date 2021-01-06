#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::{
        master::{MasterSkillRecord, EMF_DATA_SIZE, EMF_SKILL_DATA_SIZE},
        PubRecord,
    },
    {EOByte, EOShort, StreamBuilder, StreamReader},
};

/// data structure of a single emf record
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MasterRecord {
    /// links to a vendor id
    pub vendor_id: EOShort,
    /// name of the skill master
    pub name: String,
    /// number of skills the master teaches
    pub skills_length: EOShort,
    /// skills the master teaches
    pub skills: Vec<MasterSkillRecord>,
}

impl MasterRecord {
    /// creates a new MasterRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl PubRecord for MasterRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.vendor_id = reader.get_short();
        self.name = reader.get_prefix_string();
        reader.get_byte();
        reader.get_byte();
        reader.get_byte();
        self.skills_length = reader.get_short();
        self.skills = Vec::with_capacity(self.skills_length as usize);
        for _ in 0..self.skills_length {
            let mut skill = MasterSkillRecord::new();
            skill.deserialize(reader);
            self.skills.push(skill);
        }
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(
            EMF_DATA_SIZE
                + self.name.len()
                + 1
                + EMF_SKILL_DATA_SIZE * (self.skills_length as usize),
        );

        builder.add_short(self.vendor_id);
        builder.add_prefix_string(&self.name);
        builder.add_byte(0x01);
        builder.add_byte(0xFB);
        builder.add_byte(0x01);
        builder.add_short(self.skills_length);

        for skill in &self.skills {
            builder.append(&mut PubRecord::serialize(skill));
        }

        builder.get()
    }
}
