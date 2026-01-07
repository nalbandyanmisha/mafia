use super::{Activity, LobbyStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Lobby(LobbyStatus),
    Game(Activity),
}
