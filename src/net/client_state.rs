/// describes the state of a client
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ClientState {
    Uninitialized,
    Initialized,
    LoggedIn,
    Playing,
}
