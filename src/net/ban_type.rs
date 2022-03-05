/// describes players ban type during init sequence
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum BanType {
    Temporary = 0,
    Permanent = 2,
}

impl Default for BanType {
    fn default() -> Self {
        Self::Temporary
    }
}
