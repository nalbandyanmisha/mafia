use std::fmt::{self, Display};

// impl Display for Phase {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use Phase::*;
//         let text = match self {
//             Night(_) => "Night",
//             Day(_) => "Day",
//         };
//         write!(f, "{text}")
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum PhaseKind {
//     Night,
//     Day,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum CheckPhase {
//     Sheriff,
//     Don,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum NightPhase {
//     RoleAssignment,
//     SheriffReveal,
//     DonReveal,
//     MafiaBriefing,
//     MafiaShoot,
//     Investigation(CheckPhase),
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum VotingPhase {
//     Nomination,
//     VoteCast,
//     TieDiscussion,
//     TieRevote,
//     Resolution,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum DayPhase {
//     Morning,
//     Discussion,
//     Voting(VotingPhase),
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Phase {
//     Night(NightPhase),
//     Day(DayPhase),
// }
//
// impl Phase {
//     pub fn kind(&self) -> PhaseKind {
//         match self {
//             Phase::Night(_) => PhaseKind::Night,
//             Phase::Day(_) => PhaseKind::Day,
//         }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Time {
    Night,
    Morning,
    Day,
    Evening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NightActivity {
    RoleAssignment, // game setup zero night only
    SheriffReveal,
    DonReveal,
    MafiaBriefing,
    MafiaShooting,
    SheriffCheck,
    DonCheck,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorningActivity {
    FinalSpeech, // optional, single actor
    Guessing,    // optional, single actor, 3 guesses
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayActivity {
    Discussion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EveningActivity {
    NominationAnnouncement,
    Voting,
    TieDiscussion,
    TieVoting,
    FinalVoting,
    FinalSpeech,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Activity {
    Night(NightActivity),
    Morning(MorningActivity),
    Day(DayActivity),
    Evening(EveningActivity),
}

impl Activity {
    pub fn time(&self) -> Time {
        match self {
            Activity::Night(_) => Time::Night,
            Activity::Morning(_) => Time::Morning,
            Activity::Day(_) => Time::Day,
            Activity::Evening(_) => Time::Evening,
        }
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct Phase {
//     pub activity: Activity,
// }
//
// impl Phase {
//     pub fn new(activity: Activity) -> Self {
//         Self { activity }
//     }
//
//     pub fn time(&self) -> Time {
//         match self.activity {
//             Activity::Night(_) => Time::Night,
//             Activity::Morning(_) => Time::Morning,
//             Activity::Day(_) => Time::Day,
//             Activity::Evening(_) => Time::Evening,
//         }
//     }
// }
