/// differentiates if you're inviting someone or requesting to join
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum PartyRequestType {
    Join = 0,
    Invite = 1,
}
