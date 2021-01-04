use num_traits::FromPrimitive;

use crate::data::{
    pubs::item::{ItemSize, ItemSpecial, ItemSubType, ItemType, EIF_DATA_SIZE},
    pubs::PubRecord,
    {EOByte, EOChar, EOInt, EOShort, EOThree, StreamBuilder, StreamReader},
};

/// data structure of a single eif record
#[derive(Debug, Clone, Default)]
pub struct ItemRecord {
    /// used to identify an item
    ///
    /// it is the record's index in the [super::ItemFile]
    pub id: EOInt,
    /// the name of the item
    pub name: String,
    /// used by the client to find the graphics index in the resource files
    pub graphic_id: EOShort,
    /// the item's type
    pub item_type: ItemType,
    /// the item's sub-type
    pub sub_type: ItemSubType,
    /// the item's "special"
    pub special: ItemSpecial,
    /// amount of hp restored by "Heal" items
    pub hp: EOShort,
    /// amount of tp restored by "Heal" items
    pub tp: EOShort,
    /// base min_damage stat for weapons
    pub min_damage: EOShort,
    /// base max_damage stat for weapons
    pub max_damage: EOShort,
    /// bonus accuracy points
    pub accuracy: EOShort,
    /// bonus evade points
    pub evade: EOShort,
    /// bonus armor points
    pub armor: EOShort,
    /// bonus strength points
    pub strength: EOChar,
    /// bonus intelligence points
    pub intelligence: EOChar,
    /// bonus wisdom points
    pub wisdom: EOChar,
    /// bonus agility points
    pub agility: EOChar,
    /// bonus constitution points
    pub constitution: EOChar,
    /// bonus charisma points
    pub charisma: EOChar,
    item_specific_param_1: EOThree,
    item_specific_param_2: EOChar,
    item_specific_param_3: EOChar,
    /// level required to use item
    pub level_req: EOShort,
    /// id of class (or base class) required to use item
    pub class_req: EOShort,
    /// number of strength points required to use item
    pub strength_req: EOShort,
    /// number of intelligence points required to use item
    pub intelligence_req: EOShort,
    /// number of wisdom points required to use item
    pub wisdom_req: EOShort,
    /// number of agility points required to use item
    pub agility_req: EOShort,
    /// number of constitution points required to use item
    pub constitution_req: EOShort,
    /// number of charisma points required to use item
    pub charisma_req: EOShort,
    /// the items "element" power
    pub element: EOChar,
    /// bonus points for "element" power
    pub element_power: EOChar,
    /// the item's weight value
    pub weight: EOChar,
    /// the item's size
    ///
    /// see [ItemSize] for details. This is how much space the item
    /// takes up in a player's inventory.
    pub size: ItemSize,
}

impl ItemRecord {
    /// the map id a scroll warps the player to
    pub fn scroll_map(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// graphic id used by client for equipable items
    ///
    /// used by the client to find the graphics index in the resource files
    pub fn doll_graphic_id(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// amount of experience points awarded by "EXPReward" items
    pub fn exp_reward(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// hair color index used by "HairDye" items
    pub fn hair_color(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// effect index to be played by "EffectPotion" items
    pub fn effect(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// key id for opening locked doors/chests on maps
    pub fn key(&self) -> EOThree {
        self.item_specific_param_1
    }
    /// gender requirement for equipable items
    pub fn gender(&self) -> EOChar {
        self.item_specific_param_2
    }
    /// the map x a scroll warps the player to
    pub fn scroll_x(&self) -> EOChar {
        self.item_specific_param_2
    }
    /// the map y a scroll warps the player to
    pub fn scroll_y(&self) -> EOChar {
        self.item_specific_param_3
    }
    /// graphic id used by client for equipable items
    ///
    /// used by the client to find the graphics index in the resource files
    /// this is sent to the client as a shield graphic id
    pub fn dual_wield_doll_graphic_id(&self) -> EOChar {
        self.item_specific_param_3
    }
    /// creates a new ItemRecord with the given id
    pub fn new(id: EOInt) -> Self {
        let mut item = Self::default();
        item.id = id;
        item
    }
}

impl PubRecord for ItemRecord {
    fn deserialize(&mut self, reader: &mut StreamReader) {
        self.name = reader.get_prefix_string();
        self.graphic_id = reader.get_short();
        let item_type_char = reader.get_char();
        self.item_type = match ItemType::from_u8(item_type_char) {
            Some(b) => b,
            None => panic!("Failed to convert char to ItemType: {}", item_type_char),
        };
        let sub_type_char = reader.get_char();
        self.sub_type = match ItemSubType::from_u8(sub_type_char) {
            Some(b) => b,
            None => panic!("Failed to convert char to ItemSubType: {}", sub_type_char),
        };
        let special_char = reader.get_char();
        self.special = match ItemSpecial::from_u8(special_char) {
            Some(b) => b,
            None => panic!("Failed to convert char to ItemSpecial: {}", special_char),
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
        let item_size_char = reader.get_char();
        self.size = match ItemSize::from_u8(item_size_char) {
            Some(b) => b,
            None => panic!("Failed to convert byte to ItemSize: {}", item_size_char),
        };
    }
    fn serialize(&self) -> Vec<EOByte> {
        let mut builder = StreamBuilder::with_capacity(EIF_DATA_SIZE + self.name.len() + 1);
        builder.add_prefix_string(&self.name);
        builder.add_short(self.graphic_id);
        builder.add_byte(self.item_type as EOByte);
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
