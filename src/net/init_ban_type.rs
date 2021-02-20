/// describes players ban type during init sequence
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum InitBanType {
    Temporary = 0,
    Permanent = 2,
}

impl Default for InitBanType {
    fn default() -> Self {
        Self::Temporary
    }
}
