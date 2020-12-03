/// differentiates between pages of the quest book
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum QuestPage {
    Progress = 1,
    History = 2,
}
