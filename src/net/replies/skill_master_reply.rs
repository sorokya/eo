/// different replies when trying to learn a skill
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SkillMasterReply {
    RemoveItems = 1,
    WrongClass = 2,
}
