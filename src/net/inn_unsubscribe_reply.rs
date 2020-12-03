/// reply from the server when attempting to unsubscribe from an inn
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum InnUnsubscribeReply {
    NotCitizen = 0,
    Unsubscribed = 1,
}
