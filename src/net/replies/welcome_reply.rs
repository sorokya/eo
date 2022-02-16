/// reply when trying to login to a character
#[derive(Debug, Clone, Copy, PartialEq, Primitive)]
pub enum WelcomeReply {
    SelectCharacter = 1,
    EnterGame = 2,
    Busy = 3,
    LoggedIn = 4,
}

impl Default for WelcomeReply {
    fn default() -> Self {
        Self::SelectCharacter
    }
}
