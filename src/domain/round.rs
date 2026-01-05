#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(pub usize);

impl RoundId {
    pub fn new(value: usize) -> Self {
        RoundId(value)
    }

    pub fn is_first(&self) -> bool {
        self.0 == 0
    }

    pub fn current(&self) -> usize {
        self.0
    }

    pub fn next(&self) -> Self {
        RoundId(self.current() + 1)
    }

    pub fn previous(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(RoundId(self.current() - 1))
        }
    }

    pub fn advance(&mut self) {
        self.0 += 1;
    }
}

impl std::fmt::Display for RoundId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RoundId> for usize {
    fn from(round_id: RoundId) -> Self {
        round_id.0
    }
}

impl From<usize> for RoundId {
    fn from(value: usize) -> Self {
        RoundId(value)
    }
}
