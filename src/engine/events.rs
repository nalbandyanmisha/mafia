use crate::engine::state::{chair::Chair, player::Player};

pub enum Event {
    PlayerJoined { player: Player, chair: Chair },
    PlayerLeft { player: Player, chair: Chair },
    PlayerWarned { player: Player, chair: Chair },
    PlayerPardoned { player: Player, chair: Chair },
    PlayerNominated { player: Player, chair: Chair },
    PlayerKilled { player: Player, chair: Chair },
    PlayerElimenated { player: Player, chair: Chair },
    PhaseAdvanced,
}
