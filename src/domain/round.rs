#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(pub usize);

impl RoundId {
    pub fn new(value: usize) -> Self {
        RoundId(value)
    }

    pub fn is_first(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for RoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl RoundId {
    pub fn next(&self) -> RoundId {
        RoundId(self.0 + 1)
    }
}

impl From<RoundId> for usize {
    fn from(round_id: RoundId) -> Self {
        round_id.0
    }
}
