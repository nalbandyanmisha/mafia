use crate::domain::phase::Phase;
use crate::engine::state::table::chair::Chair;

#[derive(Debug)]
pub enum Event {
    PlayerJoined {
        player: String,
        chair: Chair,
    },
    PlayerLeft {
        player: String,
        chair: Chair,
    },
    GameStarted,
    TurnCompleted,
    ActorAdvanced {
        chair: Chair,
    },
    PlayerWarned {
        target: Chair,
        warnings: u8,
    },
    PlayerPardoned {
        target: Chair,
        warnings: u8,
    },
    PlayerNominated {
        by: Chair,
        target: Chair,
    },
    PlayerKilled {
        target: Chair,
    },
    CheckPerformed {
        by: Chair,
        target: Chair,
        role_revealed: Option<String>,
    },
    PlayerEliminated {
        target: Chair,
    },
    PlayerRemoved {
        target: Chair,
    },
    PhaseAdvanced {
        phase: Phase,
    },
    NextSpeaker {
        chair: Chair,
    },
    EndDay,
}
