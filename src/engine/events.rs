use crate::domain::{phase::Phase, position::Position, role::Role};

#[derive(Debug)]
pub enum Event {
    PlayerJoined {
        name: String,
    },
    PlayerLeft {
        name: String,
    },
    GameStarted,
    TurnCompleted,
    ActorAdvanced {
        chair: Position,
    },
    PlayerWarned {
        target: Position,
        warnings: u8,
    },
    PlayerPardoned {
        target: Position,
        warnings: u8,
    },
    PlayerNominated {
        by: Position,
        target: Position,
    },
    PlayerKilled {
        target: Position,
    },
    CheckPerformed {
        by: Position,
        target: Position,
        role_revealed: Option<Role>,
    },
    PlayerEliminated {
        target: Position,
    },
    PlayerRemoved {
        target: Position,
    },
    PhaseAdvanced {
        phase: Phase,
    },
    NextSpeaker {
        chair: Position,
    },
    EndDay,
}
