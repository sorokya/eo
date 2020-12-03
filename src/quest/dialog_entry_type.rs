/// differentiates between quest
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum DialogEntryType {
    Text = 1,
    Link = 2,
}
