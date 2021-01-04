/// these represent the different sizes an item can be in a player's inventory
///
/// the NxN notation is Width by Height.
///
/// for example
/// ```text
/// 1x1 is a single square
/// X
///
/// 1x3 is three vertical squares
/// X
/// X
/// X
///
/// 2x4 is a full 8 squares
/// XX
/// XX
/// XX
/// XX
/// ```
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
