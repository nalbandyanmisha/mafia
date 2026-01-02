use clap::ValueEnum;
use std::str::FromStr;
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Copy, ValueEnum)]
pub enum Role {
    #[default]
    Citizen,
    Mafia,
    Don,
    Sheriff,
}

impl From<Role> for &'static str {
    fn from(r: Role) -> Self {
        match r {
            Role::Citizen => "citizen",
            Role::Mafia => "mafia",
            Role::Don => "don",
            Role::Sheriff => "sheriff",
        }
    }
}

impl From<Role> for String {
    fn from(r: Role) -> Self {
        <&str>::from(r).to_string()
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "citizen" => Ok(Role::Citizen),
            "mafia" => Ok(Role::Mafia),
            "don" => Ok(Role::Don),
            "sheriff" => Ok(Role::Sheriff),
            other => Err(format!("Invalid role: {other}")),
        }
    }
}
