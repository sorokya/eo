/// Describes the characters race
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Race {
    White = 0,
    Tan = 1,
    Yellow = 2,
    Orc = 3,
    Skeleton = 4,
    Panda = 5,
    Fish = 6,
}

impl Default for Race {
    fn default() -> Race {
        Race::White
    }
}
