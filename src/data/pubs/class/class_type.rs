#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum ClassType {
    Melee = 0,
    Rogue = 1,
    Magic = 2,
    Archer = 3,
    Peasant = 4,
}

impl Default for ClassType {
    fn default() -> Self {
        Self::Peasant
    }
}
