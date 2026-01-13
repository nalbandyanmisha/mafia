use super::{Activity, LobbyStatus};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EngineState {
    Lobby(LobbyStatus),
    Game(Activity),
}
