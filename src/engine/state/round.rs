#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoundId(pub usize);

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

#[derive(Debug, Default)]
pub struct Round {
    pub nominated_players: Vec<String>,
    pub killed_players: Vec<Option<u8>>,
}
