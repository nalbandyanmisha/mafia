use crate::engine::state::{player::Player, table::chair::Chair};

pub enum Event {
    PlayerJoined { player: Player, chair: Chair },
    PlayerLeft { player: Player, chair: Chair },
    PlayerWarned { player: Player, chair: Chair },
    PlayerPardoned { player: Player, chair: Chair },
    PlayerNominated,
    PlayerKilled { player: Player, chair: Chair },
    PlayerElimenated { player: Player, chair: Chair },
    PhaseAdvanced,
}
