#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use crate::data::{
    pubs::npc::{NPCType, ENF_DATA_SIZE},
    EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader,
};

/// data structure of a single enf record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NPCRecord {
    /// used to identify an NPC
    ///
    /// it is the record's index in the [super::NPCFile]
    pub id: EOInt,
    /// the name of the NPC
    pub name: String,
    /// used by the client to find the npc graphics in the resource files
    pub graphic_id: EOShort,
    /// true if npc is a boss
    pub boss: bool,
    /// true if npc is a child
    pub child: bool,
    /// the npc's type
    pub npc_type: NPCType,
    pub npc_specific_param: EOShort,
    /// the npc's start hp
    pub hp: EOThree,
    /// base min_damage stat an npc can do
    pub min_damage: EOShort,
    /// base max_damage stat an npc can do
    pub max_damage: EOShort,
    /// accuracy points used for damage calculations
    pub accuracy: EOShort,
    /// evade points used for damage calculations
    pub evade: EOShort,
    /// armor points used for damage calculations
    pub armor: EOShort,
    /// element weakness
    pub element_weak: EOShort,
    /// element weakness power
    pub element_weak_power: EOShort,
    /// experience points granted when npc is killed
    pub experience: EOThree,
    unknown1: EOChar,
    unknown2: EOShort,
    unknown3: EOChar,
    unknown4: EOShort,
    unknown5: EOShort,
    unknown6: EOChar,
}

impl NPCRecord {
    /// an identifier used for shops, quests, skill masters, etc.
    pub fn vendor_id(&self) -> EOShort {
        self.npc_specific_param
    }
    /// if the monster should be protected from kill stealing
    pub fn is_ks_protected(&self) -> bool {
        self.npc_specific_param == 0
    }
    /// creates a new NPCRecord with the given id
    pub fn new(id: EOInt) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Serializeable for NPCRecord {
    fn deserialize(&mut self, reader: &StreamReader) {
        self.name = reader.get_prefix_string();
        self.graphic_id = reader.get_short();
        self.unknown1 = reader.get_char();
        self.boss = reader.get_short() == 1;
        self.child = reader.get_short() == 1;
        let type_short = reader.get_short();
        self.npc_type = match NPCType::from_u16(type_short) {
            Some(npc_type) => npc_type,
            _ => panic!("Failed to convert short to NPCType: {}", type_short),
        };
        self.npc_specific_param = reader.get_short();
        self.hp = reader.get_three();
        self.unknown2 = reader.get_short();
        self.min_damage = reader.get_short();
        self.max_damage = reader.get_short();
        self.accuracy = reader.get_short();
        self.evade = reader.get_short();
        self.armor = reader.get_short();
        self.unknown3 = reader.get_char();
        self.unknown4 = reader.get_short();
        self.unknown5 = reader.get_short();
        self.element_weak = reader.get_short();
        self.element_weak_power = reader.get_short();
        self.unknown6 = reader.get_char();
        self.experience = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(ENF_DATA_SIZE + self.name.len() + 1);
        builder.add_prefix_string(&self.name);
        builder.add_short(self.graphic_id);
        builder.add_char(self.unknown1);
        builder.add_short(self.boss as EOShort);
        builder.add_short(self.child as EOShort);
        builder.add_short(self.npc_type as EOShort);
        builder.add_short(self.npc_specific_param);
        builder.add_three(self.hp);
        builder.add_short(self.unknown2);
        builder.add_short(self.min_damage);
        builder.add_short(self.max_damage);
        builder.add_short(self.accuracy);
        builder.add_short(self.evade);
        builder.add_short(self.armor);
        builder.add_char(self.unknown3);
        builder.add_short(self.unknown4);
        builder.add_short(self.unknown5);
        builder.add_short(self.element_weak);
        builder.add_short(self.element_weak_power);
        builder.add_char(self.unknown6);
        builder.add_three(self.experience);
        builder.get()
    }
}
