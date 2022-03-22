/// reply when creating an account
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum AccountReply {
    Exists = 1,
    NotApproved = 2,
    Created = 3,
    Unknown = 4,
    ChangeFailed = 5,
    Changed = 6,
}

impl Default for AccountReply {
    fn default() -> Self {
        Self::Exists
    }
}
