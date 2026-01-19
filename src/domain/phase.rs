use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Day {
    Night,
    Morning,
    Noon,
    Evening,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Day::*;
        match self {
            Night => write!(f, "Night"),
            Morning => write!(f, "Mornig"),
            Noon => write!(f, "Noon"),
            Evening => write!(f, "Evening"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NightActivity {
    RoleAssignment, // game setup zero night only
    SheriffReveal,
    DonReveal,
    MafiaBriefing,
    MafiaShooting,
    SheriffCheck,
    DonCheck,
}

impl fmt::Display for NightActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NightActivity::*;
        match self {
            RoleAssignment => write!(f, "Role Assignment"),
            SheriffReveal => write!(f, "Sheriff Reveal"),
            DonReveal => write!(f, "Don Reveal"),
            MafiaBriefing => write!(f, "Mafia Briefing"),
            MafiaShooting => write!(f, "Mafia Shooting"),
            SheriffCheck => write!(f, "Sheriff Check"),
            DonCheck => write!(f, "Don Check"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MorningActivity {
    DeathSpeech, // optional, single actor
    Guessing,    // optional, single actor, 3 guesses
}

impl fmt::Display for MorningActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use MorningActivity::*;
        match self {
            DeathSpeech => write!(f, "Death Speech"),
            Guessing => write!(f, "Guessing"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NoonActivity {
    Discussion,
}

impl fmt::Display for NoonActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use NoonActivity::*;
        match self {
            Discussion => write!(f, "Discussion"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EveningActivity {
    Voting,
    TieDiscussion,
    TieVoting,
    FinalVoting,
    FinalSpeech,
}

impl fmt::Display for EveningActivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EveningActivity::*;
        match self {
            Voting => write!(f, "Voting"),
            TieDiscussion => write!(f, "Tie Discussion"),
            TieVoting => write!(f, "Tie Voting"),
            FinalVoting => write!(f, "Final Voting"),
            FinalSpeech => write!(f, "Final Speech"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Activity {
    Night(NightActivity),
    Morning(MorningActivity),
    Noon(NoonActivity),
    Evening(EveningActivity),
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Activity::*;
        match self {
            Night(activity) => write!(f, "{activity}"),
            Morning(activity) => write!(f, "{activity}"),
            Noon(activity) => write!(f, "{activity}"),
            Evening(activity) => write!(f, "{activity}"),
        }
    }
}

impl Activity {
    pub fn daytime(&self) -> Day {
        match self {
            Activity::Night(_) => Day::Night,
            Activity::Morning(_) => Day::Morning,
            Activity::Noon(_) => Day::Noon,
            Activity::Evening(_) => Day::Evening,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct DayIndex(pub usize);

impl DayIndex {
    pub fn new(value: usize) -> Self {
        DayIndex(value)
    }

    pub fn is_first(&self) -> bool {
        self.0 == 0
    }

    pub fn is_second(&self) -> bool {
        self.0 == 1
    }

    pub fn current(&self) -> usize {
        self.0
    }

    pub fn next(&self) -> Self {
        DayIndex(self.current() + 1)
    }

    pub fn previous(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(DayIndex(self.current() - 1))
        }
    }

    pub fn advance(&mut self) {
        self.0 += 1;
    }
}

impl std::fmt::Display for DayIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DayIndex> for usize {
    fn from(round_id: DayIndex) -> Self {
        round_id.0
    }
}

impl From<usize> for DayIndex {
    fn from(value: usize) -> Self {
        DayIndex(value)
    }
}
