use crate::engine::state::*;

pub enum Command {
    Join { name: String },
    Leave { name: String },
    Warn { chair: chair::Chair },
    Pardon { chair: chair::Chair },
    Shoot { chair: chair::Chair },
    NextPhase,
}
