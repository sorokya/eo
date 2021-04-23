/// reply when logging into game server
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum LoginReply {
    WrongUsername = 1,
    WrongPassword = 2,
    OK = 3,
    AccountBanned = 4,
    LoggedIn = 5,
    Busy = 6,
}
