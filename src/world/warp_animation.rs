/// describes the animation used when warping
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum WarpAnimation {
    r#None = 0,
    Scroll = 1,
    Admin = 2,
}

impl Default for WarpAnimation {
    fn default() -> Self {
        WarpAnimation::r#None
    }
}
