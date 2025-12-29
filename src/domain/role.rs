use clap::ValueEnum;
use std::fmt::{self, Display};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Copy, ValueEnum)]
pub enum Role {
    #[default]
    Citizen,
    Mafia,
    Don,
    Sheriff,
}

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::Citizen => "Citizen",
            Role::Mafia => "Mafia",
            Role::Don => "Don",
            Role::Sheriff => "Sheriff",
        };
        write!(f, "{role_str}")
    }
}
