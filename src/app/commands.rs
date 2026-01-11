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
    Advance,
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
    #[command(subcommand)]
    Assign(AssignCommand),

    // app lelvel commands
    Timer {
        seconds: u64,
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
