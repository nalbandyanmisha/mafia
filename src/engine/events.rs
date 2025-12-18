use crate::engine::state::{phase::Phase, player::Player, table::chair::Chair};

#[derive(Debug)]
pub enum Event {
    PlayerJoined { player: Player, chair: Chair },
    PlayerLeft { player: Player, chair: Chair },
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
