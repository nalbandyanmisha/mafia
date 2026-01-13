use serde::Serialize;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LobbyStatus {
    Waiting,
    Ready,
}
