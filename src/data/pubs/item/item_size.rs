#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum ItemSize {
    Size1x1 = 0,
    Size1x2 = 1,
    Size1x3 = 2,
    Size1x4 = 3,
    Size2x1 = 4,
    Size2x2 = 5,
    Size2x3 = 6,
    Size2x4 = 7,
}

impl Default for ItemSize {
    fn default() -> Self {
        Self::Size1x1
    }
}
