use num_traits::FromPrimitive;

use crate::data::{pubs::npc::*, pubs::*, *};

#[derive(Debug, Clone, Default)]
pub struct NPCRecord {
    pub id: EOInt,
    pub name: String,
    pub graphic_id: EOShort,
    pub boss: EOShort,
    pub child: EOShort,
    pub npc_type: NPCType,
    pub vendor_id: EOShort,
    pub hp: EOThree,
    pub min_damage: EOShort,
    pub max_damage: EOShort,
    pub accuracy: EOShort,
    pub evade: EOShort,
    pub armor: EOShort,
    pub element_weak: EOShort,
    pub element_weak_power: EOShort,
    pub experience: EOThree,
}

impl NPCRecord {
    pub fn new(id: EOInt) -> Self {
        let mut record = Self::default();
        record.id = id;
        record
    }
}

impl PubRecord for NPCRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.name = reader.get_prefix_string();
        self.graphic_id = reader.get_short();
        reader.get_char();
        self.boss = reader.get_short();
        self.child = reader.get_short();
        let type_short = reader.get_short();
        self.npc_type = match NPCType::from_u16(type_short) {
            Some(npc_type) => npc_type,
            _ => panic!("Failed to convert short to NPCType: {}", type_short),
        };
        self.vendor_id = reader.get_short();
        self.hp = reader.get_three();
        reader.get_short();
        self.min_damage = reader.get_short();
        self.max_damage = reader.get_short();
        self.accuracy = reader.get_short();
        self.evade = reader.get_short();
        self.armor = reader.get_short();
        reader.get_char();
        reader.get_short();
        reader.get_short();
        self.element_weak = reader.get_short();
        self.element_weak_power = reader.get_short();
        reader.get_char();
        self.experience = reader.get_three();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::new();
        builder.add_prefix_string(&self.name);
        builder.add_short(self.graphic_id);
        builder.add_char(0);
        builder.add_short(self.boss);
        builder.add_short(self.child);
        builder.add_short(self.npc_type as u16);
        builder.add_short(self.vendor_id);
        builder.add_three(self.hp);
        builder.add_short(0);
        builder.add_short(self.min_damage);
        builder.add_short(self.max_damage);
        builder.add_short(self.accuracy);
        builder.add_short(self.evade);
        builder.add_short(self.armor);
        builder.add_char(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(self.element_weak);
        builder.add_short(self.element_weak_power);
        builder.add_char(0);
        builder.add_three(self.experience);
        builder.get()
    }
}
