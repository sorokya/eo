/// reply when creating an account
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum AccountReply {
    Exists = 1,
    NotApproved = 2,
    Created = 3,
    ChangeFailed = 5,
    Changed = 6,
    // TODO: Figure out real value
    Continue = 1000,
}

impl Default for AccountReply {
    fn default() -> Self {
        Self::Exists
    }
}
