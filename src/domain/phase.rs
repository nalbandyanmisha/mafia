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
