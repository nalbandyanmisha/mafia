use crate::domain::phase::Phase;
use crate::engine::state::table::chair::Chair;

#[derive(Debug)]
pub enum Event {
    PlayerJoined { player: String, chair: Chair },
    PlayerLeft { player: String, chair: Chair },
    GameStarted,
    TurnCompleted,
    ActorAdvanced { chair: Chair },
    PlayerWarned { chair: Chair, warnings: u8 },
    PlayerPardoned { chair: Chair, warnings: u8 },
    PlayerNominated { by: Chair, target: Chair },
    PlayerKilled { chair: Chair },
    PlayerEliminated { chair: Chair },
    PlayerRemoved { chair: Chair },
    PhaseAdvanced { phase: Phase },
    NextSpeaker { chair: Chair },
    EndDay,
}
