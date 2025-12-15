use crate::engine::state::chair::Chair;
use crate::engine::state::nomination::Nomination;
use crate::engine::state::vote::Vote;
use std::collections::HashSet;
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

#[derive(Debug, Default, Clone)]
pub struct Round {
    pub nominations: Vec<Nomination>,
    pub votes: Vec<Vote>,
    pub kill: Option<Chair>,
    pub eliminated_players: Vec<Chair>,
    pub removed_players: Vec<Chair>,
}

impl Round {
    pub fn new() -> Self {
        Round {
            nominations: Vec::new(),
            votes: Vec::new(),
            kill: None,
            eliminated_players: Vec::new(),
            removed_players: Vec::new(),
        }
    }

    pub fn nominate(&mut self, by: Chair, chair: Chair) {
        self.nominations.push(Nomination { by, target: chair });
    }

    pub fn vote(&mut self, voter: Chair, target: Chair) {
        self.votes.push(Vote { voter, target });
    }

    pub fn get_nominations(&self) -> Vec<Chair> {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();

        for nomination in &self.nominations {
            let target = nomination.target;
            if !seen.contains(&target) {
                unique.push(target);
                seen.insert(target);
            }
        }

        unique
    }
}
