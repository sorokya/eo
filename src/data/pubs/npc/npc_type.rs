#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum NPCType {
    NPC = 0,
    Passive = 1,
    Aggressive = 2,
    Unknown1 = 3,
    Unknown2 = 4,
    Unknown3 = 5,
    Shop = 6,
    Inn = 7,
    Unknown4 = 8,
    Bank = 9,
    Barber = 10,
    Guild = 11,
    Priest = 12,
    Law = 13,
    Skills = 14,
    Quest = 15,
}

impl Default for NPCType {
    fn default() -> Self {
        Self::NPC
    }
}
