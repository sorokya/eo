/// differentiates between training stat points and skill points
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum TrainType {
    Stat = 1,
    Skill = 2,
}
