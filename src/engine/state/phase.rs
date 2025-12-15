use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum Phase {
    #[default]
    Lobby,
    Night,
    Morning,
    Day,
    Voting,
}

impl Phase {
    pub fn next(&mut self) -> Result<(), String> {
        *self = match self {
            Phase::Lobby => Phase::Night,
            Phase::Night => Phase::Morning,
            Phase::Morning => Phase::Day,
            Phase::Day => Phase::Voting,
            Phase::Voting => Phase::Night,
        };
        Ok(())
    }
}

impl Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Phase::Lobby => "Lobby",
            Phase::Night => "Night",
            Phase::Morning => "Morning",
            Phase::Day => "Day",
            Phase::Voting => "Voting",
        };
        write!(f, "{s}")
    }
}
