/// describes a facing direction for entities in the game
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum Direction {
    Down = 0,
    Left = 1,
    Up = 2,
    Right = 3,
}
