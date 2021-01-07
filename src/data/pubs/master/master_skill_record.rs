#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::data::{
    pubs::master::EMF_SKILL_DATA_SIZE, EOByte, EOChar, EOInt, EOShort, Serializeable,
    StreamBuilder, StreamReader,
};

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MasterSkillRecord {
    /// id of the skill to learn
    pub skill_id: EOShort,
    /// level required to learn the skill
    pub level_req: EOChar,
    /// class or base class required to learn the skill
    pub class_req: EOChar,
    /// price to learn the skill
    pub price: EOInt,
    /// id of skill player must know to learn this skill
    pub skill_req1: EOShort,
    /// id of skill player must know to learn this skill
    pub skill_req2: EOShort,
    /// id of skill player must know to learn this skill
    pub skill_req3: EOShort,
    /// id of skill player must know to learn this skill
    pub skill_req4: EOShort,
    /// number of strength points player needs to learn this skill
    pub strength_req: EOShort,
    /// number of intelligence points player needs to learn this skill
    pub intelligence_req: EOShort,
    /// number of wisdom points player needs to learn this skill
    pub wisdom_req: EOShort,
    /// number of agility points player needs to learn this skill
    pub agility_req: EOShort,
    /// number of constitution points player needs to learn this skill
    pub constitution_req: EOShort,
    /// number of charisma points player needs to learn this skill
    pub charisma_req: EOShort,
}

impl MasterSkillRecord {
    /// creates a new MasterSkillRecord with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Serializeable for MasterSkillRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.skill_id = reader.get_short();
        self.level_req = reader.get_char();
        self.class_req = reader.get_char();
        self.price = reader.get_int();
        self.skill_req1 = reader.get_short();
        self.skill_req2 = reader.get_short();
        self.skill_req3 = reader.get_short();
        self.skill_req4 = reader.get_short();
        self.strength_req = reader.get_short();
        self.intelligence_req = reader.get_short();
        self.wisdom_req = reader.get_short();
        self.agility_req = reader.get_short();
        self.constitution_req = reader.get_short();
        self.charisma_req = reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(EMF_SKILL_DATA_SIZE);
        builder.add_short(self.skill_id);
        builder.add_char(self.level_req);
        builder.add_char(self.class_req);
        builder.add_int(self.price);
        builder.add_short(self.skill_req1);
        builder.add_short(self.skill_req2);
        builder.add_short(self.skill_req3);
        builder.add_short(self.skill_req4);
        builder.add_short(self.strength_req);
        builder.add_short(self.intelligence_req);
        builder.add_short(self.wisdom_req);
        builder.add_short(self.agility_req);
        builder.add_short(self.constitution_req);
        builder.add_short(self.charisma_req);
        builder.get()
    }
}
