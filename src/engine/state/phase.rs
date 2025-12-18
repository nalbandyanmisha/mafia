use std::fmt::{self, Display};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Lobby,
    Night,
    Morning,
    Day,
    Voting,
}

#[derive(Debug, thiserror::Error)]
pub enum PhaseError {
    #[error("Cannot advance phase")]
    CannotAdvance,
}

impl Phase {
    pub fn advance_phase(&mut self) -> Result<Phase, PhaseError> {
        *self = match self {
            Phase::Lobby => Phase::Night,
            Phase::Night => Phase::Morning,
            Phase::Morning => Phase::Day,
            Phase::Day => Phase::Voting,
            Phase::Voting => Phase::Night, // cycle after Voting
        };
        Ok(self.phase())
    }

    pub fn phase(&self) -> Phase {
        *self
    }
}

impl Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
