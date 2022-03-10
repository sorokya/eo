/// describes the animation used when warping
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum WarpAnimation {
    Scroll = 1,
    Admin = 2,
}

impl Default for WarpAnimation {
    fn default() -> Self {
        WarpAnimation::Scroll
    }
}
