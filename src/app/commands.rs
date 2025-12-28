use clap::Parser;

/// All user-facing commands
#[derive(Debug, Parser)]
pub enum Command {
    // app level commands to map to engine commands
    Join { name: String },
    Leave { name: String },
    Start,
    AdvanceActor,
    AssignRole,
    RevokeRole,
    Warn { position: u8 },
    Pardon { position: u8 },
    Nominate { position: u8 },
    Shoot { position: u8 },
    Next,

    // app lelvel commands
    Timer { seconds: u64 },
    Quit,
}
