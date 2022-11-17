/// describes the state of a client
#[derive(Debug, Eq, PartialEq, Copy)]
pub enum ClientState {
    Uninitialized,
    Initialized,
    LoggedIn,
    Playing,
}
