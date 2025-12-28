use crate::{domain::role::Role, engine::state::table::chair::Chair};

#[derive(Debug)]
pub enum Command {
    Join { name: String },
    Leave { name: String },
    Start,
    AdvanceActor,
    AssignRole,
    RevokeRole,
    Warn { chair: Chair },
    Pardon { chair: Chair },
    Nominate { target: Chair },
    Shoot { chair: Chair },
    NextPhase,
}
