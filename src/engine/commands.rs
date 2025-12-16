use crate::engine::state::*;

pub enum Command {
    Join { name: String },
    Leave { name: String },
    Warn { chair: table::chair::Chair },
    Pardon { chair: table::chair::Chair },
    Nominate { target: table::chair::Chair },
    Shoot { chair: table::chair::Chair },
    NextPhase,
    NextSpeaker,
}
