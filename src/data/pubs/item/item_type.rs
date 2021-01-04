/// an item's type
///
/// Used to categorize items based on functionality.
/// Tell's the server/client what an item does and how it can be used.
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum ItemType {
    Static = 0,
    Unknown1 = 1,
    Money = 2,
    Heal = 3,
    Teleport = 4,
    Spell = 5,
    EXPReward = 6,
    StatReward = 7,
    SkillReward = 8,
    Key = 9,
    Weapon = 10,
    Shield = 11,
    Armor = 12,
    Hat = 13,
    Boots = 14,
    Gloves = 15,
    Accessory = 16,
    Belt = 17,
    Necklace = 18,
    Ring = 19,
    Armlet = 20,
    Bracer = 21,
    Beer = 22,
    EffectPotion = 23,
    HairDye = 24,
    CureCurse = 25,
}

impl Default for ItemType {
    fn default() -> Self {
        Self::Static
    }
}
