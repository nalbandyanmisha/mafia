#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day {
    Night,
    Morning,
    Noon,
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
    DeathSpeech, // optional, single actor
    Guessing,    // optional, single actor, 3 guesses
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoonActivity {
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
    Noon(NoonActivity),
    Evening(EveningActivity),
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
