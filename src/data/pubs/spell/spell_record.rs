#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use num_traits::FromPrimitive;

use crate::data::{
    pubs::spell::{SpellTargetRestrict, SpellTargetType, SpellType, ESF_DATA_SIZE},
    EOByte, EOChar, EOInt, EOShort, EOThree, Serializeable, StreamBuilder, StreamReader,
};

/// data structure of a single esf record
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpellRecord {
    /// used to identify a spell
    ///
    /// it is the record's index in the [super::SpellFile]
    pub id: EOInt,
    /// the spell's name
    pub name: String,
    /// "shout" that appears above casters head when spell is used
    pub shout: String,
    /// graphic index for the spell's icon displayed in the players spell tab
    pub icon_id: EOShort,
    /// used by the client to find the spell's graphics in the resource files
    pub graphic_id: EOShort,
    /// amount of tp consumed when spell is used
    pub tp_cost: EOShort,
    /// amount of sp consumed when spell is used
    pub sp_cost: EOShort,
    /// time it takes for a spell to cast
    ///
    /// as far as I can tell the unit is 470ms (from eoserv) + the client has
    /// a global cool down of 600ms that may also be enforced by the server.
    pub cast_time: EOChar,
    /// the spell's type
    pub spell_type: SpellType,
    /// the spell's element
    pub element: EOChar,
    /// the spell's element power
    pub element_power: EOShort,
    /// restricts the target of the spell by target attributes
    pub target_restrict: SpellTargetRestrict,
    /// specifies the type of target
    pub target_type: SpellTargetType,
    /// base min_damage the spell does
    pub min_damage: EOShort,
    /// base max_damage the spell does
    pub max_damage: EOShort,
    /// accuracy points used to calculate damage the spell does
    pub accuracy: EOShort,
    /// amount of hp the spell restores
    pub hp: EOShort,
}

impl SpellRecord {
    /// creates a new SpellRecord with the given id
    pub fn new(id: EOInt) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

impl Serializeable for SpellRecord {
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
