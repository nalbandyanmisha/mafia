use crate::engine::state::table::chair::Chair;

#[derive(Debug)]
pub enum Command {
    Join { name: String },
    Leave { name: String },
    Warn { chair: Chair },
    Pardon { chair: Chair },
    Nominate { target: Chair },
    Shoot { chair: Chair },
    NextPhase,
    NextSpeaker,
}

