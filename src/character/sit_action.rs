/// describes a characters "sit" action. Either sitting or standing
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum SitAction {
    Sit = 1,
    Stand = 2,
}
