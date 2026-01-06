use super::{LobbyStatus, phase::Phase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineState {
    Lobby(LobbyStatus),
    Game(Phase),
}
