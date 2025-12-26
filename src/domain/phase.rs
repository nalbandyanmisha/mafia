use std::fmt::{self, Display};

impl Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Phase::*;
        let text = match self {
            Lobby(_) => "Lobby",
            Night(_) => "Night",
            Day(_) => "Day",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobbyPhase {
    Waiting,
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckPhase {
    Sheriff,
    Don,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NightPhase {
    RevealRoles,
    MafiaIntro,
    MafiaShoot,
    Investigation(CheckPhase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotingPhase {
    Nomination,
    VoteCast,
    TieDiscussion,
    TieRevote,
    Resolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayPhase {
    Morning,
    Discussion,
    Voting(VotingPhase),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Lobby(LobbyPhase),
    Night(NightPhase),
    Day(DayPhase),
}
