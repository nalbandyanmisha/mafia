use crate::engine::state::table::chair::Chair;

#[derive(Debug)]
pub enum Command {
    Join { name: String },
    Leave { name: String },
    Start,
    AdvanceActor,
    AssignRole,
    RevokeRole,
    Warn { target: Chair },
    Pardon { target: Chair },
    Nominate { target: Chair },
    Vote { targets: Vec<Chair> },
    Shoot { target: Chair },
    Check { target: Chair },
    NextPhase,
}
