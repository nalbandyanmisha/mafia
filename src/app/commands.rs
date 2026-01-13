use clap::{Parser, Subcommand};

use crate::domain::role::Role;

/// All user-facing commands
#[derive(Debug, Parser)]
pub enum Command {
    // app level commands to map to engine commands
    Join {
        name: String,
    },
    Leave {
        name: String,
    },
    Start,
    Next,
    AssignRole,
    RevokeRole,
    Warn {
        position: u8,
    },
    Pardon {
        position: u8,
    },
    Nominate {
        position: u8,
    },

    Vote {
        positions: Vec<u8>,
    },

    Shoot {
        position: u8,
    },
    Check {
        position: u8,
    },
    Guess {
        targets: Vec<u8>,
    },

    Assign {
        #[command(subcommand)]
        command: Option<AssignCommand>,
    },

    // app lelvel commands
    Timer {
        seconds: u64,
    },
    End {
        file_name: String,
    },
    Quit,
}

#[derive(Debug, Subcommand)]
pub enum NextCommand {
    Phase,
    Actor,
}

#[derive(Debug, Subcommand)]
pub enum AssignCommand {
    Player {
        name: String,
    },
    Role {
        #[arg(value_enum)]
        role: Option<Role>,
    },
}
