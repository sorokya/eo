/// reply when creating an account
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum AccountReply {
    Exists = 1,
    NotApproved = 2,
    Created = 3,
    ChangeFailed = 5,
    Changed = 6,
}
