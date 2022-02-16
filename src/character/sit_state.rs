/// describes a characters sitting state in the game
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SitState {
    Standing = 0,
    Chair = 1,
    Floor = 2,
}

impl Default for SitState {
    fn default() -> Self {
        SitState::Standing
    }
}
