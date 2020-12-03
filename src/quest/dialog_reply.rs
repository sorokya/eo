/// differentiates between the player clicking a link or the the "OK" button
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum DialogReply {
    OK = 1,
    Link = 2,
}
