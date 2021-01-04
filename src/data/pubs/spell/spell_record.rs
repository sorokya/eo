use num_traits::FromPrimitive;

use crate::data::{pubs::spell::*, pubs::*, *};

#[derive(Debug, Clone, Default)]
pub struct SpellRecord {
    pub id: EOInt,
    pub name: String,
    pub shout: String,
    pub icon_id: EOShort,
    pub graphic_id: EOShort,
    pub tp_cost: EOShort,
    pub sp_cost: EOShort,
    pub cast_time: EOChar,
    pub spell_type: SpellType,
    pub element: EOChar,
    pub element_power: EOShort,
    pub target_restrict: SpellTargetRestrict,
    pub target_type: SpellTargetType,
    pub min_damage: EOShort,
    pub max_damage: EOShort,
    pub accuracy: EOShort,
    pub hp: EOShort,
}

impl SpellRecord {
    pub fn new(id: EOInt) -> Self {
        let mut record = Self::default();
        record.id = id;
        record
    }
}

impl PubRecord for SpellRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        let name_length = reader.get_char();
        let shout_length = reader.get_char();
        self.name = reader.get_fixed_string(name_length as usize);
        self.shout = reader.get_fixed_string(shout_length as usize);
        self.icon_id = reader.get_short();
        self.graphic_id = reader.get_short();
        self.tp_cost = reader.get_short();
        self.sp_cost = reader.get_short();
        self.cast_time = reader.get_char();
        reader.get_char();
        reader.get_char();
        let type_three = reader.get_three();
        self.spell_type = match SpellType::from_u32(type_three) {
            Some(spell_type) => spell_type,
            _ => panic!("Failed to convert three to SpellType: {}", type_three),
        };
        self.element = reader.get_char();
        self.element_power = reader.get_short();
        let target_restrict_char = reader.get_char();
        self.target_restrict = match SpellTargetRestrict::from_u8(target_restrict_char) {
            Some(target_restrict) => target_restrict,
            _ => panic!(
                "Failed to convert char to SpellTargetRestrict: {}",
                target_restrict_char
            ),
        };
        let target_type_char = reader.get_char();
        self.target_type = match SpellTargetType::from_u8(target_type_char) {
            Some(target_type) => target_type,
            _ => panic!(
                "Failed to convert char to SpellTargetType: {}",
                target_type_char
            ),
        };
        reader.get_char();
        reader.get_char();
        reader.get_short();
        self.min_damage = reader.get_short();
        self.max_damage = reader.get_short();
        self.accuracy = reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_char();
        self.hp = reader.get_short();
        reader.get_short();
        reader.get_char();
        reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_short();
        reader.get_short();
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder =
            StreamBuilder::with_capacity(ESF_DATA_SIZE + self.name.len() + self.shout.len() + 2);
        builder.add_char(self.name.len() as EOChar);
        builder.add_char(self.shout.len() as EOChar);
        builder.add_string(&self.name);
        builder.add_string(&self.shout);
        builder.add_short(self.icon_id);
        builder.add_short(self.graphic_id);
        builder.add_short(self.tp_cost);
        builder.add_short(self.sp_cost);
        builder.add_char(self.cast_time);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_three(self.spell_type as EOThree);
        builder.add_char(self.element);
        builder.add_short(self.element_power);
        builder.add_char(self.target_restrict as EOChar);
        builder.add_char(self.target_type as EOChar);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_short(0);
        builder.add_short(self.min_damage);
        builder.add_short(self.max_damage);
        builder.add_short(self.accuracy);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_char(0);
        builder.add_short(self.hp);
        builder.add_short(0);
        builder.add_char(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.add_short(0);
        builder.get()
    }
}
