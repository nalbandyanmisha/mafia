use std::fmt::{self, Display};

impl Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Phase::*;
        let text = match self {
            Night(_) => "Night",
            Day(_) => "Day",
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    Night,
    Day,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckPhase {
    Sheriff,
    Don,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NightPhase {
    RoleAssignment,
    SheriffReveal,
    DonReveal,
    MafiaBriefing,
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
    Night(NightPhase),
    Day(DayPhase),
}

impl Phase {
    pub fn kind(&self) -> PhaseKind {
        match self {
            Phase::Night(_) => PhaseKind::Night,
            Phase::Day(_) => PhaseKind::Day,
        }
    }
}

pub enum TurnContext {
    RoleAssignment,
    DayDiscussion,
    VotingDiscussion,
    VoteCasting,
    Investigation,
}
