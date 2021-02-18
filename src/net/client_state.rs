/// describes the state of a client
pub enum ClientState {
  Uninitialized,
  Initialized,
  LoggedIn,
  Playing,
}