use num_traits::FromPrimitive;

use crate::data::{pubs::item::*, pubs::*, *};

#[derive(Debug, Clone, Default)]
pub struct ItemRecord {
    pub id: EOInt,
    pub name: String,
    pub graphic: EOShort,
    pub r#type: ItemType,
    pub sub_type: ItemSubType,
    pub special: ItemSpecial,
    pub hp: EOShort,
    pub tp: EOShort,
    pub min_damage: EOShort,
    pub max_damage: EOShort,
    pub accuracy: EOShort,
    pub evade: EOShort,
    pub armor: EOShort,
    pub strength: EOChar,
    pub intelligence: EOChar,
    pub wisdom: EOChar,
    pub agility: EOChar,
    pub constitution: EOChar,
    pub charisma: EOChar,
    item_specific_param_1: EOThree,
    item_specific_param_2: EOChar,
    item_specific_param_3: EOChar,
    pub level_req: EOShort,
    pub class_req: EOShort,
    pub strength_req: EOShort,
    pub intelligence_req: EOShort,
    pub wisdom_req: EOShort,
    pub agility_req: EOShort,
    pub constitution_req: EOShort,
    pub charisma_req: EOShort,
    pub element: EOChar,
    pub element_power: EOChar,
    pub weight: EOChar,
    pub size: ItemSize,
}

impl ItemRecord {
    pub fn scroll_map(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn doll_graphic(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn exp_reward(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn hair_color(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn effect(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn key(&self) -> EOThree {
        self.item_specific_param_1
    }
    pub fn gender(&self) -> EOChar {
        self.item_specific_param_2
    }
    pub fn scroll_x(&self) -> EOChar {
        self.item_specific_param_2
    }
    pub fn scroll_y(&self) -> EOChar {
        self.item_specific_param_3
    }
    pub fn dual_wield_doll_graphic(&self) -> EOChar {
        self.item_specific_param_3
    }
    pub fn new(id: EOInt) -> Self {
        let mut item = Self::default();
        item.id = id;
        item
    }
}

impl PubRecord for ItemRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.name = reader.get_prefix_string();
        self.graphic = reader.get_short();
        self.r#type = match ItemType::from_u8(reader.get_char()) {
            Some(b) => b,
            None => panic!("Failed to convert byte to ItemType"),
        };
        self.sub_type = match ItemSubType::from_u8(reader.get_char()) {
            Some(b) => b,
            None => panic!("Failed to convert byte to ItemSubType"),
        };
        let special_char = reader.get_char();
        self.special = match ItemSpecial::from_u8(special_char) {
            Some(b) => b,
            None => panic!("Failed to convert byte to ItemSpecial: {}", special_char),
        };
        self.hp = reader.get_short();
        self.tp = reader.get_short();
        self.min_damage = reader.get_short();
        self.max_damage = reader.get_short();
        self.accuracy = reader.get_short();
        self.evade = reader.get_short();
        self.armor = reader.get_short();
        reader.get_char();
        self.strength = reader.get_char();
        self.intelligence = reader.get_char();
        self.wisdom = reader.get_char();
        self.agility = reader.get_char();
        self.constitution = reader.get_char();
        self.charisma = reader.get_char();
        reader.get_char();
        reader.get_char();
        reader.get_char();
        reader.get_char();
        reader.get_char();
        reader.get_char();
        self.item_specific_param_1 = reader.get_three();
        self.item_specific_param_2 = reader.get_char();
        self.item_specific_param_3 = reader.get_char();
        self.level_req = reader.get_short();
        self.class_req = reader.get_short();
        self.strength_req = reader.get_short();
        self.intelligence_req = reader.get_short();
        self.wisdom_req = reader.get_short();
        self.agility_req = reader.get_short();
        self.constitution_req = reader.get_short();
        self.charisma_req = reader.get_short();
        self.element = reader.get_char();
        self.element_power = reader.get_char();
        self.weight = reader.get_char();
        reader.get_char();
        self.size = match ItemSize::from_u8(reader.get_char()) {
            Some(b) => b,
            None => panic!("Failed to convert byte to ItemSize"),
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(EIF_DATA_SIZE + self.name.len() + 1);
        builder.add_prefix_string(&self.name);
        builder.add_short(self.graphic);
        builder.add_byte(self.r#type as EOByte);
        builder.add_byte(self.sub_type as EOByte);
        builder.add_byte(self.special as EOByte);
        builder.add_short(self.hp);
        builder.add_short(self.tp);
        builder.add_short(self.min_damage);
        builder.add_short(self.max_damage);
        builder.add_short(self.accuracy);
        builder.add_short(self.evade);
        builder.add_short(self.armor);
        builder.add_char(0);
        builder.add_char(self.strength);
        builder.add_char(self.intelligence);
        builder.add_char(self.wisdom);
        builder.add_char(self.agility);
        builder.add_char(self.constitution);
        builder.add_char(self.charisma);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_char(0);
        builder.add_three(self.item_specific_param_1);
        builder.add_char(self.item_specific_param_2);
        builder.add_char(self.item_specific_param_3);
        builder.add_short(self.level_req);
        builder.add_short(self.class_req);
        builder.add_short(self.strength_req);
        builder.add_short(self.intelligence_req);
        builder.add_short(self.wisdom_req);
        builder.add_short(self.agility_req);
        builder.add_short(self.constitution_req);
        builder.add_short(self.charisma_req);
        builder.add_char(self.element);
        builder.add_char(self.element_power);
        builder.add_char(self.weight);
        builder.add_char(0);
        builder.add_char(self.size as EOChar);
        builder.get()
    }
}
